//! Selector matching, cascade, computed values (§5.8).
//!
//! Thread: document thread.
//! Neighbors: `aurora_css`, `aurora_dom` in; consumed by `aurora_layout`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
