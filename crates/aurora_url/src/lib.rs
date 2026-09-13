//! URL parsing, serialization, joining, and origin computation (§5.1).
//!
//! Thread: any (pure functions, no state).
//! Neighbors: none in; consumed by `aurora_net`, `aurora_security`, `aurora_storage`, `aurora_runtime`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
