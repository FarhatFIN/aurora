//! Public API surface of `aurora_dom` (§5.6 sketch, M2 subset).
//!
//! Re-exported from `lib.rs` via `pub use facade::*;` (§4.10).

pub use crate::arena::{ElementData, Node, NodeData, NodeId};
pub use crate::serialize::serialize;
pub use crate::tree::{Children, Document, DomError, QuirksMode};
