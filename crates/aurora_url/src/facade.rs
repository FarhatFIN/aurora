//! Public API surface of `aurora_url` (§5.1 sketch).
//!
//! Re-exported from `lib.rs` via `pub use facade::*;` — this file is the
//! crate's single re-export surface (§4.10), so future process boundaries
//! (§4.7) can rely on it.

pub use crate::form::form_urlencode;
pub use crate::host::{Host, HostError};
pub use crate::origin::{Origin, origin};
pub use crate::parser::ParseError;
pub use crate::percent::{AsciiSet, percent_decode, percent_encode};
pub use crate::url::{Path, Url};

impl Url {
    /// Parses an absolute URL with the basic URL parser (no base).
    ///
    /// # Errors
    /// Returns [`ParseError`] for any input the standard's parser rejects;
    /// parsing is total — malformed input never panics (§4.6 class 3).
    pub fn parse(input: &str) -> Result<Url, ParseError> {
        crate::parser::run(input, None)
    }

    /// Parses `input` with `base` as the base URL for relative references.
    ///
    /// # Errors
    /// Same as [`Url::parse`], plus relative-without-base when `input` has
    /// no scheme and no usable base applies.
    pub fn parse_with_base(input: &str, base: &Url) -> Result<Url, ParseError> {
        crate::parser::run(input, Some(base))
    }

    /// Resolves a reference string against this URL (§5.1 "join").
    ///
    /// # Errors
    /// Same as [`Url::parse_with_base`].
    pub fn join(&self, reference: &str) -> Result<Url, ParseError> {
        crate::parser::run(reference, Some(self))
    }
}
