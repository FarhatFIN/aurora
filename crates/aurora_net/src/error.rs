//! Error taxonomy and cancellation for the network stack (§4.6 class 1,
//! §5.2.5).

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Network stack errors (§5.2's `NetError`). Values, not exceptions: every
/// variant is an expected outcome of fetching (§4.6 class 1).
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum NetError {
    /// DNS resolution failed or timed out.
    Dns,
    /// TCP connection failed (connect refused/timeout/unreachable).
    Connect,
    /// TLS handshake or certificate verification failed; the payload names
    /// the phase for the error page (§5.3: no click-through).
    Tls(&'static str),
    /// An HTTP/1.1 protocol violation; the connection is dead afterwards.
    Protocol(&'static str),
    /// A phase deadline elapsed.
    Timeout,
    /// The cancel token was set mid-operation.
    Aborted,
    /// More than 20 redirect hops.
    TooManyRedirects,
    /// The scheme has no loader (error page in the UI later, §6.9 item 1).
    UnsupportedScheme,
    /// An I/O error outside a named phase.
    Io,
}

/// Cooperative cancellation for fetches (§4.4: atomics for liveness flags —
/// the one permitted shared-mutable-state exception).
#[derive(Clone, Default)]
pub struct CancelToken(Arc<AtomicBool>);

impl CancelToken {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Signals every operation sharing this token to stop.
    pub fn cancel(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }

    pub(crate) fn check(&self) -> Result<(), NetError> {
        if self.is_cancelled() {
            Err(NetError::Aborted)
        } else {
            Ok(())
        }
    }
}
