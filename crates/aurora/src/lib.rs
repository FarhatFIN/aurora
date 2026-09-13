//! The `WebView` facade: the only public API of the engine (§4.1, §4.8).
//!
//! Thread: boundary crate; no thread of its own.
//! Neighbors: `aurora_runtime` in; consumed by `aurora_shell` and embedders.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
