# PROGRESS

The single source of truth for where the project is (§12.1). Updated every
session, before the report ends. Reality wins over this file, then this file
is corrected.

## Current state

- Milestone: M1 — Fetch and render text
- WBS completion: 0% (auto: `scripts/wbs-progress.sh` — 0 of 2,680 items)
- Standing placeholders: 0
- Open AURORA-SHORTCUT tags: 0
- Bench deltas vs last milestone: none (benchmarks land with the §10.1 table)

## Open questions

None.

## Debts

None. (`docs/known-issues.md` is empty; no `AURORA-SHORTCUT` tags in the tree.)

## Decision log

| Session | Decision | ADR |
|---|---|---|
| 1 | Rust 1.95.0 / edition 2024; §9.6 lint set via `[workspace.lints]` (mechanism equivalent of §4.10's per-crate attributes); three-platform CI per §3.5, superseding §7.2's "two-platform" wording | 0001 |
| 1 | Prompt source (`parts/`, `generated/`, `tools/`) vendored at repo root so WBS ticks follow the §6.12 data-file → regenerate → review discipline | 0002 |
| 1 | All 19 crates start with zero dependencies (§3.2 Tier 0); dependency edges appear with the milestone that needs them | — |

## Session log

(newest first, §11.4 template)

## Session 2026-09-13-1900 — M0 bootstrap: workspace, lint set, scripts, CI
- **Milestone:** M0 — Bootstrap and toolchain   **WBS items touched:** none (M0 is infrastructure only, §6.13)
- **Implemented:**
  - Cargo workspace with all 19 crates of the §4.3 map (18 in `crates/`, the
    `aurora_shell` application in `apps/`): each with the §4.10 skeleton
    (crate doc naming responsibility/thread/neighbors, `facade.rs` as the
    single re-export surface, an M0 smoke test), zero dependencies.
  - §9.6 lint set via `[workspace.lints]`; `rustfmt.toml`, `clippy.toml`,
    `rust-toolchain.toml` (MSRV 1.95.0) committed (ADR-0001).
  - Shell `--version` demo: prints name, workspace version, and git commit
    stamped by `build.rs`.
  - Scripts: `test-fast.sh` (fmt, clippy `-D warnings`, tests),
    `test-all.sh` (fast tier + dependency policy + WBS regeneration
    asserts), `check-deps.sh` (§4.3 allowed-deps map, §3.3 approved/forbidden
    lists, shell boundary), `wbs-progress.sh` (§6.14 report incl. `--strict`),
    `new-milestone.sh`, session-report template.
  - Prompt source vendored (`parts/`, `generated/`, `tools/`) with the
    byte-identical-regeneration assert in the full tier (ADR-0002).
  - Docs of §11.2 as stubs: README, ARCHITECTURE, GLOSSARY, UA-STYLESHEET,
    devtools-protocol, known-issues, spec-notes, ADR-0001/0002, CHANGELOG;
    tests/ inventory README.
  - CI: fast tier on Linux/Windows/macOS, full tier nightly.
- **Tested:** fast tier — 19 passed / 0 failed / 0 skipped (one smoke test per
  crate; bin test in `aurora_shell`); `cargo fmt --check` clean;
  `cargo clippy --workspace --all-targets -- -D warnings` clean;
  `scripts/check-deps.sh` OK; `scripts/wbs-progress.sh` reports 2,680 open / 0 done.
- **Demo (§7.2, verified at HEAD):** `cargo run -p aurora_shell -- --version`
  → `aurora-shell 0.1.0 (commit 2ccc011)`
- **Bench:** n/a (no benchmarks exist yet; §10.1 table lands with first benches)
- **Decisions:** see decision log above (ADR-0001, ADR-0002).
- **Debts opened/closed:** none. Environment note: rustfmt/clippy installed via
  distro packages (no rustup on this machine); `rust-toolchain.toml` advisory
  here, binding in CI — recorded in ADR-0001.
- **Next task:** §6.9 item 1 via §5.1 — begin M1 with `aurora_url`: the WHATWG
  URL parser and its unit/golden tests (§12.3 rule c, dependency-first: URL is
  the prerequisite of every §6.9 network item).
