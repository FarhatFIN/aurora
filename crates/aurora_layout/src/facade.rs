//! Public API surface of `aurora_layout`.
//!
//! Deliberately empty at M0 (§7.2): the crate owns no behavior yet. Items are
//! added here — and re-exported from `lib.rs` via `pub use facade::*;` — by
//! the milestone that first needs them. This file stays the single re-export
//! surface so future process boundaries (§4.7) can rely on it.
