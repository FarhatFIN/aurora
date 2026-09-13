//! Public API surface of `aurora_net` (§5.2 sketch, M1 subset).
//!
//! Re-exported from `lib.rs` via `pub use facade::*;` — this file is the
//! crate's single re-export surface (§4.10).

pub use crate::body::{Framing, framing_for};
pub use crate::conn::Pool;
pub use crate::error::{CancelToken, NetError};
pub use crate::headers::{HeaderMap, InvalidHeader};
pub use crate::loader::{Response, fetch, fetch_once, wire_path};
pub use crate::message::{
    Method, RedirectPolicy, Request, ResponseHead, USER_AGENT, host_header_value,
};
