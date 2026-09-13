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
