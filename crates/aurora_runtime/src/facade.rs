//! Public API surface of `aurora_runtime`.
//!
//! Re-exported from `lib.rs` via `pub use facade::*;` (§4.10).

pub use crate::fetch::{FetchOutcome, fetch};
pub use aurora_net::{CancelToken, NetError};
