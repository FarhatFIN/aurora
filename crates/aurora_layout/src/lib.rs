//! Box building, fragment tree, block/inline/flex/grid/table layout (§5.9–§5.10).
//!
//! Thread: document thread.
//! Neighbors: `aurora_style`, `aurora_text`, `aurora_dom` in; consumed by `aurora_paint`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
