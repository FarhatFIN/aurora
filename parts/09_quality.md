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
