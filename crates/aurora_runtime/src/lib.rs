//! Event loop, timers, microtasks, host bindings, console, fetch API (§5.15).
//!
//! Thread: document thread.
//! Neighbors: `aurora_js`, `aurora_dom`, `aurora_net`, `aurora_url`,
//! `aurora_encoding` in; consumed by `aurora`, `aurora_shell`.
//!
//! The M1 slice is the fetch entry point (§4.3 assigns the fetch-API here);
//! the event loop and bindings land with M9/M10. AURORA is specified in
//! `BROWSER_ENGINE_PROMPT.md` at the repository root; the public API
//! surface of this crate is exactly what `src/facade.rs` re-exports (§4.10).

// §9.6: unit tests are exempt from the unwrap/expect restrictions.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod facade;
mod fetch;

pub use facade::*;
