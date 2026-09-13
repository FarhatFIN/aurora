# Changelog

All notable changes per milestone exit (§11.6). Releases are tags on the main
line; `v0.1.0` is tagged at M13.

## [0.1.0] — unreleased (M0)

### Added
- Cargo workspace with the 19 crates of the §4.3 crate map, each an empty-but-
  real skeleton with crate documentation, a `facade.rs` public-surface module,
  and a smoke test.
- The §9.6 lint set enforced workspace-wide; rustfmt/clippy configs; MSRV
  pinned to 1.95.0 (ADR-0001).
- `aurora_shell --version` demo (name, version, git commit).
- Test tiers `scripts/test-fast.sh` / `scripts/test-all.sh`; dependency
  policy check `scripts/check-deps.sh`; WBS progress report
  `scripts/wbs-progress.sh`; milestone switch `scripts/new-milestone.sh`.
- Master specification vendored with its WBS generator and byte-identical
  regeneration assert (ADR-0002); WBS tracks 2,680 items.
- CI: fast tier on Linux/Windows/macOS, full tier nightly.
- Documentation set of §11.2 as accurate stubs.

### Known issues
- None.
