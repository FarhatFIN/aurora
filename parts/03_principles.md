## PART 2 — Operating Principles & Working Method

### §2.1 The engineering loop

Every unit of work — from "add one CSS property" to "build the flexbox layout engine" —
goes through the same loop. Skipping steps is how engines rot before they are done.

1. **Pick one task.** From the WBS (Part 6) or the current milestone's exit checklist.
   One. If the task feels too big to state in one sentence, decompose it first.
2. **Read before writing.** Read the subsystem spec in Part 5, the standard sections the
   task depends on, and the existing code that will be touched — callers and callees.
   Never edit code you have not read.
3. **Write the failing test first** where the task is behavioral (§8). For parser and
   engine work this is usually a golden file or a unit test with a tiny input and a
   tiny expected structure.
4. **Implement the smallest correct version.** Smallest means: no configuration options,
   no speculative generalization, no abstraction for a second caller that does not exist.
5. **Verify.** Run the new test, the subsystem's test module, and the fast test tier
   (§8.8). If a previously passing test broke, stop and understand why before continuing.
6. **Refactor in the same task, not later.** Leave the code cleaner than you found it,
   but never mix a pure refactor into a behavior-changing commit (§9.9).
7. **Record.** Update `PROGRESS.md`, tick the WBS item, write the session report entry
   (§11.4), commit (§11.3).

The loop is also the antidote to the two classic AI failure modes: building a beautiful
thing nobody asked for (skip step 1 and you get this), and shipping code that was never
run (skip step 5 and you get this).

### §2.2 The spec-first rule

For any behavior defined by a standard, the standard is read **before** the code is
written, not after the code disagrees with reality. Working rules:

- Cite the standard by section anchor in code comments only when the constraint is
  non-obvious (`// HTML §13.2.6.5: "in body" insertion mode, start tag "table"`).
  Do not decorate obvious code with spec citations.
- When the standard is enormous (HTML is), read only the section in force plus its
  immediate dependencies; keep a `docs/spec-notes/<topic>.md` file with a 5–15 line
  summary of what you read, so the next session does not re-read 400 lines to relearn it.
- When two standards disagree (it happens), implement the one the web platform actually
  converged on (usually the WHATWG living standard), record the conflict in an ADR.
- When the standard specifies an algorithm in numbered steps, translate the numbered
  steps into code in the same order with the same names. Deviate only with a comment
  explaining the deviation and why it is safe. This single rule is most of what makes
  spec-conformant code reviewable.

### §2.3 Vertical slices over horizontal layers

Do not build the "perfect URL module" for three months before the first byte of HTML is
parsed. Each milestone in Part 7 is a **vertical slice**: it ends with a demo that does
something visible end to end, with earlier subsystems at their crudest acceptable level.

Rules that follow from this:

- A slice may use a deliberately crude placeholder (e.g., before §5.11 exists, measure
  text with a fixed-width heuristic) **only if** the placeholder is recorded in
  `PROGRESS.md` under "Standing placeholders" with a pointer to the WBS item that
  replaces it. An unrecorded placeholder is a bug (§9.10).
- When a later milestone deepens a subsystem, the earlier slice's tests must keep
  passing — they are the contract.
- Never widen a slice's scope to make an implementation prettier. The WBS exists so
  that completeness is tracked per item, not per mood.

### §2.4 Testing discipline

The full strategy is Part 8; these are the non-negotiables that apply to every task:

- New behavior gets at least one happy-path test and one meaningful edge-case test
  before the task is called done. An edge case is empty input, malformed input, an
  empty collection, or a boundary value — pick the one most likely to actually break.
- A bug fix lands with a regression test that fails without the fix.
- You ran the tests. "They should pass" is not a state that exists.
- A test that fails intermittently or by environment is a defect of equal rank to a
  product bug. Fix it or quarantine it with an explanatory issue comment in the test
  file and a `PROGRESS.md` entry. Never delete a failing test to make a task pass.
- Expected outputs in tests are written by hand from the standard (or from a first
  implementation you have manually verified), never by regenerating the golden file
  from the code until you have eyeballed the diff.

### §2.5 Autonomy and escalation

You operate autonomously. Act without asking when the action is reversible, in scope,
and the answer is discoverable by reading code, docs, standards, or running a command —
which covers roughly 95% of decisions in this project, including all API design inside
a subsystem, all internal data structures, all test design, and all refactoring.

Stop and escalate (write the blocking question into `PROGRESS.md` under "Open
questions", then continue with the best available default) when:

- A choice would be destructive or hard to reverse (deleting a milestone's worth of
  work, replacing a design that shipped in an earlier milestone) — prefer recording an
  ADR and doing the reversible version.
- Two subsystems' contracts genuinely cannot both be satisfied — this is an
  architecture bug in this document; implement the smaller deviation and file it.
- You need credentials, network access you do not have, or a platform you cannot build
  for — record the limitation, degrade gracefully, keep moving.

Never stall. An autonomous agent that waits for a human that is not watching has
failed twice: once by blocking, once by hiding the blockage. The protocol is: make the
best decision, write down that you made it and why, and continue.

### §2.6 The complexity budget

Every mechanism earns its complexity. Before adding any of — a trait, a generic
parameter, a callback, a new crate, a background thread, a cache, a configuration knob —
answer in one written sentence: *what concretely breaks without it?* If the honest
answer is "nothing today", do not add it. Concretely:

- Maximum two levels of generic indirection. Beyond that, prefer `enum` dispatch.
- No `dyn Trait` across crate boundaries unless the crate map (§4.3) marks it as a
  seam (currently: the renderer backend and the platform layer only).
- A subsystem may have at most one background thread unless its Part 5 section says
  otherwise; communication is by message queue (§4.4), never by shared mutable state.
- No public `async` API in the engine. Concurrency is explicit and thread-based
  (§3.7). If an API would be nicer async, that is a signal to restructure the pipeline,
  not to add a runtime.

### §2.7 Honesty about hacks

When a deadline-equivalent pressure exists (a milestone demo needs a crude path), you
may take a shortcut **only** if it is marked in the code with a standard tag:

```rust
// AURORA-SHORTCUT(milestone): describes what is faked, and links the WBS item
// or PROGRESS.md entry that will replace it.
```

A shortcut without a tag and a tracking pointer is treated by the self-review checklist
(§9.7) as broken code. The count of open `AURORA-SHORTCUT` tags is part of every
milestone exit report, and M13 requires it to be zero (§7.15).

### §2.8 Idempotence and recoverability

Design every operation to survive being interrupted and re-run:

- Builds and tests are naturally idempotent; keep them that way (no tests that depend
  on wall-clock time without a fixed clock injection).
- The HTTP disk cache and storage files use write-to-temp-then-rename so a crash leaves
  the old value, not a corrupt one (§5.17).
- `PROGRESS.md` updates are append-mostly; a partially written session report is
  resumed, not restarted (§12.5).
- Long-running commands (the full test suite, pixel baselines) are re-runnable; their
  outputs are derived artifacts, never hand-edited.

### §2.9 Session reporting contract

Every working session ends with a report appended to `PROGRESS.md` (§12.1) containing,
in this order: (1) milestone and WBS items touched; (2) what was implemented; (3) what
was tested and the actual result counts (e.g., `412 passed, 0 failed`); (4) decisions
made and their one-line justifications, each cross-referenced to an ADR if
architecture-level; (5) new open shortcuts, placeholders, or debts, each with a
tracking pointer; (6) the exact next task, chosen per §12.3. A session that cannot
fill item 6 did not finish; choose the task before closing the report.

### §2.10 Worked example: the loop applied

A small feature traced through §2.1 end to end, so the method is concrete.
Task: "`text-decoration-style: wavy`" (WBS §6.3, text group, owner §5.16.2).

1. **Pick one task** — exactly this property; resist adding `text-underline-offset`
   while you are in the file.
2. **Read first**: CSS Text Decoration 4 §5.2 (the `wavy` keyword and its
   rendering guidance), then the existing decoration code paths in
   `aurora_paint` (where `underline`/`line-through` are already drawn), and the
   text-decoration tests.
3. **Test first**: the pixel case — a page with `text-decoration-style: wavy`
   over a 40-char string; the parser cases — valid (`wavy`), invalid (`curly`),
   and inherited-vs-set cascade behavior. Expected pixels derived from the
   spec's amplitude/period guidance (§5.11 metrics feed the wave).
4. **Smallest correct version**: the registry already accepts `wavy`? Then the
   work is one computed-value field plus the raster path — a stroked sine
   approximation with the dash machinery reused.
5. **Verify**: new tests pass; the existing decoration pixel corpus is
   byte-identical (the change is additive); fast tier green.
6. **Refactor**: the three decoration styles now share a "decoration line
   painter" — extracted here, since this was the third caller.
7. **Record**: §6.3 checkbox ticked; session report entry with the pixel-diff
   count; commit `paint: add wavy text-decoration-style (WBS §6.3 text/wavy)`.

Total: one session, one commit, four tests, zero scope creep. Scale the same
shape up for milestone-sized work — the steps do not change; only the number
of loop iterations does.
