//! Origins, same-origin policy checks, CSP parser and enforcement hooks (§5.18).
//!
//! Thread: any (pure policy decisions).
//! Neighbors: `aurora_url` in; consumed by `aurora_runtime`, `aurora_net`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
