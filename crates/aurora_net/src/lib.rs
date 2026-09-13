//! HTTP/1.1 client: requests, redirects, chunked bodies, content decoding, sniffing (§5.2).
//!
//! Thread: loader threads (§4.4).
//! Neighbors: `aurora_url`, `aurora_platform` in; consumed by `aurora_runtime`, `aurora_storage`.
//!
//! The stack never panics on malformed input: every protocol violation is a
//! `NetError::Protocol` and the connection dies (§5.2 invariant). All tests
//! run against a local in-process mock server — no external network (§5.2).
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface
//! of this crate is exactly what `src/facade.rs` re-exports (§4.10).

// §9.6: unit tests are exempt from the unwrap/expect restrictions.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod body;
mod chunked;
mod conn;
mod error;
mod facade;
mod headers;
mod loader;
mod message;

pub use facade::*;
