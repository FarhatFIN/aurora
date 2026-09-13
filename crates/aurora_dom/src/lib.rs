//! DOM core: arena of nodes, Document/Element, events, ranges; hosts selector runtime data (§5.6).
//!
//! Thread: document thread exclusively.
//! Neighbors: none in; consumed by `aurora_html`, `aurora_style`, `aurora_layout`, `aurora_runtime`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
