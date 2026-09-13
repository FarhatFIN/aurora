//! HTML tokenizer and tree constructor: insertion modes, foster parenting, parse errors (§5.4–§5.5).
//!
//! Thread: document thread.
//! Neighbors: `aurora_dom`, `aurora_encoding` in; consumed by `aurora_runtime`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the public API surface of this crate is exactly what `src/facade.rs`
//! re-exports (§4.10).

mod facade;
mod tables;

pub use facade::*;
