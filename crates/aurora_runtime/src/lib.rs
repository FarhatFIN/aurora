//! Event loop, timers, microtasks, host bindings, console, fetch API (§5.15).
//!
//! Thread: document thread.
//! Neighbors: `aurora_js`, `aurora_dom`, `aurora_net`, `aurora_style`, `aurora_layout`, `aurora_paint` in; consumed by aurora, `aurora_shell`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
