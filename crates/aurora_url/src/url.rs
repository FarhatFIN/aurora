//! The URL record and its serializer (URL Standard §4 "URLs", §4.4
//! "URL serializer").

use crate::host::{self, Host};

/// A parsed URL record (URL Standard §4.4). All text fields hold
/// percent-encoded output of the parser — the record is already normalized;
/// nothing is re-encoded at use time (§5.1 invariant).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Url {
    pub(crate) scheme: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) host: Option<Host>,
    pub(crate) port: Option<u16>,
    /// The path: either the segment list (each segment without its
    /// leading `/`) or the opaque path of a cannot-be-a-base URL.
    pub(crate) path: Path,
    pub(crate) query: Option<String>,
    pub(crate) fragment: Option<String>,
}

/// A URL path: the segment list of a hierarchical URL, or the opaque path
/// of a cannot-be-a-base URL (URL Standard §4.1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Path {
    /// Segment list; empty for e.g. `foo://host` with no path.
    Segments(Vec<String>),
    /// Opaque path (`mailto:me@example.com`), percent-encoded (C0 set).
    Opaque(String),
}

impl Url {
    /// The all-empty record the parser starts from.
    #[must_use]
    pub(crate) fn empty_record() -> Self {
        Self {
            scheme: String::new(),
            username: String::new(),
            password: String::new(),
            host: None,
            port: None,
            path: Path::Segments(Vec::new()),
            query: None,
            fragment: None,
        }
    }

    /// The scheme, lowercase.
    #[must_use]
    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    /// The host, or None for a URL with a null host (no authority).
    #[must_use]
    pub fn host(&self) -> Option<&Host> {
        self.host.as_ref()
    }

    /// The port; None when absent or equal to the scheme's default.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// The serialized path: `/a/b` for hierarchical URLs, the opaque path
    /// otherwise. (The §5.1 sketch's `&str` return needs a `String` here
    /// because segments are stored unjoined; the shape is otherwise equal.)
    #[must_use]
    pub fn path(&self) -> String {
        match &self.path {
            Path::Opaque(opaque) => opaque.clone(),
            Path::Segments(segments) => {
                let mut out = String::new();
                for segment in segments {
                    out.push('/');
                    out.push_str(segment);
                }
                out
            }
        }
    }

    /// The query without the `?`, percent-encoded; None when absent.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }

    /// The fragment without the `#`, percent-encoded; None when absent.
    #[must_use]
    pub fn fragment(&self) -> Option<&str> {
        self.fragment.as_deref()
    }

    /// The username (percent-encoded userinfo).
    #[must_use]
    pub fn username(&self) -> &str {
        &self.username
    }

    /// The password (percent-encoded userinfo); empty when absent.
    #[must_use]
    pub fn password(&self) -> &str {
        &self.password
    }

    /// Whether the scheme is special (URL Standard §4.2): http, https, ws,
    /// wss, ftp, or file.
    #[must_use]
    pub fn is_special(&self) -> bool {
        is_special_scheme(&self.scheme)
    }

    /// Whether the URL has an opaque path (a cannot-be-a-base URL).
    #[must_use]
    pub fn cannot_be_a_base(&self) -> bool {
        matches!(self.path, Path::Opaque(_))
    }

    /// The default port of the scheme, if any (URL Standard §4.2 table).
    #[must_use]
    pub fn default_port(&self) -> Option<u16> {
        default_port(&self.scheme)
    }

    /// Serializes the URL (URL Standard §4.4 "URL serializer").
    ///
    /// `exclude_fragment` drops the fragment (the wire form sent to servers,
    /// §6.9 item 2).
    #[must_use]
    pub fn serialize(&self, exclude_fragment: bool) -> String {
        let mut output = String::with_capacity(64);
        output.push_str(&self.scheme);
        output.push(':');
        if let Some(host) = &self.host {
            output.push_str("//");
            // "url includes credentials": username or password non-empty.
            if !self.username.is_empty() || !self.password.is_empty() {
                output.push_str(&self.username);
                if !self.password.is_empty() {
                    output.push(':');
                    output.push_str(&self.password);
                }
                output.push('@');
            }
            output.push_str(&host::serialize(host));
            if let Some(port) = self.port {
                output.push(':');
                output.push_str(&port.to_string());
            }
        } else if let Path::Segments(segments) = &self.path {
            // Host-null fixup: prevent `web+demo:/.//p` from serializing to
            // `web+demo://p` (URL Standard §4.4).
            if segments.len() > 1 && segments[0].is_empty() {
                output.push_str("/.");
            }
        }
        output.push_str(&self.serialize_path());
        if let Some(query) = &self.query {
            output.push('?');
            output.push_str(query);
        }
        if !exclude_fragment && let Some(fragment) = &self.fragment {
            output.push('#');
            output.push_str(fragment);
        }
        output
    }

    /// The URL path serializer (§4.4): segments joined with `/`, or the
    /// opaque path as-is.
    #[must_use]
    pub fn serialize_path(&self) -> String {
        match &self.path {
            Path::Opaque(opaque) => opaque.clone(),
            Path::Segments(segments) => {
                let mut out = String::new();
                for segment in segments {
                    out.push('/');
                    out.push_str(segment);
                }
                out
            }
        }
    }

    /// The segments of a hierarchical path; empty for opaque paths.
    #[must_use]
    pub fn segments(&self) -> &[String] {
        match &self.path {
            Path::Segments(segments) => segments,
            Path::Opaque(_) => &[],
        }
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.serialize(false))
    }
}

/// Whether `scheme` is special (URL Standard §4.2 table).
#[must_use]
pub fn is_special_scheme(scheme: &str) -> bool {
    matches!(scheme, "http" | "https" | "ws" | "wss" | "ftp" | "file")
}

/// The default port of `scheme` per the §4.2 table; None for `file` and
/// non-special schemes.
#[must_use]
pub fn default_port(scheme: &str) -> Option<u16> {
    match scheme {
        "http" | "ws" => Some(80),
        "https" | "wss" => Some(443),
        "ftp" => Some(21),
        _ => None,
    }
}
