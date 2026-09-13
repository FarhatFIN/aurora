//! CSS tokenizer, parser, stylesheet objects, selectors AST, property registry, values (§5.7).
//!
//! Thread: document thread.
//! Neighbors: none in; consumed by `aurora_style`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
