//! Text encodings: UTF-8/16, windows-1252, encoding detection (§5.4 upstream).
//!
//! Thread: any (pure functions).
//! Neighbors: none in; consumed by `aurora_html`, `aurora_net`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the public API surface of this crate is exactly what `src/facade.rs`
//! re-exports (§4.10).

// §9.6: unit tests are exempt from the unwrap/expect restrictions.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod decode;
mod facade;
mod tables;

pub use facade::*;
