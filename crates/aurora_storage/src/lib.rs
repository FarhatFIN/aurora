//! Cookies, localStorage, sessionStorage, HTTP disk cache (§5.17).
//!
//! Thread: storage writer thread (§4.4).
//! Neighbors: `aurora_net`, `aurora_platform` in; consumed by `aurora_runtime`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
