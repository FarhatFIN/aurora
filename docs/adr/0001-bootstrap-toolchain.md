# ADR-0001: Bootstrap toolchain and lint mechanism

- **Status:** accepted (M0, first session)
- **Context:** §3.1 mandates Rust (edition 2021 or newer, stable toolchain);
  §3.4 pins the MSRV in `rust-toolchain.toml` and commits `rustfmt.toml` /
  `clippy.toml` in the first session; §9.6 fixes the lint set. The development
  machine (Kali Linux) provides Rust 1.95.0 as distro packages *without
  rustup*; `rustfmt` and `clippy` were installed to match (`rustfmt 1.9.0`,
  `clippy 0.1.95`). The dev environment is Linux-only today (§3.5).
- **Decision:**
  1. Edition **2024** ("2021 or newer" per §3.1); MSRV pinned to **1.95.0**,
     the current stable at project start.
  2. The §9.6 lint set is declared once in `[workspace.lints]` of the root
     `Cargo.toml` and inherited by every member via `[lints] workspace =
     true`. This is the mechanism-level equivalent of the per-crate
     `#![deny]`/`#![warn]` attribute block in §4.10 — same lints, single
     source of truth, no 19-way drift. `cargo clippy --workspace
     --all-targets -- -D warnings` makes pedantic warnings fatal, as §3.4
     requires.
  3. CI matrix is **three platforms** (Linux, Windows, macOS) per §3.5
     ("Tier-1 platforms … must have CI"), which supersedes §7.2's
     "two-platform CI matrix" wording — recorded as a spec inconsistency per
     §0.4.
  4. `rust-toolchain.toml` is advisory on machines without rustup; the pinned
     channel binds wherever rustup exists (CI).
- **Consequences:** zero-dependency workspace at M0 (§3.2); new Tier 1
  dependencies require the milestone that needs them plus a one-line
  justification in the member's `Cargo.toml` and must pass
  `scripts/check-deps.sh`. Dropping or adding a platform later needs an ADR.
