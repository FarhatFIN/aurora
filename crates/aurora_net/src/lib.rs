//! HTTP/1.1 client: requests, redirects, chunked bodies, content decoding, sniffing (§5.2).
//!
//! Thread: loader threads (§4.4).
//! Neighbors: `aurora_url`, `aurora_platform` in; consumed by `aurora_runtime`, `aurora_storage`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
