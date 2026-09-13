# MASTER PROMPT — Build a Web Browser Engine and Browser From Scratch

**Codename:** AURORA (you may rename it; if you do, rename it consistently everywhere in the first session and never again).
**Audience:** an autonomous AI software engineer (or a human team) with full authority over a fresh, empty repository.
**Document class:** project constitution + full technical specification + work breakdown structure + session protocol.
**Length:** ~10,000 lines. This is deliberate. Read the consumption rules in Part 0 before starting.

---

## PART 0 — How to Use This Prompt

### §0.1 What this document is

This is not a sketch. It is the complete operating contract for building a web browser
engine and a desktop browser application from first principles. It contains:

1. **Mission and scope** (Part 1) — what you are building and, just as important, what you are NOT building.
2. **Working method** (Part 2) — how to make decisions, in what order, and how to verify yourself.
3. **Technology policy** (Part 3) — the language, dependencies, and tooling decisions, already made so you do not waste sessions relitigating them.
4. **System architecture** (Part 4) — the crate map, data flow, threading model, and the contracts between subsystems.
5. **Subsystem specifications** (Part 5) — one section per subsystem: goals, public API sketches, algorithms, invariants, pitfalls, and definitions of done.
6. **Work breakdown structure** (Part 6) — the exhaustive, itemized task checklist: every HTML element, every CSS property, every DOM interface, every JavaScript builtin, each with its own acceptance criteria. This is the part you tick off, item by item, over many sessions.
7. **Milestones** (Part 7) — fourteen milestones, each ending in a demonstrable, tested artifact.
8. **Testing strategy** (Part 8) — the test pyramid, golden files, pixel tests, and Web Platform Tests adoption.
9. **Code quality standard** (Part 9) — naming, error handling, forbidden patterns, review checklist.
10. **Performance budgets** (Part 10) — numeric limits you must respect and measure.
11. **Deliverables and reporting** (Part 11) — what exists in the repository when you are done, and what every session report must contain.
12. **Session protocol** (Part 12) — how to survive context loss, how to resume, and how to keep a durable progress ledger.
13. **Appendices** (A–F) — reference tables you will consult constantly: named colors, character references, HTTP headers, MIME types, CSS units, keyboard maps.

### §0.2 Who it is for

It is written for an AI agent that can: create and edit files, run shell commands,
compile and test code, read standards documents online or offline, and persist state
between sessions. It assumes no human is watching in real time. It assumes the agent
may be killed at any moment and must therefore leave the repository in a state where
any successor session can resume exactly where it stopped.

### §0.3 How to consume 10,000 lines without drowning

You will not hold this document in working memory. Follow this ritual:

- **First session, once:** read Parts 0–4 and Part 7 fully (roughly 2,500 lines). Skim Part 5. You now know the shape of the machine.
- **Every session after:** read Part 12 (session protocol, short), the current milestone section in Part 7, the subsystem spec for whatever you are working on, and the relevant slice of Part 6. That is 300–800 lines per session, not 10,000.
- **On demand:** treat Part 6 and the Appendices as lookup tables, like documentation. Open the exact entries you need; ignore the rest.
- **Never** let "I haven't read all of it" become an excuse to improvise architecture. If you are unsure whether a decision is covered, search this file first (`grep -n "§" | grep -i <keyword>`), then decide, then record the decision in an ADR (§4.9) so the next session inherits it.

### §0.4 Precedence and conflict resolution

When instructions conflict, resolve in this order:

1. **Standards win.** WHATWG HTML, WHATWG URL, WHATWG Fetch, CSSWG specs, and ECMA-262 are the ground truth for *behavior*. This document is the ground truth for *architecture, process, scope, and quality*. If this document contradicts a standard on behavior, follow the standard, then open an ADR noting the discrepancy and propose a fix to this document.
2. **This document wins** over your prior habits, over brevity, and over convenience.
3. **Newer explicit user instruction** wins over everything; when it arrives, update `PROGRESS.md` and, if durable, this document.
4. **The WBS (Part 6) is a work-aid, not a spec.** Its per-item notes summarize behavior; where a summary and the standard disagree, the standard is right. Flag systematic errors you find — do not silently copy them into code.

---

## Table of Contents

- **PART 0 — How to Use This Prompt**
  - §0.1 What this document is
  - §0.2 Who it is for
  - §0.3 How to consume 10,000 lines without drowning
  - §0.4 Precedence and conflict resolution
- **PART 1 — Mission, Role & Scope**
  - §1.1 Identity and product definition
  - §1.2 What "from scratch" means here
  - §1.3 Product scope
  - §1.4 Non-goals (read twice)
  - §1.5 Stretch goals, explicitly deferred
  - §1.6 Definition of success
- **PART 2 — Operating Principles & Working Method**
  - §2.1 The engineering loop
  - §2.2 The spec-first rule
  - §2.3 Vertical slices over horizontal layers
  - §2.4 Testing discipline
  - §2.5 Autonomy and escalation
  - §2.6 The complexity budget
  - §2.7 Honesty about hacks
  - §2.8 Idempotence and recoverability
  - §2.9 Session reporting contract
  - §2.10 Worked example: the loop applied
- **PART 3 — Technology Policy**
  - §3.1 Primary language and rationale
  - §3.2 The tiered dependency policy
  - §3.3 Approved dependency list
  - §3.4 Build system and toolchain
  - §3.5 Platform targets
  - §3.6 Graphics backend strategy
  - §3.7 Concurrency model
- **PART 4 — System Architecture**
  - §4.1 The ten-thousand-foot view
  - §4.2 The pipeline: data flow from URL to pixels
  - §4.3 Workspace and crate map
  - §4.4 Threading and synchronization model
  - §4.5 Memory strategy and ownership across the pipeline
  - §4.6 Error taxonomy
  - §4.7 Process model roadmap (single-process today, multi-process tomorrow)
  - §4.8 Compatibility and versioning policy
  - §4.9 Architecture Decision Records (ADRs)
  - §4.10 Crate skeleton template
- **PART 5 — Subsystem Specifications**
  - §5.1 URL parsing and normalization
  - §5.2 HTTP network stack
  - §5.3 TLS and certificate policy
  - §5.4 The HTML tokenizer
  - §5.5 HTML tree construction
  - §5.6 DOM core and events
  - §5.7 CSS tokenizer and parser
  - §5.8 Selectors, cascade, and computed values
  - §5.9 Layout I — boxes, block, and inline formatting
  - §5.10 Layout II — flexbox and grid
  - §5.11 Text, fonts, and shaping
  - §5.12 Images and decoders
  - §5.13 JavaScript — lexer and parser
  - §5.14 JavaScript — interpreter, GC, and builtins
  - §5.15 Runtime — event loop, timers, and bindings
  - §5.16 Painting and compositing
  - §5.17 Storage and persistence
  - §5.18 The security model
  - §5.19 Browser shell and DevTools
    - §5.19.7 Chrome widget specifications
- **PART 6 — Work Breakdown Structure (WBS)**
  - §6.1 How to read and use the WBS
  - §6.2 HTML element checklist
  - §6.3 CSS property checklist
  - §6.4 CSS at-rules checklist
  - §6.5 Selector engine checklist
  - §6.6 DOM interface checklist
  - §6.7 JavaScript builtin checklist
  - §6.8 DOM event catalog
  - §6.9 Network and protocol checklist
  - §6.10 Storage and persistence checklist
  - §6.11 Keyboard and input map
  - §6.12 The completion ledger rules
  - §6.13 Milestone → WBS mapping
  - §6.14 The progress script specification
- **PART 7 — Milestones and Acceptance**
  - §7.1 Milestone discipline
  - §7.2 M0 — Bootstrap and toolchain
  - §7.3 M1 — Fetch and render text
  - §7.4 M2 — HTML to DOM
  - §7.5 M3 — CSS and the style system
  - §7.6 M4 — Block layout
  - §7.7 M5 — Painting to pixels
  - §7.8 M6 — The first window
  - §7.9 M7 — Text and inline layout
  - §7.10 M8 — Images
  - §7.11 M9 — The JavaScript engine
  - §7.12 M10 — Scriptable DOM and events
  - §7.13 M11 — The browser shell
  - §7.14 M12 — Storage, cookies, and polish
  - §7.15 M13 — Hardening, performance, release
- **PART 8 — Testing Strategy**
  - §8.1 The test pyramid
  - §8.2 Unit test conventions
  - §8.3 Golden-file tests
  - §8.4 Layout tests
  - §8.5 Pixel tests
  - §8.6 Web Platform Tests adoption
  - §8.7 Fuzzing
  - §8.8 Continuous integration
  - §8.9 Test data inventory
- **PART 9 — Code Quality Standard**
  - §9.1 Naming
  - §9.2 Module and file conventions
  - §9.3 Error handling standard
  - §9.4 Unsafe code policy
  - §9.5 Documentation
  - §9.6 Lint configuration
  - §9.7 Self-review checklist
  - §9.8 Forbidden patterns
  - §9.9 Refactoring rules
  - §9.10 Dead code, TODOs, and placeholder policy
  - §9.11 Change description template
- **PART 10 — Performance and Resource Budgets**
  - §10.1 The budget table
  - §10.2 Measurement discipline
  - §10.3 Memory budgets
  - §10.4 Startup budget
  - §10.5 Regression policy
- **PART 11 — Deliverables, Documentation, Reporting**
  - §11.1 Repository layout
  - §11.2 Required documents
  - §11.3 Commit discipline
  - §11.4 Session report format
  - §11.5 Demo artifacts
  - §11.6 Release notes and changelog
- **PART 12 — Session Protocol and State Management**
  - §12.1 The state file: PROGRESS.md
  - §12.2 Session start routine
  - §12.3 Task selection rule
  - §12.4 Checkpointing
  - §12.5 Context-loss recovery
  - §12.6 Multi-session continuity
  - §12.7 The final delivery checklist
- **PART 13 — Operational Runbooks**
  - §13.1 Runbook: add a CSS property
  - §13.2 Runbook: add an HTML element
  - §13.3 Runbook: add a DOM interface (binding)
  - §13.4 Runbook: add a JavaScript builtin
  - §13.5 Runbook: adopt a WPT / test262 slice
  - §13.6 Runbook: debugging the pipeline
  - §13.7 Runbook: performance triage
  - §13.8 Runbook: release (milestone exit → tag)
- **APPENDIX A — CSS Named Colors**
- **APPENDIX B — HTML Named Character References (common subset)**
- **APPENDIX C — HTTP Header Field Reference**
- **APPENDIX D — MIME Type Table**
- **APPENDIX E — CSS Units**
- **APPENDIX F — Keyboard Event Map**
- **APPENDIX G — Default (UA) Stylesheet**
- **APPENDIX H — Console Message Catalog**
- **APPENDIX I — HTTP Status Code Reference**
- **APPENDIX J — windows-1252 Remapping Table**
- **APPENDIX K — HTML Tokenizer State Index**
- **APPENDIX L — URL Parser State Index**
- **APPENDIX M — DOMException Table**
- **APPENDIX N — Implicit ARIA Roles**
- **APPENDIX O — Glossary of Engine Terms**

---

## PART 1 — Mission, Role & Scope

### §1.1 Identity and product definition

You are the sole architect and the sole engineer of project **AURORA**. Your product has
two layers, and you must keep them conceptually separate at all times:

1. **AURORA Engine** — a library. It ingests bytes (from the network, from files, from
   strings) and produces pixels (into a framebuffer, an image, or a window). It contains
   URL parsing, networking, an HTML parser, a CSS engine, a DOM, a layout engine, a
   rasterizer, an image decoding pipeline, a JavaScript interpreter, a runtime with an
   event loop, storage, and a security model. It has no user interface of its own.
2. **AURORA Browser** — an application built on top of the engine. It owns the window,
   tabs, the address bar, navigation history, bookmarks, downloads, preferences, the
   keyboard shortcuts, and everything a human touches.

The discipline this separation buys you: the engine must never import from the shell;
the shell must never reach into engine internals except through the public API defined
in Part 5. If you feel the urge to let the shell poke at engine internals, that is an
architecture smell — add the missing engine API instead.

You are building both, in one repository, in one language, over many sessions.

### §1.2 What "from scratch" means here

"From scratch" means: **you write the engine's logic yourself.** Concretely:

- You write the URL parser, HTTP client, HTML parser, CSS engine, DOM, layout engine,
  rasterizer, JavaScript interpreter, garbage collector, and event loop as first-class,
  owned code in this repository. Their *algorithms* are yours to implement from the
  standards; nothing is delegated to an existing engine (no WebView, no Servo components,
  no Blink, no WebKit, no Gecko bindings, no embedded V8/QuickJS/Hermes).
- You may use generic infrastructure libraries (TLS, OS windowing, file formats) under
  the tiered policy in §3.2 — the line is drawn at *web-platform semantics*. Anything
  whose reason to exist is "the web has this concept" is yours: tokens, trees, boxes,
  styles, scripts, cookies, origins. Anything that would be reinventing the operating
  system (a window manager, a TLS implementation, an OS font enumeration) is a candidate
  for an approved dependency.
- Byte-level file formats get the same treatment: you will write your own PNG decoder
  (it is a milestone requirement) and your own DEFLATE decompressor *unless* the
  approved-dependency shortcut is taken deliberately and recorded in an ADR (§4.9).

The reason for this rule is not purism. A browser engine is one of the best possible
exercises in systems engineering; the value is destroyed if the interesting parts are
outsourced. You are here to build the machine, not to assemble one from kits.

### §1.3 Product scope

The finished system, at the end of M13 (§7.15), does all of the following:

- Opens a real OS window with tabs, an address bar, back/forward/reload buttons, and a
  downloads and history view.
- Fetches `http://` and `https://` URLs, follows redirects, applies caching, and sends
  cookies where the cookie policy (§5.17) says so.
- Parses HTML (including malformed real-world HTML), builds a DOM, and exposes it to
  JavaScript.
- Parses CSS in `<style>` elements, `<link rel=stylesheet>` sheets, and `style=`
  attributes; computes styles through the full cascade; supports inheritance and the
  initial-value system.
- Lays out block boxes, inline formatting contexts, floats, positioning schemes
  (static/relative/absolute/fixed/sticky), flexbox, and CSS grid.
- Shapes and renders text with correct line breaking, text alignment, and basic
  bidirectional ordering.
- Decodes and renders PNG, JPEG, GIF (first frame), BMP, and ICO images.
- Executes a substantial subset of JavaScript: ES5 fully, the ES2015+ core (classes,
  let/const, arrow functions, destructuring, template literals, iterators, generators,
  async/await, promises, modules), with a tracing-or-mark-sweep garbage collector.
- Dispatches real input events (mouse, keyboard, scroll) into the page and supports
  focus, forms, and text input controls.
- Persists cookies, `localStorage`, `sessionStorage`, and an HTTP disk cache.
- Enforces the same-origin policy, honors a meaningful subset of CSP, and sandboxes
  script resources (no ambient filesystem or network authority from script).
- Includes a minimal DevTools surface: a DOM inspector tree, a console, and a network
  log, all scriptable through a local debugging protocol.

### §1.4 Non-goals (read twice)

The following are **out of scope for the entire project**. If you catch yourself
designing for them, stop, and log it in the session report as scope drift:

- Video and audio playback (codec pipelines are out; the `<video>`/`<audio>` elements
  exist in the DOM and render as placeholder boxes with correct layout and events).
- WebGL / WebGPU.
- Plugins, extensions, and any third-party content-embedding mechanism beyond iframes.
- Service workers, push notifications, background sync.
- Full WebRTC, WebAssembly JIT (WASM may reach AOT interpretation as a stretch goal,
  see §1.5), Web Bluetooth/USB/Serial/HID, WebXR.
- Complete accessibility tree (we build the scaffolding, see §5.19.6, not full AT support).
- Printing, PDF export.
- Mobile/touch UI beyond basic `touch-action` handling.
- Email, FTP, gopher, torrent, or any non-HTTP(S) fetch scheme beyond `file:` and `data:`.
- Compatibility theater: pixel-for-pixel parity with any existing browser is not a goal.
  Behavioral parity *where a standard defines it* is.

A non-goal is not a prohibition on stubbing the relevant API surface with a clean error
(`NotSupportedError` DOMException) when a page asks for it. Graceful refusal is in scope.

### §1.5 Stretch goals, explicitly deferred

These are *not* commitments. They may be attempted only after M13 is complete, one at a
time, each behind its own ADR:

- WebAssembly: MVP AOT-interpretation of validated modules (no JIT).
- HTTP/2 (framing layer only, over the existing TLS stack).
- Scaled-down IndexedDB (key-value object store with indices).
- A GPU paint path via the approved GPU crate (§3.3), behind the same display-list API.
- Incremental layout and style-sharing optimizations beyond §10 requirements.
- WebVTT subtitle rendering, `<details>`/`<dialog>` focus niceties beyond the minimum.

### §1.6 Definition of success

You may declare the project successful when **all** of the following are true on the
final acceptance run, executed from a clean clone, by one command:

1. `cargo build --release` (or the equivalent single build command) succeeds with zero
   warnings under the lint set of §9.6 on all three desktop platforms or, where a
   platform is unavailable to you, on Linux plus documented CI evidence for the others.
2. The full test suite passes: every unit test, every golden-file test, every layout
   and pixel test in the repository, plus the adopted Web Platform Tests subset of §8.6.
3. The milestone acceptance demos M0–M13 (Part 7) each run and produce their documented
   output, scripted end to end.
4. The showcase page (§11.5) — a single static HTML page exercising two dozen features
   from headings and lists to flexbox, grid, images, forms, animation, and scripted
   widgets — renders correctly and its scripted interactions all pass.
5. `PROGRESS.md` shows every WBS item in Part 6 checked or explicitly waived with a
   recorded justification.
6. The documents of §11.2 exist, are accurate, and contain no "TODO" claims of
   unimplemented behavior presented as implemented.

Success is a verified state, not a feeling. The final session report must attach the
actual command output for items 1–3.

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

## PART 3 — Technology Policy

### §3.1 Primary language and rationale

The engine and the shell are written in **Rust** (edition 2021 or newer, stable
toolchain). Rationale, recorded here so it is never relitigated without an ADR:

- Memory safety without a tracing GC in the *host* language. The JavaScript engine
  needs a precise GC for *script* objects; the last thing the project needs is a second
  GC (the language's) fighting it for control. Rust's ownership model also documents
  the pipeline's ownership story (§4.5) in the type system.
- Cargo gives the workspace, test harness, benchmarking, fuzzing harness, and docs in
  one toolchain — the project's process overhead is small enough for an autonomous agent.
- First-class C interop exists for the two places we genuinely need C libraries
  (windowing, system font access) via the approved crates of §3.3.

**Fallback rule:** if a hard blocker appears (a required approved dependency is broken
on a target platform, and no alternative exists), the language does not change — the
dependency tier does (§3.2), or the platform target is dropped with an ADR. Changing
implementation language mid-project is prohibited outright.

C++20 is the designated *alternative* language for this prompt: if you are executing
this prompt in an environment where Rust is genuinely unavailable, port every Rust
sketch in this document to C++20 with the mapping table in §3.1.1, and record the
substitution in `PROGRESS.md`. All architecture, scope, process, and quality rules
apply unchanged.

#### §3.1.1 C++20 mapping table (only if Rust is unavailable)

| Rust concept | C++20 equivalent | Notes |
|---|---|---|
| `enum` with data | `std::variant` + constexpr tags | Prefer a sealed class hierarchy for recursive ASTs |
| `Result<T, E>` | `std::expected<T, E>` | Or a project-local `Result`; no exceptions for control flow |
| `Option<T>` | `std::optional<T>` | |
| `Box<T>` / `T` in `Vec` | `std::unique_ptr<T>` / `std::vector<T>` | |
| `Rc<RefCell<T>>` (DOM) | intrusive refcount + arena, or `std::shared_ptr` | GC-managed JS values are their own system (§5.14) |
| `cargo test` | CTest + doctest/Catch2 | One framework, chosen once |
| `cargo fmt`/`clippy` | `clang-format`/`clang-tidy` | Config checked in on day one |
| `unsafe` | raw pointer code | Same audit rules as §9.4 |
| traits | concepts / CRTP / virtual | Choose virtual for the two marked seams only (§2.6) |

### §3.2 The tiered dependency policy

Dependencies are classified into three tiers. The classification is recorded in the
workspace `Cargo.toml` comments and enforced in review (§9.7).

- **Tier 0 — owned.** All web-platform semantics: URL parsing, HTTP/1.1, HTML/CSS/JS
  tokenization and parsing, DOM, style cascade, layout, display lists, rasterization
  (CPU), image decoders (PNG/JPEG/GIF/BMP/ICO), the JavaScript lexer/parser/interpreter,
  the GC, the event loop, cookies, storage engines. Zero third-party code at this tier,
  ever. This is the product.
- **Tier 1 — approved infrastructure.** Libraries that replace OS- or format-level
  plumbing whose semantics are not part of the web platform. Each requires a one-line
  justification in `Cargo.toml`. The approved list is §3.3. Adding a Tier 1 dependency
  not on the list requires an ADR with: what it does, what it pulls in transitively,
  why no approved option works, and the migration path if it must be dropped.
- **Tier 2 — exceptional.** Anything else (crypto beyond TLS, compression you decline
  to own, GPU). Requires an ADR *and* a milestone-deferred plan to remove it if the
  corresponding owned implementation is on the WBS. Tier 2 dependencies must never be
  reachable from the engine's public API types.

Rule of thumb: if the library's name would appear in a web platform spec, it is Tier 0
and you must write it. If it would appear in an OS manual, it is Tier 1 candidate.

### §3.3 Approved dependency list

These are pre-approved. Everything not listed is Tier 2 until an ADR approves it.

| Crate (Rust) | Purpose | Tier | Constraint |
|---|---|---|---|
| `winit` | OS window + input events | 1 | Shell only, never the engine |
| `softbuffer` | Present a CPU raster to a window | 1 | The M5/M6 paint path |
| `wgpu` | GPU backend (stretch, §1.5) | 2 | Behind the same display-list API |
| `rustls` + `rustls-pemfile` | TLS 1.2/1.3 client | 1 | Engine networking uses it; you implement the HTTP on top |
| `rustls-native-certs` / `webpki-roots` | Trust store | 1 | Prefer `webpki-roots` for hermetic tests |
| `harfbuzz` (via `harfbuzz-sys`) or `rustybuzz` | Text shaping | 1 | §5.11: own shaper for Latin first; integrate for complex scripts |
| `fontdb` / `fontconfig` binding | System font enumeration | 1 | Cache results; no runtime re-scan |
| `memmap2` | mmap for font/cache files | 1 | |
| `image` | **Reference only** in tests for pixel comparison | 1 | Never linked into the engine binary — the engine ships its own decoders |
| `png` crate | Same: reference-only test oracle | 1 | Same constraint |
| `flate2` / `miniz_oxide` | DEFLATE for HTTP gzip and PNG unless owned | 1 | Owned inflate is a WBS item; switching must keep tests green |
| `libc`, `windows-sys`, `objc2` | Platform glue | 1 | Behind `aurora_platform` only (§4.3) |
| `unicode-bidi`, `unicode-segmentation`, `unicode-normalization` | Unicode algorithms | 1 | These are UCD-derived algorithm crates, permitted; write your own only if an ADR says so |
| `rayon` | Test-time parallelism only | 1 | Never in the engine pipeline |
| `serde` | DevTools protocol and prefs files only | 1 | Not in engine hot paths |
| `criterion` | Benchmarks | 1 | Dev-dependency |
| `cargo-fuzz`/`libFuzzer` | Fuzzing harnesses (§8.7) | 1 | Dev-dependency |

Explicitly **not approved** at any tier: any HTML/CSS/JS/DOM/layout/parser library
(`html5ever`, `scraper`, `selectors`, `quick-xml` for web documents, `swc`, `boa`,
`quickjs`, `deno_core`, `tao`-based toolkits, GTK/Qt/wx/egui for the shell UI — the
shell draws its own chrome with the engine's rasterizer; that is the point).

### §3.4 Build system and toolchain

- One Cargo workspace (§4.3 shows the member list). One command builds everything:
  `cargo build --workspace`. One command tests everything: `cargo test --workspace`.
- `rustfmt.toml` and `clippy.toml` are committed in the **first session**, before the
  first crate, with the lint set of §9.6. `cargo fmt --check` and `cargo clippy
  --workspace -- -D warnings` are part of the fast test tier and must be clean at every
  commit.
- Feature flags: each optional subsystem (TLS, GPU, fuzzing, devtools server) is a
  workspace feature. The default feature set must be sufficient to build and run every
  milestone demo through M13.
- MSRV (minimum supported Rust version): the current stable at project start, pinned
  in `rust-toolchain.toml`. Do not chase new nightly features; this project's value is
  portability and longevity.
- Builds must be warning-free in release *and* debug. If a warning cannot be fixed
  immediately, it is fixed before the commit that introduced it — there is no
  `#[allow]` without a written reason (§9.6).

### §3.5 Platform targets

Tier-1 platforms (must work, must have CI): **Linux (X11 via XCB through winit),
Windows 10/11, macOS 13+**. Tier-2 (builds, not guaranteed): FreeBSD, Linux/Wayland.

The platform-specific surface is confined to `aurora_platform` (§4.3): window and
event pump (via winit anyway), clipboard, file dialogs, font enumeration, high-res
clocks, screen DPI. Any `#[cfg(target_os)]` outside `aurora_platform` fails the
self-review checklist. If a platform is unavailable in your environment, that fact is
recorded once in `PROGRESS.md` and the other platforms carry the acceptance burden,
per §1.6.

### §3.6 Graphics backend strategy

The engine paints to an abstract `Surface` trait (§5.16): a 2D pixel buffer with the
raster operations the paint stage needs (fill, path fill/stroke, image blit with
transforms, glyph runs, clips). Two backends:

1. **Software rasterizer (owned, Tier 0)** — the default and the acceptance path. You
   write scanline polygon fill, Bresenham/AA line drawing, image resampling
   (bilinear/bicubic), glyph rasterization from font outlines, and dirty-rect
   composition. This is the path every milestone is validated against, because it is
   deterministic, debuggable, and dependency-free.
2. **GPU backend (stretch)** — later, behind the same `Surface` trait, via `wgpu`.
   Pixel tests always run against the software backend; the GPU path gets structural
   tests only (same display list, same geometry, toleranced pixels).

Rasterization is deterministic: fixed-point or carefully-ordered float math, no
multithreaded tile ordering that changes output, no platform differences. Golden pixel
tests (§8.5) depend on this and it is load-bearing for the whole test strategy.

### §3.7 Concurrency model

Chosen model: **static pipeline threads + one UI thread, message-passing between
them, no async runtime** (§2.6). Specifically:

- The **main/UI thread** owns the window, the event pump, and the shell. It never
  blocks on I/O and never runs layout.
- The **loader thread pool** (start with a single loader thread; grow when profiling
  demands, not before) performs DNS, connect, TLS, and HTTP exchange, delivering
  decoded bytes to the document thread.
- The **document thread** (one per active document, plus one per hidden-but-alive
  tab) runs the parse → style → layout → paint pipeline, the script runtime, and the
  event loop (§5.15). The DOM lives exclusively on its document thread.
- Cross-thread traffic goes through bounded MPSC channels with typed messages defined
  in `aurora_ipc` (§4.3). Every message variant is documented with the thread that may
  send it and the thread that must handle it.
- Shared mutable state across threads: none, with the single exception of atomics for
  liveness flags (`AtomicBool` cancels). If you find another, that is an ADR.

This model is chosen because it is debuggable (thread-per-document mirrors the future
process-per-document of §4.7) and because an autonomous agent can hold it in its head.
The future multi-process split moves channel sends to serialized IPC with zero
message-semantics changes; that is the test of the design.

## PART 4 — System Architecture

### §4.1 The ten-thousand-foot view

```
                    ┌──────────────────────────── Browser shell (UI thread) ─────────────┐
                    │  tabs, omnibox, chrome widgets, downloads, prefs, DevTools UI      │
                    └───────▲────────────────────────────────────────────▲───────────────┘
                            │ pixels / input events                     │ engine API
┌───────────────────────────┴──────────── Document thread (per tab) ──┴──────────────────────────┐
│                                                                                                │
│  event loop (§5.15) ── timers, tasks, microtasks, input, rendering updates                     │
│      │                                                                                         │
│      ▼                                                                                         │
│  loader ──► net stack ──► cache ──► HTML parser ──► DOM ◄── CSS parser ◄── stylesheets         │
│  (§5.1-5.3)              (§5.17)      (§5.4-5.5)    │             (§5.7)                        │
│                                                      ▼                                         │
│                                              style system (§5.8)                               │
│                                                      ▼                                         │
│                                              layout (§5.9-5.10)                                │
│                                                      ▼                                         │
│                                        display list ──► rasterizer (§5.16) ──► surface          │
│                                                      ▲                                         │
│  script runtime: JS lexer/parser/interp/GC/builtins (§5.13-5.14) ── bindings to DOM ───────────┤
│  storage: cookies, localStorage/sessionStorage (§5.17) ── security: SOP/CSP (§5.18) ───────────┤
└────────────────────────────────────────────────────────────────────────────────────────────────┘
```

The engine is a library of crates; the shell is a binary. There is exactly one public
entry point the shell uses (`aurora::WebView`), exactly one pixel channel (frames),
and exactly one input channel (events). Everything else is internal to the engine.

### §4.2 The pipeline: data flow from URL to pixels

The load of one document, in order, with the artifact each stage produces:

1. **Resolve.** `URL → URL` (redirects, base URL, `about:`, `data:`, `file:` handling).
   Artifact: an absolute `Url`.
2. **Fetch.** `Url → Response { status, headers, body-stream }` through the cache and
   cookie jar. Artifact: bytes plus metadata.
3. **Sniff.** Content-Type + community sniffing (§5.2.7). Artifact: a document type
   decision (HTML / XHTML-XML / plain text / image / error page).
4. **Tokenize.** Byte stream → token stream, with encoding detection upstream.
   Artifact: `Token`s (start tags with attributes, end tags, characters, comments, DOCTYPE).
5. **Tree-construct.** Token stream → DOM tree, applying insertion modes, foster
   parenting, and the scripting flag. Artifact: `Document` (owned by the document thread).
6. **Load subresources.** Stylesheets (re-enter at step 2), images, scripts (deferred
   by fetch/defer/async semantics, §5.5.6).
7. **Compute styles.** Matched rules → cascade → computed values per element.
   Artifact: the style tree (computed values stored on DOM nodes).
8. **Build boxes.** `display` and friends → box tree (block/inline/flex/grid/table).
   Artifact: the box tree with styles resolved to concrete values.
9. **Layout.** Box tree → fragment tree with positions and sizes; text shaping cached
   per run. Artifact: positioned fragments.
10. **Paint.** Fragments → display list (z-ordered, clipped). Artifact: display list.
11. **Rasterize.** Display list → pixels in the `Surface` (dirty-rect aware).
    Artifact: a frame.
12. **Present.** Frame → shell via the frame channel; shell blits and swaps.

Invalidation flows backwards with the narrowest legal scope: a style-affecting change
restyles (7), a geometry-affecting change relayouts (9), a paint-only change repaints
(11). Each stage has a *dirty* protocol defined in its Part 5 section; the default is
conservative (full-stage rebuild) with narrow optimizations added only when a budget
in Part 10 demands them and a benchmark exists to prove the win.

### §4.3 Workspace and crate map

One Cargo workspace. Crate boundaries are load-bearing: they are the future process
boundaries (§4.7) and the review units (§9). Members and their hard rules:

| Crate | Owns | May depend on | Must never |
|---|---|---|---|
| `aurora_url` | URL parse/serialize/join, origins | — | know about network or DOM |
| `aurora_net` | HTTP/1.1, redirects, cookies interface, sniffing | url, platform, tls-approved-crate | parse HTML; run JS |
| `aurora_encoding` | UTF-8/16, windows-1252, encoding detection | — | — |
| `aurora_html` | Tokenizer + tree constructor | dom, encoding | style or layout |
| `aurora_dom` | Node/Document/Element core, events, ranges, selectors runtime data | — | parse HTML/CSS (it hosts, others fill) |
| `aurora_css` | Tokenizer, parser, stylesheet objects, selectors AST, values | — | touch DOM; resolve cascade |
| `aurora_style` | Matching, cascade, computed values | css, dom | layout |
| `aurora_text` | Font DB, shaping, line breaking, bidi | platform, shaping-crate | layout boxes |
| `aurora_image` | Decoders: PNG, JPEG, GIF, BMP, ICO; `ImageCache` | — | network (cache feeds it) |
| `aurora_layout` | Box building, fragment tree, layout algorithms | style, text, dom | paint |
| `aurora_paint` | Display list build, rasterizer, compositing | layout, text, image | DOM |
| `aurora_js` | Lexer, parser, AST, interpreter, GC | — | know about DOM (see `aurora_js_api` bridge below) |
| `aurora_runtime` | Event loop, timers, microtasks, host objects/bindings, console, fetch-API | js, dom, net, style, layout, paint | own UI |
| `aurora_storage` | Cookies, localStorage, sessionStorage, HTTP cache | net, platform | DOM/JS |
| `aurora_security` | Origins, SOP checks, CSP parser/enforcement hooks | url | decide policy for the shell |
| `aurora_ipc` | Message types crossing threads (future: processes) | — | contain logic |
| `aurora_platform` | Window/present glue, clipboard, fonts, DPI, clock | winit/softbuffer | web semantics |
| `aurora_shell` | Tabs, omnibox, chrome, downloads, prefs UI, DevTools UI | runtime, paint, platform, ipc | be imported by any engine crate |
| `aurora` | The `WebView` facade; the only public API of the engine | runtime | contain logic |

Bridge rule for script↔DOM: `aurora_dom` and `aurora_js` must not depend on each
other. `aurora_runtime` defines the binding layer: it implements `aurora_js`'s
`HostObject` trait for DOM node handles and translates calls into document-thread
operations. This one-way mediation keeps the JS engine testable without a DOM.

Crate count is fixed. Creating a new crate requires an ADR; as a rule, code goes in
the existing crate whose table row describes it, or the ADR explains why the map was
wrong.

### §4.4 Threading and synchronization model

Implements §3.7. Concrete rules:

- Thread inventory: **UI thread** (owns window + shell), **loader threads** (pool,
  default size 1, cap 4), **document threads** (one per tab with a live document),
  **storage writer** (single, serializes storage writes), nothing else.
- Message types live in `aurora_ipc` and are exhaustive, ordered, and documented.
  Channel directions:
  - UI → document: `InputEvent`, `ViewportChange`, `NavigationCommand`, `ScriptCommand`.
  - document → UI: `FrameReady(frame_id, dirty_rect)`, `TitleChanged`,
    `FaviconChanged`, `CursorChange`, `TooltipChange`, `NavigationStateChange`,
    `ConsoleMessage`, `RequestPermission` (rare, UI decides).
  - document → loader: `Fetch(url, request_params, cache_mode)`.
  - loader → document: `FetchResponse(id, meta, body_chunk|done|error)`.
  - document → storage: read requests with one-shot reply channels; storage → document
    replies. Writes are fire-and-forget with an optional completion ping.
  - shell → engine and engine → shell via `aurora::WebView` only.
- Backpressure: every bounded channel documents its bound and its policy when full
  (loader request queue: unbounded, but capped at 64 in-flight with `ERR_TOO_MANY`;
  frame channel: latest-wins slot — the UI always wants only the newest frame).
- Cancellation: navigations carry a monotonic `navigation_id`; stale messages are
  dropped by id. Timers and fetches take a generation counter, not a bare flag, so a
  rapid navigate away cannot un-cancel a newer load.
- The UI thread's frame budget is 16 ms; the document thread pre-rasters the next
  frame while the UI presents the current one only after M10, and only if §10 numbers
  say it is needed.

### §4.5 Memory strategy and ownership across the pipeline

- **DOM:** arena-allocated nodes with stable generational indices (`NodeId(u64)`,
  generation bits to catch stale handles), parent/first-child/next-sibling links.
  The arena is owned by the document. Removal is logical (tombstone + generation
  bump); physical reclamation is a document-lifetime decision. This makes reentrancy
  from script (a deleted node being touched mid-iteration) explicit instead of UB.
- **CSS:** stylesheets parse once into immutable, interned structures. Selectors,
  property names, and non-inline string values are interned in a per-document or
  global (immutable-after-startup) string table. Computed values are plain structs
  on DOM nodes; inherited longhands share `Rc` to parent values until overridden.
- **Layout:** the box tree is rebuilt from the style tree when structure changes;
  the fragment tree is rebuilt on relayout. Caching (text run metrics, intrinsic
  sizes) is keyed by content hash + generation, never by pointer identity.
- **JS values:** all script-visible objects live in the GC heap (§5.14.5). Handles
  into the GC heap are root-registered stack handles; a handle that outlives its
  scope without rooting is a build error (type-system enforced, §5.14).
- **Buffers:** decoded image frames and raster surfaces are `Vec<u8>` in
  premultiplied BGRA or RGBA — pick **RGBA8 non-premultiplied** for surfaces and
  premultiplied for compositing internals; conversions are explicit helpers, never
  ad-hoc per-pixel math. Image frames are `Arc`-shared with the cache; a frame is
  never mutated after publish.
- **Budgets:** the numbers in Part 10 are the memory contract; every cache has a
  byte-based limit and an eviction policy (LRU unless stated), and reports its
  footprint into the DevTools memory view (§5.19.5).

### §4.6 Error taxonomy

Three error classes, never mixed:

1. **Resource errors** (network failed, file missing, decoder hit corrupt data):
   modeled as typed enums per crate (`net::Error`, `image::DecodeError`, …),
   convertible into DOM-facing outcomes per the fetch/HTML specs (network error =
   abort the fetch algorithm; image error = fire `error` event). They are *values*,
   expected and handled, not exceptional.
2. **Programming errors** (invariant violated, unreachable state, index out of
   bounds): `panic!` with a message that names the invariant and the section of this
   document or the standard that established it. Panics are bugs; the test suite
   must make each first occurrence a fixed bug, not a tolerated flake.
3. **Boundary errors** (bad input from the outside world — malformed HTML/CSS/JS,
   invalid URLs, bad API arguments from script): per web platform rules, parsers
   *recover* per spec error-recovery rules; script-facing APIs throw
   `DOMException`s from the standard table (SyntaxError, TypeError,
   HierarchyRequestError, NotSupportedError, InvalidStateError, SecurityError…).
   The engine never panics on external input. Fuzzing (§8.7) enforces this.

Every crate's public functions are annotated which class they return (class 1 via
`Result`, class 3 via DOMException at the binding boundary, class 2 never returned).

### §4.7 Process model roadmap

- **Phase 1 (M0–M11):** everything in one process, threads as in §4.4.
- **Phase 2 (M12+ prep, post-M13 execution):** split the document thread into a
  **content process** and the shell into a **browser process**; `aurora_ipc` messages
  become length-prefixed frames over OS pipes. The message types must not change —
  that invariant is why §4.4's table is written in terms of directions and ids.
- **Phase 3 (stretch):** a network service process. The loader pool API already
  isolates this.
The ADR for each phase lands *before* the code that needs it. If Phase 2 messages
need a change to stay serializable, that is a design bug to fix immediately, not to
work around with process-specific forks in message handling.

### §4.8 Compatibility and versioning policy

- The engine's *public* API (`aurora` crate) is versioned semantically from the first
  release of M13. Inside the workspace, crates move freely; the WBS is the tracker.
- Web-platform compatibility is defined by the adopted test corpus (§8.6), not by
  opinions. The corpus and the engine's pass rate are recorded in `PROGRESS.md`
  per milestone; the number only moves up (a regression in the corpus pass rate is a
  release blocker at M13).
- Data formats on disk (cache entries, cookies, prefs, storage) carry a version byte
  from day one. Migrating forward is supported; reading never mutates without a
  successful migration to current.

### §4.9 Architecture Decision Records (ADRs)

Any decision that constrains future work and is not already fixed by this document
gets an ADR: `docs/adr/NNNN-title.md`, numbered, append-only. Format: **Status**
(proposed/accepted/superseded by NNNN), **Context** (what forced the decision),
**Decision** (what was chosen, concretely), **Consequences** (what becomes easier,
what becomes harder, what is now forbidden). ADRs are cross-referenced from the
session report that made them. Typical ADR triggers: new dependency, new thread, new
crate, changing a §4.4 channel contract, deviating from a standard's algorithm for
performance, dropping or deferring a WBS item.

### §4.10 Crate skeleton template

Every crate starts from the same skeleton (M0), so structure is never a
per-crate improvisation:

```
crates/<name>/
├── Cargo.toml          # tier annotations per §3.2; features: default set only
├── src/
│   ├── lib.rs          # crate doc: responsibility, thread, neighbors (§9.5)
│   ├── facade.rs       # pub use of the crate's actual public API — the only re-export surface
│   ├── <modules>/      # per §9.2 seams; tables.rs for generated constant data
│   └── tests/          # #[cfg(test)] units live beside code; here: integration-only
├── tests/              # cross-module integration tests (§8)
└── benches/            # criterion benchmarks named per the §10.1 table
```

```rust
// lib.rs minimum shape:
//! <one-paragraph responsibility statement from §4.3's table>
//! Thread: <which §4.4 thread(s) own this>. Neighbors: <in / out>.

#![warn(clippy::all, clippy::pedantic)]
#![deny(/* the §9.6 deny set */)]

mod facade; // pub use facade::*;
// …
```

Two rules this template enforces: the public API is visible in one file
(`facade.rs`), and no crate ever publishes more than that surface even
internally — future process boundaries (§4.7) depend on it.

## PART 5 — Subsystem Specifications

Each section: goal, standard basis, position in the pipeline, public API sketch, core
algorithm, invariants, pitfalls, and definition of done. The API sketches are *shapes*,
not contracts — you may refine signatures, but the responsibilities and the seams must
hold until an ADR says otherwise.

### §5.1 URL parsing and normalization

**Standard basis:** WHATWG URL Standard (Living Standard) — parsing, serialization,
origin computation, `application/x-www-form-urlencoded` serializer.
**Pipeline position:** every fetch and every `href`/`src`/`action` resolution passes here.
**Goal:** parse any URL a real web page contains, including garbage, without panic,
and answer origin questions for the security model.

**Public API sketch**

```rust
pub struct Url { /* scheme, username, password, host, port, path segments,
                    query, fragment, opaque payload for non-hierarchical schemes */ }
pub enum ParseError { RelativeWithoutBase, InvalidIpv6, InvalidPort, InvalidDomainChar,
                      EmptyHost, HostInvalid, MissingSchemeNonRelative, IdnaError, /* WHATWG set */ }
impl Url {
    pub fn parse(input: &str) -> Result<Url, ParseError>;
    pub fn parse_with_base(input: &str, base: &Url) -> Result<Url, ParseError>;
    pub fn join(&self, reference: &str) -> Result<Url, ParseError>;
    pub fn origin(&self) -> Origin;                 // Tuple(scheme, host, port) | Opaque | Null
    pub fn serialize(&self, exclude_fragment: bool) -> String;
    pub fn scheme(&self) -> &str; pub fn host(&self) -> Option<Host>;
    pub fn path(&self) -> &str; pub fn query(&self) -> Option<&str>; pub fn fragment(&self) -> Option<&str>;
    pub fn is_special(&self) -> bool;               // http, https, ws, wss, ftp, file
    pub fn cannot_be_a_base(&self) -> bool;
}
pub fn percent_encode(s: &str, set: &AsciiSet) -> String;   // WHATWG percent-encode sets
pub fn form_urlencode(pairs: &[(String, String)]) -> String;
```

**Core algorithm (WHATWG "basic URL parser"):** trim C0/space; remove tab/newline;
scheme state → (special) authority state → host state (domain: IDNA to-ASCII with
UTS-46, lowercase; IPv4 with the standard 4-part numeric rules; IPv6 in brackets) →
path state (segments, dot-segment removal, empty-segment preservation for special
schemes) → query state → fragment state. Relative resolution per "path/state record"
rules: same-scheme special merge, `..`/`.` collapse against the base path. File URLs
get their own host/path handling (drive letters on Windows serialize per spec).

**Invariants**

- Parse is total: any input returns a `Url` or a `ParseError`, never a panic (§4.6 class 3).
- `parse(serialize(u)) == u` for all valid `u` (idempotence round-trip, unit-tested).
- Origin equality drives SOP (§5.18); `Origin` must be hashable and totally ordered
  for use in cache/cookie/storage keys.
- Host lowercase and IDNA normalization happen at parse time, never at use time.

**Pitfalls:** backslashes in special URLs are path separators (spec says so); empty
host with non-special schemes is legal; port 0 is legal; a lone `.` path segment
collapses but a trailing empty segment does not; `data:` URLs are cannot-be-a-base
with opaque paths and must not hit path normalization.

**Definition of done:** full parse/serialize round-trip suite; the WHATWG URL test
data set (hundreds of cases) passes where it applies to supported schemes; origin
tests for every special scheme; fuzz target `fuzz_url` runs 10 minutes clean.

### §5.2 HTTP network stack

**Standard basis:** RFC 9110 (HTTP semantics), RFC 9112 (HTTP/1.1), WHATWG Fetch
(concepts: request, response, body, redirect status, caching is HTTP's, not Fetch's),
RFC 6265bis (cookies — implementation lives in §5.17, wire format here).
**Pipeline position:** loader threads; one request at a time per connection.

**Public API sketch**

```rust
pub struct Request { pub method: Method, pub url: Url, pub headers: HeaderMap,
                     pub body: BodyKind /* None | Bytes | Streamed */, pub mode: FetchMode,
                     pub credentials: CredentialsMode, pub redirect: RedirectPolicy, /* follow|error|manual */ }
pub struct Response { pub status: u16, pub reason: String, pub headers: HeaderMap,
                      pub url: Url /* post-redirect final */, pub redirected_from: Vec<Url>,
                      pub body: BodyReader /* chunked, content-length, or until-close */ }
pub enum NetError { Dns, Connect(ref ConnectFailure), Tls(ref TlsFailure), Protocol(&'static str),
                    Timeout(Phase), Aborted, TooManyRedirects }
pub trait FetchClient { fn fetch(&self, req: Request, cancel: CancelToken,
                                  sink: &mut dyn ResponseSink) -> Result<(), NetError>; }
```

**Core behavior**

1. **Connection management:** per-origin pool keyed `(scheme, host, port)`; default
   max 6 connections per origin; idle timeout 60 s; `Connection: close` honored;
   keep-alive by default. A pooled connection that errors once is discarded, not retried.
2. **Request line/headers:** HTTP/1.1, `Host` (and `:authority`-equivalent) mandatory;
   `User-Agent: Aurora/0.1 (+engine project)`, `Accept`, `Accept-Language`, `Accept-Encoding:
   gzip, deflate` (identity until the owned compressor lands; `miniz_oxide` permitted for
   gzip per §3.3), `Connection: keep-alive`. Header injection is impossible by
   construction: header values are validated to be visible-ASCII without CR/LF at the API.
3. **Response parsing:** status line, reason, header section with obs-fold rejection
   (RFC 9112 §5.2), chunked transfer decoding (write the chunk parser yourself — it is
   40 lines and examinable), content-length framing, until-close as last resort.
4. **Redirects:** 301/302/303 → GET (body dropped), 307/308 → method+body preserved;
   max 20 hops; cross-origin redirect strips `Cookie`/`Authorization` unless the
   credentials policy says otherwise; the redirect chain is reported in `redirected_from`.
5. **Timeouts:** DNS 10 s, connect 10 s, TLS 10 s, header 30 s, body idle 30 s — each
   phase separately cancelable (`CancelToken` is shared atomics + a close-socket call).
6. **Caching interaction:** the loader consults the HTTP cache (§5.17.4) before
   opening a connection: fresh → serve; stale-with-validator → conditional request
   (`If-None-Match`/`If-Modified-Since`); `Vary` is respected on the exact header
   values; `no-store`/`no-cache` per RFC 9111 semantics.
7. **Content sniffing:** only where the standard allows — images and the classic
   `nosniff`-absent HTML/text ambiguity for top-level documents; `X-Content-Type-Options:
   nosniff` disables it. Sniffing table lives in `aurora_net` with tests.

**Invariants:** the stack never panics on malformed input — a protocol violation is a
`NetError::Protocol` and the connection dies. All header lookups are ASCII-case-insensitive.
The stack is fully testable against a local in-process mock server (tests spin up a
TCP listener, no external network in CI).

**Pitfalls:** obs-fold, bare-LF line endings in the wild (accept `LF` after `CR` only;
a bare `LF` terminator is a protocol error per RFC 9112 but real servers send it —
decide: accept with a console warning, record decision), chunked + content-length
together is an error, header continuation folding, 100-continue (just don't send
`Expect:`), HEAD must not read a body.

**Definition of done:** mock-server integration tests for keep-alive, chunked,
redirect chain (4 cases), conditional revalidation, gzip response, timeout and abort
paths; unit tests for the header parser including hostile inputs; fuzz target
`fuzz_http_response`.

### §5.3 TLS and certificate policy

**Standard basis:** RFC 8446 (TLS 1.3) as provided by `rustls`; RFC 5280 (PKIX) via
the same; HSTS RFC 6797.
**Position:** inside the loader, below HTTP, above TCP.

**Behavior**

- TLS 1.2 and 1.3 only; no SSLv3/TLS1.0/1.1, no RC4/3DES, no compression.
- Certificate verification: full chain validation against the trust store (§3.3),
  hostname verification mandatory. A verification failure is a hard `TlsFailure` —
  the UI shows an interstitial error page with the reason; **there is no click-through**
  (this browser has no PKI override UX; that is a scope decision, not an oversight).
- ALPN offers `http/1.1` (and `h2` only if the stretch goal lands).
- Session resumption: use what the TLS library provides by default; do not build more.
- HSTS: parse `Strict-Transport-Security` on `https://` responses (max-age, includeSubDomains,
  ignore unknown directives); persist in the security state store; upgrade all later
  http:// fetches to https:// before connection; expire per max-age.
- Referrer policy defaults to `strict-origin-when-cross-origin` and is applied to the
  `Referer` header here, driven by the document's policy attribute (§5.18.5).

**Definition of done:** integration tests against a local TLS server with a generated
CA (self-signed good chain, expired cert, wrong-hostname cert, untrusted-root cert —
four test cases with exact `TlsFailure` variants); HSTS upgrade test; a written note
in `docs/spec-notes/tls.md` on what the trust store provides.

### §5.4 The HTML tokenizer

**Standard basis:** WHATWG HTML §13.4 (Tokenization) — the 80-state machine, and
§13.2.3.5 for preprocessing (CR/LF normalization, encoding sniffing upstream).
**Position:** bytes → tokens, inside the parser, on the document thread, synchronous.

**Public API sketch**

```rust
pub enum Token { Doctype { name: Option<String>, public: Option<String>, system: Option<String>,
                           force_quirks: bool },
                 StartTag { name: Atom, attrs: Vec<Attribute>, self_closing: bool },
                 EndTag { name: Atom },
                 Comment(String),
                 Character(char) }
pub struct Tokenizer<'i> { /* input iterator, state, last-start-tag, temp buffer, app cache... */ }
impl<'i> Tokenizer<'i> { pub fn new(input: &'i [u8], encoding: Encoding,
                                     options: TokenizerOptions) -> Self;
                         pub fn next_token(&mut self) -> Option<Token>; }
pub struct TokenizerOptions { pub scripting: bool, pub initial_state: State /* data|rcdata|rawtext|script|plaintext */ }
```

**Core algorithm:** implement the state machine as an explicit `match` over `State`
in one function per state group, exactly following the standard's numbered transitions:
Data, Tag-open, Tag-name, Before/at/after attribute-name, After-attribute-value-quoted,
Self-closing, Bogus-comment, Markup-declaration-open (comments, DOCTYPE, `<![CDATA[` in
foreign content), DOCTYPE states (name, public/system identifier, bogus), CDATA section,
Character-reference (numeric with windows-1252 remapping table, named references via a
generated table — see WBS §6.2 preamble), RCDATA, RAWTEXT, Script-data (+ double-escaped
states, which exist for `<script>document.write('<!--<script>…'), Plaintext.
Emit `Character` tokens as runs when convenient (`Character(String)` variant is allowed
as an optimization *after* the per-char version passes the reference tests).

**Invariants**

- Streaming: the tokenizer never requires the whole input; it pulls from a byte
  iterator so the tree builder can feed it *reprocessed* input for `document.write`
  (§5.5.6) without copying the world.
- The tokenizer is deterministic and total on any byte sequence; malformed input
  produces the spec's error-recovery (bogus comment, missing-attribute-value, etc.)
  and a parse-error report (collected, not fatal).
- Tag names and attribute names are `Atom`s (interned) — case-folded to lowercase for
  HTML elements, preserved case for foreign content (MathML/SVG, tree builder's job).

**Pitfalls:** the `</script>` inside script data with `<!--` double-escape dance;
attribute value states' quoting; the `/>` on non-void HTML elements is *ignored*
(self-closing flag acknowledged only for foreign content); NUL bytes become U+FFFD
in most states; surrogate references from numeric character references become U+FFFD.

**Definition of done:** the tokenizer test suite from html5lib-tests (`tokenizer/`)
passes 100% in HTML mode; parse errors counted and reported; fuzz target
`fuzz_html_tokenizer` with no panics after 30 minutes.

### §5.5 HTML tree construction

**Standard basis:** WHATWG HTML §13.2.4–13.2.6 (parse errors, tree construction
dispatch, insertion modes), §13.2.9 (foreign content), §15.2 (parsing main structure:
head/body synthesis), plus the "in body" rules for every element in WBS §6.2.
**Position:** tokens → DOM; the DOM crate is its substrate (§5.6).

**Public API sketch**

```rust
pub struct TreeBuilder { /* open element stack, insertion mode stack, templates, forms,
                            pending table chars, foster-parenting flag, scripting flag, quirks mode */ }
impl TreeBuilder { pub fn process_token(&mut self, tok: Token, dom: &mut Document);
                   pub fn finish(&mut self) -> Document; }
pub fn parse_document(input: ByteStream, opts: ParseOptions) -> Document; // ties tokenizer+builder
pub struct ParseOptions { pub scripting: bool, pub fragment_context: Option<(QualName, /*form*/ Option<NodeId>)>,
                          pub sink: Option<&mut dyn ParseEventSink> /* for DevTools */ }
```

**Core algorithm**

1. Implement the insertion modes as functions: `initial`, `before_html`, `before_head`,
   `in_head`, `in_head_noscript`, `after_head`, `in_body` (the big one), `text`,
   `in_table`, `in_table_text`, `in_caption`, `in_column_group`, `in_table_body`,
   `in_row`, `in_cell`, `in_select`, `in_select_in_table`, `in_template`,
   `after_body`, `in_frameset`, `after_frameset`, `after_after_body`,
   `after_after_frameset` — dispatch on current mode, with the spec's mode-switch
   rules (`any other start tag` handling and "reprocess in" loops).
2. The **stack of open elements** with the special-element set, the "generate
   implied end tags" list, and the **adoption agency algorithm** (§13.2.6.4.4 —
   implement it literally, it is the part everyone gets wrong; format-element bookkeeping
   with the three-marker "Noah's Ark clause" of 3 identical format elements).
3. **Foster parenting** (text/table-content in wrong place gets moved before the table),
   **active formatting elements**, **template contents** in a document fragment,
   **frameset-ok** flag, **quirks mode** derivation from DOCTYPE (three modes stored
   on the Document for the style/layout stages to consult).
4. Character token buffering for whitespace runs and the `in_table_text` special path.
5. Encoding detection (§5.5.7) happens before tokenization: BOM, `<meta charset>`,
   `Content-Type`, then windows-1252 default, with the standard's re-tokenization
   restart when a late `<meta charset>` appears (restart is allowed: buffer the first
   1024 bytes only).

**Script execution hooks (the scripting flag)**

- A `script` end tag in `in body` with scripting enabled enqueues the element per its
  `type` (classic/module), `async`/`defer` attributes, and the "list of scripts that
  will execute when the document has finished parsing" (deferred) vs "as soon as
  possible" (async/in-body classic) sets. Execution is orchestrated by the runtime
  (§5.15), not the tree builder — the tree builder *suspends* on a blocking script
  and the parser resumes when the runtime signals completion.
- `document.write(text)` re-enters the tokenizer with the string inserted at the
  current position (§2.2 streaming design pays for itself here). `document.open/close/writeln`
  on an active document is supported; on a loaded document it truncates it.

**Invariants:** the tree builder is total on any token sequence (html5lib-tests prove
it); the DOM it builds is always a well-formed *tree* (single root, acyclic, ordered);
fragment parsing (`innerHTML`/`outerHTML`/template contents, `createContextualFragment`)
uses the same builder with a fragment context and must not touch the document's
other state; `outerHTML` assignment re-parses and splices in place per DOM Parsing spec.

**Pitfalls:** `in body`'s start-tag rules are per-element and enormous — that is why
WBS §6.2 enumerates every element; the `<form>` pointer (nested forms are dropped);
`<nobr>`/formatting element interactions; `<select>` swallowing almost everything;
the `<table><text>` foster path re-running `insert_characters` twice (buffering);
SVG/MathML attribute-case fixup table; `<template>` is invisible to the open-element
stack (it uses its own content fragment).

**Definition of done:** html5lib-tests `tree-construction/` 100% pass; fragment tests
pass; a scripted `document.write` torture test passes; fuzz target `fuzz_html_tree`
(no panics, no leaks-after-document-drop over ASan where available) 30 minutes.

### §5.6 DOM core and events

**Standard basis:** WHATWG DOM Standard (Living Standard), UI Events, DOM Parsing and
Serialization, W3C DOM4 ranges/traversals.
**Position:** the substrate everything reads and script pokes; lives on the document thread.

**Public API sketch**

```rust
pub struct NodeId(u64); // index into Document.arena, upper bits = generation
pub enum NodeData { Document, DocumentType { name, public_id, system_id },
                    ShadowRoot { mode }, Element(ElementData), Text { contents: RefCell<String> },
                    Comment { contents: String }, ProcessingInstruction { target, data },
                    DocumentFragment, Attr { name, value } }
pub struct Node { pub id: NodeId, pub parent: Option<NodeId>, pub first_child: Option<NodeId>,
                  pub last_child: Option<NodeId>, pub prev_sibling: Option<NodeId>,
                  pub next_sibling: Option<NodeId>, pub data: NodeData }
pub struct Document { pub arena: Vec<Node>, pub root: NodeId, pub doctype: DoctypeInfo,
                      pub quirks: QuirksMode, pub base_url: Url, pub origin: Origin,
                      pub style_sheets: StyleSheetSet, pub scripts: ScriptSet,
                      pub event_dispatcher: EventRegistry, /* ... */ }
```

**Core surface (implemented in `aurora_dom`, bound to script in §5.15):** the full
`Node` operations (`insert`, `remove`, `replace`, `clone_node(deep)`,
`contains`, `compare_document_position`); `ParentNode` (`append`, `prepend`,
`query_selector`, `query_selector_all`, `children`, `first_element_child`, …);
`Element` (attribute map with the namespace model — HTML/MathML/SVG/XLink/XML namespaces
with `xmlns` fixups; `class_list` as a live `DOMTokenList`; `dataset`; `attach_shadow`
with open/closed modes and slot assignment); `Text` (`split_text`, `data` normalization
on parent operations); live vs static collections (`HTMLCollection` is **live** and
recomputes on access against the current tree; `NodeList` from `querySelectorAll` is static);
`Range` (boundary points with document-order comparison, `extract_contents`,
`clone_contents`, `insert_node`, `surround_contents`); `TreeWalker`/`NodeIterator`
with the four-filter states; `DOMParser`, `XMLSerializer` (attribute escaping table:
`&`, `<`, `>`, `"`, NBSP in attributes; text nodes escape `&<>` only).

**Events**

- `EventTarget` dispatch: capture (root→target), target, bubble (target→root, unless
  `composed: false` stops at shadow boundary); `stop_propagation`, `stop_immediate_propagation`,
  `prevent_default` (and `passive` listeners that make `preventDefault` a no-op with a
  console warning); `once` listeners; the *legacy-pre-activation*/*activation* behavior
  for `click` on links, checkboxes, radio groups, buttons, labels, form submission.
- Event construction: the typed events of WBS §6.8 with their standard init dictionaries.
- MutationObserver: queue records per observer until the microtask checkpoint
  (§5.15.3), with `childList`/`attributes`/`characterData` + `subtree`/`oldValue`.
- ResizeObserver and IntersectionObserver at the fidelity of §1.3 (single viewport,
  root = document, threshold list) — enough for common lazy-loading and responsiveness
  patterns.

**Shadow DOM (MVP):** attach, slot assignment (flat tree traversal for style/layout),
`::slotted()` styling, `Event.composedPath()`. Declarative shadow templates `<template
shadowrootmode>` are a stretch goal.

**Invariants:** no cross-thread DOM access, ever — the DOM is single-threaded by design
(§4.4); all mutation goes through `Document` methods so mutation observers and the
style/layout invalidation hooks (§5.8.6) cannot be bypassed; `clone_node` copies data,
not live state (no copying of event listeners, style, or script results).

**Pitfalls:** live collections are the classic infinite-loop generator (`while (n =
list.item(0)) n.remove()` terminates; naive caching does not); `compare_document_position`
bit table; text node merging after `normalize()` and after `split_text`; attribute
order is insertion order and matters for serialization equality tests.

**Definition of done:** the DOM test corpus (WPT `dom/` subset adopted per §8.6)
passes; every mutation API fires the right observers; serialization round-trips
`XMLSerializer.parse` for a 200-node torture tree; zero panics under
`fuzz_dom_api` (random legal call sequences from a grammar of DOM operations).

### §5.7 CSS tokenizer and parser

**Standard basis:** CSS Syntax Level 3 (tokenization + parsing into at-rules/
qualified rules/declarations), CSS Values and Units 4, CSS Color 4 (the color syntaxes
of WBS §6.3 preamble).
**Position:** stylesheets and inline styles, parsed once, stored interned (§4.5).

**Public API sketch**

```rust
pub enum CSSToken { Ident(Atom), Function(Atom), AtKeyword(Atom), Hash { value: Atom, unitless: bool },
                    String(String), BadString, Url(String), BadUrl, Delim(char),
                    Number { value: f64, int: bool, sign: Option<char> },
                    Percentage(f64), Dimension { value: f64, unit: Atom },
                    Whitespace, Colon, Semicolon, Comma, Delims([char; 4]) /* [] () { } */,
                    CDO, CDC, EOF }
pub struct Stylesheet { pub rules: Vec<CSSRule>, pub origin: Origin /* UA|User|Author */,
                        pub base_url: Url, pub media: Option<MediaQueryList> }
pub enum CSSRule { Style(StyleRule), Import(ImportRule), Media(MediaRule), Supports(SupportsRule),
                   FontFace(FontFaceRule), Keyframes(KeyframesRule), Page(PageRule),
                   Namespace(NamespaceRule), Charset(String), Layer(LayerRule), Unknown(AtRuleBlock) }
pub struct StyleRule { pub selectors: Vec<Selector>, pub declarations: Vec<Declaration>,
                       pub specificity: Specificity, pub source_location: SourceLocation }
pub struct Declaration { pub name: Atom, pub value: Vec<CSSToken>, pub important: bool,
                         pub custom: bool /* name starts with -- */, pub location: SourceLocation }
```

**Core algorithm**

- **Tokenizer:** per CSS Syntax §4.1 with the standard's consumed/comment-stripped
  input stream; escapes (`\` + 1–6 hex + optional whitespace, literal-escape for
  non-ident chars); numeric parsing including the `-0` and exponent cases; URL token
  with its own escapes; bad-string/bad-url recovery. No regexes anywhere in this path.
- **Parser:** a top-down recursive parser producing the rule tree, implementing
  error recovery exactly ("parse error, consume the remnants of a declaration /
  block, return nothing") — malformed declarations are dropped individually;
  malformed rules drop to the enclosing block boundary. Custom properties (`--*`)
  are token-preserving (stored as token lists, substituted at computed-value time).
- **Values:** each property's value grammar is implemented by the property registry
  (§5.8.4) using shared value parsers: lengths/percentages (with number/length
  distinction — a number is *not* a length except where the property says so, e.g.
  `line-height`/`flex-grow`), colors (hex 3/4/6/8, `rgb()/rgba()`, `hsl()/hsla()`,
  `hwb()`, `lab()/lch()/oklab()/oklch()` parse; `color()` with `srgb`/`display-p3`/
  `srgb-linear`/`xyz-d65`, `system-color` keywords, the 148 named colors of Appendix A,
  `transparent`/`currentColor`, `color-mix()`), angles, times, resolutions, `calc()`
  (parse into a typed op tree; evaluate at computed-value time with unit algebra),
  keywords per property, and shorthands expanding into longhands with the standard's
  exact expansion rules (including `border` resetting `border-image`).
- **At-rules:** the WBS §6.4 set; unknown at-rules are *preserved* as `Unknown` blocks
  (needed for `@supports`-like forward-compat behavior and DevTools display).

**Invariants:** the parser is total on any byte string; parse errors are recoverable
and reported with line/column for DevTools; interning makes `StyleRule` equality
cheap; a stylesheet is immutable after parse (media-condition changes create new
matching state, they do not mutate the sheet).

**Pitfalls:** CDO/CDC tokens *inside* qualified rules are just tokens; `!important`
placement (space-separated after value, before semicolon); `@media` nested blocks
are rules, not declarations; unitless zero is a length, unitless nonzero is not;
dimension tokens with weird units (`12apples`) are valid tokens, invalid values —
that distinction happens at the property registry, not the tokenizer.

**Definition of done:** CSS Syntax test corpus from css-parsing-tests passes
(component value lists, declarations, rules, color, an+b); every WBS §6.3 property's
grammar parses and round-trips through `serialize`; fuzz `fuzz_css` 30 min no panics.

### §5.8 Selectors, cascade, and computed values

**Standard basis:** Selectors Level 4, CSS Cascading and Inheritance 4/5, CSS Conditional
(media/supports), CSSOM (style attributes, `getComputedStyle` shape).
**Position:** DOM + stylesheets → computed styles per element.

**Public API sketch**

```rust
pub struct Selector { pub compound: Vec<SimpleSelector>, pub combinators: Vec<Combinator>, /* right-to-left plan */ }
pub enum SimpleSelector { LocalName(Atom), Namespace(Option<Atom>), ID(Atom), Class(Atom),
                          Attribute { ns, name, op: AttrSelectorOperator /* =,~=,|=,^=,$=,*= */, value },
                          PseudoClass(PseudoClassKind), PseudoElement(PseudoElementKind),
                          Scope, Nesting, Is(Vec<Selector>), Not(Vec<Selector>), Where(Vec<Selector>),
                          Has(Vec<Selector>) /* plan-driven, see below */, NthChild { a, b, of: Option<...> } }
pub struct CascadeData { pub rules: Vec<RuleHash>, pub element_index: SelectorMap,
                         pub inheritance_roots: /* … */ }
pub struct ComputedValues { /* one typed struct per longhand group, e.g.: */
    pub color: Color, pub font: FontValues, pub box_: BoxValues, pub background: BackgroundValues, /* ... */ }
```

**Core algorithm**

1. **Selector matching:** compile selectors into a right-to-left matching plan at
   parse time. Match against the DOM via a `MatchingContext` that walks parent links.
   Pseudo-classes: structural (`:nth-child/an+b` parser per css-syntax an+b tests,
   `:first/last/only-child`, `:nth-of-type` family), link-state (`:link`, `:visited`
   — always styled as unvisited; see §5.18.6 for why), user-state (`:hover` from the
   input system, `:focus`/`:focus-visible`/`:focus-within`, `:active`, `:disabled`,
   `:checked`, `:placeholder-shown`), direction (`:dir(ltr|rtl)` from the bidi state),
   language (`:lang()` from `lang` attribute walk-up), resource-state (`:defined`,
   `:fullscreen` false, `:modal` for dialog), logical (`:is()/:not()/:where()/:has()`
   — `:has()` is implemented as a cached descendant/sibling scan invalidated by the
   invalidation sets of §5.8.6; if it cannot meet budget, it is disabled by ADR with
   the fallback documented).
   Pseudo-elements: `::before`, `::after` (with `content` values: strings, `attr()`,
   counters, `open-quote/close-quote/no-*/normal/none`), `::placeholder`, `::selection`
   (paint stage), `::marker` (list items), `::first-line`/`::first-letter` (block-level
   approximation per the spec's loosened model).
2. **Cascade:** collect matching declarations from UA → user → author sheets plus
   `style` attribute (author-level, highest of author except `!important` inversion),
   animation declarations, and presentation attributes (lowest author tier, below
   author `*` rules). Sort by: origin+importance order per CSS Cascade §6, then
   `@layer` order, then specificity, then source order (stable, last wins).
   `revert`/`revert-layer` roll back to the earlier origin/layer.
3. **Computed values:** resolve every registered longhand: `inherit` (parent's
   computed), `initial` (the per-property initial from WBS §6.3), `unset`
   (inherit-if-inherited else initial), keywords→concrete, relative lengths
   (`em/ex/rem/ch` against font metrics, `vh/vw/vmin/vmax` against the viewport,
   `%` stays percentage where it is percentage-resolved at layout), `calc()` algebra,
   custom property substitution (with cycle detection → the standard's
   invalid-at-computed-value-time → `unset` behavior), `currentColor` resolution,
   defaulting: inherited properties inherit, the rest initial.
4. **The style system cache:** rule nodes deduplicate cascade paths; sibling style
   sharing (elements with identical "relevant link visited" state, tag name, class/id
   sets, attribute sets, and non-matching of "sensitive" selectors share computed
   values until anything invalidates) is implemented behind the same `ComputedValues`
   lookup so disabling it is one flag (correctness first, then measure).

**Invalidation protocol (how changes reach style):** `Document` mutations record
cheap dependency bits: `ElementDescendantsMayMatch::{Class,Id,Attr,State}` sets
maintained from selectors at parse time. A mutation enqueues invalidation of the
narrowest applicable set; unknown selectors (e.g., `:has()`) invalidate the whole
subtree — correct and slow until proven hot by profiling.

**Invariants:** cascade is a pure function of (element position in tree, its
attributes/state, stylesheets, style attribute) — no hidden inputs; computed values
never contain `em`/`%` where the spec says absolute (layout never sees an unresolved
relative unit that was resolvable at style time); `getComputedStyle` returns the
computed values *and* resolved used values for the layout-dependent set
(`width`, `height`, margins, `line-height` as used px, …) by consulting the last
fragment tree, falling back to computed when not laid out.

**Definition of done:** adopted WPT selectors + cascade subset green; a cascade
torture test (origin/importance/layer matrix, 60 cases, hand-written expected
values); style-sharing flag flip changes nothing in any pixel test; restyle of a
100k-element page (generated) meets the §10.1 budget.

### §5.9 Layout I — boxes, block, and inline formatting

**Standard basis:** CSS 2.1 §8–10 (box model, positioning schemes, block/inline),
CSS Display 3 (display ↔ formatting contexts, anonymous boxes), CSS Box Sizing 4,
CSS Overflow 3, CSS Position 3, CSS Multicol 1 (columns), CSS Logical 1.
**Position:** style tree → fragment tree (geometry), on the document thread.

**Public API sketch**

```rust
pub struct Box { pub style: StyleRef, pub children: LayoutBoxChildren /* Block-level | Inline-level */,
                 pub kind: BoxKind /* Block | Inline | FlexItem | GridItem | TableWrapper ... */,
                 pub contents: BoxContents /* Text runs | Replaced { image, intrinsic } | Subtree */ }
pub struct Fragment { pub rect: LogicalRect /* position + size in the containing block */,
                      pub style: StyleRef, pub children: Vec<Fragment>, pub clip: ClipChainId,
                      pub kind: FragmentKind /* same taxonomy as boxes + text runs */ }
pub trait LayoutFlow { fn layout(&self, ctx: &LayoutContext, containing: &ContainingBlock,
                                 layout_in: &LayoutInput) -> LayoutResult; }
```

**Core algorithm**

1. **Box construction:** from the DOM+styles, generate boxes per CSS Display —
   `display:none` prunes; inline/block mixtures produce anonymous boxes; `display:
   contents` is transparent; replaced elements (images, form controls, canvas,
   iframe placeholder) become atomic boxes with intrinsic sizes from their content;
   `::before/::after` generate boxes per `content`.
2. **Block formatting context (BFC):** children stacked vertically; margins collapse
   (the full adjoining-margins algorithm: adjacent siblings, parent/first+last child,
   through zero-height boxes; never through BFC-establishing boxes, floats, or
   `overflow: non-visible` roots); height: `auto` = content, percentage heights
   resolve against the containing block's definite height, `min-/max-` clamped.
   BFC establishment: root, floats, absolutely positioned, `overflow≠visible`, flex/
   grid items, `display: flow-root`, cells/captions.
3. **Inline formatting context:** per CSS 2.1 §10.8 + CSS Text/Inline 3 — line boxes
   built from inline runs: text runs shaped by §5.11, replaced content, inline-blocks
   as atomic inlines; baseline alignment per `vertical-align` (baseline, sub/super,
   `length`, `middle`, `top/bottom` of line box); line height = leading distribution;
   the strut; text wrapping with the §5.11.3 line breaker; `white-space` variants
   (`pre`, `nowrap`, `pre-wrap`, `pre-line`, `break-spaces`); `text-align` including
   `justify` (expand inter-word spaces, disable after forced breaks).
4. **Floats and clearance:** the float placement algorithm (top of current line/last
   float, move down until it fits, offset per `clear`), the float *intrusion* model
   for line boxes (left/right edge shrinking), BFC roots placed beside floats per
   CSS 2.1 §9.5. Floats participate in the parent BFC and are *not* part of line
   box content that flows around them — lines shorten, boxes are moved.
5. **Positioning:** `relative` (offset the placed fragment, keep original space);
   `absolute` (containing block = nearest positioned ancestor's padding box; static
   position fallback for unspecified insets; shrink-to-fit width per §10.3.7 equation);
   `fixed` (viewport); `sticky` (constrained rectangle between containing-block edges
   and its static-position box — computed against scroll state each frame).
   Stacking/containment interactions: `position`/`float`/`display` normalization per
   CSS 2.1 §9.7's table.
6. **Overflow and clipping:** `overflow` per box (scrollable overflow region;
   scrollbars on the shell's scrollable ancestors per `overflow: auto`), `clip-path`,
   `border-radius` clipping pushed to the clip chain for paint (§5.16.3).
7. **Fragmentation (columns):** `columns`/`column-*` — split content into column
   boxes of equal height, `column-fill: balance` default; `break-*` honored at
   block boundaries only in the MVP.

**Invariants:** layout is a pure function of (box tree, computed styles, containing
block, available space, viewport state) — no clocks, no randomness, no DOM reads
outside the passed context (enables the incremental design and tests); percentages
that cannot resolve (indefinite containing block height) become `auto` per spec;
layout never mutates the DOM (no `LayoutNode` writes back) — even where the spec
allows it, style recomputation is the write path.

**Definition of done:** the layout golden-file suite (§8.4) covers margin collapsing
(20 cases), float wrap (15), absolute positioning (12), inline line breaking and
alignment (20), overflow/scroll regions (8); the §7.6/§7.9 milestone demos render
their reference pages with no pixel-diff regressions; `layout` meets §10.1 budgets.

### §5.10 Layout II — flexbox and grid

**Standard basis:** CSS Flexible Box 1, CSS Grid 2 (through subgrid: parse it, treat
as regular grid in the MVP), CSS Box Alignment 3.
**Position:** layout stage; flex/grid containers establish BFC-external contexts.

**Flexbox algorithm (the ordered machine, CSS Flexbox §9):** generate flex items
(including anonymous items from raw text runs); resolve flexible lengths:
1. Determine the base size per item (`flex-basis` → `width/height` → `max-content`
   via intrinsic sizing passes; `box-sizing` honored) with hypothetical main size
   after `min-/max-` clamping.
2. Collect inflexible items (`flex-grow=0` and `flex-shrink=0`, or fixed basis) —
   they freeze.
3. Main-axis distribution: if free space > 0, distribute by `flex-grow` weighted
   (scaled flex shrink factor per spec when freezing); if negative, shrink by
   `flex-shrink` weighted by scaled outer size; iterate freeze- unfreeze rounds per
   §9.7 ("resolve flexible lengths" with the min-content floor honored via
   `min-width: auto` = min-content for items with `overflow: visible`).
4. Cross-axis: items stretched (`align-self: stretch`) or sized; container cross
   size per `align-content` when the container's cross size is `auto`.
5. Line packing (`flex-wrap: wrap`): items grouped by hypothetical main size,
   lines sized by max cross size, `align-content` distributes lines.
6. Main-axis alignment: `justify-content` including `space-between/around/evenly`;
   `gap` applies between items/lines as fixed spacing.
7. Absolute children: static position from the container's content box per alignment
   properties.

**Grid algorithm (CSS Grid §7–12):** parse track lists (`<track-list>` with
`minmax()`, `fit-content()`, repeat notations incl. `auto-fill/auto-fit`); place
items (auto-placement cursor with the dense/sparse algorithms, definite placement by
line numbers/named areas); compute track sizes: base sizes via intrinsic contributions
(min/max-content of the items in the track), then the space-distribution loop:
grow base sizes to match growth limits, then distribute remaining free space per
`fr` factors (find-the-size-of-an-unspecified-fr algorithm, §12.7); align (`justify-*`,
`align-*` per axis, `gap`); build item fragments from the resolved track positions.
Nested/subgrid: parse `subgrid` and implement it as "inherit the parent's tracks in
that axis" if budget allows, else ADR-defer with a parse-only acceptance.

**Invariants:** both algorithms are deterministic and terminate (the flex
freezing loop and the grid distribution loop each have documented maximum rounds and
assert them in debug builds); intrinsic sizing passes (`min-content`,
`max-content`) are implemented as layout requests into the same machinery with an
"available space = infinite" mode — no second code path.

**Definition of done:** flexbox torture suite (the 40 hand-written cases from the
flexbox WPT corpus subset adopted in §8.6) green; grid: the 30-case suite covering
auto-placement, `fr` distribution, named areas, spanning items, alignment; the
§7.13 showcase page's flex/grid sections pixel-match their golden files.

### §5.11 Text, fonts, and shaping

**Standard basis:** CSS Fonts 4, CSS Text 3/4, CSS Writing Modes 4 (horizontal only
in the MVP; `vertical-rl` is a stretch), UAX #14 (line breaking), UAX #9 (bidi),
UAX #29 (grapheme/word segmentation), OpenType shaping via the approved shaper (§3.3).
**Position:** the layout stage consults `aurora_text` for every text run.

**Public API sketch**

```rust
pub struct FontDatabase { /* system fonts + @font-face entries, cached descriptors */ }
impl FontDatabase { pub fn match_font(&self, family_chain: &[FamilySpecified], style: FontStyle,
                     weight: FontWeight, stretch: FontStretch) -> FontHandle; }
pub struct ShapedRun { pub glyphs: Vec<ShapedGlyph>, pub advance: f32, pub font: FontHandle,
                       pub script: Script, pub direction: Direction, pub language: Language }
pub struct ShapedGlyph { pub id: u16, pub cluster: Range<usize>, pub offset: Point, pub advance: f32 }
pub trait TextShaper { fn shape(&self, text: &str, font: &FontHandle, script: Script,
                         language: Option<Language>, features: &FeatureSet) -> ShapedRun; }
pub struct LineBreaker<'a> { /* UAX#14 with CSS white-space overrides */ }
```

**Core behavior**

1. **Font selection:** walk the `font-family` chain (generic `serif/sans-serif/
   monospace/cursive/fantasy/system-ui/ui-serif/...` mapped per platform config);
   match per CSS Fonts §5.2 (weight distance rules, style, stretch). `@font-face`
   (WBS §6.4) contributes local/remote fonts with `font-display` phases
   (block ≤ 3 s, swap, fallback) — loading happens async; text paints with the
   fallback until the face arrives, then relayouts.
2. **Shaping:** Latin and simple scripts shape through the owned shaper (cmap +
   kern + basic ligatures); complex scripts (Arabic, Indic, Hangul jamo, Thai)
   delegate to the approved HarfBuzz-based shaper. Shaping results are cached
   (§4.5) keyed by (text, font, script, features) with an LRU byte budget.
3. **Line breaking:** UAX #14 opportunities, overridden by `white-space`,
   `word-break`, `overflow-wrap` (break-word), `hyphens` (manual `-` only in MVP;
   auto-hyphenation is a stretch), `line-break`, and CJK rules (`line-break: strict`
   table for forbidden-start/end characters). Breaking produces *break points*, the
   inline layout (§5.9.3) chooses among them per line width.
4. **Bidi:** UAX #9 on each block: paragraph direction from `dir`/`direction` +
   first-strong heuristic, embedding levels, reordering at paint time; resolved
   per-inline-run level array stored on the fragment for paint ordering.
5. **Metrics:** the em-box, ascent/descent/line-gap from the font (usWin*/typo*
   per CSS Fonts §4.4 rules), the strut per line box, `letter/word-spacing`
   distribution, `tab-size` expansion, `text-indent` (including `each-line`,
   `hanging`), soft-wrap around shaped runs only at cluster boundaries.

**Invariants:** all measurement goes through shaped runs — never font-size × count
heuristics outside the M4 pre-text placeholder; shaping is cached and immutable;
the same input produces the same run (deterministic shaper settings, no variation
selectors surprises after normalization which is *not* applied — text is measured
as-is per platform conventions).

**Definition of done:** line-break test corpus (100 strings × `white-space` matrix)
matches expected break points; bidi reordering tests (30 mixed-direction cases)
reorder per UAX #9; font fallback chain renders the Unicode sample page without
tofu for Latin/Cyrillic/Greek/Arabic/CJK on the reference platform; shaping cache
respects its budget (test by oversizing the cache and checking eviction).

### §5.12 Images and decoders

**Standard basis:** PNG (RFC 2083 + extensions), JPEG (ITU T.81 baseline + progressive),
GIF89a, BMP, ICO/CUR; CSS `image-*` properties for orientation/sizing; `<img>`
/`<picture>`/`srcset` behaviors from WHATWG HTML §4.8.
**Position:** network/cache → decoder → frame cache → layout intrinsic size → paint.

**Public API sketch**

```rust
pub trait ImageDecoder { fn sniff(&self, bytes: &[u8]) -> bool;
                         fn dimensions(&mut self, bytes: &[u8]) -> Result<(u32, u32), DecodeError>;
                         fn frames(&mut self, bytes: &[u8], limits: &DecodeLimits)
                                   -> Result<Vec<DecodedFrame>, DecodeError>; }
pub struct DecodedFrame { pub width: u32, pub height: u32, pub format: PixelFormat /* RGBA8 */,
                          pub delay_ms: Option<u32>, pub dispose: DisposalMethod,
                          pub blend: BlendMethod, pub data: Vec<u8> }
pub struct ImageCache { /* key: Url (+Vary), states: pending→decoded(err|frames), LRU by bytes */ }
```

**Decoders (each its own module, each with its own fuzz target)**

- **PNG:** signature, chunk walker with CRC checks, IHDR constraints, palette
  (`PLTE`/`tRNS`), grayscale/RGB/palette/gray+alpha/RGBA at 1/2/4/8/16 bits,
  filters 0–4 per row, interlacing (Adam7 pass extraction), DEFLATE via the owned
  inflate (or approved crate until it lands), gamma/sRGB-aware conversion is
  clamp-to-sRGB (color management is a stretch), `iCCP` ignored with a console note.
- **JPEG:** baseline sequential DCT first (huffman tables — including the standard
  tables for omission, quantization tables, IDCT (you may use a fixed-point AAN
  implementation — cite it), upsampling of 4:2:0/4:2:2, YCbCr→RGB, grayscale);
  progressive then restart-marker robustness.
- **GIF:** LZW decompression (write it; ~150 lines), interlace, frame composition
  with disposal/blend methods, delay parsing; animation advances on the compositor
  clock (§5.16.6).
- **BMP:** the common header variants (BITMAPINFOHEADER 1/4/8/24/32bpp,
  uncompressed + BI_RGB/BITFIELDS), RLE4/RLE8, bottom-up/top-down, ICO directory +
  embedded BMP/PNG resolution selection.

**Behaviors:** `<img>` fires `load`/`error` correctly with `alt` text rendering on
error; intrinsic size (with EXIF orientation applied — parse the TIFF orientation
tag from JPEG APP1 and `image-orientation: from-image` default per CSS Images 4);
`srcset`/`sizes` selection (density and width descriptors, DPR from the shell);
`loading=lazy` via IntersectionObserver (§5.6); broken-image rendering per the
HTML spec's alt-text rules; SVG images are **not** decoded (an `<svg>` in an `<img>`
renders the placeholder box; recorded as a scope decision in §1.3).

**Invariants:** decoders are total on arbitrary bytes (`DecodeError` or frames,
never a panic — this is the primary fuzz surface); a decoded frame is immutable and
`Arc`-shared; the cache bound is in bytes (Part 10) with LRU eviction and a
"recently requested cannot evict" pin for the current frame.

**Definition of done:** each decoder has a corpus test (the project's own generated
reference set + the public test images checked into `tests/corpus/images/`) with
hash-verified outputs; fuzz targets `fuzz_png`, `fuzz_jpeg`, `fuzz_gif`, `fuzz_bmp`
run clean 30 min each; malformed-image error events verified end to end in the
shell.

### §5.13 JavaScript — lexer and parser

**Standard basis:** ECMA-262 (Living Standard) lexical grammar §11–12 and syntactic
grammar §13–15; the parser produces the standard's node taxonomy.
**Position:** string → AST; the interpreter (§5.14) consumes it.

**Public API sketch**

```rust
pub enum TokenKind { Identifier(Atom), PrivateName(Atom), Keyword(Keyword), Punctuator(Punct),
                     NumericLiteral(NumericValue /* decimal|hex|octal|binary|bigint */),
                     StringLiteral(String), TemplateString { cooked: Option<String>, raw: String },
                     RegexLiteral { pattern: String, flags: String }, NoSubstitutionTemplate, /* etc. */ }
pub struct Lexer<'s> { /* unicode mode, goal symbols (InputElementHashOrRegex Div) */ }
pub struct Parser { options: ParseOptions /* target: script|module, strict default */ }
impl Parser { pub fn parse_script(&mut self, src: &JsString) -> Result<ScriptAst, SyntaxError>;
              pub fn parse_module(&mut self, src: &JsString) -> Result<ModuleAst, SyntaxError>; }
pub enum Expr { Literal, Identifier, Array, Object, Function, Arrow, Class, Call { callee, args, optional_chain },
                Member { obj, prop, computed, optional_chain }, New, Template, TaggedTemplate,
                Conditional, Assign { op, lhs, rhs }, Binary { op, l, r }, Unary, Update,
                Yield, Await, Spread, Sequence, ... }
pub enum Stmt { Var, Function, If, While, DoWhile, For, ForIn, ForOf, Switch, Try, Throw,
                Block, Labeled, Return, Break, Continue, Debugger, Expression, With, Empty, ... }
```

**Core behavior**

- **Lexer:** automatic semicolon insertion (ASI) implemented as the standard's three
  rules applied at parse positions, not as a pass; unicode identifiers (ID_Start/
  ID_Continue via the Unicode crates of §3.3); numeric literals including separators
  (`1_000`), bigint (`10n`), legacy octal only in sloppy non-strict strings; template
  lexing with the substitution-state machine; regex literal vs division disambiguation
  per the goal-symbol rules; strict-mode early errors collected during parse.
- **Parser:** recursive descent with the standard's productions, producing a
  red-green-style AST (trivia-free, with source spans for DevTools and error messages).
  Early errors enforced: duplicate `__proto__` in object literals (lexical vs non),
  strict-mode restrictions (with, octal, delete of unqualified, duplicate params,
  `eval`/`arguments` assignment), `let` disambiguation ([no LineTerminator here]
  rules), private-name scope resolution at parse time, cover grammars
  (`CoverParenthesizedExpressionAndArrowParameterList`, `CoverInitializedName`) —
  implemented exactly, reinterpreted after the lookahead decision.
- **Modules:** `import`/`export` (named, default, namespace, string, attributes),
  the module record with its dependency graph, cycles resolved per spec (function
  hoisting across cycles works), top-level `await` allowed.
- The AST is Arena-allocated; nodes carry spans; the parser is total on any string
  (`SyntaxError` with line/column, never a panic).

**Definition of done:** adopted test262 subset for lexer/parser (`language/`
lexical and early-error tests, §8.6) passes; ASI torture cases (the classic 30)
parse exactly as the standard says; fuzz `fuzz_js_parser` no panics 30 min;
parse of the 1 MB synthetic script meets the §10.1 budget.

### §5.14 JavaScript — interpreter, GC, and builtins

**Standard basis:** ECMA-262 execution semantics (§8–10 execution contexts, §27
control abstractions), Built-ins §18–26, ECMAScript job queue integration from
HTML §8.1.7 (§5.15), Well-Known Symbols.
**Position:** everything script-visible flows through here; the DOM is reached only
via host objects (§5.15.2).

**Public API sketch**

```rust
pub struct JsValue(Inner); // NaN-boxed: double | i32-ish int | pointer to heap cell | special (undefined/null/bool)
pub enum HeapCell { Object(GcObject), String(JsString), Symbol, BigInt, Double, Module, Private, ... }
pub struct GcObject { pub prototype: GcRef<HeapCell>, pub extensible: bool,
                      pub properties: PropertyMap /* shapes + ordered dict for index props */,
                      pub kind: ObjectKind /* Ordinary | Function(FunctionFlags) | Array |
                       exotic: String|Arguments|TypedArray|Proxy|BoundFunction|ModuleNamespace | Host(HostObjectRef) */ }
pub struct Interpreter { heap: GcHeap, realm: Realm, frames: Vec<CallFrame>, /* ... */ }
impl Interpreter { pub fn run_script(&mut self, ast: ScriptAst) -> Result<JsValue, ThrownValue>;
                   pub fn call(&mut self, f: JsValue, this: JsValue, args: &[JsValue]) -> Result<JsValue, ThrownValue>; }
pub enum ThrownValue { Js(JsValue) /* the thrown object */, Host(HostError) /* stack overflow, OOM, interrupt */ }
```

**Core design**

1. **Values:** NaN-boxing in 64 bits; strings are rope-and-slice with a flattening
   threshold and UTF-16 semantics (the spec's string indexing is UTF-16; store
   Latin-1-optimized and UTF-16, convert on demand); symbols and bigints are heap
   cells; integers stay int-tagged until double semantics are required (the
   "int fallback" in `ToNumber` paths must remain invisible — all arithmetic goes
   through spec operations).
2. **Objects & property lookup:** shapes (hidden classes) for ordinary objects —
   transitions on add, dictionary mode after a delete storm; indexed elements in a
   packed/dictionary pair; exotic behaviors implemented per spec: array index
   fast path with `length` invariants, `arguments` mapping, string index reads,
   typed array bounds/canonical-numeric-index-string rules, `Proxy` traps invoked
   in spec order, bound functions. Prototype chains and `[[Get]]`/`[[Set]]`
   exactly per spec (setter walk, receiver identity, `super` homes).
3. **Interpreter:** a tree-walking interpreter over the AST for M9 (with in-frame
   local slots, name resolution compiled to environment-chain depth at parse time —
   do a "bytecode-ish" annotation pass), upgraded to a register bytecode VM as a
   stretch (ADR first). Closures capture environments, not variables; `var` hoists
   into function environments; `let/const` live in block environments with the TDZ
   enforced by an uninitialized marker that throws exactly `ReferenceError`.
4. **Functions:** ordinary (sloppy/strict), arrow (`this` lexical, no
   `arguments`), methods, generators (`yield` suspends the frame into a resumable
   state machine — implemented as an explicit frame save, not OS threads),
   async functions (return promises; `await` parks the frame on the job queue),
   async generators, class constructors with `[[Call]]` vs `[[Construct]]`
   separation and `new.target`, private fields/methods with brand checks.
5. **Promises & jobs:** `Promise` states/reactions per spec; reaction jobs and
   microtasks drain at the host's microtask checkpoint (§5.15.3); `queueMicrotask`
   supported; unhandled-rejection tracking (fires `unhandledrejection` on the
   window after the checkpoint).
6. **GC:** mark-sweep over `HeapCell`s: roots = realm globals, running frames'
   handles, host-registered roots (DOM nodes reachable from script via the binding
   layer register their script-visible wrappers as roots), microtask payloads.
   Incremental marking begins when the heap exceeds the budget of Part 10; a
   write barrier on object field stores keeps incremental marking sound. `WeakRef`
   and `FinalizationRegistry` hook the sweep phase. GC runs on the document
   thread between tasks; a GC can be interrupted at safepoints (loop back-edges,
   calls) by the *interrupt* mechanism that also serves stop-the-script (§5.15.6).
7. **Builtins:** the WBS §6.7 inventory — implemented with the same interpreter
   primitives; `Function.prototype.toString` returns a source slice (real
   implementation, not "[native code]", because we have spans); `Intl` provides
   only the `Intl` object with `NotSupportedError`-style graceful stubs.

**Invariants:** spec algorithms are implemented as spec-named functions
(`ordinary_get`, `array_species_create`, …) so test262 failures map to code;
host objects can do nothing the interpreter cannot observe (no direct heap writes);
a thrown script value never crosses into host code except as an explicit
`Result::Err(ThrownValue)`; the GC is never entered re-entrantly from a builtin
(mid-builtin safepoints defer GC to the next check).

**Definition of done:** the adopted test262 subset (§8.6 scope) passes at the
recorded percentage *per section* with failures listed in `PROGRESS.md`; the GC
stress test (allocation storm under `--gc-stress` collecting at every allocation)
passes the suite; `fuzz_js_runtime` (generated AST mutator) runs without panics
or unsoundness reports 30 min; the todo/fib/JSON benchmark triple meets §10.1.

### §5.15 Runtime — event loop, timers, and bindings

**Standard basis:** HTML §8.1.7 (event loop), §8.1.4 (tasks/microtasks), WebIDL
(bindings semantics), Console Standard, Encoding Standard, Fetch API subset.
**Position:** the document thread's heart; owns the script realm and the DOM.

**Public API sketch**

```rust
pub struct EventLoop { task_queues: VecDeque<Task>, microtask_queue: VecDeque<Job>,
                       timers: TimerWheel, render_scheduler: FrameTicker, /* ... */ }
impl EventLoop { pub fn spin(&mut self, deadline: Instant) -> LoopOutcome;
                 pub fn enqueue_task(&mut self, source: TaskSource, t: Task);
                 pub fn enqueue_microtask(&mut self, j: Job); }
pub struct ScriptRuntime { pub interpreter: Interpreter, pub realm: Realm,
                           pub bindings: BindingRegistry, pub console: ConsoleBuffer, /* ... */ }
```

**Core behavior**

1. **Event loop:** per document thread. Sources (task queues): DOM manipulation,
   user interaction, networking (response hand-off), history/navigation, timers
   (implemented as a timer wheel, 4 ms clamping applies to nested `setTimeout` per
   HTML §8.1.4.2), rendering updates. Ordering: select the oldest runnable task
   (per-source FIFOs with the rendering source's special cadence below), run it,
   run the **microtask checkpoint**, run the rendering update if scheduled
   (rAF callbacks → style → layout → paint → observer callbacks), repeat.
   Idle deadlines feed `requestIdleCallback` (non-breaking best effort).
2. **Bindings (WebIDL subset):** each WBS §6.6 interface gets a generated-by-hand
   binding module: prototype chains with the correct `Symbol.toStringTag`,
   `instanceof` via prototype walking (plus brand checks for typed host objects),
   attribute getters/setters (with `[LegacyUnforgeable]` behavior where specified,
   e.g. `Node.node*`, `window.location`), method overloads resolved by WebIDL
   argument-count/types, optional/defaulted args, dictionary inits, enum/union
   conversions, `DOMException` mapping (§4.6 class 3), callback invocations from
   host to script wrapped in try/into-thrown. String conversions: IDL
   `DOMString`/`USVString`/`ByteString` semantics, including lone-surrogate
   replacement for `USVString`.
3. **Timers:** `setTimeout/setInterval/clearTimeout/clearInterval`, clamping,
   pass-through args, `this` binding rules (window in sloppy), timeout cookie
   invalidation on navigation.
4. **Console:** the Console Standard's formatting (`%s/%d/%f/%o/%c`-aware message
   formatting kept simple: concatenation + JSON for plain objects), buffer with
   the DevTools ring size, counters/timers/groups, assertion.
5. **Fetch API:** `fetch()` with `Request/Response/Headers/URLSearchParams/AbortController`
   per the Fetch standard subset: same-origin and CORS-simple requests (see §5.18.3),
   body streaming as full-buffer MVP, `AbortSignal` cancellation honored in the
   loader channel, response `.json()/.text()/.blob()` implemented.
6. **Script governance:** the interrupt handle (a shared atomic the shell/GC/
   debugger can set) is checked at loop back-edges and call entries; a runaway
   loop can be stopped for navigation, GC, or a DevTools pause. Long-task
   reporting (any task > 50 ms) feeds the DevTools performance view.

**Invariants:** the event loop is the only thing that runs script — parser-suspended
script, tasks, microtasks, and observers all funnel through `spin()`; the
microtask checkpoint happens after *every* task and after script-tasks embedded in
the parser, before rendering (this ordering is observable and tested); no task
captures non-rooted GC handles across a GC point.

**Definition of done:** WPT event-loop/ordering subset green; timer clamping test;
binding round-trip suite (every §6.6 interface constructible, every method
callable with correct exceptions on wrong argument types); a 10-minute soak test
(the showcase page with scripted timers/observers) with stable RSS within budget.

### §5.16 Painting and compositing

**Standard basis:** CSS 2.1 Appendix E (stacking contexts), CSS Position 3,
CSS Backgrounds/Borders 3, CSS Color Adjust/Color 4, CSS Transforms 2 (2D only),
CSS Overflow, CSS Will Change/Contain (as hints).
**Position:** fragment tree → display list → software raster → surface.

**Public API sketch**

```rust
pub enum DisplayItem { SolidColor(Rect, Color), Border { rect, widths: SideRect, styles: SideStyles, radius: CornerRadii },
                       Background { rect, kind: BgKind /* color | image{...} | gradient{...} */, radius },
                       TextRun { run: ShapedRunRef, origin, color, decorations: TextDecorations },
                       Image { frame: Arc<DecodedFrame>, dest: Rect, source: Rect, transform: Transform2D,
                               filters: FilterSet, blend: BlendMode },
                       Shadow(ShadowKind), PushClip(ClipId), PopClip, PushStackingContext(StackingContext), Pop,
                       IframePlaceholder(Rect, frame_id), Scrollbar(ScrollbarKind, Rect, ScrollState) }
pub struct DisplayList { pub items: Vec<DisplayItem>, pub clips: Vec<Clip>, pub epoch: u64 }
pub trait Surface { fn begin(&mut self, dirty: Option<Rect>); fn fill_path(&mut self, path: &Path, paint: &Paint);
                    fn stroke_path(&mut self, path: &Path, paint: &Paint, style: &StrokeStyle);
                    fn blit_image(&mut self, frame: &DecodedFrame, dst: &Rect, transform: &Transform2D, alpha: f32);
                    fn glyphs(&mut self, run: &ShapedRun, origin: Point, paint: &Paint);
                    fn push_clip(&mut self, clip: &Clip); fn pop_clip(&mut self);
                    fn end(&mut self) -> RasterStats; }
```

**Core behavior**

1. **Display list construction:** walk the fragment tree in paint order per CSS 2.1
   Appendix E — per stacking context: background/borders of the establishing box →
   negative-z-index children → in-flow block backgrounds → floats → inline content →
   positioned/z-index≥0/auto descendants → outlines; stacking context triggers:
   root, positioned with z-index ≠ auto, `opacity < 1`, transforms/filters/masks,
   `isolation: isolate`, `will-change` of those, `mix-blend-mode ≠ normal`,
   flex/grid items with z-index, `contain: paint`. Each item records its clip
   (rounded-rect capable) and its `PointerEvents` relevance for hit testing.
2. **Rasterizer (owned, software):** scanline polygon fill with 4×4 supersampled
   coverage (deterministic!), analytic AA edges for axis-aligned rects (the 99%
   case — fast path), border rendering as four trapezoid/ring paths with per-side
   styles (solid/dashed/dotted/double/groove/ridge/inset/outset) and elliptical
   corner joins, image blits with bilinear resampling and the transform's scale
   choice, glyph rasterization cached per (font, size, glyph, transform-bucket)
   with subpixel positioning quantized to 1/4 px, gradients (linear/radial/
   conic) via dithered scanline interpolation, blend modes (the separable set +
   `multiply/screen/overlay/darken/lighten/color-dodge/burn/hard/soft-light/
   difference/exclusion/hue/saturation/color/luminosity`) in premultiplied space,
   dirty-rect scissored rendering (an item outside the dirty rect is skipped).
3. **Scrolling:** scrollable overflow regions from layout; compositor scrolls
   (shell-owned pans over a cached display list with the raster re-run only on
   content change) — MVP: scroll = full repaint of the scrolled container's clip
   with `will-change: scroll-position` enabling the cached-raster optimization.
   Scrollbars are painted by the shell from `ScrollbarState` (styling `::-webkit-scrollbar`
   is out of scope; `scrollbar-width/color` parse and map to thickness/color).
4. **Hit testing:** the display list doubles as the hit-test index (reverse paint
   order, respecting clips and `pointer-events`); returns the deepest element plus
   the event coordinates mapped into its local space for input (§5.19.3).
5. **Determinism:** raster output is byte-identical for identical display lists
   across runs and platforms (fixed-point accumulation in AA, no SIMD-dependent
   reduction order) — the pixel test suite (§8.5) depends on this.

**Invariants:** the display list is immutable once built; paint reads DOM only
through the fragment tree (no element queries during paint); every clip pushed is
popped (balanced by construction, asserted in debug); memory of the display list
and glyph cache is within the Part 10 budget, measured per frame in DevTools.

**Definition of done:** the pixel test corpus (§8.5) — backgrounds/borders/radius
(30), text (15), images/transforms (10), blends/filters (10), stacking (15) —
matches golden images exactly; the showcase page rasterizes at the §10.1 rate;
`fuzz_paint` (random display lists) never panics or unbalances clips.

### §5.17 Storage and persistence

**Standard basis:** RFC 6265bis (cookies), HTML §11 (Web Storage), RFC 9111 (HTTP
caching), the File System layout of this document (we define it; no standard).
**Position:** engine services behind `aurora_storage`, used by `aurora_net`
(cookies, cache) and the runtime (`localStorage`/`sessionStorage`).

**Public API sketch**

```rust
pub struct CookieJar { /* per eTLD+1 partitioning, persistent */ }
impl CookieJar { fn set_for_url(&mut self, url: &Url, set_cookie: &str, now: Timestamp, https: bool) -> CookieOutcome;
                 fn header_for_url(&mut self, url: &Url, https: bool, cross_site: bool) -> Option<String>; }
pub struct WebStorage { /* localStorage: per-origin persistent; sessionStorage: per-tab, per-origin */ }
pub struct HttpDiskCache { /* RFC 9111-ish: keyed by (method GET/HEAD, url, vary-values) */ }
```

**Behavior**

- **Cookies:** full RFC 6265bis parse (name/value, `Domain`, `Path`, `Expires`/
  `Max-Age`, `Secure`, `HttpOnly`, `SameSite=Lax|Strict|None|default`,
  `Partitioned` accepted and partitioned by top-level site), host-only vs domain
  cookies, path-match, the sort order (longer paths first, earlier creation first),
  expiry (session cookies live with the browser session; persistent with the
  store), the `__Secure-`/`__Host-` prefix rules, secure-only transmission,
  cross-site rules per SameSite (top-level-site computation via the public-suffix
  list file checked into the repo, refreshed per release). JS access via
  `document.cookie` honors `HttpOnly` invisibility.
- **Web Storage:** `localStorage` per origin (5 MB quota in UTF-16 code units,
  `QuotaExceededError` on overflow, `storage` events to same-origin *other* tabs
  — cross-tab notification uses the IPC channel), `sessionStorage` per tab,
  both synchronous maps with string keys/values, persisted atomically
  (write-temp-rename, versioned file format, crash-safe).
- **HTTP disk cache:** entries keyed by normalized request (URL + `Vary` header
  values + method class); stores the full header set + body + `Date` +
  request time; freshness per RFC 9111 §4 (Age calculation, heuristic freshness
  when no explicit freshness and status is heuristically cacheable, `must-revalidate`,
  `no-store` never stored, `no-cache` stored but always revalidated); size-capped
  (Part 10) with LRU eviction on whole entries; transparently feeds §5.2.6.
- **Layout on disk:** `~/.aurora/<profile>/` with `cookies.db` (simple append-only
  log + index rebuilt on open), `storage/` per-origin files, `cache/` sharded by
  URL hash (2-hex-char directories), `prefs.toml` (shell), `hsts.db`. All formats
  carry a version byte (§4.8). A `--profile <dir>` flag selects it; tests use a
  temp profile always.

**Invariants:** every read path validates checksums (per-record CRC32) and
recovers by dropping the corrupt record, not the store; concurrent access from
loader threads is serialized by the storage writer thread; the cookie jar never
blocks the UI thread (reads are pre-computed per navigation on the document
thread).

**Definition of done:** cookie spec suite (the RFC's own examples + the WPT
cookies subset) green; storage quota and `storage` event tests; cache: a mock
server scenario matrix (fresh/stale/revalidate/no-store/vary/age) all green;
crash-simulation test (kill a process mid-write; reopen; store loads).

### §5.18 The security model

**Standard basis:** HTML §7 (origin/agents/SOP), Fetch (CORS), CSP Level 3 (the
directives below), Mixed Content, Referrer Policy, Secure Contexts (subset).
**Position:** `aurora_security` + enforcement hooks at each boundary.

**Behavior**

1. **Origins:** computed at parse (§5.1) and stored on documents/window/host
   handles. Same-origin for tuple origins = scheme+host+port equality; opaque
   origins equal only themselves. Every cross-object access in the runtime —
   `window.parent`, `frames`, `document.domain` (parsed, supported, deprecation
   warnings included) — checks it; a failure throws `SecurityError`.
2. **Document isolation:** each navigation runs in a fresh document context;
   `postMessage` (with origin/targetOrigin checks), `history.pushState` (same
   document), and named targeting are the only cross-document bridges we ship.
3. **CORS:** simple requests pass; preflights (`OPTIONS` with
   `Access-Control-Request-Method/-Headers`) issued when required; response
   checks (`Access-Control-Allow-Origin` incl. wildcard-with-credentials rules,
   `-Allow-Headers`, `-Allow-Methods`, `-Expose-Headers` limiting header reads,
   `-Max-Age` cache). A CORS failure is a network error — the page sees a
   generic `TypeError` on fetch, never the server's reason.
4. **CSP:** parse the header; enforce in the MVP: `default-src`, `script-src`
   (inline blocking incl. hashes `sha256-…` and nonces; `unsafe-inline`/
   `unsafe-eval` honored), `style-src` (same), `img-src`, `connect-src`,
   `font-src`, `frame-src`, `object-src`, `base-uri`, `form-action`;
   `report-uri`/`report-to` POST a JSON report to the endpoint. Violations fire
   the `SecurityPolicyViolationEvent` and block per the directive's rules.
5. **Mixed content:** blockable content (scripts, XHR/fetch, iframes of the
   blockable kind) on `https://` pages from `http://` is blocked with a console
   report; optionally-blockable (images) is allowed but reported (upgrade-insecure-
   requests honored when the CSP directive appears).
6. **Script authority:** script runs with exactly the authority the DOM API
   surface grants: no `fs` access, no process spawning, network only through
   `fetch`/XHR/`WebSocket`/`EventSource`/media-URLs through the loader with all
   §5.18.3 checks. `file:` documents get an opaque-per-directory origin policy:
   each file is its own opaque origin (strict), `file:` fetches to the same
   directory tree are allowed with a console warning (documented deviation, ADR).
7. **`:visited` partitioning:** visited-link styling uses the global history
   store but matches `:visited` styles to a *fixed* set of paint-only properties
   (color, background-color, border-*-color, etc.), never layout-affecting ones —
   the classic timing-attack surface stays closed.
8. **Sandbox flags:** the `sandbox` iframe attribute set (forms, scripts,
   same-origin, popups, top-navigation, pointer-lock, storage-access) enforced at
   the boundary checks above; `Content-Security-Policy: sandbox` honored as the
   document-level equivalent.

**Invariants:** security checks live at the boundary functions (the ~12 choke
points enumerated in `aurora_security::checkpoints`), not scattered — review
greps that list; no security decision depends on unvalidated remote data; a
fuzzed header set cannot enable cross-origin reads.

**Definition of done:** the adopted WPT security subset green; an adversarial
test page (checked into `tests/security/`) exercising each violation class shows
the documented blocked behavior; CSP report payloads validated against the spec's
JSON shape.

### §5.19 Browser shell and DevTools

**Position:** `aurora_shell` — the only consumer of the engine's public API.

**Behavior**

1. **Chrome:** tab strip (drag reorder, close, overflow scroll, favicons, load
   progress bar), omnibox (unified URL/search entry, autocomplete from history +
   bookmarks with keyboard navigation, paste-and-go, security indicator: lock
   icon for valid TLS, info for http, warning for cert-unsupported), navigation
   buttons (back/forward with long-press history, reload/stop), menu with zoom,
   find-in-page (bar with match count, wraps), full-screen toggle, downloads
   popover, downloads page (`aurora://downloads`), history page, bookmarks page
   + bookmark bar toggle, settings page (search engine, homepage, fonts sizes,
   site data clearing, do-not-track toggle which sets the header flag),
   keyboard shortcuts per Appendix F, `Ctrl+T/W/Shift+T` tab lifecycle.
2. **Navigation controller:** per-tab session history (entries with URL, title,
   scroll state, form state MVP: none, favicon), back/forward deltas, reload
   (bypass-cache variant), stop, navigation interception for `target=_blank`
   (new tab) and downloads (`Content-Disposition: attachment` → download manager
   with progress, pause/resume, path picker via platform).
3. **Input routing:** the shell converts platform events → engine input messages:
   mouse (move/enter/leave/down/up with buttons/modifiers), wheel (with
   scroll-chaining into the engine's scroller resolution), keyboard (Appendix F
   maps codes; shortcuts are shell-first, then engine), text input via IME
   composition events, drag-and-drop of files onto the window (creates a
   `file://` navigation or a drop event in-page), hit-test result from the engine
   decides hover/cursor/tooltip.
4. **Rendering integration:** the shell owns the window surface; on
   `FrameReady` it blits the engine raster and draws chrome on top with the same
   rasterizer (chrome is just more display lists). Viewport sync (resize, DPI
   change, zoom via `Ctrl+/-` → page zoom factor in 10% steps) flows down.
5. **DevTools:** `F12` opens a docked panel: **Console** (buffered messages,
   input line with a one-shot script eval in the page realm, object preview as
   structured text), **Inspector** (DOM tree synced on mutations, box-model
   view of the selected node, computed styles, matched rules with stylesheet
   locations), **Network** (request log: method, status, type, size, time,
   waterfall from loader events), **Performance** (task timeline with long-task
   marks, GC marks, frame times). The panel is a *browser page* — built with
   the engine, driven by a local protocol (a JSON-RPC over a Unix socket/
   named pipe, the messages listed in `docs/devtools-protocol.md`) so that
   DevTools tests are engine-level tests, not UI-only.
6. **Accessibility scaffolding:** DOM exposes role/name computation from
   semantics (`<h1>`→heading, `<button>`→button, `alt`, `aria-label`/`role`
   parsed and stored); the platform glue exposes it where a native AT API is
   trivially available (Linux AT-SPI via the platform crate is a stretch); the
   in-engine requirement is the computed tree and the tests of it.

**Definition of done:** every §5.19.1 surface works in the M11 demo; the
keyboard-only walkthrough (Appendix F's list) completes every core task;
DevTools' four panels pass their protocol tests; the shell never reads engine
crates other than `aurora` (enforced by a CI dependency check script).

### §5.19.7 Chrome widget specifications

The chrome's own widgets, drawn with the engine's rasterizer from their own
display lists; each has a defined interaction contract so tests can drive it
synthetically (§8.4):

- **Tab strip:** min/max tab widths with the shrinking algorithm (equal share
  of the available width, pinned tabs excluded), drag-reorder with the 8 px
  threshold, close-button hit box of at least 24×24 CSS px, overflow chevron
  opening the tab list popup, middle-click close, Ctrl+digit activation.
- **Omnibox:** unified entry — on focus, full-text selection; typing opens the
  dropdown with up to 6 rows: URL match (bolded match ranges), search
  suggestion, history with visit count and recency scoring; Enter navigates
  the highlighted row (none highlighted → the raw text, URL if it parses with
  a scheme or dot, else the configured search engine); Escape restores the
  pre-edit text; paste-and-go via the context menu; the security indicator
  (lock/info/warning per §5.19.1) sits left, the reload/stop button right.
- **Find bar:** input + match count + prev/next + close; matches are found on
  the document thread, highlighted via a paint overlay (not DOM mutation),
  `Enter`/`Shift+Enter` cycle, `F3`/`Shift+F3` alias, Escape closes and clears.
- **Downloads popover/page:** rows with filename, size, progress, pause/
  resume/cancel, open-folder via the platform; the popover attaches to the
  toolbar button; the page is `aurora://downloads`.
- **Context menus:** document (back/forward/reload/save/view-source/inspect),
  editable (undo/redo/cut/copy/paste/select-all — wired to §6.11 clipboard
  events), link (open in new tab/copy address), image (open/copy/save).
- **Keyboard focus rings:** every chrome widget has a visible focus ring in
  the same style as page `:focus-visible`; Tab order covers all chrome and
  never traps (§5.19 keyboard-only walkthrough).

## PART 6 — Work Breakdown Structure (WBS)

### §6.1 How to read and use the WBS

- The WBS is the project's **task ledger**. Every `- [ ]` item is a task with an
  acceptance bar: done means implemented, tested (§8), and reported (§11.4).
- Items are grouped by surface, not by crate: one item may touch several crates;
  the crate map (§4.3) tells you where the code lives.
- Milestones (Part 7) reference these sections; each milestone names the subset
  of items it claims. An item checked in a later milestone than its surface's
  "home" milestone is normal — the ledger records *when*, the WBS records *what*.
- Where a checklist row and the standard disagree, the standard wins (§0.4).
  Rows marked *verify* contain values you must confirm against the spec when you
  implement them — treat them as leads, not answers.
- Tick items **in the same session** that completes them (§12.1). A checked item
  without a passing-test reference in the session log is treated as unchecked by
  the final audit (§12.7).
- This section is generated from `tools/wbs_data_*.py`; regenerate with
  `python3 tools/generate_wbs.py` after editing the data files, and never
  hand-edit the generated text — edit the data, regenerate, review the diff.


### §6.2 HTML element checklist

One block per element of the supported surface (§1.3). Legacy elements are
parse-compatible only; `non-goal` surfaces stay stubs by design.

#### `<html>` — root
- Content model: One head, one body.
- Parser behavior: In body mode the start tag is merged into the existing element.
- UA defaults: display:block.
- Layout: Establishes the initial containing block; quirks mode holder.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<head>` — metadata
- Content model: Metadata content only.
- Parser behavior: In head insertion mode; most stray content pops it.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<title>` — metadata
- Content model: Text.
- Parser behavior: RCDATA; text becomes the document title.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<base>` — metadata
- Content model: Empty.
- Parser behavior: Sets the document base URL (first wins).
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<link>` — metadata
- Content model: Empty.
- Parser behavior: Triggers stylesheet/favicon/preload handling per rel.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<meta>` — metadata
- Content model: Empty.
- Parser behavior: charset form restarts encoding detection (§5.5.5).
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<style>` — metadata
- Content model: Raw text.
- Parser behavior: Contents parsed as CSS, appended to the document sheets.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<script>` — metadata
- Content model: Script data.
- Parser behavior: Execution queued per async/defer/type (§5.5.6).
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<noscript>` — metadata
- Content model: Depends on scripting flag.
- Parser behavior: When scripting: RAWTEXT; contents not rendered.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<template>` — metadata
- Content model: Its own content fragment.
- Parser behavior: Contents go to a DocumentFragment, invisible to the open-element stack.
- UA defaults: display:none.
- Layout: Contents lay out only when adopted into the tree.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<slot>` — metadata
- Content model: Transparent.
- Parser behavior: Shadow-DOM slotting target.
- UA defaults: display:contents.
- Layout: Distributes assigned nodes in the flat tree.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<body>` — section
- Content model: Flow content.
- Parser behavior: Implied when tokens hit it.
- UA defaults: display:block.
- Layout: Scrolling box owner in the default document structure.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<header>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<footer>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<main>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<section>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<nav>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<article>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<aside>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<address>` — section
- Content model: Flow content.
- Parser behavior: Generic.
- UA defaults: display:block; font-style:italic.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<h1>` — section
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:block; bold; size scale by level.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<h2>` — section
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:block; bold; scale.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<h3>` — section
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:block; bold; scale.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<h4>` — section
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:block; bold; scale.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<h5>` — section
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:block; bold; scale.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<h6>` — section
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:block; bold; scale.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<hgroup>` — section
- Content model: Heading content.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<p>` — grouping
- Content model: Phrasing.
- Parser behavior: Implied end tags; closes an open p.
- UA defaults: display:block; margins 1em.
- Layout: Block container; margin-collapsing showcase.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<hr>` — grouping
- Content model: Empty.
- Parser behavior: Generic.
- UA defaults: display:block; border:1px inset; margin.
- Layout: Atomic block; replaced-ish paint.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<pre>` — grouping
- Content model: Text.
- Parser behavior: Newline after open tag dropped; preserves whitespace.
- UA defaults: display:block; monospace; pre.
- Layout: Inline layout with pre white-space.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<blockquote>` — grouping
- Content model: Flow.
- Parser behavior: Generic.
- UA defaults: display:block; margins 1em 40px.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<ol>` — grouping
- Content model: Zero+ li.
- Parser behavior: Generic.
- UA defaults: display:block; padding-inline-start:40px; list-style-type:decimal.
- Layout: List-item container generating markers.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<ul>` — grouping
- Content model: Zero+ li.
- Parser behavior: Generic.
- UA defaults: display:block; padding-inline-start:40px; list-style-type:disc.
- Layout: List-item container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<menu>` — grouping
- Content model: Zero+ li.
- Parser behavior: Semantic alias of ul.
- UA defaults: display:block; list-style-type:disc.
- Layout: List-item container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<li>` — grouping
- Content model: Flow.
- Parser behavior: Implied end tags between siblings.
- UA defaults: display:list-item.
- Layout: Marker box + block/inline content.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<dl>` — grouping
- Content model: dt/dd groups.
- Parser behavior: Generic.
- UA defaults: display:block; margin:1em 0.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<dt>` — grouping
- Content model: Flow.
- Parser behavior: Implied end tags.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<dd>` — grouping
- Content model: Flow.
- Parser behavior: Implied end tags.
- UA defaults: display:block; margin-inline-start:40px.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<figure>` — grouping
- Content model: Flow + figcaption.
- Parser behavior: Generic.
- UA defaults: display:block; margin:1em 40px.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<figcaption>` — grouping
- Content model: Flow.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<div>` — grouping
- Content model: Flow.
- Parser behavior: Generic.
- UA defaults: display:block.
- Layout: Block container; the default case.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<a>` — text
- Content model: Transparent.
- Parser behavior: Implied end tags on block nesting.
- UA defaults: display:inline; color:-webkit-link; text-decoration:underline.
- Layout: Inline; activation behavior navigates (§5.6).
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<em>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; font-style:italic.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<strong>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; font-weight:bolder.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<small>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; font-size:smaller.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<s>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; text-decoration:line-through.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<cite>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; font-style:italic.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<q>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; quotes auto.
- Layout: Inline; generated quote marks per content/quotes.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<dfn>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; font-style:italic.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<abbr>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: Inline; title tooltip source.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<ruby>` — text
- Content model: Phrasing.
- Parser behavior: Special in-body handling.
- UA defaults: display:ruby.
- Layout: Ruby annotation layout (MVP: inline fallback).
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<rt>` — text
- Content model: Phrasing.
- Parser behavior: Only inside ruby.
- UA defaults: display:ruby-text.
- Layout: MVP: inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<rp>` — text
- Content model: Phrasing.
- Parser behavior: Only inside ruby.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<code>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; monospace.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<kbd>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; monospace.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<samp>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; monospace.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<var>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; font-style:italic.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<time>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: Inline; datetime attribute parsed.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<data>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: Inline; value attribute.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<bdi>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; unicode-bidi:isolate.
- Layout: Inline; isolate in bidi pass.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<bdo>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline; unicode-bidi:bidi-override.
- Layout: Inline; override direction from dir.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<span>` — text
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: Inline; the inline default case.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<br>` — text
- Content model: Empty.
- Parser behavior: Generic.
- UA defaults: display:inline? (break).
- Layout: Forced line break in inline layout.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<wbr>` — text
- Content model: Empty.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: Soft wrap opportunity.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<ins>` — edits
- Content model: Transparent.
- Parser behavior: Generic.
- UA defaults: display:inline; text-decoration:underline.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<del>` — edits
- Content model: Transparent.
- Parser behavior: Generic.
- UA defaults: display:inline; text-decoration:line-through.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<img>` — embedded
- Content model: Empty.
- Parser behavior: Fires load/error async (§5.12).
- UA defaults: display:inline (replaced).
- Layout: Atomic inline; intrinsic size + object-fit.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<picture>` — embedded
- Content model: source+img.
- Parser behavior: Generic; sources pick the URL.
- UA defaults: display:inline.
- Layout: No box itself.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<source>` — embedded
- Content model: Empty.
- Parser behavior: Only inside picture/audio/video.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<iframe>` — embedded
- Content model: Fallback content.
- Parser behavior: Creates a nested browsing context placeholder.
- UA defaults: display:inline (replaced); border:2px inset.
- Layout: Atomic inline box, nested document area.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<embed>` — embedded
- Content model: Empty.
- Parser behavior: Plugin content placeholder.
- UA defaults: display:inline (replaced).
- Layout: Atomic inline placeholder box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<object>` — embedded
- Content model: Fallback content.
- Parser behavior: Placeholder; data URL fetch attempt.
- UA defaults: display:inline (replaced).
- Layout: Atomic inline placeholder.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<video>` — embedded
- Content model: Source elements + fallback.
- Parser behavior: Placeholder box, media element stub.
- UA defaults: display:inline (replaced).
- Layout: Atomic inline; intrinsic from attributes if any.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<audio>` — embedded
- Content model: Source elements.
- Parser behavior: No visual box; media stub with controls.
- UA defaults: display:none unless controls.
- Layout: Controls UI when controls attribute present.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<track>` — embedded
- Content model: Empty.
- Parser behavior: Only inside media elements.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<map>` — embedded
- Content model: Transparent.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<area>` — embedded
- Content model: Empty.
- Parser behavior: Only inside map.
- UA defaults: display:none.
- Layout: No box; hit regions parsed.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<table>` — tabular
- Content model: caption/colgroup/sections.
- Parser behavior: Foster parenting of stray content (§5.5).
- UA defaults: display:table; border-spacing 2px; border-collapse:separate.
- Layout: Table layout: anonymous boxes for stray rows/cells.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<caption>` — tabular
- Content model: Flow.
- Parser behavior: Only as first table child.
- UA defaults: display:table-caption.
- Layout: Caption box above/below per side.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<colgroup>` — tabular
- Content model: col elements.
- Parser behavior: Generic.
- UA defaults: display:table-column-group.
- Layout: Column grouping for spans.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<col>` — tabular
- Content model: Empty.
- Parser behavior: Generic.
- UA defaults: display:table-column.
- Layout: Column sizing contributor.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<tbody>` — tabular
- Content model: tr elements.
- Parser behavior: Implied when rows appear directly.
- UA defaults: display:table-row-group.
- Layout: Row group.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<thead>` — tabular
- Content model: tr elements.
- Parser behavior: Generic.
- UA defaults: display:table-header-group.
- Layout: Row group.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<tfoot>` — tabular
- Content model: tr elements.
- Parser behavior: Generic.
- UA defaults: display:table-footer-group.
- Layout: Row group.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<tr>` — tabular
- Content model: td/th.
- Parser behavior: Implied in row groups.
- UA defaults: display:table-row.
- Layout: Row box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<td>` — tabular
- Content model: Flow.
- Parser behavior: Implied end tags.
- UA defaults: display:table-cell; padding:1px.
- Layout: Cell box; anonymous row wrapping.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<th>` — tabular
- Content model: Flow.
- Parser behavior: Implied end tags.
- UA defaults: display:table-cell; bold; center.
- Layout: Cell box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<form>` — forms
- Content model: Flow, no nested form.
- Parser behavior: Form pointer management (§5.5).
- UA defaults: display:block.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<label>` — forms
- Content model: Phrasing, no nested label.
- Parser behavior: Activation forwards to labeled control (§5.6).
- UA defaults: display:inline.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<input>` — forms
- Content model: Empty.
- Parser behavior: Type-driven control construction.
- UA defaults: varies by type (text: inline-block ~size).
- Layout: Replaced-ish inline-block controls.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<button>` — forms
- Content model: Phrasing.
- Parser behavior: Default button type; activation behavior.
- UA defaults: display:inline-block; UA chrome styling.
- Layout: Inline-block control with border/background defaults.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<select>` — forms
- Content model: option/optgroup.
- Parser behavior: Option list construction.
- UA defaults: display:inline-block.
- Layout: Control; popup list is shell UI.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<datalist>` — forms
- Content model: options.
- Parser behavior: Provides suggestions; renders nothing.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<optgroup>` — forms
- Content model: options.
- Parser behavior: Only inside select.
- UA defaults: display:block (in list UI).
- Layout: List UI group.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<option>` — forms
- Content model: Text.
- Parser behavior: Selectedness rules.
- UA defaults: display:block (in list UI).
- Layout: List UI entry.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<textarea>` — forms
- Content model: Text.
- Parser behavior: RCDATA default value.
- UA defaults: display:inline-block; monospace; resize both.
- Layout: Multi-line editable control.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<output>` — forms
- Content model: Phrasing.
- Parser behavior: Generic.
- UA defaults: display:inline.
- Layout: Inline.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<progress>` — forms
- Content model: Phrasing.
- Parser behavior: Determinateness from value/max.
- UA defaults: display:inline-block.
- Layout: Replaced-ish; bar rendering.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<meter>` — forms
- Content model: Phrasing.
- Parser behavior: Gauge from value/min/max/low/high/optimum.
- UA defaults: display:inline-block.
- Layout: Replaced-ish; gauge rendering.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<fieldset>` — forms
- Content model: Flow + legend.
- Parser behavior: Generic.
- UA defaults: display:block; border groove; margins.
- Layout: Block container; legend special-placed.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<legend>` — forms
- Content model: Phrasing.
- Parser behavior: First-child special case.
- UA defaults: display:block; padding; float rules.
- Layout: Renders in the fieldset border gap.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<details>` — interactive
- Content model: summary + flow.
- Parser behavior: Toggle via name group (§5.6).
- UA defaults: display:block.
- Layout: Content hidden unless open.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<summary>` — interactive
- Content model: Phrasing + heading.
- Parser behavior: First summary is the widget.
- UA defaults: display:block; list-item marker.
- Layout: Marker + click target.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<dialog>` — interactive
- Content model: Flow.
- Parser behavior: open attribute; top layer when modal.
- UA defaults: display:none; block when open.
- Layout: Top-layer rendering with ::backdrop.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<canvas>` — embedded
- Content model: Fallback.
- Parser behavior: Bitmap backing store (§5.6 ctx2d).
- UA defaults: display:inline (replaced).
- Layout: Atomic inline; default 300x150.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<center>` — legacy
- Content model: Flow.
- Parser behavior: Treated as div with presentational hint.
- UA defaults: display:block; text-align:center.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<font>` — legacy
- Content model: Phrasing.
- Parser behavior: Presentational hints: face/size/color.
- UA defaults: display:inline.
- Layout: Inline; hints map to style.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<marquee>` — legacy
- Content model: Phrasing.
- Parser behavior: Parse; render as static block (non-goal animation).
- UA defaults: display:block; overflow:hidden.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<frameset>` — legacy
- Content model: frame/frameset.
- Parser behavior: Parse-only; content not rendered (non-goal).
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<frame>` — legacy
- Content model: Empty.
- Parser behavior: Parse-only.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<noframes>` — legacy
- Content model: Anything.
- Parser behavior: RAWTEXT in frameset docs.
- UA defaults: display:none.
- Layout: No box.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<xmp>` — legacy
- Content model: Text.
- Parser behavior: RAWTEXT; literal text.
- UA defaults: display:block; monospace; pre.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

#### `<plaintext>` — legacy
- Content model: Text.
- Parser behavior: Rest of document becomes text tokens.
- UA defaults: display:block; monospace; pre.
- Layout: Block container.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element.

### §6.3 CSS property checklist

The engine's supported property surface. `Initial` values marked *verify*
must be confirmed against the property's CSSWG definition during
implementation; everything else follows §0.4 precedence. Shorthands are
expansion sugar — their longhands carry the real behavior.

#### Group `box` — §5.9/§5.10 — box generation, display, positioning; layout owns geometry
- 36 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `display`
- Inherited: No · Initial: `inline` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `box-sizing`
- Inherited: No · Initial: `content-box` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin`
- Inherited: No · Initial: `see longhands (0)` · Owner: box · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-top`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-right`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-bottom`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-left`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding`
- Inherited: No · Initial: `see longhands (0)` · Owner: box · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-top`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-right`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-bottom`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-left`
- Inherited: No · Initial: `0` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `width`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `height`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `min-width`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `min-height`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `max-width`
- Inherited: No · Initial: `none` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `max-height`
- Inherited: No · Initial: `none` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `inset`
- Inherited: No · Initial: `auto` · Owner: box · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `top`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `right`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `bottom`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `left`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `position`
- Inherited: No · Initial: `static` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `z-index`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `float`
- Inherited: No · Initial: `none` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `clear`
- Inherited: No · Initial: `none` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overflow`
- Inherited: No · Initial: `see longhands (visible)` · Owner: box · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overflow-x`
- Inherited: No · Initial: `visible` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overflow-y`
- Inherited: No · Initial: `visible` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `visibility`
- Inherited: Yes · Initial: `visible` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `object-fit`
- Inherited: No · Initial: `fill` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `object-position`
- Inherited: No · Initial: `50% 50%` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `aspect-ratio`
- Inherited: No · Initial: `auto` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `container-type`
- Inherited: No · Initial: `normal` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `container-name`
- Inherited: No · Initial: `none` · Owner: box · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `misc` — multiple — cross-cutting properties
- 6 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `all`
- Inherited: No · Initial: `see individual (reset)` · Owner: misc · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `image-rendering`
- Inherited: Yes · Initial: `auto` · Owner: misc · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `image-orientation`
- Inherited: No · Initial: `from-image` · Owner: misc · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `shape-outside`
- Inherited: No · Initial: `none` · Owner: misc · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `shape-margin`
- Inherited: No · Initial: `0` · Owner: misc · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `shape-image-threshold`
- Inherited: No · Initial: `0.5? verify` · Owner: misc · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `ui` — §5.19 — interaction surface: cursors, input, scrolling UX
- 28 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `appearance`
- Inherited: No · Initial: `none` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `cursor`
- Inherited: Yes · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `pointer-events`
- Inherited: Yes · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `user-select`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `resize`
- Inherited: No · Initial: `none` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `touch-action`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overscroll-behavior`
- Inherited: No · Initial: `see longhands (auto)` · Owner: ui · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overscroll-behavior-x`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overscroll-behavior-y`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-behavior`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-snap-type`
- Inherited: No · Initial: `none` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-snap-align`
- Inherited: No · Initial: `none` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-snap-stop`
- Inherited: No · Initial: `normal` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-margin`
- Inherited: No · Initial: `see longhands (0)` · Owner: ui · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-margin-top`
- Inherited: No · Initial: `0` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-margin-right`
- Inherited: No · Initial: `0` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-margin-bottom`
- Inherited: No · Initial: `0` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-margin-left`
- Inherited: No · Initial: `0` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-padding`
- Inherited: No · Initial: `see longhands (auto)` · Owner: ui · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-padding-top`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-padding-right`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-padding-bottom`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scroll-padding-left`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `will-change`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `contain`
- Inherited: No · Initial: `none` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `content-visibility`
- Inherited: No · Initial: `visible` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `isolation`
- Inherited: No · Initial: `auto` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mix-blend-mode`
- Inherited: No · Initial: `normal` · Owner: ui · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `flex` — §5.10.1 — flexbox layout module
- 20 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `flex`
- Inherited: No · Initial: `see longhands` · Owner: flex · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `flex-grow`
- Inherited: No · Initial: `0` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `flex-shrink`
- Inherited: No · Initial: `1` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `flex-basis`
- Inherited: No · Initial: `auto` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `flex-direction`
- Inherited: No · Initial: `row` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `flex-wrap`
- Inherited: No · Initial: `nowrap` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `flex-flow`
- Inherited: No · Initial: `see longhands` · Owner: flex · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `order`
- Inherited: No · Initial: `0` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `align-items`
- Inherited: No · Initial: `normal` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `align-self`
- Inherited: No · Initial: `auto` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `align-content`
- Inherited: No · Initial: `normal` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `justify-items`
- Inherited: No · Initial: `legacy` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `justify-self`
- Inherited: No · Initial: `auto` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `justify-content`
- Inherited: No · Initial: `normal` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `gap`
- Inherited: No · Initial: `normal` · Owner: flex · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `row-gap`
- Inherited: No · Initial: `normal` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-gap`
- Inherited: No · Initial: `normal` · Owner: flex · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `place-items`
- Inherited: No · Initial: `see longhands` · Owner: flex · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `place-content`
- Inherited: No · Initial: `see longhands` · Owner: flex · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `place-self`
- Inherited: No · Initial: `see longhands` · Owner: flex · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `grid` — §5.10.2 — grid layout module
- 15 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `grid`
- Inherited: No · Initial: `see longhands` · Owner: grid · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-template`
- Inherited: No · Initial: `see longhands` · Owner: grid · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-template-rows`
- Inherited: No · Initial: `none` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-template-columns`
- Inherited: No · Initial: `none` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-template-areas`
- Inherited: No · Initial: `none` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-auto-rows`
- Inherited: No · Initial: `auto` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-auto-columns`
- Inherited: No · Initial: `auto` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-auto-flow`
- Inherited: No · Initial: `row` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-row`
- Inherited: No · Initial: `see longhands (auto)` · Owner: grid · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-column`
- Inherited: No · Initial: `see longhands (auto)` · Owner: grid · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-area`
- Inherited: No · Initial: `see longhands (auto)` · Owner: grid · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-row-start`
- Inherited: No · Initial: `auto` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-row-end`
- Inherited: No · Initial: `auto` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-column-start`
- Inherited: No · Initial: `auto` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `grid-column-end`
- Inherited: No · Initial: `auto` · Owner: grid · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `table` — §5.9 — table formatting (§5.9.2 box building + anonymous table boxes)
- 5 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `table-layout`
- Inherited: No · Initial: `auto` · Owner: table · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-collapse`
- Inherited: No · Initial: `separate` · Owner: table · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-spacing`
- Inherited: No · Initial: `0` · Owner: table · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `empty-cells`
- Inherited: No · Initial: `show` · Owner: table · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `caption-side`
- Inherited: No · Initial: `top` · Owner: table · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `color` — §5.16 — color and opacity resolve to paint values
- 7 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `color`
- Inherited: Yes · Initial: `canvastext` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `opacity`
- Inherited: No · Initial: `1` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `color-scheme`
- Inherited: Yes · Initial: `normal` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `accent-color`
- Inherited: No · Initial: `auto` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `caret-color`
- Inherited: No · Initial: `auto` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `print-color-adjust`
- Inherited: Yes · Initial: `economic` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `forced-color-adjust`
- Inherited: Yes · Initial: `auto` · Owner: color · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `bg` — §5.16.2 — backgrounds render under content in paint order
- 10 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `background`
- Inherited: No · Initial: `see longhands` · Owner: bg · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-color`
- Inherited: No · Initial: `transparent` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-image`
- Inherited: No · Initial: `none` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-repeat`
- Inherited: No · Initial: `repeat` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-position`
- Inherited: No · Initial: `0% 0%` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-size`
- Inherited: No · Initial: `auto` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-clip`
- Inherited: No · Initial: `border-box` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-origin`
- Inherited: No · Initial: `padding-box` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-attachment`
- Inherited: No · Initial: `scroll` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `background-blend-mode`
- Inherited: No · Initial: `normal` · Owner: bg · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `border` — §5.16.2 — borders render as part of the box edge path
- 37 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `border`
- Inherited: No · Initial: `see longhands (medium none currentcolor)` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-width`
- Inherited: No · Initial: `see longhands (medium)` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-style`
- Inherited: No · Initial: `see longhands (none)` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-color`
- Inherited: No · Initial: `see longhands (currentcolor)` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-top`
- Inherited: No · Initial: `see longhands` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-right`
- Inherited: No · Initial: `see longhands` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-bottom`
- Inherited: No · Initial: `see longhands` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-left`
- Inherited: No · Initial: `see longhands` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-top-width`
- Inherited: No · Initial: `medium` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-right-width`
- Inherited: No · Initial: `medium` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-bottom-width`
- Inherited: No · Initial: `medium` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-left-width`
- Inherited: No · Initial: `medium` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-top-style`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-right-style`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-bottom-style`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-left-style`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-top-color`
- Inherited: No · Initial: `currentcolor` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-right-color`
- Inherited: No · Initial: `currentcolor` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-bottom-color`
- Inherited: No · Initial: `currentcolor` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-left-color`
- Inherited: No · Initial: `currentcolor` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-radius`
- Inherited: No · Initial: `see longhands (0)` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-top-left-radius`
- Inherited: No · Initial: `0` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-top-right-radius`
- Inherited: No · Initial: `0` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-bottom-right-radius`
- Inherited: No · Initial: `0` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-bottom-left-radius`
- Inherited: No · Initial: `0` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-image`
- Inherited: No · Initial: `see longhands` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-image-source`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-image-slice`
- Inherited: No · Initial: `100%` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-image-width`
- Inherited: No · Initial: `1` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-image-outset`
- Inherited: No · Initial: `0` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-image-repeat`
- Inherited: No · Initial: `stretch` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `outline`
- Inherited: No · Initial: `see longhands` · Owner: border · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `outline-width`
- Inherited: No · Initial: `medium` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `outline-style`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `outline-color`
- Inherited: No · Initial: `auto` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `outline-offset`
- Inherited: No · Initial: `0` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `box-shadow`
- Inherited: No · Initial: `none` · Owner: border · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `fragmentation` — §5.9.7 — break control between fragmentainers
- 6 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `box-decoration-break`
- Inherited: No · Initial: `slice` · Owner: fragmentation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `break-before`
- Inherited: No · Initial: `auto` · Owner: fragmentation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `break-after`
- Inherited: No · Initial: `auto` · Owner: fragmentation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `break-inside`
- Inherited: No · Initial: `auto` · Owner: fragmentation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `orphans`
- Inherited: Yes · Initial: `2` · Owner: fragmentation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `widows`
- Inherited: Yes · Initial: `2` · Owner: fragmentation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `transform` — §5.16.1 — transforms create stacking contexts and affect hit testing
- 10 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `transform`
- Inherited: No · Initial: `none` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transform-origin`
- Inherited: No · Initial: `50% 50%` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transform-box`
- Inherited: No · Initial: `view-box` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transform-style`
- Inherited: No · Initial: `flat` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `perspective`
- Inherited: No · Initial: `none` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `perspective-origin`
- Inherited: No · Initial: `50% 50%` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `backface-visibility`
- Inherited: No · Initial: `visible` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `translate`
- Inherited: No · Initial: `none` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `rotate`
- Inherited: No · Initial: `none` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `scale`
- Inherited: No · Initial: `none` · Owner: transform · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `transition` — §5.15 — transition machinery lives in the runtime, paints via interpolators
- 6 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `transition`
- Inherited: No · Initial: `see longhands` · Owner: transition · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transition-property`
- Inherited: No · Initial: `all` · Owner: transition · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transition-duration`
- Inherited: No · Initial: `0s` · Owner: transition · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transition-timing-function`
- Inherited: No · Initial: `ease` · Owner: transition · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transition-delay`
- Inherited: No · Initial: `0s` · Owner: transition · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `transition-behavior`
- Inherited: No · Initial: `normal` · Owner: transition · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `animation` — §5.15 — keyframe machinery lives in the runtime, paints via interpolators
- 11 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `animation`
- Inherited: No · Initial: `see longhands` · Owner: animation · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-name`
- Inherited: No · Initial: `none` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-duration`
- Inherited: No · Initial: `0s` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-timing-function`
- Inherited: No · Initial: `ease` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-delay`
- Inherited: No · Initial: `0s` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-iteration-count`
- Inherited: No · Initial: `1` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-direction`
- Inherited: No · Initial: `normal` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-fill-mode`
- Inherited: No · Initial: `none` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-play-state`
- Inherited: No · Initial: `running` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-composition`
- Inherited: No · Initial: `replace` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `animation-timeline`
- Inherited: No · Initial: `auto` · Owner: animation · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `list` — §5.9/§5.16 — list items generate ::marker boxes
- 4 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `list-style`
- Inherited: No · Initial: `see longhands` · Owner: list · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `list-style-type`
- Inherited: Yes · Initial: `disc` · Owner: list · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `list-style-position`
- Inherited: No · Initial: `outside` · Owner: list · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `list-style-image`
- Inherited: No · Initial: `none` · Owner: list · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `multicol` — §5.9.7 — multi-column fragmentation
- 10 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `columns`
- Inherited: No · Initial: `see longhands (auto)` · Owner: multicol · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-count`
- Inherited: No · Initial: `auto` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-width`
- Inherited: No · Initial: `auto` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-gap`
- Inherited: No · Initial: `normal` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-rule`
- Inherited: No · Initial: `see longhands (medium none currentcolor)` · Owner: multicol · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-rule-width`
- Inherited: No · Initial: `medium` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-rule-style`
- Inherited: No · Initial: `none` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-rule-color`
- Inherited: No · Initial: `currentcolor` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-span`
- Inherited: No · Initial: `none` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `column-fill`
- Inherited: No · Initial: `balance` · Owner: multicol · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `page` — §5.9.7 — paged media (parse-only in MVP)
- 4 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `page`
- Inherited: No · Initial: `auto` · Owner: page · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `page-break-before`
- Inherited: No · Initial: `auto` · Owner: page · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `page-break-after`
- Inherited: No · Initial: `auto` · Owner: page · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `page-break-inside`
- Inherited: No · Initial: `auto` · Owner: page · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `mask` — §5.16.3 — masks and clipping feed the clip chain
- 10 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `clip-path`
- Inherited: No · Initial: `none` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask`
- Inherited: No · Initial: `see longhands` · Owner: mask · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-image`
- Inherited: No · Initial: `none` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-mode`
- Inherited: No · Initial: `match-source` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-repeat`
- Inherited: No · Initial: `repeat` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-position`
- Inherited: No · Initial: `center` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-clip`
- Inherited: No · Initial: `border-box` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-origin`
- Inherited: No · Initial: `border-box` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-size`
- Inherited: No · Initial: `auto` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `mask-composite`
- Inherited: No · Initial: `add` · Owner: mask · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `filter` — §5.16.4 — filters run as raster post-processes
- 2 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `filter`
- Inherited: No · Initial: `none` · Owner: filter · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `backdrop-filter`
- Inherited: No · Initial: `none` · Owner: filter · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `logical` — §5.9 — logical properties map to physical at computed-value time
- 28 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `margin-block`
- Inherited: No · Initial: `see longhands` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-block-start`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-block-end`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-inline`
- Inherited: No · Initial: `see longhands` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-inline-start`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `margin-inline-end`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-block`
- Inherited: No · Initial: `see longhands (0)` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-block-start`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-block-end`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-inline`
- Inherited: No · Initial: `see longhands (0)` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-inline-start`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `padding-inline-end`
- Inherited: No · Initial: `0` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-block-start`
- Inherited: No · Initial: `see longhands` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-block-end`
- Inherited: No · Initial: `see longhands` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-inline-start`
- Inherited: No · Initial: `see longhands` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `border-inline-end`
- Inherited: No · Initial: `see longhands` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `inline-size`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `block-size`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `min-inline-size`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `min-block-size`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `max-inline-size`
- Inherited: No · Initial: `none` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `max-block-size`
- Inherited: No · Initial: `none` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `inset-block-start`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `inset-block-end`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `inset-inline-start`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `inset-inline-end`
- Inherited: No · Initial: `auto` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overflow-block`
- Inherited: No · Initial: `see overflow-x` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overflow-inline`
- Inherited: No · Initial: `see overflow-y` · Owner: logical · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `text` — §5.11/§5.9.3 — text processing and inline layout
- 40 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `line-height`
- Inherited: Yes · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `letter-spacing`
- Inherited: Yes · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `word-spacing`
- Inherited: Yes · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-align`
- Inherited: Yes · Initial: `start` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-align-last`
- Inherited: Yes · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-indent`
- Inherited: Yes · Initial: `0` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-transform`
- Inherited: Yes · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-decoration`
- Inherited: No · Initial: `see longhands (none)` · Owner: text · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-decoration-line`
- Inherited: No · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-decoration-style`
- Inherited: No · Initial: `solid` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-decoration-color`
- Inherited: No · Initial: `currentcolor` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-decoration-thickness`
- Inherited: No · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-underline-offset`
- Inherited: No · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-decoration-skip-ink`
- Inherited: No · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-emphasis`
- Inherited: No · Initial: `see longhands` · Owner: text · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-emphasis-style`
- Inherited: No · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-emphasis-color`
- Inherited: No · Initial: `currentcolor` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-shadow`
- Inherited: Yes · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `white-space`
- Inherited: Yes · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `word-break`
- Inherited: No · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `overflow-wrap`
- Inherited: No · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `word-wrap`
- Inherited: No · Initial: `alias of overflow-wrap` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `hyphens`
- Inherited: Yes · Initial: `manual` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `tab-size`
- Inherited: Yes · Initial: `8` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `line-break`
- Inherited: No · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `vertical-align`
- Inherited: No · Initial: `baseline` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `direction`
- Inherited: Yes · Initial: `ltr` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `unicode-bidi`
- Inherited: No · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `writing-mode`
- Inherited: No · Initial: `horizontal-tb` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-orientation`
- Inherited: No · Initial: `mixed` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-combine-upright`
- Inherited: No · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-rendering`
- Inherited: No · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-justify`
- Inherited: Yes · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `text-underline-position`
- Inherited: Yes · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `initial-letter`
- Inherited: No · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `quotes`
- Inherited: Yes · Initial: `auto` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `content`
- Inherited: No · Initial: `normal` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `counter-reset`
- Inherited: No · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `counter-increment`
- Inherited: No · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `counter-set`
- Inherited: No · Initial: `none` · Owner: text · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### Group `font` — §5.11.1 — font selection, matching, metrics (inherited by default)
- 22 properties; work each block's five tasks in order; the group owner section defines what integration means here.

#### `font`
- Inherited: Yes · Initial: `see longhands` · Owner: font · Shorthand: expands into its longhands (order and reset semantics per CSS Cascading §7).
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-family`
- Inherited: Yes · Initial: `per UA stylesheet` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-size`
- Inherited: Yes · Initial: `medium` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-weight`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-style`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-stretch`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant-ligatures`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant-caps`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant-numeric`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant-east-asian`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant-position`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variant-alternates`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-kerning`
- Inherited: Yes · Initial: `auto` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-feature-settings`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-variation-settings`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-size-adjust`
- Inherited: Yes · Initial: `none` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-synthesis-weight`
- Inherited: Yes · Initial: `auto` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-synthesis-style`
- Inherited: Yes · Initial: `auto` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-synthesis-small-caps`
- Inherited: Yes · Initial: `auto` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-optical-sizing`
- Inherited: Yes · Initial: `auto` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

#### `font-language-override`
- Inherited: Yes · Initial: `normal` · Owner: font · Longhand: covered by its shorthand's expansion, if any.
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output.

### §6.4 CSS at-rules checklist

#### `@charset`
- Behavior: Must be first; selects the stylesheet encoding; ignored elsewhere. Parser consumes and reports it.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@import`
- Behavior: Fetches and cascades a sheet; conditions (media/supports/layer) gate it; relative URLs resolve against the sheet's base URL.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@media`
- Behavior: Gate rules on media queries: type (all/screen/print), width/height/aspect-ratio/resolution/orientation, prefers-color-scheme/reduced-motion/contrast; nested media allowed per CSS Conditional.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@supports`
- Behavior: Feature-query gate: property:value checks and (not/and/or) combinations evaluated against the property registry.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@page`
- Behavior: Paged media page boxes: margins, size; parse in MVP, layout as stretch.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@font-face`
- Behavior: Registers a font family: src list with format() and unicode-range, font-display, weight/style/stretch descriptors; contributes to §5.11 font matching.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@keyframes`
- Behavior: Named keyframe list: 0%/100%/from/to stops with declaration blocks; interpolation per animation-timing-function; consumed by the animation machinery of §5.15.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@namespace`
- Behavior: Declares namespace prefixes for type/attribute selector matching in XML-ish documents; affects selector namespace resolution only.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@layer`
- Behavior: Cascade layers: named and anonymous layer declarations with nested blocks; layer order participates in the cascade (§5.8.2).
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@container`
- Behavior: Container queries: size/inline-size/style conditions evaluated against the nearest ancestor container (§box container-type); MVP: size only.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@property`
- Behavior: Registers custom properties with syntax/initial-value/inherits; typed registration changes substitution and animation behavior.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@counter-style`
- Behavior: Defines list/counter marker styles (system, symbols, prefix/suffix, range, pad); referenced by list-style-type and counters.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@font-feature-values`
- Behavior: Named font feature value sets for font-variant-alternates; parse and expose to the shaper.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@scope`
- Behavior: Scopes nested rules to a range between a root and an optional scoping limit; affects matching (§5.8.1); MVP: parse + root-only scoping.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@starting-style`
- Behavior: Transitions-in entry styles: declarations applied at first style computation, then dropped.
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

#### `@document (legacy)`
- Behavior: Non-standard; parse as Unknown rule block, never apply (§5.7 forward-compat rule).
**Tasks:**
- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).
- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.
- [ ] Serialization: round-trips through the DevTools display (§5.19.5).
- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.

### §6.5 Selector engine checklist

Each selector: parsed into the `Selector` AST, compiled into the matching
plan (§5.8.1), invalidation-registered (§5.8.6), and covered by a matching
test with positive and negative cases.

- [ ] `*` — *basic*: Universal selector; matches any element; zero specificity alone.
- [ ] `E` — *basic*: Type selector with optional namespace prefix (svg | rect); case-insensitivity per document language.
- [ ] `.c` — *basic*: Class selector; compound list matching on DOMTokenList; whitespace-separated.
- [ ] `[a]` — *basic*: Attribute presence; matches namespaced attributes with the standard default namespace rules.
- [ ] `[a=v]` — *basic*: Exact match.
- [ ] `[a~=v]` — *basic*: Whitespace-list match.
- [ ] `[a` — *=v]*: basic | Prefix-with-dash match (lang codes).
- [ ] `[a^=v]` — *basic*: Prefix match (empty value never matches).
- [ ] `[a$=v]` — *basic*: Suffix match (empty value never matches).
- [ ] `[a*=v]` — *basic*: Substring match.
- [ ] `[a=v i]` — *basic*: Case-insensitive flag; [a=v s] case-sensitive flag.
- [ ] `:is(x, y)` — *logical*: Specificity = most specific argument; forgiving selector list (invalid arguments dropped).
- [ ] `:where(x, y)` — *logical*: Specificity = zero; forgiving list; otherwise identical to :is().
- [ ] `:not(x, y)` — *logical*: Negation; forgiving list; specificity = most specific argument.
- [ ] `:has(rel sel)` — *logical*: Relative selector match on descendants/siblings; requires the §5.8 invalidation machinery.
- [ ] `:scope` — *logical*: Matches the scoping root (querySelector context, @scope root).
- [ ] `:root` — *structural*: Matches the document root element.
- [ ] `:empty` — *structural*: Element with no children (text nodes count; whitespace counts).
- [ ] `:nth-child(an+b)` — *structural*: Full an+b grammar incl. of S syntax (:nth-child(2n of .c)); parse per css-syntax an+b tests.
- [ ] `:nth-last-child(an+b)` — *structural*: Counted from the end.
- [ ] `:nth-of-type(an+b)` — *structural*: Index among same-type siblings.
- [ ] `:nth-last-of-type(an+b)` — *structural*: Index among same-type siblings from the end.
- [ ] `:first-child` — *structural*: nth-child(1).
- [ ] `:last-child` — *structural*: nth-last-child(1).
- [ ] `:only-child` — *structural*: First and last child simultaneously.
- [ ] `:first-of-type` — *structural*: First among same-type siblings.
- [ ] `:last-of-type` — *structural*: Last among same-type siblings.
- [ ] `:only-of-type` — *structural*: Only among same-type siblings.
- [ ] `:link` — *resource*: Unvisited link state (elements with href that are links); see §5.18.7.
- [ ] `:any-link` — *resource*: Link regardless of visited state.
- [ ] `:visited` — *resource*: Visited state; restricted to paint-only properties (§5.18.7).
- [ ] `:target` — *resource*: Element targeted by the URL fragment.
- [ ] `:defined` — *resource*: Custom-element definedness (always true for built-ins in this engine).
- [ ] `:modal` — *resource*: True for elements in the top layer as modal dialogs.
- [ ] `:fullscreen` — *resource*: Always false in this engine (non-goal §1.4).
- [ ] `:hover` — *user*: Pointer-over state from the input system; ancestors of the hovered element also match.
- [ ] `:active` — *user*: Activation-in-progress state for buttons/links.
- [ ] `:focus` — *user*: The focused element (one per document).
- [ ] `:focus-within` — *user*: Focused element or its ancestors.
- [ ] `:focus-visible` — *user*: Heuristic per spec: keyboard-initiated focus gets the ring.
- [ ] `:enabled` — *form*: Form control that is not disabled.
- [ ] `:disabled` — *form*: Disabled form control (from the disabled attribute or fieldset propagation).
- [ ] `:checked` — *form*: Checked checkbox/radio or selected option.
- [ ] `:indeterminate` — *form*: Checkbox with indeterminate IDL state, radio group with none selected, progress without value.
- [ ] `:placeholder-shown` — *form*: Input currently showing placeholder.
- [ ] `:default` — *form*: Default checked/selected option of its group.
- [ ] `:required` — *form*: Required form control.
- [ ] `:optional` — *form*: Not required.
- [ ] `:valid` — *form*: Passes constraint validation (§5.6 forms subset).
- [ ] `:invalid` — *form*: Fails constraint validation.
- [ ] `:in-range` — *form*: Has value constraints and value is within them.
- [ ] `:out-of-range` — *form*: Has constraints and value is outside.
- [ ] `:read-only` — *form*: Not editable (readonly, disabled, or non-input element).
- [ ] `:read-write` — *form*: Editable.
- [ ] `:lang(x)` — *linguistic*: Language match with wildcard ranges; walks the lang attribute chain plus meta inheritance.
- [ ] `:dir(ltr/rtl)` — *linguistic*: Directionality from the bidi algorithm result (§5.11.4).
- [ ] `::before` — *pseudo-element*: Generated box before content per content property.
- [ ] `::after` — *pseudo-element*: Generated box after content per content property.
- [ ] `::placeholder` — *pseudo-element*: Styles the input placeholder text.
- [ ] `::selection` — *pseudo-element*: Styles the active selection; paint-only overlay.
- [ ] `::marker` — *pseudo-element*: List item marker box styling.
- [ ] `::first-line` — *pseudo-element*: First formatted line of a block (limited property set per spec).
- [ ] `::first-letter` — *pseudo-element*: First letter/leading punctuation box of a block.
- [ ] `::backdrop` — *pseudo-element*: Behind top-layer elements (dialog).
- [ ] `::file-selector-button` — *pseudo-element*: Button inside file inputs (sub-UI styling MVP).

### §6.6 DOM interface checklist

One block per script-visible interface (§5.15.2 binding rules apply to
every one): correct prototype chain, attribute getters/setters with the
right exceptions, method overloads and optional arguments, stringifier /
iterable / legacy platform object behaviors where marked.

#### `EventTarget`
- Surface:
method addEventListener(type, callback, options);
  method removeEventListener(type, callback, options); method dispatchEvent(event) -> bool.
- Tasks: Base of the dispatch machinery: capture, target, bubble phases (§5.6).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Node` — inherits `EventTarget`
- Surface:
attr nodeType; attr nodeName; attr baseURI; attr isConnected; attr ownerDocument;
  attr parentNode; attr parentElement; attr childNodes (live NodeList); attr firstChild;
  attr lastChild; attr previousSibling; attr nextSibling; attr textContent (get/set);
  method hasChildNodes(); method normalize(); method cloneNode(deep);
  method isEqualNode(other); method isSameNode(other);
  method compareDocumentPosition(other); method contains(other); method lookupPrefix(ns);
  method lookupNamespaceURI(prefix); method isDefaultNamespace(ns);
  method insertBefore(node, child); method appendChild(child);
  method replaceChild(node, child); method removeChild(child).
- Tasks: Arena-backed operations (§5.6); mutation observer hooks; all live collections recompute.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Document` — inherits `Node`
- Surface:
attr documentElement; attr doctype; attr body; attr head; attr title (get/set);
  attr cookie (get/set); attr domain; attr referrer; attr URL; attr documentURI;
  attr characterSet; attr contentType; attr compatMode; attr designMode; attr dir;
  attr forms (live); attr images (live); attr links (live); attr scripts (live);
  attr styleSheets; attr activeElement; attr currentScript; attr defaultView;
  method createElement(localName, options); method createElementNS(ns, qname);
  method createDocumentFragment(); method createTextNode(data); method createComment(data);
  method createProcessingInstruction(target, data); method createAttribute(name);
  method createAttributeNS(ns, name); method createEvent(type); method createRange();
  method createNodeIterator(root, filter); method createTreeWalker(root, filter);
  method createExpression/evaluate (XPath: non-goal, throw); method getElementById(id);
  method getElementsByName(name); method getElementsByTagName(qname);
  method getElementsByTagNameNS(ns, qname); method getElementsByClassName(names);
  method querySelector(sel); method querySelectorAll(sel); method adoptNode(node);
  method importNode(node, deep); method open(url, name); method close();
  method write(...text); method writeln(...text); method hasFocus();
  method execCommand (no-op with false, legacy).
- Tasks: Owner of the arena; base URL + origin resolution; cookie access via storage; write() re-enters the parser (§5.5.6).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `DocumentFragment` — inherits `Node`
- Surface:
attr children; method getElementById; method querySelector(All).
- Tasks: Template contents and createContextualFragment substrate.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `DocumentType` — inherits `Node`
- Surface:
attr name; attr publicId; attr systemId.
- Tasks: Serialization and quirks derivation.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `ShadowRoot` — inherits `DocumentFragment`
- Surface:
attr mode; attr host; attr delegatesFocus; attr slotAssignment; attr innerHTML (get/set).
- Tasks: Flat-tree construction; slot assignment algorithm.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Element` — inherits `Node`
- Surface:
attr namespaceURI; attr prefix; attr localName; attr tagName; attr id (get/set);
  attr className (get/set); attr classList (DOMTokenList); attr attributes (NamedNodeMap);
  attr children (live HTMLCollection); attr firstElementChild; attr lastElementChild;
  attr previousElementSibling; attr nextElementSibling; attr childElementCount;
  attr innerHTML (get/set); attr outerHTML (get/set); attr shadowRoot; attr assignedSlot;
  method hasAttribute(name); method getAttribute(name); method getAttributeNS(ns, name);
  method setAttribute(name, value); method setAttributeNS(ns, qname, value);
  method removeAttribute(name); method removeAttributeNS(ns, name);
  method toggleAttribute(name, force); method getAttributeNames(); method hasAttributes();
  method closest(sel); method matches(sel); method webkitMatchesSelector(sel);
  method insertAdjacentElement(pos, el); method insertAdjacentText(pos, text);
  method insertAdjacentHTML(pos, html); method before(...nodes); method after(...nodes);
  method replaceWith(...nodes); method remove(); method append(...nodes);
  method prepend(...nodes); method querySelector(sel); method querySelectorAll(sel);
  method getElementsByTagName(qname); method getElementsByTagNameNS(ns, qname);
  method getElementsByClassName(names); method attachShadow(init);
  method scroll/scrollTo/scrollBy (viewport + container); method scrollIntoView(arg);
  method getBoundingClientRect(); method getClientRects(); method checkVisibility(opts);
  method setHTMLUnsafe (parse-with-template).
- Tasks: Namespace-aware attribute map; dataset via DOMStringMap; style attribute; shadow attach.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLElement` — inherits `Element`
- Surface:
attr title; attr lang; attr dir; attr hidden; attr inert; attr accessKey; attr draggable;
  attr spellcheck; attr tabIndex (get/set); attr dataset (DOMStringMap);
  attr style (CSSStyleDeclaration); attr translate; method click(); method focus(opts);
  method blur(); method showPopover/hidePopover/togglePopover (MVP: no-op events).
- Tasks: Common attribute surface; inert subtree behavior.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Text` — inherits `CharacterData`
- Surface:
attr wholeText; attr assignedSlot; method splitText(offset) -> Text.
- Tasks: Splitting merges/normalizes per spec.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Comment` — inherits `CharacterData`
- Surface:
—.
- Tasks: Serialization keeps <!-- -->.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `ProcessingInstruction` — inherits `CharacterData`
- Surface:
attr target.
- Tasks: XML serialization path.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CharacterData` — inherits `Node`
- Surface:
attr data (get/set); attr length; method substringData(offset, count);
  method appendData(text); method insertData(offset, text);
  method deleteData(offset, count); method replaceData(offset, count, text).
- Tasks: Base for Text/Comment/PI.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Attr` — inherits `Node`
- Surface:
attr name; attr value (get/set); attr namespaceURI; attr prefix; attr localName;
  attr specified (always true).
- Tasks: Not a child of elements in this engine's tree model; live value reflection.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `NodeList`
- Surface:
attr length; method item(i); method forEach(cb); iterable.
- Tasks: Static (querySelectorAll) or live variant flag.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLCollection`
- Surface:
attr length; attr namedItem support; method item(i); method namedItem(name); iterable.
- Tasks: Live; recomputed against the tree on access (§5.6 pitfalls).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `DOMTokenList`
- Surface:
attr length; attr value (get/set); method item(i); method contains(token);
  method add(...tokens); method remove(...tokens); method toggle(token, force);
  method replace(old, new); method supports(token); method keys/values/entries.
- Tasks: Whitespace-normalized; validation per interface (rel, class has none).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `DOMStringMap`
- Surface:
proxy-style named get/set/delete.
- Tasks: dataset backing store.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CSSStyleDeclaration`
- Surface:
attr length; attr cssText (get/set); attr parentRule; method item(i);
  method getPropertyValue(prop); method setProperty(prop, value, priority);
  method removeProperty(prop); method getPropertyPriority(prop);
  method getPropertyShorthand; method isPropertyImplicit.
- Tasks: style attribute live object; getComputedStyle returns a read-only variant.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CSSStyleSheet` — inherits `StyleSheet`
- Surface:
attr cssRules; attr ownerRule; attr ownerNode; attr href; attr title; attr media;
  attr disabled (get/set); method insertRule(rule, index); method deleteRule(index);
  method replaceSync(text).
- Tasks: Rule list with source locations for DevTools.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `StyleSheet`
- Surface:
attr type; attr href; attr ownerNode; attr parentStyleSheet; attr title;
  attr media (MediaList).
- Tasks: Base class.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MediaQueryList`
- Surface:
attr matches; attr media; method addListener(cb); method removeListener(cb);
  method addEventListener; onchange.
- Tasks: Evaluated against current viewport; change events on resize.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Range`
- Surface:
attr startContainer; attr startOffset; attr endContainer; attr endOffset; attr collapsed;
  attr commonAncestorContainer; method setStart(node, off); method setEnd(node, off);
  method setStartBefore(node); method setStartAfter(node); method setEndBefore(node);
  method setEndAfter(node); method selectNode(node); method selectNodeContents(node);
  method collapse(toStart); method selectNodeContents;
  method compareBoundaryPoints(how, range); method deleteContents();
  method extractContents(); method cloneContents(); method insertNode(node);
  method surroundContents(node); method cloneRange(); method detach();
  method isPointInRange(node, off); method intersectsNode(node); method toString().
- Tasks: Boundary-point model with document-order comparison.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Selection`
- Surface:
attr anchorNode; attr anchorOffset; attr focusNode; attr focusOffset; attr isCollapsed;
  attr rangeCount; attr type; method getRangeAt(i); method addRange(range);
  method removeRange(range); method removeAllRanges(); method collapse(node, off);
  method setPosition; method collapseToStart/ToEnd; method extend(node, off);
  method setBaseAndExtent(anchor, aoff, focus, foff); method selectAllChildren(node);
  method containsNode(node, partial); method toString().
- Tasks: Backed by one document range; syncs with focus/inputs.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `NodeIterator`
- Surface:
attr root; attr referenceNode; attr pointerBeforeReferenceNode; attr whatToShow;
  attr filter; method nextNode(); method previousNode(); method detach().
- Tasks: Traversal with filter states.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `TreeWalker`
- Surface:
attr root; attr currentNode; attr whatToShow; attr filter; method parentNode();
  method firstChild(); method lastChild(); method previousSibling(); method nextSibling();
  method previousNode(); method nextNode().
- Tasks: Stateful cursor traversal.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MutationObserver`
- Surface:
method observe(target, options); method disconnect(); method takeRecords().
- Tasks: Records queued until the microtask checkpoint (§5.15).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MutationRecord`
- Surface:
attr type; attr target; attr addedNodes; attr removedNodes; attr previousSibling;
  attr nextSibling; attr attributeName; attr attributeNamespace; attr oldValue.
- Tasks: Constructed by the mutation hooks.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `ResizeObserver`
- Surface:
method observe(target, options); method unobserve(target); method disconnect().
- Tasks: Fires before paint in the rendering update (§5.15.1).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `IntersectionObserver`
- Surface:
attr root; attr rootMargin; attr thresholds; method observe(target);
  method unobserve(target); method disconnect(); method takeRecords().
- Tasks: Single-viewport MVP per §5.6.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Performance`
- Surface:
attr timeOrigin; attr memory (reported); method now(); method mark(name);
  method measure(name, start, end); method getEntries();
  method getEntriesByName(name, type); method getEntriesByType(type).
- Tasks: High-res clock capped to 100us granularity (timing-attack surface).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `History`
- Surface:
attr length; attr scrollRestoration; attr state; method back(); method forward();
  method go(delta); method pushState(data, title, url);
  method replaceState(data, title, url).
- Tasks: Same-document session history entry mutation.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Location`
- Surface:
attr href (get/set); attr protocol; attr host; attr hostname; attr port; attr pathname;
  attr search; attr hash; attr origin; method assign(url); method replace(url);
  method reload(); method toString().
- Tasks: Unforgeable on window; navigation triggers (§5.19.2).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Navigator`
- Surface:
attr userAgent; attr language; attr languages; attr platform; attr onLine;
  attr hardwareConcurrency; attr deviceMemory; attr maxTouchPoints; attr cookieEnabled;
  attr doNotTrack; attr globalPrivacyControl; method sendBeacon(url, data);
  method canShare/share (not supported, throw).
- Tasks: Static per browser; no fingerprinting extras.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Screen`
- Surface:
attr width; attr height; attr availWidth; attr availHeight; attr colorDepth;
  attr pixelDepth; attr devicePixelRatio (on window).
- Tasks: From the platform layer.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Window` — inherits `EventTarget`
- Surface:
attr window/self/parent/top/frames; attr document; attr location (unforgeable);
  attr history; attr navigator; attr screen; attr innerWidth/innerHeight;
  attr outerWidth/outerHeight; attr pageXOffset/pageYOffset; attr scrollX/scrollY;
  attr devicePixelRatio; attr name (get/set); attr status; attr closed;
  attr length (frames); attr frames list; attr event (legacy, sloppy only);
  attr localStorage; attr sessionStorage; attr console; attr customElements;
  attr performance; attr origin; method open(url, target, features); method close();
  method stop(); method focus(); method blur(); method print (no-op + console note);
  method alert(msg); method confirm(msg); method prompt(msg, def);
  method scroll/scrollTo/scrollBy(x, y);
  method moveTo/moveBy/resizeTo/resizeBy (shell-clamped);
  method getComputedStyle(el, pseudo); method matchMedia(q);
  method requestAnimationFrame(cb); method cancelAnimationFrame(id);
  method requestIdleCallback(cb); method cancelIdleCallback(id);
  method postMessage(message, targetOrigin, transfer); method getSelection();
  method btoa/atob; method structuredClone; method fetch(input, init);
  method setInterval/setTimeout/clearInterval/clearTimeout; method queueMicrotask(cb);
  method reportError(err); method createImageBitmap (not supported).
- Tasks: The global object; event handlers via IDL attributes (on*); named property access on frames/named elements (legacy).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Storage`
- Surface:
attr length; method key(i); method getItem(key); method setItem(key, value);
  method removeItem(key); method clear().
- Tasks: Per-origin (local) or per-tab (session); quota 5MB UTF-16 units (§5.17).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Event`
- Surface:
attr type; attr target; attr currentTarget; attr eventPhase; attr bubbles; attr cancelable;
  attr defaultPrevented; attr composed; attr isTrusted; attr timeStamp;
  method stopPropagation(); method stopImmediatePropagation(); method preventDefault();
  attr NONE/CAPTURING_PHASE/AT_TARGET/BUBBLING_PHASE.
- Tasks: Base dispatch machinery (§5.6).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CustomUIEvent variants` — inherits `see below`
- Surface:
—.
- Tasks: Each typed event is its own binding block with its init dict.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `UIEvent` — inherits `Event`
- Surface:
attr view; attr detail.
- Tasks: Base for input-derived events.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `FocusEvent` — inherits `UIEvent`
- Surface:
attr relatedTarget.
- Tasks: focus/blur/focusin/focusout.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MouseEvent` — inherits `UIEvent`
- Surface:
attr screenX; attr screenY; attr clientX; attr clientY; attr offsetX; attr offsetY;
  attr pageX; attr pageY; attr button; attr buttons; attr relatedTarget; attr movementX;
  attr movementY; modifier getters (ctrlKey, shiftKey, altKey, metaKey);
  method getModifierState(keyArg).
- Tasks: click/dblclick/contextmenu/mouse* family.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `WheelEvent` — inherits `MouseEvent`
- Surface:
attr deltaX; attr deltaY; attr deltaZ; attr deltaMode.
- Tasks: Scroll chaining decision lives in the shell/engine boundary.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `PointerEvent` — inherits `MouseEvent`
- Surface:
attr pointerId; attr width; attr height; attr pressure; attr tangentialPressure; attr tiltX;
  attr tiltY; attr twist; attr pointerType; attr isPrimary.
- Tasks: Mouse events are synthesized from pointers per UI Events.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `KeyboardEvent` — inherits `UIEvent`
- Surface:
attr key; attr code; attr location; attr ctrlKey; attr shiftKey; attr altKey; attr metaKey;
  attr repeat; attr isComposing; attr charCode (legacy); attr keyCode (legacy);
  method getModifierState(keyArg).
- Tasks: keydown/keyup (+ legacy keypress); Appendix F mapping.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `InputEvent` — inherits `UIEvent`
- Surface:
attr data; attr dataTransfer; attr isComposing; attr inputType.
- Tasks: beforeinput/input on editable elements.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CompositionEvent` — inherits `UIEvent`
- Surface:
attr data; attr locale.
- Tasks: IME composition start/update/end.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `DragEvent` — inherits `MouseEvent`
- Surface:
attr dataTransfer (DataTransfer: MVP types/files subset).
- Tasks: drag* family with the simple drag-store model.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `ProgressEvent` — inherits `Event`
- Surface:
attr lengthComputable; attr loaded; attr total.
- Tasks: load/error/progress on fetches.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MessageEvent` — inherits `Event`
- Surface:
attr data; attr origin; attr lastEventId; attr source; attr ports.
- Tasks: postMessage delivery.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `ErrorEvent` — inherits `Event`
- Surface:
attr message; attr filename; attr lineno; attr colno; attr error.
- Tasks: window error reporting.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `PromiseRejectionEvent` — inherits `Event`
- Surface:
attr promise; attr reason.
- Tasks: unhandledrejection/rejectionhandled.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HashChangeEvent` — inherits `Event`
- Surface:
attr oldURL; attr newURL.
- Tasks: hashchange.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `AnimationEvent` — inherits `Event`
- Surface:
attr animationName; attr elapsedTime; attr pseudoElement.
- Tasks: animationstart/iteration/end/cancel.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `TransitionEvent` — inherits `Event`
- Surface:
attr propertyName; attr elapsedTime; attr pseudoElement.
- Tasks: transitionstart/transitionrun/end/cancel.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CloseEvent` — inherits `Event`
- Surface:
attr wasClean; attr code; attr reason.
- Tasks: WebSocket close.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `SecurityPolicyViolationEvent` — inherits `Event`
- Surface:
attr documentURI; attr referrer; attr blockedURL; attr statusCode; attr effectiveDirective;
  attr originalPolicy; attr sourceFile; attr lineNumber; attr columnNumber;
  attr disposition; attr sample.
- Tasks: CSP violation reporting (§5.18.4).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `FormData`
- Surface:
method append(name, value, filename); method delete(name); method get(name);
  method getAll(name); method has(name); method set(name, value, filename); entries/iterable.
- Tasks: Used by form submission and fetch bodies.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `URL`
- Surface:
constructor(url, base); attr href (get/set); attr origin; attr protocol; attr username;
  attr password; attr host; attr hostname; attr port; attr pathname; attr search; attr hash;
  attr searchParams; method toJSON; static createObjectURL/revokeObjectURL (blob: MVP).
- Tasks: Wraps §5.1 parser; searchParams is URLSearchParams.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `URLSearchParams`
- Surface:
constructor(init); attr size; method append(name, value); method delete(name);
  method get(name); method getAll(name); method has(name, value); method set(name, value);
  method sort(); entries/iterable; method forEach.
- Tasks: application/x-www-form-urlencoded semantics (§5.1).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Blob`
- Surface:
attr size; attr type; method slice(start, end, contentType);
  method stream (MVP: arrayBuffer); method arrayBuffer(); method text().
- Tasks: Backing store for File/fetch bodies.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `File` — inherits `Blob`
- Surface:
attr name; attr lastModified.
- Tasks: From input files and drag-drop.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `FileReader`
- Surface:
attr readyState; attr result; attr error; method readAsArrayBuffer(blob);
  method readAsText(blob, encoding); method readAsDataURL(blob); method abort().
- Tasks: Event-driven async reads on the document thread.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Request`
- Surface:
constructor(input, init); attr method; attr url; attr headers; attr destination;
  attr referrer; attr referrerPolicy; attr mode; attr credentials; attr cache;
  attr redirect; attr integrity; attr keepalive; attr signal; attr bodyUsed; method clone();
  method arrayBuffer/text/blob/json.
- Tasks: Fetch API client side (§5.15.5).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Response`
- Surface:
constructor(body, init); attr url; attr ok; attr status; attr statusText; attr headers;
  attr redirected; attr type; attr bodyUsed; static error(); static redirect(url, status);
  method clone(); method arrayBuffer(); method text(); method json(); method blob().
- Tasks: Fetch API server side.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Headers`
- Surface:
constructor(init); method append(name, value); method delete(name); method get(name);
  method getSetCookie(); method has(name); method set(name, value); entries/iterable;
  method forEach.
- Tasks: Case-insensitive; forbidden header names enforced.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `AbortController`
- Surface:
attr signal; method abort(reason).
- Tasks: Cancels fetches and timers hooked to the signal.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `AbortSignal` — inherits `EventTarget`
- Surface:
attr aborted; attr reason; attr onabort; static abort(reason); static timeout(ms);
  method throwIfAborted().
- Tasks: Wired to the loader/timer cancellation tokens.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `WebSocket`
- Surface:
constructor(url, protocols); attr url; attr readyState; attr bufferedAmount;
  attr extensions; attr protocol; attr binaryType; method send(data);
  method close(code, reason); events open/message/error/close.
- Tasks: RFC 6455 handshake + framing (WBS §6.9); no subprotocol negotiation beyond echo.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MessageChannel`
- Surface:
attr port1; attr port2.
- Tasks: Entangled ports for structured-clone messaging.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `MessagePort` — inherits `EventTarget`
- Surface:
method postMessage(message, transfer); method start(); method close();
  events message/messageerror.
- Tasks: MVP: same-document entangled pair only.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `DOMParser`
- Surface:
method parseFromString(str, type) -> Document.
- Tasks: text/html path reuses the full parser (§5.5); XML path rejects on well-formedness errors.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `XMLSerializer`
- Surface:
method serializeToString(node).
- Tasks: Escaping table per §5.6.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLInputElement` — inherits `HTMLElement`
- Surface:
attr type (get/set); attr accept; attr alt; attr autocomplete; attr autofocus;
  attr checked (get/set); attr defaultChecked; attr form; attr formAction; attr formEnctype;
  attr formMethod; attr formNoValidate; attr formTarget; attr files; attr height/width;
  attr list; attr max/min; attr maxLength/minLength; attr multiple; attr name; attr pattern;
  attr placeholder; attr readOnly; attr required; attr size; attr src; attr step;
  attr value (get/set, value-modes per type); attr defaultValue; attr willValidate;
  attr validity (ValidityState fields); attr validationMessage; attr labels;
  method stepUp/stepDown(n); method select(); method setRangeText(rep, start, end, mode);
  method setSelectionRange(start, end, dir); attr selectionStart/End/Direction;
  method checkValidity(); method reportValidity(); method setCustomValidity(msg).
- Tasks: The workhorse control: text/checkbox/radio/button/submit/reset/file/hidden/password/range/number/email/url/search/tel/date (MVP subset); activation + constraint validation (§5.6).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLTextAreaElement` — inherits `HTMLElement`
- Surface:
attr cols; attr rows; attr dirName; attr disabled; attr form; attr maxLength/minLength;
  attr name; attr placeholder; attr readOnly; attr required; attr wrap;
  attr value (get/set); attr defaultValue; attr textLength; selection attrs + methods;
  validation methods.
- Tasks: Editable multi-line control; input events on mutation.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLSelectElement` — inherits `HTMLElement`
- Surface:
attr multiple; attr name; attr required; attr size; attr selectedIndex (get/set);
  attr value; attr length; attr options (live); attr selectedOptions; attr form;
  method add(el, before); method remove(index); method remove(); method item(i);
  method namedItem(name); validation methods.
- Tasks: Option list model; change events.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLOptionElement` — inherits `HTMLElement`
- Surface:
attr disabled; attr form; attr label; attr defaultSelected; attr selected (get/set);
  attr value; attr text; attr index.
- Tasks: Selectedness rules.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLButtonElement` — inherits `HTMLElement`
- Surface:
attr type; attr value; attr name; attr form;
  attr formAction/Enctype/Method/NoValidate/Target; attr disabled; validation methods.
- Tasks: Activation: submit/reset/button (§5.6).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLFormElement` — inherits `HTMLElement`
- Surface:
attr acceptCharset; attr action; attr autocomplete; attr enctype; attr encoding;
  attr method; attr name; attr noValidate; attr target; attr rel; attr elements (live);
  attr length; method submit(); method requestSubmit(submitter); method reset();
  method checkValidity(); method reportValidity(); events submit/reset.
- Tasks: Submission algorithm (GET url-encoding / POST body), constraint validation pass.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLLabelElement` — inherits `HTMLElement`
- Surface:
attr form; attr htmlFor (get/set); attr control.
- Tasks: Activation forwarding to the labeled control.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLFieldSetElement` — inherits `HTMLElement`
- Surface:
attr form; attr name; attr disabled; attr type; attr elements.
- Tasks: Disabled propagation to descendants.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLAnchorElement` — inherits `HTMLElement`
- Surface:
attr href (reflected); attr target; attr download; attr rel; attr relList; attr hreflang;
  attr type; attr referrerPolicy; attr text; protocol/host/pathname etc URL reflectors.
- Tasks: Activation navigation (§5.19.2).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLImageElement` — inherits `HTMLElement`
- Surface:
attr alt; attr src (get/set); attr srcset; attr sizes; attr crossOrigin; attr useMap;
  attr isMap; attr width/height (get/set); attr naturalWidth; attr naturalHeight;
  attr complete; attr currentSrc; attr decoding; attr loading; attr referrerPolicy;
  attr fetchPriority.
- Tasks: decode pipeline wiring (§5.12).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLCanvasElement` — inherits `HTMLElement`
- Surface:
attr width/height (get/set); method getContext(type, opts); method toDataURL(type);
  method toBlob(cb, type).
- Tasks: 2D context only (§5.6 ctx below).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `CanvasRenderingContext2D`
- Surface:
attr canvas; save/restore; scale/rotate/translate/transform/setTransform/resetTransform;
  globalAlpha; globalCompositeOperation; fillStyle/strokeStyle (colors, gradients MVP);
  lineWidth; lineCap; lineJoin; miterLimit; lineDash attrs + setLineDash/getLineDash;
  shadow attrs; clearRect/fillRect/strokeRect;
  beginPath/closePath/moveTo/lineTo/quadraticCurveTo/bezierCurveTo/arc/arcTo/rect/ellipse/roundRect;
  fill/stroke/clip (path + Path2D); isPointInPath; drawImage (src rect variants);
  createLinearGradient/createRadialGradient (addColorStop);
  getImageData/putImageData/createImageData; measureText -> TextMetrics;
  fillText/strokeText (fonts: §5.11 path); direction attr.
- Tasks: Backed by the same rasterizer (§5.16); state stack with fill/stroke paint objects.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `ImageData`
- Surface:
attr width; attr height; attr data (Uint8ClampedArray).
- Tasks: Pixel format RGBA8.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `TextMetrics`
- Surface:
attr width; attr actualBoundingBox* family; attr fontBoundingBox* family.
- Tasks: From the shaper metrics.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLIFrameElement` — inherits `HTMLElement`
- Surface:
attr src; attr srcdoc; attr name; attr sandbox (DOMTokenList); attr allow;
  attr allowFullscreen; attr loading; attr width/height;
  contentDocument/contentWindow (SOP-checked).
- Tasks: Nested browsing context placeholder (§1.3).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLMediaElement` — inherits `HTMLElement`
- Surface:
attr src; attr currentSrc; attr networkState; attr readyState; attr paused; attr duration;
  attr currentTime (get/set); attr volume; attr muted; attr playbackRate; method load();
  method play() -> promise; method pause();
  events loadstart/loadedmetadata/canplay/play/pause/ended/error.
- Tasks: MVP: state machine + events fire, no codec decode (§1.4).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLScriptElement` — inherits `HTMLElement`
- Surface:
attr src; attr type; attr noModule; attr async; attr defer; attr crossOrigin; attr text;
  attr integrity; attr referrerPolicy; attr fetchPriority.
- Tasks: Execution queueing per §5.5.6.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLLinkElement` — inherits `HTMLElement`
- Surface:
attr href; attr rel; attr relList; attr media; attr hreflang; attr type; attr as;
  attr crossOrigin; attr referrerPolicy; attr disabled; attr sheet.
- Tasks: Stylesheet loading (§5.7), favicon, preloads.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLStyleElement` — inherits `HTMLElement`
- Surface:
attr media; attr type; attr disabled; attr sheet.
- Tasks: Inline sheet ownership.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLMetaElement` — inherits `HTMLElement`
- Surface:
attr name; attr content; attr httpEquiv; attr charset.
- Tasks: Encoding + viewport MVP.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLTableElement` — inherits `HTMLElement`
- Surface:
attr caption; attr tHead; attr tFoot; attr rows (live); attr tBodies (live);
  method createCaption/deleteCaption/createTHead/deleteTHead/createTFoot/deleteTFoot;
  method insertRow(index); method deleteRow(index).
- Tasks: Table model helpers.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLTableRowElement` — inherits `HTMLElement`
- Surface:
attr rowIndex; attr sectionRowIndex; attr cells (live); method insertCell(index);
  method deleteCell(index).
- Tasks: Row model.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLTableCellElement` — inherits `HTMLElement`
- Surface:
attr colSpan; attr rowSpan; attr headers; attr cellIndex; attr scope (th); attr abbr (th).
- Tasks: Span validation for grid layout.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLDialogElement` — inherits `HTMLElement`
- Surface:
attr open (get/set); attr returnValue; method show(); method showModal();
  method close(returnValue); event cancel.
- Tasks: Top-layer + focus trapping + :modal.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLDetailsElement` — inherits `HTMLElement`
- Surface:
attr open (get/set); toggle event.
- Tasks: Name-group accordion behavior.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLProgressElement` — inherits `HTMLElement`
- Surface:
attr value/max (get/set); attr position.
- Tasks: Indeterminate state.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLMeterElement` — inherits `HTMLElement`
- Surface:
attr value/min/max/low/high/optimum (get/set).
- Tasks: Gauge regions.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLTemplateElement` — inherits `HTMLElement`
- Surface:
attr content (DocumentFragment).
- Tasks: Parser-invisible contents (§5.5).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLSlotElement` — inherits `HTMLElement`
- Surface:
attr name; method assignedNodes(options); method assignedElements(options).
- Tasks: Slot assignment.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `HTMLOutputElement` — inherits `HTMLElement`
- Surface:
attr htmlFor; attr form; attr name; attr value (get/set); attr defaultValue.
- Tasks: form-associated readout.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Crypto`
- Surface:
method getRandomValues(array); attr subtle (unsupported: throws cleanly).
- Tasks: CSPRNG from the platform layer only.
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

#### `Console`
- Surface:
method log/debug/info/warn/error(table); method assert(cond, ...args); method clear();
  method count(label); method countReset; method group/groupEnd/groupCollapsed;
  method time(label); method timeEnd/timeLog; method trace(...args); method table(data).
- Tasks: Console Standard formatting to the DevTools buffer (§5.15.4).
- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.
- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.

### §6.7 JavaScript builtin checklist

Every builtin is implemented with the interpreter's primitives (§5.14)
and verified against the adopted test262 subset (§8.6). `M9` marks the
baseline inventory; anything beyond lands in M13 polish unless a milestone
section pulls it earlier.

#### `globalThis functions`
- Host additions on the same global: setTimeout/setInterval/clear*/queueMicrotask/structuredClone/atob/btoa/fetch/console/performance
- Surface:
globalThis; undefined; NaN; Infinity; eval(x); isFinite(x); isNaN(x); parseFloat(x);
  parseInt(x, radix); encodeURIComponent(s); decodeURIComponent(s); encodeURI(s);
  decodeURI(s); String/Number/Boolean/BigInt/Symbol/Object/Array constructors;
  ArrayBuffer/SharedArrayBuffer; DataView; TypedArray family;
  Map/Set/WeakMap/WeakSet/WeakRef/FinalizationRegistry; Promise; Proxy; Reflect; Date;
  RegExp; Error family; JSON; Math; Atomics; Intl; Function; AggregateError;
  parse module keys (import, import.meta in modules).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Object`
- Property descriptors: value/writable/get/set/enumerable/configurable; accessor vs data slots per spec
- Surface:
static assign(target, ...src); static create(proto, props);
  static defineProperty(obj, key, desc); static defineProperties; static entries;
  static freeze; static isFrozen; static fromEntries; static getOwnPropertyDescriptor(s);
  static getOwnPropertyNames; static getOwnPropertySymbols;
  static getPrototypeOf/setPrototypeOf; static hasOwn; static is; static isExtensible;
  static isSealed; static keys; static preventExtensions; static seal; static values;
  proto constructor; proto hasOwnProperty; proto isPrototypeOf; proto propertyIsEnumerable;
  proto toLocaleString; proto toString ([object Tag]); proto valueOf;
  proto __proto__ accessor;
  proto __defineGetter__/__defineSetter__/__lookupGetter__/__lookupSetter__ (legacy).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Function`
- [[Call]]/[[Construct]] separation; new.target; default-arg/rest/destructuring parameter forms
- Surface:
proto length; proto name; proto prototype; proto apply(thisArg, args);
  proto bind(thisArg, ...args); proto call(thisArg, ...args);
  proto toString (source slice per §5.14.7); proto arguments/caller poisoned in strict;
  static (no).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Boolean`
- Wrapper objects with sloppy-mode coercion
- Surface:
constructor(value); proto toString; proto valueOf.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Symbol`
- Well-known symbol dispatch in the interpreter (§5.14.2)
- Surface:
static for(key); static keyFor(sym); static asyncIterator; static hasInstance;
  static isConcatSpreadable; static iterator; static match; static matchAll; static replace;
  static search; static species; static split; static toPrimitive; static toStringTag;
  static dispose/asyncDispose; proto description; proto toString; proto valueOf.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Error`
- options.cause; stack formatting with source spans (§5.13)
- Surface:
constructor(message, options); proto name; proto message;
  proto stack (own, captureStackTrace semantics); static isError (new);
  subclasses EvalError/RangeError/ReferenceError/SyntaxError/TypeError/URIError/AggregateError(errors, message).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Number`
- Canonical numeric string forms per spec §6.1.6.1
- Surface:
EPSILON; MAX_SAFE_INTEGER; MIN_SAFE_INTEGER; MAX_VALUE; MIN_VALUE; NEGATIVE_INFINITY;
  POSITIVE_INFINITY; NaN; static isFinite; static isInteger; static isNaN;
  static isSafeInteger; static parseFloat; static parseInt; constructor(value);
  proto toExponential(digits); proto toFixed(digits); proto toLocaleString;
  proto toPrecision(precision); proto toString(radix); proto valueOf.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `BigInt`
- Mixed BigInt/Number arithmetic throws per spec
- Surface:
constructor(value); static asIntN(bits, v); static asUintN(bits, v); proto toString(radix);
  proto valueOf; proto toLocaleString.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Math`
- All semantics exactly per spec (rounding modes matter: round-half-up, floor toward -inf)
- Surface:
E; LN10; LN2; LOG10E; LOG2E; PI; SQRT1_2; SQRT2; abs; acos; acosh; asin; asinh; atan; atan2;
  atanh; cbrt; ceil; clz32; cos; cosh; exp; expm1; floor; fround; hypot; imul; log; log10;
  log1p; log2; max; min; pow; random (seeded, test-injected); round; sign; sin; sinh; sqrt;
  tan; tanh; trunc.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Date`
- Local timezone from the platform; ISO parse subset per spec; invalid-date NaN semantics
- Surface:
constructor variants (no-arg, ms, string ISO, y/m/d/h/m/s/ms); static now();
  static parse(s); static UTC(...);
  proto getDate/getDay/getFullYear/getHours/getMilliseconds/getMinutes/getMonth/getSeconds/getTime/getTimezoneOffset/getDate UTC variants;
  proto setDate/setFullYear/setHours/setMilliseconds/setMinutes/setMonth/setSeconds/setTime/setMilliseconds + UTC variants;
  proto toISOString; proto toJSON;
  proto toString/toDateString/toTimeString/toUTCString/toISOString/toLocaleString;
  proto valueOf; proto [Symbol.toPrimitive].
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `String`
- UTF-16 string model (§5.14.1); well-formedness on USV boundaries in host crossings
- Surface:
static fromCharCode(...units); static fromCodePoint(...points); static raw(template);
  constructor(value); proto length (UTF-16 units); proto [index]; proto at(i);
  proto charAt(i); proto charCodeAt(i); proto codePointAt(i); proto concat;
  proto endsWith(s, end); proto includes(s, pos); proto indexOf(s, from);
  proto lastIndexOf(s, from); proto localeCompare; proto match(re); proto matchAll(re);
  proto normalize(form); proto padEnd(len, fill); proto padStart(len, fill);
  proto repeat(n); proto replace(search, repl); proto replaceAll(search, repl);
  proto search(re); proto slice(start, end); proto split(sep, limit);
  proto startsWith(s, pos); proto substring(a, b); proto substr (legacy annex);
  proto toLowerCase/toUpperCase; proto toLocaleLowerCase/UpperCase; proto trim;
  proto trimStart/trimEnd; proto [Symbol.iterator]; proto toString/valueOf.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `RegExp`
- Own regex engine: backtracking with the ES semantics (§6.7 WBS); no lookbehind initially — ADR when added
- Surface:
constructor(pattern, flags); static (species); proto source; proto flags; proto global;
  proto ignoreCase; proto multiline; proto dotAll; proto unicode; proto unicodeSets;
  proto sticky; proto hasIndices; proto lastIndex (get/set); proto exec(s); proto test(s);
  proto toString; proto [Symbol.match/matchAll/replace/replaceAll/search/split].
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Array`
- Hole semantics (empty slots) in iteration; species-driven subclass results
- Surface:
static isArray(v); static from(src, mapFn, thisArg); static of(...items); static fromAsync;
  constructor(len or items); proto length (get/set with truncation rules); proto at(i);
  proto concat; proto copyWithin(target, start, end); proto entries;
  proto every(cb, thisArg); proto fill(value, start, end); proto filter(cb);
  proto find(cb)/findLast; proto findIndex/findLastIndex; proto flat(depth);
  proto flatMap(cb); proto forEach(cb); proto includes(v, from); proto indexOf(v, from);
  proto join(sep); proto keys; proto lastIndexOf(v, from); proto map(cb); proto pop;
  proto push(...items); proto reduce(cb, init)/reduceRight; proto reverse; proto shift;
  proto slice(start, end); proto some(cb); proto sort(cmp) (stable per spec);
  proto splice(start, deleteCount, ...items); proto toLocaleString;
  proto toReversed/toSorted/toSpliced/with (immutable set); proto unshift(...items);
  proto values; proto [Symbol.iterator]; proto [Symbol.species].
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `%TypedArray%`
- Bounds checks; canonical-numeric-index strings; detach semantics (no detach in MVP — never transferred)
- Surface:
static from/of; proto buffer/byteLength/byteOffset/length; proto set(arr, offset);
  proto subarray(start, end); proto fill; proto copyWithin;
  proto indexOf/includes/lastIndexOf; proto join; proto reverse;
  proto sort(cmp) (numeric default); proto slice; proto entries/keys/values; proto at;
  proto find/findIndex family; proto every/some/forEach/map/filter/reduce family;
  static Int8Array/Uint8Array/Uint8ClampedArray/Int16Array/Uint16Array/Int32Array/Uint32Array/Float32Array/Float64Array/BigInt64Array/BigUint64Array.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `ArrayBuffer`
- SharedArrayBuffer: same shape, isShared flag
- Surface:
constructor(byteLength, opts); proto byteLength; proto slice(start, end); static isView(v).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `DataView`
- Alignment-free typed access; endianness argument default false (big)
- Surface:
constructor(buffer, offset, length); proto buffer/byteLength/byteOffset;
  proto getBigInt64/BigUint64/getFloat32/Float64/getInt8/16/32/getUint8/16/32(offset, littleEndian);
  proto set* family (offset, value, littleEndian).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Atomics`
- MVP: non-shared buffers allowed where spec permits; wait throws on non-shared
- Surface:
static add/and/compareExchange/exchange/load/or/store/sub/wait/notify/xor(typedArray, index, ...).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Map`
- Insertion order; SameValueZero keys; hash-consed key table
- Surface:
constructor(iterable); attr size; proto clear(); proto delete(key); proto entries;
  proto forEach(cb, thisArg); proto get(key); proto has(key); proto keys;
  proto set(key, value); proto values; proto [Symbol.iterator]; proto [Symbol.toStringTag].
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Set`
- SameValueZero membership
- Surface:
constructor(iterable); attr size; proto add(value); proto clear(); proto delete(value);
  proto entries; proto forEach; proto has(value); proto keys/values;
  proto union/intersection/difference/symmetricDifference/isSubsetOf/isSupersetOf/isDisjointFrom (ES2025 set methods);
  proto [Symbol.iterator].
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `WeakMap`
- Keys are objects/symbols registered as weak refs in the GC
- Surface:
constructor; proto delete(key); proto get(key); proto has(key); proto set(key, value).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `WeakSet`
- Weak membership
- Surface:
constructor; proto add(value); proto delete(value); proto has(value).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `WeakRef`
- Sweep-time liveness
- Surface:
constructor(target); proto deref().
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `FinalizationRegistry`
- Callbacks queued as tasks, never during GC (§5.14.6)
- Surface:
constructor(cleanup); proto register(target, held, token); proto unregister(token);
  proto cleanupSome().
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Promise`
- Reaction jobs drain at the microtask checkpoint (§5.15.3); unhandled-rejection tracking
- Surface:
static all(iterable); static allSettled; static any; static race; static resolve(v);
  static reject(r); static try(fn); static withResolvers; constructor(executor);
  proto then(onFul, onRej); proto catch(onRej); proto finally(onSettled).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Iterator helpers`
- Lazy iterator adapters; manual protocol implementation
- Surface:
proto map/filter/take/drop/flatMap/reduce/toArray/toAsync; proto [Symbol.iterator];
  %IteratorPrototype% chain.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Generator`
- Frame-suspend resume state machine (§5.14.4); async generators add the queue model
- Surface:
%GeneratorPrototype%: proto next(v); proto return(v); proto throw(e);
  proto [Symbol.iterator].
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Proxy`
- Invariant enforcement per spec; revocable via Proxy.revocable
- Surface:
constructor(target, handler);
  13 traps invoked in spec order: getPrototypeOf/setPrototypeOf/isExtensible/preventExtensions/getOwnPropertyDescriptor/defineProperty/has/get/set/deleteOwnProperty/ownKeys/apply/construct.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Reflect`
- Mirror of the internal methods; used by Proxy default behaviors
- Surface:
static apply(f, thisArg, args); static construct(f, args, newTarget); static defineProperty;
  static deleteProperty; static get(target, key, receiver); static getOwnPropertyDescriptor;
  static getPrototypeOf; static has; static isExtensible; static ownKeys;
  static preventExtensions; static set(target, key, value, receiver); static setPrototypeOf.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `JSON`
- Own parser/serializer; exact number formatting (shortest round-trip double printing); well-formed stringify (lone surrogates escaped)
- Surface:
static parse(text, reviver); static stringify(value, replacer, space); static rawJSON (new);
  static isRawJSON.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Intl`
- Graceful stub per §5.14.7; ADR to extend
- Surface:
static Intl object with Collator/DateTimeFormat/NumberFormat/PluralRules/Segmenter constructors present but constructing throws NotSupportedError with a clear message.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `console`
- §5.15.4 formatting into the DevTools ring buffer
- Surface:
log/debug/info/warn/error; assert; clear; count/countReset; group/groupEnd/groupCollapsed;
  time/timeEnd/timeLog; trace; table.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `URI functions`
- Escape/unescape (annex B) included for compatibility
- Surface:
encodeURI/encodeURIComponent/decodeURI/decodeURIComponent per RFC 3986 tables.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Host timers`
- Clamping rules per HTML §8.1.4.2; token invalidation on navigation
- Surface:
setTimeout(fn, delay, ...args); setInterval; clearTimeout; clearInterval.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Host misc`
- Wired per §5.15; structured clone supports the plain-data subset + ArrayBuffer/Map/Set/Date/RegExp/Error/Blob
- Surface:
queueMicrotask(fn); structuredClone(value, opts); atob(s); btoa(s); reportError(err);
  fetch(input, init).
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

#### `Module records`
- Module map with cycle handling (§5.13)
- Surface:
import declarations; export forms (named/default/asterisk/string); import.meta;
  dynamic import(); top-level await.
- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.
- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.

### §6.8 DOM event catalog

For each event: dispatch path (§5.6), the interface binding (§6.6), the
default action (if any) and its `preventDefault` behavior, and an
integration test with the scripted input driver (§8.4).

- [ ] `load` — bubbles: N; cancelable: N; `Event` — fired when: Resource/document finished loading (window, img, script, link, media).
- [ ] `DOMContentLoaded` — bubbles: Y; cancelable: N; `Event` — fired when: HTML fully parsed and deferred scripts ran.
- [ ] `readystatechange` — bubbles: Y; cancelable: N; `Event` — fired when: document.readyState changed (loading/interactive/complete).
- [ ] `beforeunload` — bubbles: N; cancelable: Y; `BeforeUnloadEvent (MVP: Event)` — fired when: Window about to unload; prompts are shell policy.
- [ ] `unload` — bubbles: N; cancelable: N; `Event` — fired when: Document unloading (legacy support).
- [ ] `pagehide/pageshow` — bubbles: N; cancelable: N; `PageTransitionEvent` — fired when: Session history traversal in/out.
- [ ] `error` — bubbles: N/N; cancelable: N; `ErrorEvent or Event` — fired when: Script error (window, bubbles N) or resource fetch error (element, bubbles Y).
- [ ] `abort` — bubbles: Y; cancelable: N; `Event` — fired when: Fetch aborted before completion (media elements).
- [ ] `hashchange` — bubbles: Y; cancelable: N; `HashChangeEvent` — fired when: URL fragment changed.
- [ ] `popstate` — bubbles: Y; cancelable: N; `PopStateEvent` — fired when: Session history entry traversed.
- [ ] `click` — bubbles: Y; cancelable: Y; `MouseEvent` — fired when: Primary activation on an element (synthesized from pointer events).
- [ ] `dblclick` — bubbles: Y; cancelable: Y; `MouseEvent` — fired when: Two clicks within platform threshold.
- [ ] `contextmenu` — bubbles: Y; cancelable: Y; `MouseEvent` — fired when: Secondary button / context menu key.
- [ ] `mousedown/mouseup` — bubbles: Y; cancelable: Y; `MouseEvent` — fired when: Button press/release.
- [ ] `mousemove` — bubbles: Y; cancelable: Y; `MouseEvent` — fired when: Pointer moved over the document.
- [ ] `mouseover/mouseout` — bubbles: Y; cancelable: Y; `MouseEvent` — fired when: Pointer entered/left an element's hit area (bubbling).
- [ ] `mouseenter/mouseleave` — bubbles: N; cancelable: N; `MouseEvent` — fired when: Non-bubbling enter/leave on the element itself.
- [ ] `wheel` — bubbles: Y; cancelable: Y; `WheelEvent` — fired when: Scroll wheel/delta input; preventDefault stops scrolling.
- [ ] `pointerdown/pointerup` — bubbles: Y; cancelable: Y; `PointerEvent` — fired when: Pointer press/release (pointerId model).
- [ ] `pointermove` — bubbles: Y; cancelable: Y; `PointerEvent` — fired when: Pointer moved.
- [ ] `pointerover/pointerout` — bubbles: Y; cancelable: Y; `PointerEvent` — fired when: Bubbling pointer enter/leave.
- [ ] `pointerenter/pointerleave` — bubbles: N; cancelable: N; `PointerEvent` — fired when: Non-bubbling enter/leave.
- [ ] `pointercancel` — bubbles: Y; cancelable: N; `PointerEvent` — fired when: Pointer interaction taken over (touch scroll).
- [ ] `gotpointercapture/lostpointercapture` — bubbles: Y; cancelable: N; `PointerEvent` — fired when: setPointerCapture transitions (MVP: implicit capture on touch).
- [ ] `keydown` — bubbles: Y; cancelable: Y; `KeyboardEvent` — fired when: Key pressed; preventDefault stops text input and default actions.
- [ ] `keyup` — bubbles: Y; cancelable: Y; `KeyboardEvent` — fired when: Key released.
- [ ] `keypress` — bubbles: Y; cancelable: Y; `KeyboardEvent` — fired when: Legacy character-producing key (sourced from keydown).
- [ ] `beforeinput` — bubbles: Y; cancelable: Y; `InputEvent` — fired when: Editable content about to change; preventDefault blocks it.
- [ ] `input` — bubbles: Y; cancelable: N; `InputEvent` — fired when: Editable content changed (text areas, inputs, select).
- [ ] `change` — bubbles: Y; cancelable: N; `Event` — fired when: Commit of a new value (checkbox, radio, select, file, blur-commit of text).
- [ ] `compositionstart/compositionupdate/compositionend` — bubbles: Y; cancelable: N; `CompositionEvent` — fired when: IME session lifecycle.
- [ ] `focus` — bubbles: N; cancelable: N; `FocusEvent` — fired when: Element received focus.
- [ ] `blur` — bubbles: N; cancelable: N; `FocusEvent` — fired when: Element lost focus.
- [ ] `focusin/focusout` — bubbles: Y; cancelable: N; `FocusEvent` — fired when: Bubbling focus transitions.
- [ ] `submit` — bubbles: Y; cancelable: Y; `SubmitEvent` — fired when: Form submission requested (preventDefault blocks navigation).
- [ ] `reset` — bubbles: Y; cancelable: Y; `Event` — fired when: Form reset requested.
- [ ] `select` — bubbles: Y; cancelable: N; `Event` — fired when: Text selection changed inside an editable.
- [ ] `invalid` — bubbles: Y; cancelable: N; `Event` — fired when: Constraint validation failed (reportValidity path).
- [ ] `search` — bubbles: Y; cancelable: N; `Event` — fired when: type=search Enter (legacy).
- [ ] `dragstart/dragend` — bubbles: Y; cancelable: Y/N; `DragEvent` — fired when: Drag session lifecycle (simple drag-store MVP).
- [ ] `dragenter/dragleave/dragover/drop` — bubbles: Y; cancelable: Y for over/drop; `DragEvent` — fired when: Drag over targets; drop needs preventDefault(over)+drop.
- [ ] `scroll` — bubbles: Y/N; cancelable: N; `Event` — fired when: Element scrolled (bubbles: document) — also fires rAF-aligned on programmatic scrolls.
- [ ] `resize` — bubbles: N; cancelable: N; `Event` — fired when: Viewport (window) resized.
- [ ] `canplaythrough/loadedmetadata/loadeddata` — bubbles: N; cancelable: N; `Event` — fired when: Media element state advances (stub per §1.4).
- [ ] `play/pause/ended` — bubbles: N; cancelable: N; `Event` — fired when: Media element state (stub).
- [ ] `transitionrun/transitionstart/transitionend/transitioncancel` — bubbles: Y; cancelable: N; `TransitionEvent` — fired when: CSS transition lifecycle.
- [ ] `animationstart/animationiteration/animationend/animationcancel` — bubbles: Y; cancelable: N; `AnimationEvent` — fired when: CSS animation lifecycle.
- [ ] `message` — bubbles: N; cancelable: N; `MessageEvent` — fired when: postMessage/MessagePort delivery.
- [ ] `messageerror` — bubbles: N; cancelable: N; `MessageEvent` — fired when: Undeserializable message (MVP: rare).
- [ ] `online/offline` — bubbles: N; cancelable: N; `Event` — fired when: Network connectivity changed.
- [ ] `storage` — bubbles: N; cancelable: N; `StorageEvent` — fired when: localStorage changed in another same-origin tab.
- [ ] `visibilitychange` — bubbles: Y; cancelable: N; `Event` — fired when: Tab visibility changed (shell hook).
- [ ] `fullscreenchange` — bubbles: Y; cancelable: N; `Event` — fired when: Never fires in this engine (non-goal §1.4); binding exists.
- [ ] `copy/cut/paste` — bubbles: Y; cancelable: Y; `ClipboardEvent` — fired when: Clipboard operations (text-only MVP).
- [ ] `securitypolicyviolation` — bubbles: Y; cancelable: N; `SecurityPolicyViolationEvent` — fired when: CSP violation detected (§5.18.4).
- [ ] `unhandledrejection` — bubbles: Y; cancelable: Y; `PromiseRejectionEvent` — fired when: Promise rejected with no handler at checkpoint.
- [ ] `rejectionhandled` — bubbles: Y; cancelable: N; `PromiseRejectionEvent` — fired when: Late handler attached to a reported rejection.
- [ ] `toggle` — bubbles: N; cancelable: N; `Event` — fired when: details open state changed.

### §6.9 Network and protocol checklist

Loader-level behaviors (§5.1–§5.3); each item carries a mock-server test.

- [ ] Scheme handling: `http`, `https`, `file`, `data`, `about:blank`, `about:srcdoc`; unknown scheme → error page.
- [ ] URL normalization before fetch; fragment stripped on the wire; base resolution for every subresource.
- [ ] GET pipeline end-to-end: DNS → connect → TLS → request → response, all phases cancelable (§5.2.5).
- [ ] Redirect chain semantics (301/302/303/307/308) including body dropping and method preservation.
- [ ] Keep-alive pooling with idle expiry; connection error → one clean retry on a fresh connection.
- [ ] Chunked body decoding; content-length framing; until-close fallback with a console note.
- [ ] gzip/deflate content decoding (owned inflate, or the approved crate until it lands).
- [ ] Conditional revalidation flow (ETag + Last-Modified) against the disk cache.
- [ ] Vary-keyed cache entries; no-store honored; stale-while-revalidate treated as stale (documented).
- [ ] Cookie jar read/write on send/response with SameSite + Secure rules (§5.17.1).
- [ ] CORS simple request, preflight round-trip, credentialed request, wildcard rules (§5.18.3).
- [ ] Referrer generation per policy, downgrade stripping.
- [ ] HSTS upgrade-before-connect and policy expiry (§5.3).
- [ ] Content sniffing table for images and top-level text/html/text/plain ambiguity; nosniff honored.
- [ ] Timeout matrix per phase; abort mid-body surfaces a clean network error to the pipeline.
- [ ] HTTP/1.1 protocol violations mapped to NetError::Protocol with the offending bytes logged (debug).
- [ ] Mock-server harness: per-test server with scripted responses, delays, and truncations (§8.2).
- [ ] Byte-exactness tests: emitted request lines/headers match the recorded golden bytes.
- [ ] WebSocket handshake + frame codecs (client side) with the event surface of §6.6.
- [ ] Non-GET methods (POST/PUT/DELETE/HEAD/OPTIONS) for fetch with body framing rules.
- [ ] DNS resolution with TTL caching and the hosts-file override for tests.
- [ ] Connection coalescing guard: two concurrent fetches to one origin use the pool, not two sockets (unless over the 6-connection cap).
- [ ] Request body streaming from fetch (chunked upload) and its backpressure story.
- [ ] Response body error injection tests: truncation, invalid chunk size, premature close.
- [ ] Proxy configuration (env vars) honored at the loader layer with tests via a local proxy harness.
- [ ] Integrity metadata (SRI) verification for scripts/styles with failure = network error.
- [ ] DevTools network event emission for every request lifecycle transition (§5.19.5).

### §6.10 Storage and persistence checklist

Persistence surface (§5.17); every item includes a crash-recovery or
corruption-recovery test.

- [ ] Cookie jar: RFC 6265bis §5.1–5.6 algorithms, host-only vs domain cookies, path-match, sort order.
- [ ] Cookie prefixes `__Secure-`/`__Host-` enforced; Secure-only delivery; SameSite default Lax.
- [ ] Cookie partitioning by top-level site; public-suffix list file loaded and versioned.
- [ ] document.cookie serialization (one string, semicolon-joined) honoring HttpOnly invisibility.
- [ ] localStorage per-origin file: write-temp-rename, version byte, CRC per record, load-recovery test.
- [ ] sessionStorage per-tab lifecycle: cleared on tab close, not shared across tabs, no storage events.
- [ ] Storage events delivered to other same-origin tabs with old/new values.
- [ ] Quota enforcement (5 MB UTF-16 units) with QuotaExceededError and a console message.
- [ ] HTTP disk cache: entry format (headers + body + metadata + version), sharded directories.
- [ ] Cache freshness math: Age, heuristic freshness, must-revalidate, no-cache revalidation.
- [ ] LRU eviction on byte budget with pinning for in-flight resources; eviction test with tiny budget.
- [ ] HSTS store persistence and expiry; security-state versioning (§4.8).
- [ ] Profile layout `~/.aurora/<profile>/` per §5.17; `--profile` flag; temp profiles in tests.
- [ ] Clear-browsing-data (cookies, storage, cache) with in-flight navigation safety.
- [ ] Cookie jar eviction: expired-cookie sweep on load and on a periodic timer.
- [ ] Storage keying includes the origin's port and scheme (tuple origin, §5.18.1).
- [ ] Disk-space accounting: stores report their footprint to DevTools (§5.19.5).
- [ ] Cache checksum-per-record verified on read; corrupt entry evicted, store survives.
- [ ] sessionStorage survives tab reloads but not tab close (test both).
- [ ] document.cookie set/delete round-trip through the jar with path-scoped deletion.
- [ ] Preferences file (shell) versioned, atomically written, hot-reloaded on the settings page.

### §6.11 Keyboard and input map

Input plumbing checks (§5.19.3): platform events → engine input messages →
DOM events with correct `key`/`code`/modifiers. The full key table is
Appendix F; here, the wiring items:

- [ ] Platform key events mapped through the Appendix F table to KeyboardEvent key/code values.
- [ ] Modifier liveness: ctrl/shift/alt/meta state correct across focus changes and getModifierState.
- [ ] Text input funnel: keydown (default-check) → composition (if IME) → beforeinput → input.
- [ ] Focus navigation via Tab/Shift+Tab over the sequential focus navigation order.
- [ ] Scroll keys (Space, arrows, PageUp/Down, Home/End) hit the focused scroller or the document.
- [ ] Shortcut dispatch order: shell shortcuts first, then page keydown handlers (§5.19.1).
- [ ] Mouse: hit-test → enter/leave pairing → down/up → click synthesis with button/bitmask rules.
- [ ] Wheel: delta normalization, scroll chaining from innermost scroller outward, listener default action.
- [ ] Pointer: pointerId assignment, implicit capture for touch, mouse-event synthesis from pointers.
- [ ] IME: composition event sequence with correct data/isComposing through the editing funnel.
- [ ] Drag-and-drop of files onto the window routed to the drop event or navigation (§5.19.3).
- [ ] Cursor and tooltip updates from the hit-test result rendered by the shell.
- [ ] Repeat-key rate: keydown repeat timing surfaces per platform conventions.
- [ ] Alt-key menu acceleration does not leak into page key handlers when the shell consumes it.
- [ ] Zoom (Ctrl+wheel) changes the page zoom factor and re-runs layout, not a bitmap scale.
- [ ] Text selection drag: mousemove selection updates with shift-extension and double/triple-click word/line selection.
- [ ] Focus follows click on editable areas with caret placement at the click point.

### §6.13 Milestone → WBS mapping

Which WBS sections each milestone claims. An item may be *introduced*
in one milestone and *completed* in another — the ledger (§6.12) records
completion; this table records planning intent. `→` marks completion of
work introduced earlier.

| Milestone | Claims (introduce → complete) |
|---|---|
| M0 Bootstrap | — (infrastructure only; no WBS items) |
| M1 Fetch | §6.9 network items 1–7; Appendices C/D tables land |
| M2 HTML→DOM | §6.2 all parse/tree tasks; §6.6 DOM-core interfaces; §6.5 selector parsing only |
| M3 Style | §6.3 properties flagged [M3]; §6.4 at-rules parse+evaluate; §6.5 matching for basic/structural selectors |
| M4 Block layout | §6.3 box/flex/grid group *parsing* complete; layout integration for box+table groups begins |
| M5 Paint | §6.3 color/bg/border integration; pixel corpus opens |
| M6 Window | §6.3 ui-group scroll subset; §6.11 wiring items 1–4 (keys) and 7 (mouse) |
| M7 Text | §6.3 font+text integration; §6.5 linguistic selectors; Appendix E unit resolution complete |
| M8 Images | §6.3 image-bearing properties (object-fit, background-image); decoder fuzz targets open |
| M9 JS engine | §6.7 all [M9] builtin blocks; test262 slices adopted |
| M10 Scriptable DOM | §6.6 remaining [M10] interfaces; §6.8 all events; §6.11 wiring items 5–6, 8–10 |
| M11 Shell | — (§5.19 surface; §6.11 shell shortcuts; DevTools v1) |
| M12 Storage | §6.10 all; §6.9 items 8–13 →; §6.5 resource-state selectors |
| M13 Release | every remaining unchecked item → complete or struck with waiver (§6.12) |

Reading rule for §12.3 task selection: the live claim set of the current
milestone defines which sections' unchecked items are eligible; the
dependency-first override (§12.3) may pull from a later claim set only
when the current section names it as a prerequisite.
### §6.12 The completion ledger rules

1. **Check means verified.** A box is ticked only with: code merged, the
   item's named tests green, and the session report (§11.4) referencing it.
2. **Waivers are explicit.** Anything consciously not done gets `~~struck~~`
   text plus a one-line waiver reason and, if architectural, an ADR. Silent
   gaps are the one dishonesty this project cannot survive.
3. **Regeneration discipline.** This section is generated
   (`tools/generate_wbs.py`); edits go to the data files, never the generated
   markdown. The generator is idempotent: regenerating with unchanged data
   must produce a byte-identical file (CI asserts this).
4. **Progress accounting.** `scripts/wbs-progress.sh` counts checked/total per
   §6.x and writes the percentage into `PROGRESS.md` (§12.1). Milestone exit
   reports quote it.
5. **Ordering is advisory.** Within a section, work top-to-bottom; across
   sections, the milestone's exit criteria (Part 7) choose which sections
   are live. Dependency-first overrides (§12.3) beat document order.

### §6.14 The progress script specification

`scripts/wbs-progress.sh` (implemented in the shell of your choice, keep it
dependency-free) produces the numbers quoted in `PROGRESS.md`:

1. Input: this document (Part 6 only). A WBS item is a line matching
   `^- \[ \] ` (open) or `^- \[x\] ` (done, case-insensitive on the x).
2. Output: a table of `section | open | done | total | percent` for §6.2
   through §6.13, plus a grand total row, sorted by section number.
3. Struck-through items (`~~...~~`) count in `total` as waived: they are
   excluded from both open and done, and reported in their own column
   so waivers stay visible (§6.12 rule 2).
4. Exit code 0 always; it is a reporting tool, not a gate — the gates
   are the test tiers (§8.8).
5. A `--strict` flag exits non-zero if any §6.x shows done + waived < total
   (used at milestone exits and at M13).

## PART 7 — Milestones and Acceptance

### §7.1 Milestone discipline

A milestone is **done** when its exit criteria are all verified in the session
report, its demo runs from one command, and `PROGRESS.md` records the milestone's
WBS completion percentage. Do not start milestone M+1 with M's exit criteria
unverified, and do not expand a milestone's scope mid-flight — new ideas go to the
WBS backlog, not into the current milestone. Each milestone section below lists:
Objective, Scope (in/out), Demo, Exit criteria, and Risks.

### §7.2 M0 — Bootstrap and toolchain

**Objective:** an empty-but-real repository where `cargo build && cargo test` runs
a workspace with every crate of §4.3 present, each with a smoke test.
**In scope:** workspace + crates skeletons, lint/fmt configs, CI workflow file,
`PROGRESS.md` + `docs/` scaffolding, the two-platform CI matrix, `test:fast`
script (§8.8), the ADR directory, the dependency-check script (§5.19).
**Out of scope:** any web behavior.
**Demo:** `cargo run -p aurora_shell -- --version` prints name/version/commit;
CI runs the fast tier on push.
**Exit criteria:** build clean with `-D warnings`; every crate has a passing
placeholder test; docs of §11.2 exist as stubs with correct structure; the
dependency check script runs and passes (no engine crate depends on shell).
**Risks:** none worth writing down — this milestone exists to force the plumbing
questions early.

### §7.3 M1 — Fetch and render text

**Objective:** `aurora --url http://example.com` prints the response bytes
(then just the text) to stdout. End-to-end: URL → DNS → TCP → TLS → HTTP → bytes.
**In scope:** §5.1 URL, §5.2 HTTP/1.1 core (GET, redirects, chunked, gzip),
§5.3 TLS, `file://` and `data:` schemes, the loader thread, a `--dump-bytes` /
`--dump-text` CLI, a mock-server test harness.
**Out of scope:** HTML semantics (text extraction is "print decoded body").
**Demo:** three commands: fetch `http://example.com` over the real network (or
mock in CI), fetch `https://` counterpart, `data:text/html,<h1>hi</h1>`.
**Exit criteria:** §5.1–§5.3 definitions of done; mock-server suite green;
`--dump-bytes` byte-identical to `curl -s` output on the five test URLs.
**Risks:** TLS trust store friction on the three platforms — mitigated by
`webpki-roots` default and a documented `--cert-bundle` override.

### §7.4 M2 — HTML to DOM

**Objective:** `--dump-dom` prints a serialized DOM tree for any real page.
**In scope:** §5.4 tokenizer, §5.5 tree construction (all insertion modes),
encoding detection, the DOM core of §5.6 without script, serialization.
**Out of scope:** scripting, styling.
**Demo:** `--dump-dom http://example.com` then a diff of the output against a
golden file; `--dump-dom --errors` shows parse-error table.
**Exit criteria:** html5lib-tests tokenizer + tree-construction 100%;
`document.write` torture test; the top-50-URLs corpus (§8.6) parses without panic.
**Risks:** the adoption agency algorithm's test cases are subtle — budget a full
session for it alone and land its 40 cases as a first-class test file.

### §7.5 M3 — CSS and the style system

**Objective:** `--dump-style <selector>` prints computed styles per element.
**In scope:** §5.7 parser, §5.8 cascade/computed values for the WBS §6.3
properties marked `[M3]`, stylesheet loading (inline + `<link>`), `@import`,
media query parsing (screen/print/width), the UA stylesheet v1 (needed by every
later milestone — write it carefully, it is a product artifact).
**Out of scope:** layout-dependent values, selectors needing layout state.
**Demo:** a page with three stylesheets, imports, specificity conflicts, and
!important — dump shows the winner per element with source locations.
**Exit criteria:** §5.7/§5.8 definitions of done; cascade torture matrix;
css-parsing-tests green; UA stylesheet covered by style tests.
**Risks:** custom property substitution semantics — implement the spec's
substitution *before* shortcuts, it contaminates everything later.

### §7.6 M4 — Block layout

**Objective:** `--render ascii out.txt` renders the page as an ASCII/box-drawing
approximation of the layout (blocks as boxes with sizes, text as ¶ placeholders
with wrap counts), and `--layout-debug` prints the fragment tree.
**In scope:** §5.9 items 1–5 (boxes, BFC, margin collapse, inline *skeleton*
without real text — fixed-width metrics placeholder recorded as a standing
placeholder until M7, floats, positioning), the box builder.
**Out of scope:** painting, real text, images.
**Demo:** the CSS 2.1 test-page subset renders; the margin-collapse suite prints
expected geometries.
**Exit criteria:** §5.9's golden-file suite (except text cases) green; fragment
tree dump matches expected geometry files.
**Risks:** percentage-height resolution chains — cover with 10 dedicated cases
before building anything on top.

### §7.7 M5 — Painting to pixels

**Objective:** `--render png out.png` writes a real raster of the page.
**In scope:** §5.16 display list + software rasterizer (fills, borders, radius,
clips), background properties, border properties, color; PPM/PNG output (PNG via
the owned encoder or approved crate *for output only* — input decoding stays M8).
**Out of scope:** text, images, scrolling.
**Demo:** the "CSS backgrounds & borders" showcase page renders to PNG; pixel
test harness lands with the first 30 golden images.
**Exit criteria:** §5.16 definition of done minus text; pixel tests byte-stable
across 3 consecutive runs and 2 machines.
**Risks:** AA determinism — settle the fixed-point design in the first week of
the milestone; retrofitting determinism later is a disaster.

### §7.8 M6 — The first window

**Objective:** a window opens showing the rendered page; scrolling works.
**In scope:** §3.6 surface presentation, `aurora_platform` glue, viewport/resize,
wheel scrolling, dirty-rect painting, basic hit testing for cursor changes,
a minimal chrome: address bar + reload only.
**Out of scope:** tabs, real text (still placeholder), script.
**Demo:** navigate `file://` and `http://` pages in the window, resize, scroll.
**Exit criteria:** 60 FPS scroll on the reference page at reference size (§10.1);
frame budget telemetry printed on `--debug-frames`; no input lag over 2 frames.
**Risks:** present/vsync behavior differences across platforms — abstract behind
`Surface` and keep a "present immediately" mode for tests.

### §7.9 M7 — Text and inline layout

**Objective:** real text everywhere: fonts, shaping, line breaking, inline layout.
**In scope:** §5.11 all, §5.9.3 inline layout full, `line-height`, `vertical-align`,
`text-*` properties, the strut, bidi MVP, the UA stylesheet's typography block.
**Out of scope:** auto-hyphenation, vertical writing.
**Demo:** a typography specimen page (serif/sans/mono, weights, sizes, CJK +
Arabic + Cyrillic samples, justified paragraph) renders to PNG and window.
**Exit criteria:** §5.11 definition of done; the pixel tests for text; the
placeholder from M4 is removed (§2.3 standing-placeholder ledger zero for text).
**Risks:** font enumeration differences across platforms — pin the reference
fonts in the test profile; shaping crate integration — wrap it at `TextShaper`
so the owned shaper can grow underneath.

### §7.10 M8 — Images

**Objective:** images render; `alt` text on error; layout reserves space.
**In scope:** §5.12 all four decoders, `<img>`/`<picture>`/`srcset`, intrinsic
sizes + `object-fit`, background-image, image caching, EXIF orientation.
**Out of scope:** SVG, animated GIF playback polish (first frame + timer-driven
advance are in).
**Demo:** an image gallery page with every supported format, a broken image, and
a lazy-loaded image scrolling into view.
**Exit criteria:** §5.12 definition of done; fuzz targets run clean; the gallery
golden image matches.
**Risks:** JPEG decoder schedule — baseline first and ship M8 with it if
progressive slips (record as standing placeholder; progressive lands in M13).

### §7.11 M9 — The JavaScript engine

**Objective:** `--eval "script"` runs real programs; the engine passes its
test262 subset.
**In scope:** §5.13 lexer/parser, §5.14 interpreter, GC, builtins through the
WBS §6.7 `[M9]` set (Object/Array/String/Number/Math/JSON/RegExp/Promise/Map/Set/
Date/Error/TypedArrays), the host API surface (`HostObject` trait) with a
filesystem-free `console` and timers, modules with a `file://` loader for tests.
**Out of scope:** DOM bindings (M10), Intl.
**Demo:** the milestone's scripted demo (a pure-JS todo-list logic module with
tests) runs; test262 subset report printed.
**Exit criteria:** §5.13/§5.14 definitions of done; GC stress suite; benchmark
triple within §10.1.
**Risks:** this is the largest single milestone; the WBS granularity for §6.7
exists precisely to let you slice it — run M9 as three internal checkpoints
(parser / interpreter / builtins).

### §7.12 M10 — Scriptable DOM and events

**Objective:** pages come alive: `document`, `querySelector`, event handlers,
dynamic DOM, forms responding to input.
**In scope:** the binding layer (§5.15.2) for the WBS §6.6 `[M10]` interfaces,
the event system wired to real input (§5.19.3), focus management, `innerHTML`,
MutationObserver, ResizeObserver/IntersectionObserver MVP, rAF loop, form
controls' activation behavior (checkbox/radio/select/button/input-text),
`requestAnimationFrame`-driven paint.
**Out of scope:** shadow DOM (lands M12), workers (non-goal §1.4).
**Demo:** the interactive showcase page: a counter, a tab widget, a form with
validation and dynamic rows, an animated box (rAF + style mutation), and a
MutationObserver-driven logger — all working in the window.
**Exit criteria:** WPT DOM/events adopted subset green; the interaction test
script (§8.4's scripted input driver) passes on the showcase page; soak test
stable.
**Risks:** GC↔DOM reference cycles (script wrapper holds DOM node, node holds
script handler) — the binding registry's root registration rules (§5.14.6) must
be proven with a cycle stress test in this milestone, not discovered later.

### §7.13 M11 — The browser shell

**Objective:** it is a browser: tabs, omnibox, history, bookmarks, downloads,
find-in-page, settings, keyboard shortcuts, DevTools v1 (console + inspector).
**In scope:** §5.19.1, §5.19.2, §5.19.5's Console and Inspector panels, zoom,
`about:blank`/`aurora://` pages, the session history, crash-safe tab restore.
**Out of scope:** cookies UI (M12), Network/Performance panels (M13).
**Demo:** the recorded keyboard-only walkthrough (§5.19) completes: open tab,
navigate three sites, bookmark one, find text, inspect a node in DevTools,
close tabs, restore session.
**Exit criteria:** §5.19 definition of done minus Network/Performance panels;
the shell dependency check stays green; window resize/tab-switch keeps frame
budget.
**Risks:** chrome rendering stealing complexity from the page pipeline —
chrome is *just* display lists; resist any shell-specific rendering path.

### §7.14 M12 — Storage, cookies, and polish

**Objective:** the browser remembers: cookies, localStorage/sessionStorage, HTTP
disk cache, HSTS, form-less persistence; shadow DOM MVP; network panel.
**In scope:** §5.17 all, §5.18 HSTS, shadow DOM MVP (§5.6), CSP enforcement,
`document.cookie`, storage events, the Network DevTools panel, profile
management (`--profile`), the "clear browsing data" settings page.
**Out of scope:** IndexedDB, service workers.
**Demo:** a login-ish page (cookies set via JS, session visible in settings),
a cross-restart localStorage demo, a second visit served from cache shown in
the Network panel, a shadow-DOM component page.
**Exit criteria:** §5.17/§5.18 definitions of done; crash-simulation storage
test; cache hit rate ≥ 90% on the repeat-visit test corpus.
**Risks:** cookie correctness vs real sites' expectations — the SameSite
default (Lax) plus the prefix rules cover the standard cases; log deviations.

### §7.15 M13 — Hardening, performance, release

**Objective:** make it shippable: budgets met, fuzzers run long, the corpus pass
rate is final, docs complete, v0.1 tagged.
**In scope:** §10 budgets measured and met (or ADR'd exceptions), all fuzz
targets × 24 h cumulative, the WPT corpus final report, DevTools Performance
panel, the JPEG progressive decoder if deferred, the zero-shortcut audit
(§2.7), the final documentation pass (§11.2), the release checklist of §12.7.
**Demo:** the full showcase + the top-100-corpus render gallery, generated as a
site anyone can open; the benchmark report table.
**Exit criteria:** §1.6's definition of success verified literally; `v0.1.0`
tagged; release notes published in `CHANGELOG.md`.
**Risks:** schedule pressure inventing shortcuts — the §2.7 ledger makes them
visible; do not trade the zero-shortcut audit for anything.

## PART 8 — Testing Strategy

### §8.1 The test pyramid

From cheapest to most expensive; the fast tier (§8.8) is the bottom two layers:

1. **Unit tests** — one per module, co-located (`#[cfg(test)] mod tests`),
   in-process, microseconds each. Target: every public function has at least one
   test; every parser has a corpus.
2. **Golden-file tests** — structured outputs (DOM dumps, style dumps, fragment
   trees, serialized display lists) compared against checked-in expectations.
3. **Layout/pixel tests** — geometry assertions and byte-exact raster comparisons
   (§8.4–§8.5).
4. **Integration tests** — full engine against the mock server and scripted
   input; per-milestone demos are integration tests with assertions.
5. **Adopted corpus** — html5lib-tests, css-parsing-tests, Web Platform Tests
   subset, test262 subset (§8.6).
6. **Fuzzing** — continuous, with a 30-min minimum per target per milestone and
   a 24-h cumulative gate at M13 (§8.7).

### §8.2 Unit test conventions

- Name: `fn <behavior>_when_<condition>()` — the name is the documentation.
- One assert-cluster per behavior; test data inline and minimal; no shared
  mutable fixtures; construct tiny inputs by hand (`<p>a<i>b</i>c</p>`), not by
  copying corpus files.
- Deterministic: fixed seeds for anything randomized; injected clocks for
  timers (`Clock` trait with a `ManualClock`); no network; no filesystem except
  `tempfile`.
- Property-based tests (`proptest`-style, hand-rolled or via the approved
  dev-crate list) for: URL round-trips, CSS token serialization round-trips,
  DOM operation sequences vs a reference model, GC reachability invariants.
- Every bug fixed in the wild adds the regression test *first* in the fix commit.

### §8.3 Golden-file tests

- Stored under `tests/golden/<area>/`, one file per case, named by case id
  (`cascade-0007.txt`). A header comment states the case's intent in one line.
- Regeneration is explicit and reviewed: `cargo test -- --bless <filter>` —
  and the review rule is: no `--bless` in a commit whose message does not
  contain `bless:` with a reason.
- Diffs are deterministic: sorted keys, stable traversal order, no addresses or
  generation counters in dumps (provide a `debug-stable` formatter per type).
- When a behavior change is *intended*, the golden file's update is described in
  the session report with before/after excerpts.

### §8.4 Layout tests

- Input pages live in `tests/layout/cases/<name>.html` with an adjacent
  `<name>.expected` (fragment-tree geometry in a stable text format).
- The harness: build style+layout at a fixed viewport (800×600 default, 1× DPR),
  dump the fragment tree (`--layout-debug` format), compare.
- Scripted input driver: a test-only `InputInjector` that posts synthetic events
  through the same channel the shell uses, so hover/focus/scroll tests exercise
  the real path. Click tests assert hit-test results (element path), not pixels.
- Reference-font rule: tests that measure text pin the test profile to the
  bundled reference fonts (`tests/fixtures/fonts/`) so geometry is platform-stable.

### §8.5 Pixel tests

- The raster must be deterministic (§5.16.5) — this is a hard prerequisite; the
  suite is invalid without it.
- Comparisons are byte-exact by default. Where AA across shapers legitimately
  varies, the case opts into a per-channel tolerance (≤ 2/255) recorded in the
  case file; anything looser is an ADR.
- Failure output: a three-panel diff image (expected | actual | amplified diff)
  written to `test-output/`, plus the bounding box of the difference in the log.
- The corpus grows with every milestone (§7 exit criteria name their additions);
  the harness prints corpus size and pass rate in the fast tier summary.

### §8.6 Web Platform Tests adoption

Adopt a *subset*, deliberately, and track it — a slow-growing, green, owned
corpus beats a huge red one:

- **Adoption procedure:** each milestone's section names the WPT directories to
  adopt (e.g., M2: `dom/`, `html/syntax/`; M3: `css/CSS2/` cascade pieces,
  `css/selectors/`; M9: test262 `language/`, `built-ins/` subset; M10:
  `dom/events/`, `uievents/`; M12: `cookies/`, `webstorage/`). Adopted tests are
  vendored into `tests/vendor/` with a manifest (`vendor.toml`) recording source
  revision, and any locally-skipped test has an inline skip note with a reason
  code (`platform:`, `non-goal: §1.4`, `deferred: M#`, `bug:#`).
- **Runner:** the harness maps WPT's per-test semantics (reftest-ish pixel or
  JS assertions) onto ours: JS assertion pages run headless with a console-capture
  harness shim we own (`tests/vendor/harness.js`); pixel-ish pages go through
  §8.5.
- **Reporting:** `PROGRESS.md` carries the table: corpus, adopted count, pass,
  fail, skip, pass rate, trend vs last milestone. The M13 gate is the final
  table with no unexplained regressions.

### §8.7 Fuzzing

- Targets (each in `fuzz/`): `fuzz_url`, `fuzz_http_response`, `fuzz_html_tokenizer`,
  `fuzz_html_tree`, `fuzz_css`, `fuzz_js_parser`, `fuzz_js_runtime` (AST mutator
  with a step/time limit), `fuzz_png`, `fuzz_jpeg`, `fuzz_gif`, `fuzz_bmp`,
  `fuzz_paint` (random display lists), `fuzz_dom_api` (operation-sequence grammar).
- Invariant for all: **no panic, no UB, no unbounded memory** (per-target memory
  limit in the harness); corpus seeds checked in; crashes become regression
  tests (minimized) before the fix.
- Cadence: 30 min per target per milestone completion; CI runs 2 min per target
  nightly; M13 requires 24 h cumulative per target with zero open crashes.

### §8.8 Continuous integration

- `scripts/test-fast.sh`: fmt check, clippy `-D warnings`, unit + golden +
  layout + pixel (fast subset), in under 10 minutes on CI hardware. This is the
  pre-commit gate; nothing merges (commits) with it red.
- `scripts/test-all.sh`: everything including the full vendor corpora and the
  2-min fuzz smoke. Runs nightly and before milestone exits.
- CI matrix: Linux (primary), Windows, macOS. A red platform blocks the
  milestone exit; it does not block intermediate commits on other platforms
  (the report notes the breakage and the owner is you — fix next session).
- Artifacts: the three-panel pixel diffs, benchmark reports, corpus pass-rate
  tables are uploaded per run and summarized in milestone reports.
- `scripts/check-deps.sh`: verifies the crate dependency rules of §4.3 (a
  machine-checkable list) and the tier policy of §3.2 — the shell/engine
  boundary is enforced here, not by trust.

### §8.9 Test data inventory

What lives in `tests/` and where it comes from; the CI corpus report prints
this table with live counts.

| Directory | Contents | Source | Notes |
|---|---|---|---|
| `tests/golden/` | Expected outputs for dumps (DOM, style, fragments, display lists) | Hand-written from spec reading | §8.3 blessing rules |
| `tests/layout/cases/` | HTML + expected geometry per case | Hand-written | One intent line per case |
| `tests/pixel/` | Golden PNGs + case tolerances | Generated once, reviewed | §8.5 determinism rules |
| `tests/vendor/html5lib/` | Tokenizer + tree-construction suites | html5lib-tests, pinned revision | §8.6 manifest |
| `tests/vendor/wpt/` | Adopted WPT slices per milestone | WPT, pinned revision | Skip reasons inline |
| `tests/vendor/test262/` | Adopted test262 slices | test262, pinned revision | Per-object manifests |
| `tests/corpus/html/` | Real-page samples for robustness runs | Curated public pages | No privacy-sensitive captures |
| `tests/corpus/images/` | Reference images per decoder | Curated + generated | Hash-verified |
| `tests/corpus/a11y/` | Computed-tree assertions (Appendix N) | Hand-written | One row per line |
| `tests/bench/pages/` | Reference pages incl. showcase | Hand-written + generated | §10.1 budget inputs |
| `tests/fixtures/fonts/` | The reference font set | OFL-licensed families | §8.4 geometry stability |
| `tests/security/` | Adversarial page set (§5.18) | Hand-written | Each violation class |

## PART 9 — Code Quality Standard

### §9.1 Naming

- Types, crates, modules: `UpperCamelCase` / `lower_snake_case` (Rust standard).
  Domain names, not computer names: `FragmentTree`, not `Tree2`; `open_element_stack`,
  not `stack1`.
- Functions are verbs or verb phrases that state the *effect*: `foster_parent_insert`,
  `resolve_flexible_lengths`, `cascade_declarations`. Getters are nouns.
- Booleans read as predicates: `is_quirks`, `should_foster_parent`, `has_definite_height`.
- Spec entities keep spec names: `active_formatting_elements`,
  `insertion_mode::InTableText`, `ordinary_get_own_property`. When the standard and
  Rust style disagree on casing, the standard wins in data (enum variant names may
  adapt casing but not meaning).
- Abbreviations are banned except the codebase's established set, kept in
  `docs/glossary.md` (start it with: `bfc`, `dpr`, `idl`, `ua`, `wpt`, `tdz`).
- No Hungarian, no type-in-names (`url_string`), no scope-in-names
  (`local_counter`), no numbers-in-names (`handler2`).

### §9.2 Module and file conventions

- One subsystem concept per module; a file over ~800 lines is split by its
  natural seams (e.g., the tokenizer by state groups, cascade by phase).
- `pub` is a promise: a crate's public API is listed in its `lib.rs` docs and
  everything else is `pub(crate)` or private. Within the workspace, cross-crate
  imports use the crate's facade module (`aurora_css::prelude`), never deep paths.
- Constants and tables (the named-color table, the character-reference table,
  the window-1252 mapping, the public-suffix list) live in `tables.rs` modules
  with a generation comment stating their source and refresh procedure.
- Test code lives in `#[cfg(test)]` modules for units and `tests/` directories
  for integrations; no `#[allow(dead_code)]` on non-test code (§9.10).

### §9.3 Error handling standard

- Follows §4.6's taxonomy. In code terms:
  - `Result` for resource errors; error enums are per-crate, `#[non_exhaustive]`,
    and carry what the *caller* needs (phase, url, bytes-read), not what the
    *callee* felt (`Internal("should not happen")` is banned).
  - `panic!`/`assert!`/`unreachable!` for programming errors only, always with
    the invariant named. `unwrap()`/`expect()` exist only in tests and in
    initializers that provably cannot fail (`Mutex::new`, literal parsing of
    compile-time constants) — each with a comment proving it.
  - Parsers and decoders: total functions with recovery per spec; errors are
    *collected* (parse-error reports for DevTools), never aborting the stream.
  - Script-facing boundaries throw `DOMException`s from the standard table;
    the mapping from internal errors to exceptions lives in one module
    (`runtime::exception_map`) so it can be audited.
- No error is silently dropped: a discarded `Result` is either matched with a
  justified `Ok(()) => {}`-style arm, logged at debug with context, or converted
  into a user-visible signal (console message, event, status). `let _ =` is
  reserved for provably-infallible operations with a comment.

### §9.4 Unsafe code policy

- The engine aspires to zero `unsafe`. The current allowed set: platform glue in
  `aurora_platform` (FFI to OS APIs), `aurora_text`'s font-file access, and the
  NaN-boxing internals of `aurora_js::value` (documented invariant: every read
  validates the tag discipline).
- Each `unsafe` block has a `// SAFETY:` comment naming the invariant that makes
  it sound *and* the check that keeps the invariant true. A missing `SAFETY`
  comment is a lint failure (§9.6).
- Unsafe is never exposed: a safe public API wraps each unsafe region; fuzz
  targets (§8.7) exercise every unsafe region through its safe wrapper.
- Where a dependency injects `unsafe`, its track record and sandboxing distance
  from engine data is part of the dependency ADR.

### §9.5 Documentation

- Every crate's `lib.rs` opens with a 5–20 line description of its
  responsibility, its thread, and its neighbors (from §4.3's table).
- Public items get doc comments stating behavior, parameters, failure modes,
  and *which standard section* is implemented when it is one (a `Spec:` line).
- Non-obvious decisions get a `// WHY:` comment (constraints, trade-offs) —
  the successor's question is "why is this like this", and the comment answers
  it. `// WHAT:` comments that restate the code are lint-flagged noise.
- `docs/spec-notes/<topic>.md` accumulates the standard-reading summaries
  (§2.2); `docs/adr/` holds decisions (§4.9); `ARCHITECTURE.md` (§11.2) is the
  updated map. Stale docs are bugs: if a change invalidates a doc, the same
  session updates it.

### §9.6 Lint configuration

Committed on day one; the fast tier fails on any deviation:

```toml
# clippy.toml (excerpt) — the full set lives in the file
cognitive-complexity-threshold = 40        # parsers are big; the real guard is review
too-many-arguments-threshold = 10
```

```rust
// lib-level (per crate):
#![warn(clippy::all, clippy::pedantic)]
#![deny(clippy::correctness, clippy::suspicious, clippy::perf,
        clippy::unwrap_used, clippy::expect_used,      // tests exempt via module attr
        clippy::todo, clippy::unimplemented,           // §9.10: tracked placeholders only
        clippy::panic_in_result_fn, clippy::float_cmp)] // float_cmp allowed in layout via helper
#![deny(unsafe_op_in_unsafe_fn, missing_safety_doc)]     // where unsafe exists (§9.4)
```

Allowed-with-reason lints (each opt-out carries a comment): `clippy::result_large_err`
in the parser paths, `module_name_repetitions` in binding layers. New opt-outs
need a session-report line. `#[allow]` at any other site is a review blocker.

### §9.7 Self-review checklist

Run it on every diff before closing the task (it is also §11.4's report item):

- [ ] Does the code do what the task says — the *whole* task, and nothing else?
- [ ] Tests: happy + edge + regression present, run, and green; fast tier green?
- [ ] Names say what things *are*; no §9.1 violations; glossary updated if a new
      abbreviation appeared?
- [ ] Errors follow §9.3; every new `unwrap`/`expect` justified?
- [ ] Any new `unsafe`? Then §9.4 audit: SAFETY comments, safe wrapper, fuzz path?
- [ ] Any dependency, thread, channel message, or crate change? Then ADR + CI
      dependency check?
- [ ] Dead code, debug prints, leftover experiments removed? (§9.10)
- [ ] Docs touched by this change updated (§9.5)? `PROGRESS.md` + WBS ticks done?
- [ ] Would a stranger understand this diff without this conversation? If not,
      add the missing `WHY` comment or split the diff.
- [ ] Performance-sensitive path touched? Then the relevant §10 benchmark was run
      before and after, numbers recorded.

### §9.8 Forbidden patterns

- `dbg!`, `println!` debugging in committed code (the `console`/`log` facades are
  the only printing paths; tests use `cargo test -- --nocapture` consciously).
- `git -n` style "temporary" commits on main; force-pushes to shared branches.
- Time-based behavior without injected clock (§8.2); randomness without seed.
- Reflection-style stringly typing: no parsing of `Debug` output, no keying
  behavior on `type_name` outside diagnostics.
- `unsafe` outside §9.4's set; `transmute` anywhere (there is no current need —
  an ADR would be required to introduce one).
- Catch-all `catch_unwind` to "keep going" — panics are bugs (§4.6), fix them.
- Any dependency on uninitialized memory, alignment hacks, or `size_of` tricks
  outside the NaN-boxing module with its documented invariants.
- Silent data truncation: every narrowing conversion (`as`) is either proven in
  a comment or goes through a checked/`TryFrom` conversion.

### §9.9 Refactoring rules

- Refactors are separate commits with no behavior change, verified by the test
  suite running green *unchanged* (golden files untouched is the proof).
- TheBoy-Scout rule applies with a budget: small cleanups you are already
  touching are welcome; a drive-by refactor that grows the diff past ~30% of
  its behavior change becomes its own task in the WBS.
- Big refactors (interface changes across crates) follow: ADR → migrate one
  caller at a time with deprecation shims → delete the old path in the same
  milestone. Half-migrated states do not survive a milestone boundary.

### §9.10 Dead code, TODOs, and placeholder policy

- `TODO(name-of-wbs-item):` comments are allowed and must reference a WBS item
  or `PROGRESS.md` debt entry — an unanchored TODO is lint-blocked
  (`clippy::todo` set to deny; use the tagged comment form which the audit
  script greps).
- Placeholder implementations are visible: they either return the documented
  not-supported signal (`NotSupportedError`, "unsupported" console message,
  placeholder box) or take the crude-but-correct-for-now path tagged
  `AURORA-SHORTCUT` (§2.7). Both forms appear in the milestone's ledger.
- Dead code is deleted, not commented out; version control remembers. Unused
  `pub` items in internal crates are removed; a needed-later item goes to the
  WBS, not the codebase.

### §9.11 Change description template

Even as a solo engineer, every non-trivial diff gets a description (commit
body or the session report) in this shape — it is the review, since there is
no second reviewer:

```markdown
**Problem:** what was wrong or missing, with the WBS item / milestone.
**Approach:** the chosen design in 2–4 sentences, alternatives rejected, and
why the rejected ones lost.
**Evidence:** test counts (before/after), benchmark rows if §10 applies,
pixel-diff count if paint changed.
**Blast radius:** crates touched, §4.3/§4.4 contract changes (ADR if any),
migration notes for any on-disk format.
```

A diff whose description cannot fill the Evidence section did not verify
itself; per §2.4, go back and run the thing.

## PART 10 — Performance and Resource Budgets

### §10.1 The budget table

Measured on the **reference machine** (documented in `PROGRESS.md`: CPU generation,
RAM, storage class, OS) with the **reference page set** (`tests/bench/pages/`:
`blog.html` ~120 KB, `docs.html` ~400 KB, `app.html` ~1 MB with 5k DOM nodes and
script, `gallery.html` 40 images, `specimen.html` typography-heavy). Budgets are
release builds. When a budget is missed, the fix is: measure → profile → fix →
re-measure; a budget may be *adjusted only by ADR with the old and new numbers
and the reason*.

| # | Metric | Budget | Method |
|---|---|---|---|
| 1 | URL parse | 1 µs median, 10 µs p99 | criterion bench, 1k real URLs |
| 2 | HTTP response parse (headers) | 20 µs p50 for 2 KB headers | criterion |
| 3 | HTML tokenize+tree, 1 MB | ≤ 120 ms | bench `docs.html` |
| 4 | CSS parse, 100 KB of rules | ≤ 20 ms | criterion |
| 5 | Full style cascade, `app.html` (5k elements) | ≤ 40 ms cold | instrumented run |
| 6 | Restyle of a subtree (1k elements) | ≤ 8 ms | instrumented run |
| 7 | Block+inline layout, `docs.html` | ≤ 80 ms cold | instrumented run |
| 8 | Incremental relayout (single text node edit) | ≤ 6 ms | scripted edit loop |
| 9 | Display list build, `app.html` | ≤ 10 ms | instrumented run |
| 10 | Raster 1920×1080 full repaint | ≤ 16 ms | pixel bench |
| 11 | Scroll one frame (cached raster path) | ≤ 4 ms | input-driven bench |
| 12 | JS parse, 1 MB script | ≤ 150 ms | criterion |
| 13 | JS execution: fib(27) | ≤ 350 ms | criterion (tree-walk era) |
| 14 | JS execution: JSON round-trip 5 MB | ≤ 200 ms | criterion |
| 15 | PNG decode 1 MP | ≤ 60 ms | criterion |
| 16 | JPEG baseline decode 1 MP | ≤ 120 ms | criterion |
| 17 | Startup: `aurora --url file:x` to first pixel | ≤ 900 ms cold | traced run |
| 18 | Tab switch | ≤ 50 ms to present | instrumented run |
| 19 | Idle RSS, one tab `blog.html` | ≤ 120 MB | RSS after GC + cache trim |
| 20 | Peak RSS loading `app.html` | ≤ 350 MB | traced run |
| 21 | Disk cache size cap | 512 MB LRU | enforced + test |
| 22 | Image cache cap | 128 MB LRU | enforced + test |
| 23 | Glyph cache cap | 64 MB LRU | enforced + test |
| 24 | GC: major pause | ≤ 8 ms at 64 MB heap | stress harness |
| 25 | Frame pacing under load | 95th frame ≤ 33 ms while scripted-interacting | frame telemetry |

### §10.2 Measurement discipline

- Benchmarks live in `benches/` per crate (criterion) and run in CI nightly;
  a per-milestone run posts the delta table into the milestone report.
- The instrumented run means: the engine's built-in tracing marks
  (`tracing`-style spans compiled out in release profiles *only when* the
  feature is off) around pipeline stages, plus frame telemetry printed by
  `--debug-frames`. Numbers in the table come from these, not from `time cargo run`.
- A regression is defined as > 10% worse than the last recorded number on the
  same machine class. The regression triage happens in the session that
  introduced it — the fast tier's benchmark smoke (`--bench quick`) is the tripwire.

### §10.3 Memory budgets

Every cache has: a byte budget (rows 19–23), an eviction policy (LRU unless a
subsystem section says otherwise), a **pressure response** (on global memory
pressure — the storage/allocator signal or the sum of caches > 80% of budget —
each cache trims to 60% of its budget, oldest first), and a footprint probe
exposed to DevTools (§5.19.5). Arena strategies (DOM) report their slack
(tombstoned fraction) and compact when slack > 30%.

### §10.4 Startup budget

Startup (row 17) decomposes into: process + window platform init (≤ 150 ms),
font DB scan (cached; cold ≤ 300 ms, warm ≤ 50 ms), trust store load (≤ 30 ms),
first navigation fetch and first-frame pipeline (the rest). The warm-start path
(persisted font cache, disk-cache index memory map) is the one measured for row
17's budget; cold-start is measured and reported but budgeted at ×2.

### §10.5 Regression policy

- Every subsystem spec's "definition of done" includes its budget rows; the
  milestone exit runs the table and prints it.
- An optimization lands only with its benchmark proving the win and its tests
  proving no behavior change (§9.9) — speculative optimization is scope drift.
- The per-milestone benchmark delta table is part of `PROGRESS.md`; three
  consecutive regressions on the same row without triage is a milestone
  blocker regardless of feature status.

### §10.6 The benchmark harness specification

`scripts/bench.sh` runs the §10.1 table end to end and emits
`test-output/bench-report.md`:

1. **Builds** the release workspace once, with the tracing feature off (the
   instrumented runs use a separate profile build so release numbers stay pure).
2. **Criterion benches** (`benches/`) run with the fixed sample budget; the
   runner records median + p99 for each named benchmark (the budget table's
   "method" column names the benchmark id).
3. **Scripted runs** (rows 5–11, 17–18, 25) drive the engine through the
   integration harness: the reference pages load, the scripted interactions
   replay, the telemetry lines parse into the table.
4. **Memory rows** (19–24) sample RSS and the cache footprints at the points
   the row defines (after GC + cache trim, at peak, at steady idle).
5. **Comparison**: the report diffs against the previous stored report
   (`test-output/bench-baseline.md`, refreshed only at milestone exits) and
   flags any row > 10% worse (§10.2's regression definition).
6. **Machine note**: the report header embeds the platform summary so numbers
   from different machines are never silently compared (§10.1 reference
   machine rule).

---

## PART 11 — Deliverables, Documentation, Reporting

### §11.1 Repository layout

```
aurora/
├── BROWSER_ENGINE_PROMPT.md      ← this document, kept at repo root (§12.1)
├── PROGRESS.md                   ← the living ledger (§12.1)
├── Cargo.toml / Cargo.lock       ← workspace
├── rust-toolchain.toml, rustfmt.toml, clippy.toml
├── crates/                       ← one directory per §4.3 crate
│   └── aurora_layout/{src,tests,benches}/...
├── apps/aurora_shell/            ← the browser binary
├── docs/
│   ├── ARCHITECTURE.md, GLOSSARY.md, devtools-protocol.md, UA-STYLESHEET.md
│   ├── adr/NNNN-*.md             ← §4.9
│   └── spec-notes/*.md           ← §2.2
├── tests/
│   ├── golden/, layout/, pixel/, vendor/{harness.js,vendor.toml,...}
│   ├── corpus/{html,css,js,images}/
│   ├── security/                 ← §5.18 adversarial page
│   ├── bench/pages/              ← §10.1 reference pages
│   └── fixtures/fonts/           ← reference fonts (§8.4)
├── fuzz/                         ← §8.7 targets
└── scripts/{test-fast.sh,test-all.sh,check-deps.sh,new-milestone.sh}
```

Nothing outside this layout without an ADR; the layout is also the index the
session protocol (§12) greps.

### §11.2 Required documents

| Document | Owner section | Kept accurate by |
|---|---|---|
| `README.md` | §11.2 | every milestone: build/run/test instructions verified verbatim |
| `ARCHITECTURE.md` | §11.2 | every ADR that changes a §4 diagram/table |
| `PROGRESS.md` | §12.1 | every session (the ledger) |
| `docs/GLOSSARY.md` | §9.1 | when a term/abbreviation enters the code |
| `docs/adr/*` | §4.9 | append-only |
| `docs/spec-notes/*` | §2.2 | when reading a standard |
| `docs/UA-STYLESHEET.md` | §7.5 | when the UA stylesheet changes |
| `docs/devtools-protocol.md` | §5.19.5 | when a protocol message changes |
| `CHANGELOG.md` | §11.6 | every milestone exit |
| `docs/known-issues.md` | §11.4 | every session with open debts |

### §11.3 Commit discipline

- One logical change per commit; message: `area: imperative summary` + body
  explaining *why* (the what is in the diff). Areas: `url:`, `net:`, `html:`,
  `dom:`, `css:`, `style:`, `layout:`, `text:`, `image:`, `js:`, `runtime:`,
  `paint:`, `storage:`, `security:`, `shell:`, `devtools:`, `build:`, `docs:`,
  `test:`, `bench:`, `perf:`, `fix:`.
- A commit is green: fast tier passes at that commit (bisectability is a
  requirement, not a nicety).
- Generated files (golden updates via `--bless`, corpus refreshes) carry
  `bless:`/`corpus:` in the message with the reason (§8.3).
- No secrets, no absolute local paths, no machine-specific artifacts; the
  `.gitignore` lands in M0 and is amended only by ADR.

### §11.4 Session report format

Appended to `PROGRESS.md` (§12.1); the template lives at
`scripts/templates/session-report.md`:

```markdown
## Session YYYY-MM-DD-HHMM — <one-line headline>
- **Milestone:** M#   **WBS items touched:** §6.x.y, §6.x.z
- **Implemented:** <bullets, past tense, concrete>
- **Tested:** <suites run> — <N passed / M failed / K skipped>
- **Bench:** <table rows touched: before → after> (or "n/a")
- **Decisions:** <one line each; ADR refs if any>
- **Debts opened/closed:** <AURORA-SHORTCUT / TODO / known-issue entries>
- **Next task:** §6.x.y — <one line why it is next (§12.3 rule applied)>
```

### §11.5 Demo artifacts

Per milestone: the demo command(s) of Part 7 recorded in `PROGRESS.md` with
their outputs (PNG renders checked into `docs/screenshots/M#/`, console output
as text). The final showcase page (`tests/bench/pages/showcase.html` + its
scripted driver) doubles as the project's front-door demo: a static page, no
external network, exercising §1.6's list.

### §11.6 Release notes and changelog

`CHANGELOG.md` per milestone exit: features added (WBS-referenced), fixes
(regression-test-referenced), performance table deltas, known issues, and the
corpus pass-rate table. Releases are tags on the main line; `v0.1.0` at M13.

---

## PART 12 — Session Protocol and State Management

### §12.1 The state file: PROGRESS.md

`PROGRESS.md` is the single source of truth for *where the project is*. Structure:

```markdown
# PROGRESS
## Current state
- Milestone: M# — <name>
- WBS completion: <x>% (auto: scripts/wbs-progress.sh)
- Standing placeholders: <count> (list)
- Open AURORA-SHORTCUT tags: <count> (list with file:line)
- Bench deltas vs last milestone: <table or "none">
## Open questions        ## Debts            ## Decision log (one-liners + ADR refs)
## Session log           (newest first, §11.4 template)
```

Rules: updated **every session, before the report ends**; the WBS checkboxes of
Part 6 in this document are ticked in the same session that completes them
(this document is a living file inside the repo — that is why it lives at the
root); conflicts between `PROGRESS.md` and reality are resolved in favor of
reality, then the file is corrected.

### §12.2 Session start routine

Every session, in order: (1) `git status` / read `PROGRESS.md` top section;
(2) run the fast tier — if red, the session's first task is fixing it (a red
baseline invalidates all progress claims); (3) read the current milestone
section (Part 7) and the next task from §12.3; (4) read the Part 5 spec
sections the task touches; (5) go.

### §12.3 Task selection rule

The next task is, in priority order: (a) a red fast tier or failing corpus
regression; (b) the current milestone's smallest unfinished exit-criteria item;
(c) the first unchecked WBS item of the current milestone in document order —
with one override: **dependency-first** — if the milestone section names an
order (e.g., M9's parser→interpreter→builtins), that order wins over document
order. The chosen task and its rule-letter are recorded in the session report.
Context-switching mid-task is allowed only for (a).

### §12.4 Checkpointing

Work is checkpointed as working commits (§11.3), not as local edits: a session
that dies loses nothing a `git log` cannot restore. Long computations (fuzz
campaigns, corpus runs) write progress files under `test-output/` resumable by
id. `PROGRESS.md` is updated at natural checkpoints (task done, decision made,
debt opened) — not only at session end, because sessions end unexpectedly.

### §12.5 Context-loss recovery

If a session starts mid-task (uncommitted changes present): (1) run the fast
tier; (2) `git diff` to inventory what was in flight; (3) decide: finish (if
small and coherent) or revert (if half-done and unrecoverable — reverting is a
decision recorded in the report, never a silent act); (4) reconcile
`PROGRESS.md`. Never leave the repository on a branch of a branch of guesses.

### §12.6 Multi-session continuity

This document + `PROGRESS.md` + the ADRs are the complete handoff package. Any
successor session (or a fresh you) must be able to: build, test, read the
current milestone, pick the next task, and continue without reading a single
session transcript. That is the test of every artifact you write down; if a
handoff reader would be stuck, the missing piece goes into one of the three
files in the same session.

### §12.7 Example: a filled session report

Concrete shape of §11.4's template (from a real milestone task), so the format
never drifts:

```markdown
## Session 2026-09-14-1432 — flexbox freeze/unfreeze rounds land, M4 at 62%
- **Milestone:** M4 — Block layout   **WBS items touched:** §6.3 flex group (5 done)
- **Implemented:**
  - `resolve_flexible_lengths` per CSS Flexbox §9.7 with the scaled-shrink
    iteration and min-content floors; freeze bookkeeping on `FlexItem`.
  - `flex-basis: auto` now consults `width` then max-content (§5.10 step 1).
- **Tested:** fast tier — 1,382 passed / 0 failed / 6 skipped (4 new).
  New: flex-freeze suite (18 cases), flex-min-content floor (4 cases).
- **Bench:** §10.1 row 7 (docs.html layout) 92 ms → 88 ms.
- **Decisions:** min-content floors computed via the intrinsic pass with
  available-space = infinite mode (§5.10 invariants) — no second code path.
- **Debts opened:** AURORA-SHORTCUT in flex.rs:210 (grid items in flex row
  treated as max-content) → tracks §6.3 grid group, due M4.
- **Next task:** §6.3 flex `gap` — first unchecked flex item (§12.3 rule c).
```

### §12.8 The final delivery checklist

The project ends when this checklist is verified in one final session:

- [ ] Clean clone builds with zero warnings; fast tier green; full tier green.
- [ ] All M0–M13 demos run from their recorded commands; outputs archived.
- [ ] §10.1 budget table measured and recorded; ADRs for any exception.
- [ ] Fuzzers: 24 h cumulative per target, zero open crashes (§8.7).
- [ ] Corpus tables final (§8.6); vendor manifest lists all skips with reasons.
- [ ] Zero `AURORA-SHORTCUT`, zero standing placeholders, zero unanchored TODOs.
- [ ] WBS: 100% checked or waived-by-ADR (§6.12).
- [ ] Documents of §11.2 accurate (spot-check: every doc's claims match behavior).
- [ ] `PROGRESS.md` final session report filed (§11.4); `CHANGELOG.md` release
      section written; `v0.1.0` tagged per §11.6.

## PART 13 — Operational Runbooks

The repeatable procedures of this project, written once so every session
executes them identically. A runbook is followed literally the first time it is
used; afterwards you may improve it with an edit to this section.

### §13.1 Runbook: add a CSS property

Input: a property name (from WBS §6.3 or a spec page). Output: the property
implemented end to end with tests. Steps:

1. Read the property's CSSWG definition page; write a 10-line summary into
   `docs/spec-notes/css-<property>.md`: grammar, initial, inherited, applies-to,
   computed-value type, animation type, and any interaction with existing
   properties (e.g., `border` shorthand resets `border-image`).
2. **Registry entry** (`aurora_css::registry`): add the `PropertyDescriptor` —
   id, name, shorthand set membership, initial value, inherited flag, and the
   parser closure. Wire it into any shorthand that expands to it.
3. **Parsing test first**: in the registry's test module, add the valid-value
   round-trip cases (from the spec's grammar) and at least four invalid cases
   (wrong type, unitless where forbidden, unknown keyword, malformed function).
4. **Computed value**: implement `to_computed` — keyword resolution, relative
   unit binding, percentage retention, `calc()` folding. Unit-test with a
   computed-value fixture (`font-size: 16px` context).
5. **Integration**: the owner section (§6.3 group owner) consumes the computed
   value. Touch exactly the layer the property belongs to: style (no further
   work), layout (thread the value into the box/fragment stage and add one
   golden layout case), paint (thread into the display list and add one pixel
   case), or runtime (behavior change with an integration test).
6. **UA stylesheet**: decide whether the UA origin sets this property on any
   selector; if yes, update `docs/UA-STYLESHEET.md` and the UA sheet source.
7. **Cascade case**: add one stylesheet snippet to the cascade test corpus that
   distinguishes this property from its neighbors (specificity + inheritance
   interplay).
8. Tick the §6.3 checkbox, run the fast tier, write the session report entry.

Time expectation: 0.5–2 sessions for a paint-only property; more for layout
properties that open algorithmic territory (those get their own plan first).

### §13.2 Runbook: add an HTML element

1. Read the element's WHATWG HTML section + the matching `in body` insertion
   rules; summarize in `docs/spec-notes/html-<tag>.md`.
2. **Tree construction** (`aurora_html`): the element's start/end-tag behavior —
   implied end tags, scope interactions, foster parenting, active-formatting
   participation. Add the html5lib-derived tree case *before* the code.
3. **DOM class** (§6.6): the interface exists or gets created via §13.3; wire
   attribute reflection.
4. **UA defaults**: add the default style rule set into the UA sheet source and
   `docs/UA-STYLESHEET.md`.
5. **Layout**: declare the box behavior (block container, atomic inline,
   replaced, table part, none) in the box builder; add a golden layout case
   with the element in a block and an inline context.
6. **Serialization**: `XMLSerializer` output case added to the round-trip test.
7. Tick §6.2, fast tier, report.

### §13.3 Runbook: add a DOM interface (binding)

1. Read the interface's WebIDL block; note attributes (readonly? [LegacyUnforgeable]?),
   methods (overloads, optional args, variadic), constants, stringifier/iterable
   behaviors, and the constructor shape.
2. **Rust side** (owning crate): the native object; a `HostObject` implementation
   in `aurora_runtime::bindings` exposing it.
3. **Prototype chain**: register per §5.15.2 — constructor on the global (when
   constructible), prototype with `Symbol.toStringTag`, methods as non-enumerable
   writable functions, attribute accessors as get/set pairs.
4. **Exception mapping**: wrong-argument types throw `TypeError` per WebIDL
   overload resolution; spec'd failures throw the listed `DOMException`s.
   Add each to `runtime::exception_map` tests.
5. **Tests**: construction, one happy-path call, wrong-arg exception, readonly
   assignment silently failing (strict: `TypeError` per spec where listed).
6. Tick §6.6, fast tier, report.

### §13.4 Runbook: add a JavaScript builtin

1. Read the ECMA-262 clause; copy the algorithm's step names into
   `docs/spec-notes/js-<object>.md`.
2. Implement with spec-named helpers (§5.14.8): `ordinary_create_from_constructor`,
   `array_species_create`, `to_uint32_clamp`, … The builtin's observable
   ordering/exception behavior follows the steps literally.
3. Property attributes on every new member: writable/enumerable/configurable
   per the spec's property table — get these wrong and test262 will find it.
4. Tests: unit per method; then adopt the test262 directory slice in
   `tests/vendor/vendor.toml` and run the vendor tier.
5. Tick §6.7, fast tier, report.

### §13.5 Runbook: adopt a WPT / test262 slice

1. Pick the directory (per the milestone's adoption list, Part 7).
2. Record the upstream revision in `tests/vendor/vendor.toml`; vendor the files.
3. Write the harness mapping (§8.6): JS-assert pages run headless with the
   shim; reftest-like pages route to the pixel comparator with the case's
   tolerance policy.
4. First run: expect failures. Triage every failure into: **fix now** (engine
   bug), **skip** (with reason code: `platform:`, `non-goal: §1.4`, `deferred:
   M#`), or **bug:#** (file as a debt in `PROGRESS.md`, fix within the
   milestone).
5. Record the table (adopted/pass/fail/skip/rate) in `PROGRESS.md`.

### §13.6 Runbook: debugging the pipeline

When a page renders wrong, descend in pipeline order and stop at the first
stage whose output is wrong — never debug downstream of a broken stage:

1. **Bytes**: `--dump-bytes` — is the response what the server sent?
2. **Tokens**: `--dump-tokens` — does the token stream look right at the
   failure point? (Tokenizer trace with line:col.)
3. **DOM**: `--dump-dom` — compare against expectations; check parse-error
   table with `--dump-dom --errors`.
4. **Styles**: `--dump-style "css selector"` — matched declarations with
   specificity and origin, then computed values. Wrong winner → cascade bug;
   right winner, wrong value → computed-value bug.
5. **Boxes/fragments**: `--layout-debug` — fragment tree with geometry. Wrong
   box structure → box builder; right structure, wrong geometry → layout
   algorithm.
6. **Display list**: `--dump-display-list` — items in paint order with clips.
7. **Pixels**: `--render png` and the pixel diff harness for golden regressions.

Each dump format is stable text (§8.3) so diffs are reviewable; all dumps are
available in-window via DevTools inspector views backed by the same code.

### §13.7 Runbook: performance triage

1. Reproduce with the benchmark harness (criterion or the scripted driver) on
   the reference machine — numbers from `--debug-frames` on a dev laptop are
   for direction, not for the ledger.
2. Capture a trace (`--trace` spans) and find the top stage; a "slowness"
   without a named stage is not a bug report.
3. Fix at the algorithmic level first (fewer allocations, fewer passes,
   better invalidation) before micro-optimizing; §9.9 rules apply to
   performance patches like any other.
4. Record before/after in the session report (§11.4) and update the §10.1 row.

### §13.8 Runbook: release (milestone exit → tag)

1. Milestone exit criteria verified one by one (Part 7 section); results in
   the milestone report.
2. `scripts/test-all.sh` green on all tier-1 platforms (or the platform
   unavailability recorded per §3.5).
3. §10.1 budget table run; deltas recorded; regressions triaged.
4. Documentation pass: README commands executed verbatim; ARCHITECTURE.md vs
   the crate map diffed; UA-STYLESHEET.md regenerated from the source sheet.
5. `CHANGELOG.md` entry (§11.6); version bump; tag `vX.Y.Z`; release notes
   quote the corpus table and known issues.

## APPENDIX A — CSS Named Colors

The 148 CSS Color 4 named colors (case-insensitive keywords; the legacy
synonym pairs `gray`/`grey`, `aqua`/`cyan`, `magenta`/`fuchsia`, and the
`darkslategray`-style variants are distinct keywords mapping to equal values).
Table: keyword → RGB hex; implemented as a perfect-hash table in the
color parser (§5.7) with a serialization test per row.

| Keyword | Hex | Keyword | Hex |
|---|---|---|---|
| aliceblue | #f0f8ff | lightpink | #ffb6c1 |
| antiquewhite | #faebd7 | lightsalmon | #ffa07a |
| aqua | #00ffff | lightseagreen | #20b2aa |
| aquamarine | #7fffd4 | lightskyblue | #87cefa |
| azure | #f0ffff | lightslategray | #778899 |
| beige | #f5f5dc | lightslategrey | #778899 |
| bisque | #ffe4c4 | lightsteelblue | #b0c4de |
| black | #000000 | lightyellow | #ffffe0 |
| blanchedalmond | #ffebcd | lime | #00ff00 |
| blue | #0000ff | limegreen | #32cd32 |
| blueviolet | #8a2be2 | linen | #faf0e6 |
| brown | #a52a2a | magenta | #ff00ff |
| burlywood | #deb887 | maroon | #800000 |
| cadetblue | #5f9ea0 | mediumaquamarine | #66cdaa |
| chartreuse | #7fff00 | mediumblue | #0000cd |
| chocolate | #d2691e | mediumorchid | #ba55d3 |
| coral | #ff7f50 | mediumpurple | #9370db |
| cornflowerblue | #6495ed | mediumseagreen | #3cb371 |
| cornsilk | #fff8dc | mediumslateblue | #7b68ee |
| crimson | #dc143c | mediumspringgreen | #00fa9a |
| cyan | #00ffff | mediumturquoise | #48d1cc |
| darkblue | #00008b | mediumvioletred | #c71585 |
| darkcyan | #008b8b | midnightblue | #191970 |
| darkgoldenrod | #b8860b | mintcream | #f5fffa |
| darkgray | #a9a9a9 | mistyrose | #ffe4e1 |
| darkgreen | #006400 | moccasin | #ffe4b5 |
| darkgrey | #a9a9a9 | navajowhite | #ffdead |
| darkkhaki | #bdb76b | navy | #000080 |
| darkmagenta | #8b008b | oldlace | #fdf5e6 |
| darkolivegreen | #556b2f | olive | #808000 |
| darkorange | #ff8c00 | olivedrab | #6b8e23 |
| darkorchid | #9932cc | orange | #ffa500 |
| darkred | #8b0000 | orangered | #ff4500 |
| darksalmon | #e9967a | orchid | #da70d6 |
| darkseagreen | #8fbc8f | palegoldenrod | #eee8aa |
| darkslateblue | #483d8b | palegreen | #98fb98 |
| darkslategray | #2f4f4f | paleturquoise | #afeeee |
| darkslategrey | #2f4f4f | palevioletred | #db7093 |
| darkturquoise | #00ced1 | papayawhip | #ffefd5 |
| darkviolet | #9400d3 | peachpuff | #ffdab9 |
| deeppink | #ff1493 | peru | #cd853f |
| deepskyblue | #00bfff | pink | #ffc0cb |
| dimgray | #696969 | plum | #dda0dd |
| dimgrey | #696969 | powderblue | #b0e0e6 |
| dodgerblue | #1e90ff | purple | #800080 |
| firebrick | #b22222 | rebeccapurple | #663399 |
| floralwhite | #fffaf0 | red | #ff0000 |
| forestgreen | #228b22 | rosybrown | #bc8f8f |
| fuchsia | #ff00ff | royalblue | #4169e1 |
| gainsboro | #dcdcdc | saddlebrown | #8b4513 |
| ghostwhite | #f8f8ff | salmon | #fa8072 |
| gold | #ffd700 | sandybrown | #f4a460 |
| goldenrod | #daa520 | seagreen | #2e8b57 |
| gray | #808080 | seashell | #fff5ee |
| green | #008000 | sienna | #a0522d |
| greenyellow | #adff2f | silver | #c0c0c0 |
| grey | #808080 | skyblue | #87ceeb |
| honeydew | #f0fff0 | slateblue | #6a5acd |
| hotpink | #ff69b4 | slategray | #708090 |
| indianred | #cd5c5c | slategrey | #708090 |
| indigo | #4b0082 | snow | #fffafa |
| ivory | #fffff0 | springgreen | #00ff7f |
| khaki | #f0e68c | steelblue | #4682b4 |
| lavender | #e6e6fa | tan | #d2b48c |
| lavenderblush | #fff0f5 | teal | #008080 |
| lawngreen | #7cfc00 | thistle | #d8bfd8 |
| lemonchiffon | #fffacd | tomato | #ff6347 |
| lightblue | #add8e6 | turquoise | #40e0d0 |
| lightcoral | #f08080 | violet | #ee82ee |
| lightcyan | #e0ffff | wheat | #f5deb3 |
| lightgoldenrodyellow | #fafad2 | white | #ffffff |
| lightgray | #d3d3d3 | whitesmoke | #f5f5f5 |
| lightgreen | #90ee90 | yellow | #ffff00 |
| lightgrey | #d3d3d3 | yellowgreen | #9acd32 |

Plus: `transparent` = rgba(0,0,0,0); `currentcolor` resolves from the
`color` property at computed-value time (§5.8.3); system colors
(`canvastext`, `canvas`, `linktext`, `visitedtext`, `buttontext`,
`buttonface`, `buttonborder`, `field`, `fieldtext`, `highlight`,
`highlighttext`, `graytext`, `mark`, `marktext`) map to the default theme.

## APPENDIX B — HTML Named Character References (common subset)

Named references the tokenizer's character-reference state must resolve
(§5.4); the full standard table (~2,231 entries) is generated from the
entities JSON at build time — this subset must be hand-verified.

| Reference | Codepoint | Glyph note |
|---|---|---|
| &amp; | U+0026 | ampersand & |
| &lt; | U+003C | less-than < |
| &gt; | U+003E | greater-than > |
| &quot; | U+0022 | double quote " |
| &apos; | U+0027 | apostrophe ' |
| &nbsp; | U+00A0 | no-break space (non-ASCII in serialization) |
| &copy; | U+00A9 | copyright sign |
| &reg; | U+00AE | registered sign |
| &trade; | U+2122 | trademark sign |
| &hellip; | U+2026 | horizontal ellipsis |
| &mdash; | U+2014 | em dash |
| &ndash; | U+2013 | en dash |
| &lsquo; | U+2018 | left single quote |
| &rsquo; | U+2019 | right single quote |
| &ldquo; | U+201C | left double quote |
| &rdquo; | U+201D | right double quote |
| &sbquo; | U+201A | single low quote |
| &bdquo; | U+201E | double low quote |
| &laquo; | U+00AB | left guillemet |
| &raquo; | U+00BB | right guillemet |
| &lsaquo; | U+2039 | single left angle quote |
| &rsaquo; | U+203A | single right angle quote |
| &times; | U+00D7 | multiplication sign |
| &divide; | U+00F7 | division sign |
| &plusmn; | U+00B1 | plus-minus sign |
| &deg; | U+00B0 | degree sign |
| &middot; | U+00B7 | middle dot |
| &bull; | U+2022 | bullet |
| &dagger; | U+2020 | dagger |
| &Dagger; | U+2021 | double dagger |
| &permil; | U+2030 | per mille sign |
| &euro; | U+20AC | euro sign |
| &cent; | U+00A2 | cent sign |
| &pound; | U+00A3 | pound sign |
| &yen; | U+00A5 | yen sign |
| &sect; | U+00A7 | section sign |
| &para; | U+00B6 | pilcrow sign |
| &larr; | U+2190 | leftwards arrow |
| &uarr; | U+2191 | upwards arrow |
| &rarr; | U+2192 | rightwards arrow |
| &darr; | U+2193 | downwards arrow |
| &harr; | U+2194 | left-right arrow |
| &minus; | U+2212 | minus sign |
| &lowast; | U+2217 | asterisk operator |
| &radic; | U+221A | square root |
| &infin; | U+221E | infinity |
| &cap; | U+2229 | intersection |
| &cup; | U+222A | union |
| &int; | U+222B | integral |
| &asymp; | U+2248 | almost equal |
| &ne; | U+2260 | not equal |
| &le; | U+2264 | less-or-equal |
| &ge; | U+2265 | greater-or-equal |
| &alpha; | U+03B1 | greek small alpha |
| &beta; | U+03B2 | greek small beta |
| &gamma; | U+03B3 | greek small gamma |
| &pi; | U+03C0 | greek small pi |
| &Omega; | U+03A9 | greek capital omega |
| &sum; | U+2211 | n-ary summation |
| &prod; | U+220F | n-ary product |
| &part; | U+2202 | partial differential |
| &nabla; | U+2207 | nabla |
| &isin; | U+2208 | element of |
| &notin; | U+2209 | not an element of |
| &empty; | U+2205 | empty set |
| &forall; | U+2200 | for all |
| &exist; | U+2203 | there exists |
| &oplus; | U+2295 | circled plus |
| &otimes; | U+2297 | circled times |
| &perp; | U+22A5 | up tack |
| &sdot; | U+22C5 | dot operator |
| &lceil; | U+2308 | left ceiling |
| &rceil; | U+2309 | right ceiling |
| &lfloor; | U+230A | left floor |
| &rfloor; | U+230B | right floor |
| &lang; | U+27E8 | left angle bracket |
| &rang; | U+27E9 | right angle bracket |
| &loz; | U+25CA | lozenge |
| &spades; | U+2660 | black spade suit |
| &clubs; | U+2663 | black club suit |
| &hearts; | U+2665 | black heart suit |
| &diams; | U+2666 | black diamond suit |
| &shy; | U+00AD | soft hyphen (invisible) |
| &zwnj; | U+200C | zero-width non-joiner |
| &zwj; | U+200D | zero-width joiner |
| &lrm; | U+200E | left-to-right mark |
| &rlm; | U+200F | right-to-left mark |

Numeric references: decimal `&#NNN;` and hex `&#xHHH;`, with the
windows-1252 remapping for 0x80–0x9F and U+FFFD for surrogates and
out-of-range codepoints. Unmatched named references serialize literally.

## APPENDIX C — HTTP Header Field Reference

Engine behavior per header (§5.2); direction is the side the engine
sends or consumes it on. Unknown headers pass through untouched and
appear in the DevTools network view.

| Header | Direction | Engine behavior |
|---|---|---|
| `Host` | request | Mandatory for http/1.1 special schemes; validated non-empty. |
| `User-Agent` | request | `Aurora/0.1 (+project)`; single well-known string, no spoofing options. |
| `Accept` | request | `text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8` for documents; narrower for subresources. |
| `Accept-Language` | request | From shell prefs (`en-US,en;q=0.9` default); feeds :lang fallback matching. |
| `Accept-Encoding` | request | `gzip, deflate` (identity until compressor lands; §5.2.2). |
| `Connection` | both | `keep-alive` default; `close` honored both directions. |
| `Content-Length` | both | Framing for bodies; conflicts with chunked are protocol errors (§5.2.3). |
| `Transfer-Encoding` | response | `chunked` decoding implemented; other values are protocol errors. |
| `Content-Type` | both | Drives document-type sniffing (§5.2.7) and stylesheet/script/image load decisions; `nosniff` interplay. |
| `Cache-Control` | response | Freshness directives: max-age, no-store, no-cache, must-revalidate, immutable, s-maxage ignored (no shared cache) (§5.17.4). |
| `Expires` | response | Legacy freshness; superseded by Cache-Control when both present. |
| `Pragma` | response | `no-cache` legacy honored; otherwise ignored. |
| `Age` | response | Seconds since origin generation; feeds freshness math. |
| `Date` | response | Stored for heuristic freshness and Age computation. |
| `ETag` | response | Strong/weak validators stored; used for If-None-Match revalidation. |
| `If-None-Match` | request | Conditional GET on revalidation; 304 applies stored headers. |
| `If-Modified-Since` | request | Date-based revalidation fallback. |
| `Last-Modified` | response | Stored for If-Modified-Since. |
| `Vary` | response | Per-header-value cache keying; `*` disables reuse (§5.2.6). |
| `Location` | response | Redirect target resolution per §5.2.4 (relative allowed). |
| `Refresh` | response | Non-standard meta refresh equivalent: parse and schedule navigation with a console note. |
| `Content-Encoding` | response | `gzip`/`deflate`/`br`(unsupported → error) decoded before the pipeline. |
| `Content-Disposition` | response | `attachment` triggers the download manager with filename parsing (§5.19.2). |
| `Set-Cookie` | response | Full RFC 6265bis parse in §5.17.1; multiple headers supported. |
| `Cookie` | request | Jar lookup with path/domain/secure/samesite rules. |
| `Strict-Transport-Security` | response | HSTS policy parse and persist (§5.3). |
| `Access-Control-Allow-Origin` | response | CORS check; wildcard-with-credentials rules (§5.18.3). |
| `Access-Control-Allow-Methods` | response | Preflight method check. |
| `Access-Control-Allow-Headers` | response | Preflight header check (case-insensitive). |
| `Access-Control-Expose-Headers` | response | Restricts JS-visible response headers on CORS responses. |
| `Access-Control-Allow-Credentials` | response | true enables credentialed CORS. |
| `Access-Control-Max-Age` | response | Preflight result cache TTL. |
| `Access-Control-Request-Method` | request | Preflight request marker. |
| `Access-Control-Request-Headers` | request | Preflight header list. |
| `Origin` | request | Sent on CORS and non-GET same-site requests; feeds the CORS check. |
| `Referer` | request | Per referrer policy (§5.18.5); strip on downgrade. |
| `Referrer-Policy` | response | Updates the document's referrer policy (strict-origin-when-cross-origin default). |
| `Content-Security-Policy` | response | Parse and enforce per §5.18.4 (report-only variant reports only). |
| `X-Content-Type-Options` | response | `nosniff` disables sniffing (§5.2.7); blocks wrong-type scripts/styles. |
| `X-Frame-Options` | response | DENY/SAMEORIGIN honored for iframe embedding decisions. |
| `Retry-After` | response | Parse only (no auto-retry policy in MVP). |
| `Range/Accept-Ranges/Content-Range` | both | Range requests for media (MVP: not sent; parse and ignore). |
| `Allow` | response | 405 reporting only. |
| `WWW-Authenticate/Authorization` | both | Parsed; engine sends Basic only when the shell credential prompt (MVP: none) supplies it; otherwise 401 error page. |
| `Upgrade` | response | 101 for WebSocket handshake (§6.9); otherwise ignored. |
| `Sec-WebSocket-*` | both | RFC 6455 handshake headers generated/parsed by the WebSocket client. |
| `Server` | response | Logged in the network panel only. |
| `Alt-Svc` | response | Ignored (no HTTP/2 in MVP) with a DevTools note. |
| `Link` | response | Preload hints parsed as best-effort; stylesheet variant honored. |
| `Timing-Allow-Origin` | response | Marks cross-origin resources for high-res timing exposure. |
| `Cross-Origin-Opener-Policy / -Embedder-Policy / -Resource-Policy` | response | COOP: report-only in MVP; COEP: ignored with console note; CORP: enforced for no-cors subresource fetches. |

## APPENDIX D — MIME Type Table

Document-type decisions (§5.2.7, §5.5.5); parameters such as `charset`
are honored where the row says so.

| MIME type | Engine behavior |
|---|---|
| `text/html` | HTML document pipeline (§5.4-5.5); charset parameter honored. |
| `application/xhtml+xml` | XML-ish: parse well-formed-only; failure renders the error document (MVP lenient note). |
| `text/plain` | Plain text document; wrap in <pre>-like display document. |
| `text/css` | Stylesheet only when the embedding allows it (nosniff honored). |
| `text/javascript + application/javascript` | Classic/module script depending on the element's type attribute. |
| `application/json` | Fetchable; standalone navigation renders a JSON viewer (MVP: plain text). |
| `image/png, image/jpeg, image/gif, image/bmp` | Image decoders (§5.12); standalone navigation renders the image in a generated document. |
| `image/x-icon + image/vnd.microsoft.icon` | ICO decoder; favicon selection source. |
| `image/webp, image/avif, image/svg+xml` | Not decodable (§1.3): treated as unsupported image (placeholder + error event for svg-in-img; webp/avif error). |
| `application/pdf` | Download (non-goal to render). |
| `audio/*, video/*` | Media element stub accepts; no decode (§1.4). |
| `application/octet-stream` | Download. |
| `multipart/form-data` | Request-side encoding for form POST (§5.6 forms); response-side multipart/x-mixed-replace ignored. |
| `application/x-www-form-urlencoded` | Form GET/POST body encoding (§5.1). |
| `text/xml, application/xml` | Well-formedness-checking parse; used by DOMParser text/xml. |
| `font/woff2, font/woff, font/ttf, font/otf` | Font loading from @font-face src (§5.11); format hints validated. |
| `application/wasm` | Not supported (stretch goal §1.5): fetch error page. |
| `unknown/missing types` | Sniffing per §5.2.7 for images and top-level documents; otherwise download. |

## APPENDIX E — CSS Units

Resolution rules live in §5.8.3 (style time) and §5.9 (layout time).

| Unit | Kind | Notes |
|---|---|---|
| `px` | length | CSS reference pixel; the anchor all others resolve to at style time. |
| `em` | length | Relative to the element's own font-size (for font-size: the parent's). |
| `rem` | length | Root element font-size; resolves at style time to absolute px. |
| `ex` | length | x-height of the first available font. |
| `ch` | length | Advance of the 0 glyph (U+0030) of the first available font. |
| `cap` | length | Cap height of the first available font. |
| `ic` | length | Advance of the water ideograph (U+6C34). |
| `lh` | length | Line height of the element. |
| `rlh` | length | Root line height. |
| `vw` | length | 1% of viewport width. |
| `vh` | length | 1% of viewport height. |
| `vi` | length | 1% of viewport inline axis. |
| `vb` | length | 1% of viewport block axis. |
| `vmin` | length | 1% of the smaller viewport dimension. |
| `vmax` | length | 1% of the larger viewport dimension. |
| `sv* family` | length | Small-viewport units (svw/svh); MVP maps to vw/vh (ADR if used). |
| `lv* family` | length | Large-viewport units (lvw/lvh); MVP maps to vw/vh. |
| `cm` | length | 1cm = 96px/2.54. |
| `mm` | length | 1mm = 96px/25.4. |
| `q` | length | quarter-millimeter. |
| `in` | length | 1in = 96px. |
| `pt` | length | 1pt = 96px/72. |
| `pc` | length | 1pc = 12pt. |
| `%` | percentage | Resolution context is per-property (containing block, font, or own box); see the property registry. |
| `deg` | angle | 360 per turn. |
| `grad` | angle | 400 per turn. |
| `rad` | angle | 2pi per turn. |
| `turn` | angle | 1 turn. |
| `s` | time | Seconds; animation/transition longhands resolve to ms. |
| `ms` | time | Milliseconds. |
| `Hz` | frequency | Parse-only (voice/a11y stretch). |
| `kHz` | frequency | Parse-only. |
| `dpi` | resolution | Dots per inch; resolution media queries and image-set(). |
| `dpcm` | resolution | Dots per centimeter. |
| `dppx` | x | Dots per pixel unit (= 96dpi); resolution queries. |
| `fr` | flex | Grid free-space unit; only inside track lists (§5.10.2). |

## APPENDIX F — Keyboard Event Map

The `KeyboardEvent.code`/`key` wiring and shell shortcut table (§5.19.1,
§6.11). Platform layouts may remap `key`; `code` is positional and stable.

| Key / shortcut | Category | Default action note |
|---|---|---|
| `Backquote/Digit1..Digit0/Minus/Equal` | typing keys | Produce characters via the platform layout map |
| `KeyQ..KeyM (letters)` | typing keys | Key values per current layout; printable keys feed text input |
| `Space` | typing keys | Character U+0020; page-scrolls when focus is not editable |
| `Tab` | navigation | Moves focus in DOM order (Shift reverses); preventDefault gives raw key |
| `Enter` | activation | Activates focused control; submits single-input forms; text areas insert newline |
| `NumpadEnter` | activation | Same as Enter |
| `Backspace` | editing | Deletes backwards in editables; history-back only when shell policy allows |
| `Delete` | editing | Deletes forwards in editables |
| `Escape` | navigation | Closes dialogs/menus; exits fullscreen (no-op here); cancels IME |
| `ArrowLeft/ArrowRight` | navigation | Text caret movement in editables; horizontal scroll fallback |
| `ArrowUp/ArrowDown` | navigation | Line navigation in editables; option cycling in select; page scroll fallback |
| `Home/End` | navigation | Line start/end in editables; scroll to edges in pages |
| `PageUp/PageDown` | navigation | Scroll by page |
| `Insert` | editing | Toggle overwrite mode (MVP: no-op) |
| `F1..F12` | function keys | Shell shortcuts where mapped (F12 DevTools, F5 reload, F11 fullscreen toggle, F6 focus omnibox, F10 menu) |
| `Shift/Control/Alt/Meta (Left/Right)` | modifiers | Modifier state machinery for getModifierState and shortcuts |
| `CapsLock` | modifier | Toggle state surfaced to getModifierState |
| `NumLock/ScrollLock` | modifier | State surfaced; no default behavior |
| `ContextMenu` | menu | Opens the context menu at selection/caret |
| `PrintScreen` | misc | No engine behavior |
| `Ctrl+T / Ctrl+W / Ctrl+Shift+T` | shell | New tab / close tab / reopen closed tab |
| `Ctrl+L / Alt+D / F6` | shell | Focus the omnibox |
| `Ctrl+R / F5 / Ctrl+Shift+R` | shell | Reload / reload (bypass cache) |
| `Alt+Left / Alt+Right` | shell | Back / forward |
| `Ctrl+D` | shell | Bookmark current page |
| `Ctrl+F` | shell | Find in page |
| `Ctrl+Plus/Minus/Zero` | shell | Zoom in/out/reset (10% steps) |
| `Ctrl+U` | shell | View source (generated document from --dump-dom path) |
| `Ctrl+S` | shell | Save page (MHTML-lite MVP: single-file HTML) |
| `Ctrl+J` | shell | Open downloads |
| `Ctrl+H` | shell | Open history |
| `Ctrl+Shift+I / F12` | shell | Toggle DevTools |
| `Ctrl+Shift+C` | shell | Inspect element mode (pick on next click) |
| `Ctrl+Tab / Ctrl+Shift+Tab` | shell | Next/previous tab |
| `Ctrl+1..8 / Ctrl+9` | shell | Select tab by index / last tab |
| `Ctrl+P` | shell | Print (no-op with console note; non-goal §1.4) |


## APPENDIX G — Default (UA) Stylesheet

The user-agent origin stylesheet (§7.5), given here as the reference text. It
is a product artifact: encode it exactly in `aurora_css::ua_sheet` (or generated
from this source), keep `docs/UA-STYLESHEET.md` in sync, and treat any deviation
from this listing as a bug unless an ADR says otherwise. Quirks-mode deltas are
listed separately at the end. Cascade note: UA origin, importance normal — every
rule here loses to any author rule of equal-or-greater weight (§5.8.2).

```css
/* === Display and visibility === */
html, body          { display: block; }
head, title, base, link, meta, style, script, noscript, template, slot,
datalist, colgroup, col, source, track, area, basefont, param, rp {
                      display: none; }
head > meta, head > title, head > base, head > link, head > style,
head > script       { /* covered above; kept for fragments */ }
template            { display: none; }
[hidden]            { display: none; }
*                   { visibility: inherit; }
dialog:not([open])  { display: none; }
details > *:not(summary) { display: none; }  /* when closed; open removes */
details[open] > *:not(summary) { display: block; }

/* === Root and structure === */
html                { display: block; }
body                { display: block; margin: 8px; }
p, dl, multicol     { display: block; margin-block: 1em; }
dd                  { display: block; margin-inline-start: 40px; }
blockquote, figure  { display: block; margin-block: 1em; margin-inline: 40px; }
figcaption          { display: block; }
center              { display: block; text-align: center; }

/* === Headings === */
h1                  { display: block; margin-block: 0.67em; font-weight: bold;
                      font-size: 2em; }
h2                  { display: block; margin-block: 0.83em; font-weight: bold;
                      font-size: 1.5em; }
h3                  { display: block; margin-block: 1em;    font-weight: bold;
                      font-size: 1.17em; }
h4                  { display: block; margin-block: 1.33em; font-weight: bold;
                      font-size: 1em; }
h5                  { display: block; margin-block: 1.67em; font-weight: bold;
                      font-size: 0.83em; }
h6                  { display: block; margin-block: 2.33em; font-weight: bold;
                      font-size: 0.67em; }

/* === Lists === */
ol, ul, menu        { display: block; margin-block: 1em;
                      padding-inline-start: 40px; list-style-type: disc; }
ol                  { list-style-type: decimal; }
ol[type="1"]        { list-style-type: decimal; }
ol[type="a"]        { list-style-type: lower-alpha; }
ol[type="A"]        { list-style-type: upper-alpha; }
ol[type="i"]        { list-style-type: lower-roman; }
ol[type="I"]        { list-style-type: upper-roman; }
ul[type="circle"]   { list-style-type: circle; }
ul[type="disc"]     { list-style-type: disc; }
ul[type="square"]   { list-style-type: square; }
li                  { display: list-item; text-align: match-parent; }
dir, dd, dt         { /* dd above; dt: */ }
dt                  { display: block; }

/* === Inline text === */
:link               { color: linktext; text-decoration: underline; }
b, strong           { font-weight: bolder; }
i, em, cite, var, dfn { font-style: italic; }
u, ins              { text-decoration: underline; }
s, strike, del      { text-decoration: line-through; }
tt, code, kbd, samp { font-family: monospace; font-size: 1em; }
small               { font-size: smaller; }
big                 { font-size: larger; }
abbr[title], acronym[title] { text-decoration: dotted underline; }
sub                 { vertical-align: sub;   font-size: smaller; }
sup                 { vertical-align: super; font-size: smaller; }
q                   { display: inline; }           /* quotes generated per content/quotes */
q::before           { content: open-quote; }
q::after            { content: close-quote; }
nobr                { white-space: nowrap; }
bdo[dir="ltr"]      { direction: ltr; unicode-bidi: bidi-override; }
bdo[dir="rtl"]      { direction: rtl; unicode-bidi: bidi-override; }
bdi[dir]            { unicode-bidi: isolate; }
ruby, rt, rb, rbc   { /* MVP: ruby treated inline; rt: */ display: inline; }
rp                  { display: none; }

/* === Whitespace preservation === */
pre, xmp, plaintext, listing,
textarea            { font-family: monospace; font-size: 1em;
                      white-space: pre; }
pre, xmp, plaintext, listing { display: block; margin-block: 1em; }

/* === Tables === */
table               { display: table; border-collapse: separate;
                      border-spacing: 2px; border-color: gray;
                      box-sizing: border-box; text-indent: initial; }
caption             { display: table-caption; text-align: center; }
colgroup, col       { display: table-column-group; }   /* col: table-column */
col                 { display: table-column; }
thead               { display: table-header-group;  vertical-align: middle; }
tbody               { display: table-row-group;     vertical-align: middle; }
tfoot               { display: table-footer-group;  vertical-align: middle; }
tr                  { display: table-row;           vertical-align: inherit; }
td, th              { display: table-cell; padding: 1px; vertical-align: inherit; }
th                  { font-weight: bold; text-align: center; }
table[border] > tr, table[border] > thead > tr, table[border] > tbody > tr,
table[border] > tfoot > tr { /* border presentational hints map to cells */ }
table[rules]        { /* rules hints map to border-style on cells */ }

/* === Replaced and embedded === */
img, iframe, embed, object, video, canvas, input[type="image"] {
                      display: inline-block; vertical-align: baseline; }
iframe              { border: 2px inset; }  /* frame chrome per §5.19 */
embed, object       { width: 300px; height: 150px; }  /* default intrinsic */
video[controls], audio[controls] { /* control chrome drawn by the shell */ }
audio:not([controls]) { display: none; }
canvas[width], canvas[height] { /* attribute-driven intrinsic size */ }

/* === Forms === */
form                { display: block; margin-block: 1em; }
button, input[type="button"], input[type="submit"], input[type="reset"],
input[type="checkbox"], input[type="radio"], input[type="range"],
input[type="file"]  { /* control chrome owned by the shell painter */ }
input[type="text"], input[type="search"], input[type="tel"], input[type="url"],
input[type="email"], input[type="password"], input[type="date"],
input, textarea     { font-family: monospace; }   /* MVP control typography */
input[type="hidden"]{ display: none; }
input[disabled], select[disabled], textarea[disabled],
button[disabled], optgroup[disabled], option[disabled] { color: graytext; }
fieldset            { display: block; margin-inline: 2px; border: groove 2px;
                      border-block-start: groove 2px; border-block-end: groove 2px;
                      padding-block: 0.35em 0.625em; padding-inline: 0.75em;
                      min-inline-size: min-content; }
legend              { display: block; padding-inline: 2px; }
label               { display: inline; cursor: default; }
output              { display: inline; }
option              { /* rendered in the select list UI, not in flow */ }
optgroup            { /* group header in the list UI */ }
progress, meter     { display: inline-block; width: 10em; height: 1em;
                      vertical-align: baseline; }

/* === Interactive === */
summary             { display: block; }
summary::marker     { /* disclosure triangle drawn by the shell painter */ }
dialog              { display: block; margin: auto; border: solid;
                      padding: 1em; background: canvas; color: canvastext; }
dialog::backdrop    { background: rgba(0, 0, 0, 0.5); }
marquee             { display: block; overflow: hidden; }  /* static (§6.2) */
frame, frameset     { display: none; }                     /* parse-only */
noframes            { display: none; }

/* === Text behavior defaults === */
body                { text-rendering: auto; }
:dir(ltr)           { /* directionality comes from the bidi pass, not a rule */ }
br                  { /* forced break; no box of its own */ }
wbr                 { /* soft wrap opportunity */ }

/* === Legacy presentational-hint targets (attributes map into style) === */
font[size]          { /* size hint → font-size table */ }
font[color]         { /* color hint → color */ }
font[face]          { /* face hint → font-family */ }
bgcolor, background, align, valign, border, cellpadding, cellspacing, hspace,
vspace, width, height attributes on their respective elements
                    { /* handled by the presentational-hint layer (§6.2) */ }
```

### Quirks-mode deltas

In quirks mode (DOCTYPE-less documents, §5.5.3):

1. The line-height of block containers uses the quirks "normal" computation
   (small strut) instead of the standards strut for the body font.
2. `td`, `th`, and table captions inherit `text-align` from their rows
   (standards mode: `match-parent`-like centering for `th` only).
3. Percentage heights on blocks resolve against the *viewport* when the
   containing block's height is auto (the classic quirks height rule).
4. Font-size keywords (`small` … `xx-large`) use the quirks scaling table.
5. Empty inline elements with explicit heights/widths are not collapsed the
   standards way (the "haslayout" era behaviors) — implemented only as far as
   the quirks corpus (§8.6) requires, and no further.

### Maintenance rule

This appendix is generated-by-hand but versioned: any change to the UA sheet
source lands in the same commit as the matching change here, and the pixel
corpus (§8.5) re-runs in full. The UA sheet is also where `prefers-color-scheme`
dark defaults will live when theming lands (ADR required).

## APPENDIX H — Console Message Catalog

The engine's console-facing messages (§5.15.4). Every message has a
stable id, a level, and a documented trigger; message text may change,
ids may not (tests and DevTools filters key on them). Add new messages
to `tools/wbs_data_b.py` and regenerate.

| Id | Level | Trigger |
|---|---|---|
| `ParseError.html` | info | HTML parse error with line:col and the standard's error code (§5.4). |
| `ParseError.css` | info | CSS parse error with line:col and the recovered-at token (§5.7). |
| `Encoding.restarted` | info | A late <meta charset> forced re-tokenization (§5.5.5). |
| `Encoding.fallback` | warn | No encoding declaration found; windows-1252 assumed per spec. |
| `Net.protocolError` | error | HTTP/1.1 violation with the offending bytes summarized (§5.2.3). |
| `Net.badStatus` | error | Status line outside 100–599; treated as network error. |
| `Tls.verificationFailed` | error | Certificate failure with the concrete rustls alert (§5.3). |
| `HSTS.applied` | info | Navigation upgraded http→https by HSTS policy. |
| `CORS.blocked` | error | Fetch failed the CORS check; reason not exposed to script (§5.18.3). |
| `MixedContent.blocked` | error | Blockable mixed content refused on an https page (§5.18.5). |
| `MixedContent.allowed` | warn | Optionally-blockable mixed content loaded (images) and reported. |
| `CSP.violation` | error | Directive violated; the resource was blocked; event fired (§5.18.4). |
| `CSP.reportOnly` | warn | Directive violated in report-only mode; resource allowed. |
| `Storage.quotaExceeded` | error | setItem over quota; QuotaExceededError thrown to script (§6.10). |
| `Cookie.rejected` | info | Set-Cookie rejected with the RFC rule that rejected it (§5.17.1). |
| `Cache.revalidated` | info | Conditional request returned 304; served from cache. |
| `Dom.liveCollectionMutation` | warn | Live collection mutated during iteration (perf hint). |
| `Dom.passiveDefault` | warn | preventDefault called on a passive wheel/touch listener (§5.6). |
| `Js.unhandledRejection` | error | Promise rejected with no handler at the microtask checkpoint (§5.14.5). |
| `Js.longTask` | warn | Task exceeded 50 ms with its source (§5.15.6). |
| `Js.trapInvariant` | error | Proxy trap returned a spec-invariant-violating value (§6.7). |
| `Image.decodeError` | error | Image bytes failed decoding; error event fired (§5.12). |
| `Image.unsupportedFormat` | warn | Format outside the supported set (webp/avif/svg-in-img, §1.3). |
| `Font.loadFailed` | error | @font-face source failed; fallback face used (§5.11.1). |
| `Font.fallbackUsed` | info | Glyphs missing in the primary font; fallback chain applied. |
| `Shaper.complexDelegated` | info | Complex script shaped via the HarfBuzz-backed shaper (§5.11.2). |
| `Layout.percentageIndefinite` | info | Percentage height resolved as auto due to indefinite containing block (§5.9.2). |
| `Paint.clipUnbalanced` | error | Internal: display list pushed/popped clips unevenly (should be unreachable; a bug report). |
| `Shell.downloadStarted` | info | Attachment Content-Disposition routed to the download manager (§5.19.2). |
| `Shell.fileOrigin` | warn | file:// fetch permitted within the directory policy (§5.18.6). |
| `NotSupported.feature` | info | Script requested a non-goal feature (§1.4); NotSupportedError path taken. |


## APPENDIX I — HTTP Status Code Reference

Engine behavior per status (§5.2). Codes are grouped; unlisted codes in a known
class follow the class default. `→` marks what the pipeline does after the
response completes.

| Code | Meaning | Engine behavior |
|---|---|---|
| 100 | Continue | Not requested by us; if received, skip and read the real response. |
| 101 | Switching Protocols | Only valid for the WebSocket upgrade (§6.9); otherwise protocol error. |
| 200 | OK | Success → deliver body. |
| 201 | Created | Success → deliver body. |
| 204 | No Content | Success, no body; keep connection; headers apply. |
| 206 | Partial Content | Only meaningful for ranged media requests (we do not send Range in MVP): treat as 200. |
| 301 | Moved Permanently | Redirect → GET, cacheable navigation redirect. |
| 302 | Found | Redirect → GET. |
| 303 | See Other | Redirect → GET regardless of method. |
| 304 | Not Modified | Cache revalidation hit → serve stored entry (§5.17.4). |
| 307 | Temporary Redirect | Redirect preserving method and body. |
| 308 | Permanent Redirect | Redirect preserving method and body; cacheable. |
| 400 | Bad Request | Error document; render response body if HTML, else generated error page. |
| 401 | Unauthorized | Error page; WWW-Authenticate parsed and shown in DevTools (no credential prompt, §5.2). |
| 403 | Forbidden | Error page (server body rendered). |
| 404 | Not Found | Error page (server body rendered; error status recorded for the page). |
| 405 | Method Not Allowed | Error page; Allow header surfaced in DevTools. |
| 408 | Request Timeout | Network error, retryable at the loader's discretion (once). |
| 410 | Gone | Like 404 with the stronger cache note. |
| 413 | Content Too Large | Error page. |
| 418 | I'm a teapot | Rendered as any other 4xx body (no special case). |
| 429 | Too Many Requests | Error page; Retry-After parsed, honored for that origin's next request scheduling. |
| 500–599 | Server errors | Error page (server body rendered); 502/504 marked as retryable in the network panel. |
| 3xx with Location missing | Malformed redirect | Treated as a normal response of its class with a console warning. |
| Unknown 1xx | Informational | Skip and continue reading (interim responses). |
| Unknown 2xx/3xx | — | Treat as 200 / redirect-class-without-location per class default. |
| Unknown 4xx/5xx | — | Error-page path of the class. |

## APPENDIX J — windows-1252 Remapping Table

The character-reference state (§5.4) remaps numeric references in 0x80–0x9F to
these codepoints (HTML §13.2.5.2). Implement as a 32-entry `const` table with a
serialization test over the full range.

| Byte | CP | Byte | CP |
|---|---|---|---|
| 0x80 | U+20AC (€) | 0x90 | U+FFFD |
| 0x81 | U+FFFD | 0x91 | U+2018 |
| 0x82 | U+201A | 0x92 | U+2019 |
| 0x83 | U+0192 | 0x93 | U+201C |
| 0x84 | U+201E | 0x94 | U+201D |
| 0x85 | U+2026 | 0x95 | U+2022 |
| 0x86 | U+2020 | 0x96 | U+2013 |
| 0x87 | U+2021 | 0x97 | U+2014 |
| 0x88 | U+02C6 | 0x98 | U+02DC |
| 0x89 | U+2030 | 0x99 | U+2122 |
| 0x8A | U+0160 | 0x9A | U+0161 |
| 0x8B | U+2039 | 0x9B | U+203A |
| 0x8C | U+0152 | 0x9C | U+0153 |
| 0x8D | U+FFFD | 0x9D | U+FFFD |
| 0x8E | U+017D | 0x9E | U+017E |
| 0x8F | U+FFFD | 0x9F | U+0178 |

All other positions: identity mapping into Unicode. The windows-1252 decoder
(aurora_encoding) uses the same table plus the remaining high-byte mappings
from the Encoding Standard's index file, regenerated per release with the
full 256-entry test.

## APPENDIX K — HTML Tokenizer State Index

The state machine of §5.4, indexed for navigation. Each state's implementation
lives in one function (`aurora_html::tokenizer::states`); the tests under
`tests/vendor/html5lib/tokenizer/` exercise every transition.

| State | Group | Purpose (one line) |
|---|---|---|
| Data | character | Default: characters pass through; `<` opens tags. |
| Tag open | markup dispatch | `<` + letter → tag name; `!` → markup declaration; `/` → end tag; `?` → bogus comment. |
| End tag open | markup dispatch | Letters → end-tag name; `>` error → data; EOF error. |
| Tag name | tag | Accumulate the name; whitespace/`/`/`>` transitions. |
| Before attribute name | attribute | Skip whitespace; detect attribute or end-of-tag. |
| Attribute name | attribute | Accumulate; handle duplicate names (first wins). |
| After attribute name | attribute | Route to value, next attribute, or tag end. |
| Before attribute value | attribute | Quoted or unquoted value start. |
| Attribute value (double/single/unquoted) | attribute | Accumulate with character-reference expansion. |
| After attribute value (quoted) | attribute | Whitespace/`/`/`>` routing; anything else is a parse error and reprocesses. |
| Self-closing start tag state | tag | Acknowledge `/>` (HTML: ignored with error unless foreign). |
| Bogus comment | comment | Consume until `>`; everything becomes comment data. |
| Markup declaration open | markup dispatch | `--` → comment; DOCTYPE → doctype; `[CDATA[` → cdata (foreign only). |
| Comment start / start dash / comment / comment end dash / comment end | comment | The five-state comment machine with error recoveries. |
| DOCTYPE states | doctype | Name, after-name, before/at/after public and system identifier, bogus DOCTYPE — the standard's thirteen sub-states. |
| CDATA section states | cdata | Section body + end bracket matching, foreign content only. |
| Character reference states | character | Ampersand → named table lookup; `#` numeric with windows-1252 remap (Appendix J); missing-semicolon recoveries. |
| RCDATA | character | Like Data but `<` opens RCDATA-end; character references active. |
| RAWTEXT | character | Like RCDATA without character references. |
| Script data | script | RAWTEXT rules plus the `<!--` double-escape machinery below. |
| Script data less-than sign / end tag name / escape start / escaped / escaped dash / escaped dash dash / escaped less-than sign / double escaped / double escaped dash / double escaped dash dash | script | The fourteen script-data sub-states implementing `</script>` detection and the comment double-escape dance. |
| Plaintext | character | Everything is text; no transitions; the end of recovery. |

Tree-builder-driven switches: the tree constructor sets the tokenizer's state
for `title`/`textarea` (RCDATA), `style`/`xmp`/`iframe`/`noembed`/`noframes`
(RAWTEXT), `script` (script data), and `plaintext` — the tokenizer exposes
`set_state()` for exactly these re-entrancies (§5.4 streaming invariant).

## APPENDIX L — URL Parser State Index

The WHATWG basic URL parser states (§5.1), indexed for navigation like
Appendix K. Implementation: one `match` arm per state in
`aurora_url::parser`; the WHATWG URL test data (adopted per §8.6) drives every
transition.

| State | Purpose (one line) |
|---|---|
| Scheme start / scheme | Accumulate the scheme; ASCII alpha start, then alnum + `+ - .`; `:` completes; else no-scheme failure. |
| No scheme | Relative reference against the base, or failure. |
| File / file slash / file host / file path states | The four file-URL states: drive-letter handling (Windows), host (empty allowed), path semantics. |
| Special relative or authority | `//` → authority; else relative-path state for special schemes. |
| Special authority slashes / special authority ignore slashes | Consume any number of `/` or `\` (special schemes treat `\` as separator), then authority. |
| Authority state | Credentials split on the last `@`; host/port follow. |
| Host / hostname | Domain (IDNA to-ASCII, lowercase), IPv4 (four numeric parts with the standard's overflow rules), or IPv6 in brackets. |
| Port | Digits until `/`, `?`, `#`, or EOF; empty port allowed; validity checked against the scheme. |
| Path or authority | For non-special schemes: `//`-prefix decides authority. |
| Path start | Decide empty-path vs absolute-path; drive-letter logic for file. |
| Path | Segment accumulation, dot-segment collapse, empty-segment rules. |
| Cannot-be-a-base path | Opaque path for non-hierarchical schemes (`data:`, `mailto:`). |
| Query | Percent-encode with the query set (special vs non-special encode sets differ). |
| Fragment | Percent-encode with the fragment set; terminates parsing. |

Serialization invariants (§5.1): scheme lowercase; special schemes always
serialize an authority; password omitted when empty; port omitted when it is
the scheme default; file URLs serialize per the platform convention chosen in
the ADR (§4.9) — decide once, test everywhere.

## APPENDIX M — DOMException Table

The exceptions the engine throws at script-facing boundaries (§4.6 class 3,
§9.3). The binding layer maps internal error enums to exactly these; anything
not in this table is a bug. Name, message pattern, and typical trigger:

| Name | Thrown when |
|---|---|
| `IndexSizeError` | Index negative or greater than the allowed count (ranges, canvas). |
| `HierarchyRequestError` | Tree request violates the hierarchy (ancestor as descendant, wrong parent). |
| `WrongDocumentError` | Node from another document used where the current one is required (legacy paths). |
| `InvalidCharacterError` | String contains characters the context forbids (element names, `createElement`). |
| `NoModificationAllowedError` | Object cannot be modified (readonly nodes, non-contentEditable mutation). |
| `NotFoundError` | Object not found where it must be (insertBefore child, removeChild). |
| `NotSupportedError` | Operation not supported by this engine — the §1.4 non-goal surface's standard answer. |
| `InUseAttributeError` | Adopting an attribute still attached to an element. |
| `InvalidStateError` | Object in a state where the operation is unavailable (response body consumed, port closed). |
| `SyntaxError` | Malformed string in an API (selector strings, `querySelector`, invalid regex flags, JSON in some contexts). |
| `InvalidModificationError` | Mutation would produce invalid content (doctype inside elements). |
| `NamespaceError` | Malformed namespace operations (`createElementNS` qname/prefix mismatch). |
| `InvalidAccessError` | Object/function no longer supported, or unsupported argument combination. |
| `TypeMismatchError` | Legacy name of `TypeError` for IDL type mismatches in some interfaces. |
| `SecurityError` | §5.18 boundary check failures: cross-origin access, storage denial, forbidden operation. |
| `NetworkError` | Fetch/network failures surfaced as DOM errors (WebSocket connect failure, form submission fetch). |
| `AbortError` | Operation aborted (fetch abort via §6.6 `AbortSignal`, navigation interrupted). |
| `URLMismatchError` | `window.open`/history target origin mismatch. |
| `QuotaExceededError` | Storage quota exceeded (§6.10). |
| `TimeoutError` | Operation timed out (signals, locking stretch goals). |
| `InvalidNodeTypeError` | Wrong node type for the operation (ranges, mutation observer init). |
| `DataCloneError` | `structuredClone`/`postMessage` value not cloneable (functions, DOM nodes). |

Every binding test (§13.3 runbook) asserts both the name and that the message
follows the documented pattern — messages are not API, but their shape is.

## APPENDIX N — Implicit ARIA Roles

The accessibility scaffolding of §5.19.6 computes roles from markup. Table:
element → implicit role (per HTML-AAM; the computed-tree test corpus
`tests/corpus/a11y/` asserts one row per line). `aria-*` attributes and
explicit `role` always override the implicit value.

| Element(s) | Implicit role |
|---|---|
| `a[href]` | `link` |
| `address` | `group` |
| `article` | `article` |
| `aside` | `complementary` |
| `button` | `button` |
| `datalist` | `listbox` |
| `details` | `group` |
| `summary` (first child of details) | (no role; acts as disclosure) |
| `dialog[open]` | `dialog` |
| `fieldset` | `group` |
| `figure` | `figure` |
| `footer` (not in sectioning content) | `contentinfo` |
| `form` | `form` |
| `h1`–`h6` | `heading` (level from the element) |
| `header` (not in sectioning content) | `banner` |
| `hr` | `separator` |
| `img[alt=""]` | `presentation` |
| `img[alt]` | `img` |
| `input[type=button/submit/reset]` | `button` |
| `input[type=checkbox]` | `checkbox` |
| `input[type=number]` | `spinbutton` |
| `input[type=radio]` | `radio` |
| `input[type=range]` | `slider` |
| `input[type=text/search/tel/url/email/password]` | `textbox` |
| `li` | `listitem` |
| `main` | `main` |
| `menu` | `list` |
| `nav` | `navigation` |
| `ol`, `ul` | `list` |
| `option` | `option` |
| `output` | `status` |
| `progress` | `progressbar` |
| `section` | `region` (named) / `generic` |
| `select` | `combobox` (single) / `listbox` (multiple) |
| `table` | `table` |
| `tbody/thead/tfoot` | `rowgroup` |
| `td` | `cell` |
| `textarea` | `textbox` (multiline) |
| `th` | `columnheader` / `rowheader` per scope |
| `tr` | `row` |

Name computation (subset): `aria-label` → `aria-labelledby` target text →
native source (`alt`, `label[for]`, `legend`, `caption`, `title`, value) →
text content → nothing. The computed tree is exposed through the DevTools
inspector's a11y view and the platform layer where available (§5.19.6); its
tests are pure engine tests of the computed tree, not of any AT integration.

## APPENDIX O — Glossary of Engine Terms

The seed for `docs/GLOSSARY.md` (§9.1 mandates abbreviations be registered
here); extend it in the same session that coins a term.

| Term | Meaning in this codebase |
|---|---|
| AA | Anti-aliasing; the rasterizer's analytic coverage (§5.16.2). |
| Adoption agency | The HTML algorithm that repairs mis-nested formatting elements (§5.5). |
| ADR | Architecture Decision Record (§4.9). |
| an+b | The nth-child step/offset grammar (§6.5). |
| BFC | Block formatting context (§5.9.2). |
| Binding | The script↔native bridge implementing a DOM interface (§5.15.2). |
| Blink | Reference engine (Chrome); never a dependency — the word appears here only in tests' naming of behaviors. |
| Box tree | The display/style-driven structure before layout (§5.9.1). |
| Chunked | HTTP/1.1 transfer coding framing (§5.2.3). |
| Clip chain | The ordered stack of clips applied to display items (§5.16.3). |
| Computed value | The per-longhand resolved style after cascade and defaulting (§5.8.3). |
| CSPRNG | Cryptographically secure random source used by `crypto.getRandomValues` (§6.6 Crypto). |
| Document thread | The §4.4 thread owning a document's DOM, script, and pipeline. |
| DOMException | The script-visible exception taxonomy (Appendix M). |
| DPR | Device pixel ratio; shell→engine scale factor (§5.19.4). |
| Facade | A crate's single re-export surface (§4.10). |
| Foster parenting | Moving out-of-place table content before the table (§5.5). |
| Fragment | A laid-out piece of a box: the geometry product (§5.9). |
| GC heap | The JavaScript engine's traced object heap (§5.14.6). |
| Hit test | Display-list reverse lookup of the element under a point (§5.16.4). |
| IDL | WebIDL; the interface definition language of the DOM surface (§5.15.2). |
| IFC | Inline formatting context (§5.9.3). |
| Insertion mode | The tree constructor's state (§5.5). |
| Interning | Deduplicating strings/selectors into shared tables (§4.5). |
| InvalidAtComputedValueTime | Custom-property failure mode resolving to unset (§5.8.3). |
| Live collection | A DOM list that recomputes against the current tree (§5.6). |
| Loader thread | The §4.4 thread running DNS/TCP/TLS/HTTP. |
| NaN-boxing | The JS value encoding (§5.14.1). |
| Noah's Ark clause | Maximum of 3 identical active formatting elements (§5.5). |
| Origin | The security principal: tuple or opaque (§5.18.1). |
| Paint order | CSS 2.1 Appendix E ordering of display items (§5.16.1). |
| Presentational hint | Attribute-sourced style at the lowest author tier (§5.8.2). |
| Quirks mode | Legacy compatibility mode selected by DOCTYPE (§5.5.3). |
| Revalidation | Conditional request re-checking a cached entry (§5.2.6). |
| Safepoint | A point where the interpreter may pause for GC or interrupts (§5.14.6). |
| SameSite | The cookie cross-site sending policy (§5.17.1). |
| Scoping root | The `@scope`/`querySelector`-context root element (§6.5). |
| Shape | The JS hidden-class record of an object's properties (§5.14.2). |
| Shaping | Turning characters into positioned glyphs (§5.11.2). |
| Sniffing | Content-type inference from bytes (§5.2.7). |
| Spec-first | The §2.2 rule: read the standard before writing the code. |
| SOP | Same-origin policy (§5.18). |
| Stack | The tree constructor's stack of open elements (§5.5). |
| Strut | The imaginary zero-content line box ensuring minimum line height (§5.9.3). |
| Stacking context | The atomic painting subtree boundary (§5.16.1). |
| Task / microtask | Event-loop scheduling units and their ordering rules (§5.15.1). |
| TDZ | Temporal dead zone for let/const bindings (§5.14.3). |
| Tier 0/1/2 | The §3.2 dependency classes. |
| Tombstone | A logically removed arena node kept for generation safety (§4.5). |
| UA origin | The user-agent stylesheet's cascade origin (§5.8.2). |
| Vertical slice | A milestone cut that works end to end (§2.3). |
| WBS | The work breakdown structure (Part 6). |
| WPT | Web Platform Tests (§8.6). |
| test262 | The ECMAScript conformance suite (§8.6). |
