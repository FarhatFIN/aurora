//! Message types crossing thread (future: process) boundaries (§4.4, §4.7).
//!
//! Thread: all threads (type definitions only).
//! Neighbors: none in; consumed by `aurora_runtime`, `aurora_shell`, `aurora_platform`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
