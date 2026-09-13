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
