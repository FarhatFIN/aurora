//! Connections: DNS, TCP, TLS setup (§5.3) and the per-origin keep-alive
//! pool (§5.2.1: max 6 per origin, 60 s idle expiry, one clean retry on a
//! fresh connection when a pooled one fails).

use crate::body::{PlainStream, Stream, TlsStream};
use crate::error::{CancelToken, NetError};
use aurora_url::Url;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const DNS_TIMEOUT: Duration = Duration::from_secs(10);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const TLS_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
const HEADER_TIMEOUT: Duration = Duration::from_secs(30);
const BODY_IDLE_TIMEOUT: Duration = Duration::from_secs(30);
const POOL_IDLE_EXPIRY: Duration = Duration::from_mins(1); // §5.2.1: 60 s idle expiry
const POOL_MAX_PER_ORIGIN: usize = 6;

/// A live connection with its streams ready for request exchange.
pub(crate) enum Connection {
    Plain(PlainStream),
    Tls(Box<TlsStream>),
}

impl Connection {
    pub(crate) fn as_read(&mut self) -> &mut dyn Stream {
        match self {
            Connection::Plain(plain) => plain,
            Connection::Tls(tls) => tls.as_mut(),
        }
    }

    pub(crate) fn as_write(&mut self) -> &mut dyn std::io::Write {
        match self {
            Connection::Plain(plain) => plain,
            Connection::Tls(tls) => tls.as_mut(),
        }
    }

    /// Whether the peer half-closed or errored on a keep-alive liveness
    /// probe (a 0-byte peek is not detectable without disturbing the
    /// stream, so failure is only learned on use — the pool retries once).
    pub(crate) fn set_timeouts(&self, timeout: Duration) -> Result<(), NetError> {
        let stream: &TcpStream = match self {
            Connection::Plain(plain) => &plain.0,
            // rustls StreamOwned exposes the socket through get_ref().
            Connection::Tls(tls) => tls.0.get_ref(),
        };
        stream
            .set_read_timeout(Some(timeout))
            .map_err(|_| NetError::Io)?;
        stream
            .set_write_timeout(Some(timeout))
            .map_err(|_| NetError::Io)
    }
}

/// DNS resolution with a timeout (std's resolver is unbounded; the lookup
/// runs on a helper thread so the phase stays cancelable and bounded).
pub(crate) fn resolve(url: &Url, cancel: &CancelToken) -> Result<Vec<SocketAddr>, NetError> {
    let host = match url.host() {
        Some(host) => host.serialize(),
        None => return Err(NetError::UnsupportedScheme),
    };
    let port = url
        .port()
        .or_else(|| url.default_port())
        .ok_or(NetError::UnsupportedScheme)?;
    let attempt = format!("{host}:{port}");
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(attempt.to_socket_addrs().map(Iterator::collect::<Vec<_>>));
    });
    cancel.check()?;
    let deadline = Instant::now() + DNS_TIMEOUT;
    let remaining = deadline.saturating_duration_since(Instant::now());
    if remaining.is_zero() {
        return Err(NetError::Dns);
    }
    match rx.recv_timeout(remaining) {
        Ok(Ok(addrs)) if !addrs.is_empty() => Ok(addrs),
        _ => Err(NetError::Dns),
    }
}

/// Opens a new connection to the URL's origin: TCP with a connect timeout,
/// then the rustls TLS handshake for https (§5.3: TLS 1.2/1.3, full chain
/// and hostname verification, ALPN http/1.1, no click-through).
pub(crate) fn connect(
    url: &Url,
    cancel: &CancelToken,
    tls: &Arc<rustls::ClientConfig>,
) -> Result<Connection, NetError> {
    let addrs = resolve(url, cancel)?;
    let mut last = NetError::Connect;
    for addr in addrs {
        cancel.check()?;
        match TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT) {
            Ok(stream) => {
                stream.set_nodelay(true).map_err(|_| NetError::Io)?;
                return if url.scheme() == "https" {
                    let tls = tls_handshake(stream, url, cancel, tls)?;
                    Ok(Connection::Tls(Box::new(TlsStream(tls))))
                } else {
                    Ok(Connection::Plain(PlainStream(stream)))
                };
            }
            Err(_) => {
                last = NetError::Connect;
            }
        }
    }
    Err(last)
}

fn tls_handshake(
    stream: TcpStream,
    url: &Url,
    cancel: &CancelToken,
    config: &Arc<rustls::ClientConfig>,
) -> Result<rustls::StreamOwned<rustls::ClientConnection, TcpStream>, NetError> {
    let server_name = server_name_for(url)?;
    stream
        .set_read_timeout(Some(TLS_HANDSHAKE_TIMEOUT))
        .map_err(|_| NetError::Io)?;
    let conn = rustls::ClientConnection::new(Arc::clone(config), server_name)
        .map_err(|_| NetError::Tls("handshake setup"))?;
    let mut tls = rustls::StreamOwned::new(conn, stream);
    let deadline = Instant::now() + TLS_HANDSHAKE_TIMEOUT;
    // Canonical handshake driver: process what arrived, write what is
    // pending, read more ciphertext. Driving `process_new_packets` directly
    // keeps verification failures typed (§5.3: exact TlsFailure variants).
    while tls.conn.is_handshaking() {
        cancel.check()?;
        if Instant::now() > deadline {
            return Err(NetError::Tls("handshake timeout"));
        }
        match tls.conn.process_new_packets() {
            Ok(_) => {}
            Err(error) => return Err(NetError::Tls(tls_failure_text(&error))),
        }
        let socket = &mut tls.sock;
        if let Err(error) = tls.conn.write_tls(socket)
            && error.kind() != std::io::ErrorKind::WouldBlock
        {
            return Err(NetError::Tls("handshake write failed"));
        }
        match tls.conn.read_tls(socket) {
            Ok(0) => return Err(NetError::Tls("connection closed during handshake")),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(_) => return Err(NetError::Tls("handshake read failed")),
        }
    }
    tls.conn
        .process_new_packets()
        .map_err(|error| NetError::Tls(tls_failure_text(&error)))?;
    Ok(tls)
}

fn tls_failure_text(error: &rustls::Error) -> &'static str {
    match error {
        rustls::Error::InvalidCertificate(detail) => match detail {
            rustls::CertificateError::Expired | rustls::CertificateError::ExpiredContext { .. } => {
                "certificate expired"
            }
            rustls::CertificateError::NotValidForName
            | rustls::CertificateError::NotValidForNameContext { .. } => "hostname mismatch",
            rustls::CertificateError::UnknownIssuer => "untrusted certificate authority",
            _ => "certificate verification failed",
        },
        rustls::Error::PeerIncompatible(_) => "peer incompatible",
        rustls::Error::PeerMisbehaved(_) => "peer misbehaved",
        rustls::Error::General(_) => "handshake failure",
        _ => "handshake failed",
    }
}

fn server_name_for(url: &Url) -> Result<rustls::pki_types::ServerName<'static>, NetError> {
    match url.host() {
        Some(aurora_url::Host::Domain(domain)) if !domain.is_empty() => {
            rustls::pki_types::ServerName::try_from(domain.clone())
                .map_err(|_| NetError::Tls("invalid DNS name"))
                .map(|name| name.to_owned())
        }
        Some(aurora_url::Host::Ipv4(bytes)) => Ok(rustls::pki_types::ServerName::IpAddress(
            rustls::pki_types::IpAddr::from(std::net::IpAddr::from(*bytes)),
        )),
        Some(aurora_url::Host::Ipv6(pieces)) => {
            let octets: [u8; 16] = std::array::from_fn(|idx| {
                let piece = pieces[idx / 2];
                if idx % 2 == 0 {
                    (piece >> 8) as u8
                } else {
                    (piece & 0xFF) as u8
                }
            });
            Ok(rustls::pki_types::ServerName::IpAddress(
                rustls::pki_types::IpAddr::from(std::net::IpAddr::V6(std::net::Ipv6Addr::from(
                    octets,
                ))),
            ))
        }
        _ => Err(NetError::Tls("no server name")),
    }
}

/// The pool key: scheme + host + port (§5.2.1).
#[derive(Clone, PartialEq, Eq, Hash)]
struct OriginKey(String);

fn origin_key(url: &Url) -> OriginKey {
    let port = url.port().or_else(|| url.default_port()).unwrap_or(0);
    OriginKey(format!(
        "{}|{}|{port}",
        url.scheme(),
        url.host()
            .map(aurora_url::Host::serialize)
            .unwrap_or_default()
    ))
}

struct Pooled {
    connection: Connection,
    last_used: Instant,
}

/// Per-origin keep-alive connection pool (§5.2.1). The pool prepares the
/// TLS client config once: webpki-roots by default, or an explicit root
/// set for tests and the documented `--cert-bundle` override (§7.3).
pub struct Pool {
    idle: Mutex<HashMap<OriginKey, Vec<Pooled>>>,
    pub(crate) tls_config: Arc<rustls::ClientConfig>,
}

impl Default for Pool {
    fn default() -> Self {
        Self {
            idle: Mutex::new(HashMap::new()),
            tls_config: default_tls_config(),
        }
    }
}

/// Builds the default client config: TLS 1.2/1.3, webpki-roots, ring provider.
fn default_tls_config() -> Arc<rustls::ClientConfig> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let mut roots = rustls::RootCertStore::empty();
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let builder = rustls::ClientConfig::builder_with_provider(provider)
        .with_protocol_versions(&[&rustls::version::TLS13, &rustls::version::TLS12])
        .unwrap_or_else(|_| {
            // Invariant: the pinned ring provider always offers TLS 1.2/1.3;
            // failure means the dependency set itself is broken (§4.6 class 2).
            panic!("ring provider does not offer TLS 1.2/1.3 — broken dependency pin")
        });
    let config = builder.with_root_certificates(roots).with_no_client_auth();
    Arc::new(config)
}

impl Pool {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// A pool trusting exactly `roots` instead of the webpki set — the
    /// test fixture path and the future `--cert-bundle` override (§7.3).
    ///
    /// # Errors
    /// [`NetError::Tls`] when a root certificate cannot be parsed into the
    /// trust store.
    pub fn with_root_certs(
        roots: impl IntoIterator<Item = rustls::pki_types::CertificateDer<'static>>,
    ) -> Result<Self, NetError> {
        let provider = Arc::new(rustls::crypto::ring::default_provider());
        let mut store = rustls::RootCertStore::empty();
        for cert in roots {
            store
                .add(cert)
                .map_err(|_| NetError::Tls("bad root certificate"))?;
        }
        let builder = rustls::ClientConfig::builder_with_provider(provider)
            .with_protocol_versions(&[&rustls::version::TLS13, &rustls::version::TLS12])
            .map_err(|_| NetError::Tls("no usable protocol version"))?;
        let config = builder.with_root_certificates(store).with_no_client_auth();
        Ok(Self {
            idle: Mutex::new(HashMap::new()),
            tls_config: Arc::new(config),
        })
    }

    pub(crate) fn checkout(&self, url: &Url) -> Option<Connection> {
        let mut idle = self.idle.lock().ok()?;
        let key = origin_key(url);
        let entry = idle.get_mut(&key)?;
        let now = Instant::now();
        while let Some(pooled) = entry.pop() {
            if now.duration_since(pooled.last_used) < POOL_IDLE_EXPIRY {
                return Some(pooled.connection);
            }
            // Expired entries are dropped; the loop keeps looking.
        }
        None
    }

    pub(crate) fn checkin(&self, url: &Url, connection: Connection) {
        let Ok(mut idle) = self.idle.lock() else {
            return;
        };
        let vec = idle.entry(origin_key(url)).or_default();
        if vec.len() < POOL_MAX_PER_ORIGIN {
            vec.push(Pooled {
                connection,
                last_used: Instant::now(),
            });
        }
        // Over the cap: drop the connection (it closes on drop).
    }
}

/// Timeouts for the request/response exchange phases (§5.2.5).
pub(crate) const PHASE_HEADER_TIMEOUT: Duration = HEADER_TIMEOUT;
pub(crate) const PHASE_BODY_IDLE_TIMEOUT: Duration = BODY_IDLE_TIMEOUT;
