//! The `WebView` facade: the only public API of the engine (§4.1, §4.8).
//!
//! Thread: boundary crate; no thread of its own.
//! Neighbors: `aurora_runtime` in; consumed by `aurora_shell` and embedders.
//!
//! Contains no logic (§4.3): entry points are re-exports of the runtime's
//! engine surface. The headless `aurora` binary (the milestone-demo driver)
//! lives in this package as a separate bin target and touches the engine
//! only through this facade, exactly like an embedder. AURORA is specified
//! in `BROWSER_ENGINE_PROMPT.md` at the repository root.

mod facade;

pub use facade::*;
