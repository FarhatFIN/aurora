# Changelog

All notable changes per milestone exit (§11.6). Releases are tags on the main
line; `v0.1.0` is tagged at M13.

## [0.1.0] — unreleased (M1 in progress)

### Changed (project governance)
- License set to Apache-2.0 (explicit patent grant; ADR-0005). `LICENSE`
  added at the repository root.

### Added (M1 network slice)
- `aurora_url`: WHATWG URL parser/serializer/origins with round-trip suite.
- `aurora_net`: HTTP/1.1 client (redirects, keep-alive pool, chunked,
  gzip/deflate, per-phase timeouts, cancellation), TLS via rustls.
- `aurora_encoding`: UTF-8 + windows-1252 text decoding.
- `aurora_runtime`: fetch entry point with scheme dispatch (http/https/
  file/data/about:blank).
- `aurora` headless CLI: `--url`, `--dump-bytes`, `--dump-text`.
- TLS: webpki-roots default, `Pool::with_root_certs` override; certificate-
  failure matrix tests with a committed test PKI (§5.3 DoD).
- Mock harness: scripted delays and truncations; byte-exact request-head
  golden test; error-injection suite (truncation, bad chunk size,
  premature close) — all typed protocol errors, no panics.
- M1 exit criterion verified: `--dump-bytes` byte-identical to `curl -sL`
  on five test URLs.

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
