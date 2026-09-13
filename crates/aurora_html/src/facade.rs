//! Public API surface of `aurora_html`.
//!
//! Re-exported from `lib.rs` via `pub use facade::*;` (§4.10). The M2
//! tokenizer lands here next (§5.4); the named-reference table is already
//! part of the crate's data surface.

pub use crate::tables::NAMED_REFERENCES;
pub use crate::token::{Attribute, InitialState, Token, TokenizerOptions};
pub use crate::tokenizer::Tokenizer;
