//! URL origin computation (URL Standard §4.7 "origin"); the security model
//! consumes it in §5.18.

use crate::host::Host;
use crate::url::Url;

/// A URL's origin (URL Standard §4.7). `Tuple` origins compare by scheme,
/// host, and effective port; opaque origins compare only with themselves.
///
/// Hashable and totally ordered for use as cache/cookie/storage keys (§5.1
/// invariant).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Origin {
    /// A tuple origin: scheme, host, effective port (explicit or default).
    Tuple {
        /// The scheme, lowercase.
        scheme: String,
        /// The host (domain or IP address).
        host: Host,
        /// The effective port.
        port: u16,
    },
    /// An opaque origin: `file:` URLs, non-special schemes, and
    /// cannot-be-a-base URLs. Uniquely identified so equality matches
    /// "same origin" semantics (identical inputs still differ).
    Opaque(u64),
}

static NEXT_OPAQUE_ORIGIN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

fn next_opaque_id() -> u64 {
    NEXT_OPAQUE_ORIGIN.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// Returns the origin of `url` (URL Standard §4.7): a tuple origin for
/// special schemes other than `file:`, opaque otherwise.
#[must_use]
pub fn origin(url: &Url) -> Origin {
    if url.scheme() == "file" || !url.is_special() {
        return Origin::Opaque(next_opaque_id());
    }
    match url.host() {
        Some(host) => Origin::Tuple {
            scheme: url.scheme().to_owned(),
            host: host.clone(),
            port: url.port().or_else(|| url.default_port()).unwrap_or(0),
        },
        // A special non-file URL with a null host cannot be produced by the
        // parser (host-missing failure); defend without panicking (§4.6).
        None => Origin::Opaque(next_opaque_id()),
    }
}

impl Origin {
    /// Serializes the origin: `scheme://host:port` for tuple origins
    /// (default port elided), `null` for opaque origins.
    #[must_use]
    pub fn serialize(&self) -> String {
        match self {
            Origin::Tuple { scheme, host, port } => {
                let port_suffix = match crate::url::default_port(scheme) {
                    Some(default) if default == *port => String::new(),
                    _ => format!(":{port}"),
                };
                format!("{scheme}://{}{port_suffix}", crate::host::serialize(host))
            }
            Origin::Opaque(_) => "null".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Url;

    #[test]
    fn special_schemes_have_tuple_origins() {
        let https = Url::parse("https://example.com/").unwrap();
        assert_eq!(
            origin(&https),
            Origin::Tuple {
                scheme: "https".into(),
                host: Host::Domain("example.com".into()),
                port: 443,
            }
        );
        let http_port = Url::parse("http://example.com:8080/").unwrap();
        assert_eq!(
            origin(&http_port),
            Origin::Tuple {
                scheme: "http".into(),
                host: Host::Domain("example.com".into()),
                port: 8080,
            }
        );
    }

    #[test]
    fn file_and_non_special_urls_have_null_origins() {
        for input in ["file:///etc/passwd", "mailto:a@b.c", "foo://host/x"] {
            assert!(matches!(
                origin(&Url::parse(input).unwrap()),
                Origin::Opaque(_)
            ));
        }
        // Opaque origins are distinct even for identical inputs.
        let a = origin(&Url::parse("file:///x").unwrap());
        let b = origin(&Url::parse("file:///x").unwrap());
        assert_ne!(a, b);
    }

    #[test]
    fn origin_serializes_with_default_port_elided() {
        let url = Url::parse("https://example.com/").unwrap();
        assert_eq!(origin(&url).serialize(), "https://example.com");
        let url = Url::parse("http://example.com:8080/").unwrap();
        assert_eq!(origin(&url).serialize(), "http://example.com:8080");
        let url = Url::parse("mailto:a@b.c").unwrap();
        assert_eq!(origin(&url).serialize(), "null");
    }
}
