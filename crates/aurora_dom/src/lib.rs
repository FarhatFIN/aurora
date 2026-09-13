//! DOM core: node arena, Document/Element, events, ranges (§5.6).
//!
//! Thread: document thread exclusively.
//! Neighbors: none in; consumed by `aurora_html`, `aurora_style`, `aurora_layout`, `aurora_runtime`.
//!
//! Parsing and cascade are hosted here and implemented elsewhere (§4.3):
//! `aurora_html` fills the tree, `aurora_style` reads it. All mutation goes
//! through `Document` methods (§5.6 invariant) so observers and
//! invalidation hooks cannot be bypassed. AURORA is specified in
//! `BROWSER_ENGINE_PROMPT.md` at the repository root; the public API
//! surface of this crate is exactly what `src/facade.rs` re-exports (§4.10).

// §9.6: unit tests are exempt from the unwrap/expect restrictions.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod arena;
mod facade;
mod serialize;
mod tree;

pub use facade::*;
