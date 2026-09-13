//! Text encodings: UTF-8/UTF-16, windows-1252, encoding detection (§5.4 upstream).
//!
//! Thread: any (pure functions).
//! Neighbors: none in; consumed by `aurora_html`, `aurora_net`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
