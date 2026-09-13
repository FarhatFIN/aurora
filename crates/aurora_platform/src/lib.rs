//! Platform glue: window/present, clipboard, font enumeration, DPI, clocks (§3.5).
//!
//! Thread: UI thread + helpers it spawns.
//! Neighbors: winit/softbuffer (Tier 1) in from M6; consumed by `aurora_text`, `aurora_net`, `aurora_storage`, `aurora_shell`.
//!
//! AURORA is specified in `BROWSER_ENGINE_PROMPT.md` at the repository root;
//! the crate map and its hard rules are §4.3 there. The public API surface of
//! this crate is exactly what `src/facade.rs` re-exports (§4.10).

mod facade;
