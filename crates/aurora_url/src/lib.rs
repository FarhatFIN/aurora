//! URL parsing, serialization, joining, and origin computation (§5.1).
//!
//! Thread: any (pure functions, no state).
//! Neighbors: none in; consumed by `aurora_net`, `aurora_security`, `aurora_storage`, `aurora_runtime`.
//!
//! Implements the WHATWG URL Standard's basic URL parser, host parser, and
//! serializers. Parsing is total: any input yields a `Url` or a `ParseError`,
//! never a panic (§4.6 class 3). AURORA is specified in
//! `BROWSER_ENGINE_PROMPT.md` at the repository root; the crate map and its
//! hard rules are §4.3 there. The public API surface of this crate is exactly
//! what `src/facade.rs` re-exports (§4.10).

// §9.6: unit tests are exempt from the unwrap/expect restrictions (module
// attrs would be per-module; this crate-level cfg_attr covers them all).
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod facade;
mod form;
mod host;
mod origin;
mod parser;
mod percent;
mod url;

pub use facade::*;
