//! Requests and responses (§5.2): the owned HTTP/1.1 message model.

use crate::error::NetError;
use crate::headers::HeaderMap;
use aurora_url::Url;

/// HTTP methods the loader needs in M1 (fetch extends this, §6.9 item 20).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
}

impl Method {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
        }
    }
}

/// Whether redirects are followed (§5.2.4; `manual`/`error` arrive with the
/// fetch API, §5.15.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RedirectPolicy {
    Follow,
    Error,
}

/// An HTTP request (§5.2's `Request`, minus the M1-irrelevant fetch fields).
#[derive(Clone, Debug)]
pub struct Request {
    pub method: Method,
    pub url: Url,
    pub headers: HeaderMap,
    pub redirect: RedirectPolicy,
}

impl Request {
    #[must_use]
    pub fn get(url: Url) -> Self {
        Self {
            method: Method::Get,
            url,
            headers: HeaderMap::new(),
            redirect: RedirectPolicy::Follow,
        }
    }

    /// Serializes the request head (§5.2.2): request line, mandatory Host,
    /// User-Agent/Accept/Accept-Language/Accept-Encoding/Connection defaults.
    /// `path_and_query` comes from the URL with the fragment stripped
    /// (§6.9 item 2).
    #[must_use]
    pub fn encode_head(&self, path_and_query: &str) -> String {
        let mut head = String::with_capacity(256);
        head.push_str(self.method.as_str());
        head.push(' ');
        head.push_str(path_and_query);
        head.push_str(" HTTP/1.1\r\n");
        push_header(&mut head, "Host", &host_header_value(&self.url));
        push_header(&mut head, "User-Agent", USER_AGENT);
        push_header(&mut head, "Accept", "*/*");
        push_header(&mut head, "Accept-Language", "en-US,en;q=0.9");
        push_header(&mut head, "Accept-Encoding", "gzip, deflate");
        push_header(&mut head, "Connection", "keep-alive");
        for (name, value) in self.headers.iter() {
            push_header(&mut head, name, value);
        }
        head.push_str("\r\n");
        head
    }
}

/// The `User-Agent` of this browser (§5.2.2).
pub const USER_AGENT: &str = "Aurora/0.1 (+engine project)";

fn push_header(head: &mut String, name: &str, value: &str) {
    head.push_str(name);
    head.push_str(": ");
    head.push_str(value);
    head.push_str("\r\n");
}

/// Host header: authority without userinfo, default port elided (RFC 9112
/// §3.2).
#[must_use]
pub fn host_header_value(url: &Url) -> String {
    match (url.host(), url.port()) {
        (Some(host), Some(port)) => format!("{}:{port}", host.serialize()),
        (Some(host), None) => host.serialize(),
        (None, _) => String::new(),
    }
}

/// A parsed response head; the body is read separately through
/// [`Framing`](crate::body::Framing).
#[derive(Clone, Debug)]
pub struct ResponseHead {
    pub status: u16,
    pub reason: String,
    pub headers: HeaderMap,
}

/// Parses a response head from a buffered byte slice.
///
/// Returns the head and the number of bytes consumed (head including its
/// terminator blank line). Line endings: CRLF per RFC 9112, with a bare LF
/// accepted for robustness — the recorded §5.2.8 decision (PROGRESS.md).
/// obs-fold continuation lines are a protocol error (RFC 9112 §5.2).
///
/// # Errors
/// [`NetError::Protocol`] on malformed status lines, invalid header lines,
/// or obs-fold.
pub fn parse_head(input: &[u8]) -> Result<(ResponseHead, usize), NetError> {
    let head_end = find_blank_line(input).ok_or(NetError::Protocol("no header terminator"))?;
    let head_text = std::str::from_utf8(&input[..head_end])
        .map_err(|_| NetError::Protocol("head is not UTF-8"))?;
    let mut lines = head_text
        .split('\n')
        .map(|l| l.strip_suffix('\r').unwrap_or(l));

    let status_line = lines.next().unwrap_or("");
    let mut parts = status_line.splitn(3, ' ');
    let version = parts.next().unwrap_or("");
    if version != "HTTP/1.1" && version != "HTTP/1.0" {
        return Err(NetError::Protocol("unsupported HTTP version"));
    }
    let status: u16 = parts
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or(NetError::Protocol("bad status code"))?;
    if !(100..=599).contains(&status) {
        return Err(NetError::Protocol("status out of range"));
    }
    let reason = parts.next().unwrap_or("").to_owned();

    let mut headers = HeaderMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            return Err(NetError::Protocol("obs-fold continuation line"));
        }
        let (name, value) = line
            .split_once(':')
            .ok_or(NetError::Protocol("bad header line"))?;
        headers
            .append(name.trim_end(), value.trim())
            .map_err(|_| NetError::Protocol("invalid header"))?;
    }
    let consumed = head_end + 2; // past the blank line's two bytes
    Ok((
        ResponseHead {
            status,
            reason,
            headers,
        },
        consumed,
    ))
}

/// Finds the empty line ending the header block; returns the offset of the
/// blank line's first byte (the position AFTER the last header's newline).
fn find_blank_line(input: &[u8]) -> Option<usize> {
    let mut i = 0usize;
    while i + 1 < input.len() {
        if input[i] == b'\n' && input[i + 1] == b'\n' {
            return Some(i + 1);
        }
        if i + 3 < input.len() && &input[i..i + 4] == b"\r\n\r\n" {
            return Some(i + 2);
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_request_head_with_defaults() {
        let url = Url::parse("http://example.com:8080/a/b?q#frag").unwrap();
        let request = Request::get(url);
        let head = request.encode_head("/a/b?q");
        let lines: Vec<&str> = head.lines().collect();
        assert_eq!(lines[0], "GET /a/b?q HTTP/1.1");
        assert!(lines.contains(&"Host: example.com:8080"));
        assert!(lines.contains(&"User-Agent: Aurora/0.1 (+engine project)"));
        assert!(lines.contains(&"Accept-Encoding: gzip, deflate"));
        assert!(lines.contains(&"Connection: keep-alive"));
        // Fragment stripped on the wire.
        assert!(!head.contains("frag"));
    }

    #[test]
    fn host_header_elides_default_port() {
        let url = Url::parse("https://example.com/x").unwrap();
        assert_eq!(host_header_value(&url), "example.com");
        let url = Url::parse("https://example.com:8443/x").unwrap();
        assert_eq!(host_header_value(&url), "example.com:8443");
    }

    #[test]
    fn parses_head_crlf_and_bare_lf() {
        let input = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<body>";
        let (head, used) = parse_head(input).unwrap();
        assert_eq!(head.status, 200);
        assert_eq!(head.reason, "OK");
        assert_eq!(head.headers.get("content-type"), Some("text/html"));
        assert_eq!(used, input.len() - 6);

        let input = b"HTTP/1.1 404 Not Found\nServer: t\n\nx";
        let (head, _) = parse_head(input).unwrap();
        assert_eq!(head.status, 404);
    }

    #[test]
    fn rejects_obs_fold_and_bad_versions() {
        let folded = b"HTTP/1.1 200 OK\r\nX-A: v\r\n  continued\r\n\r\n";
        assert!(matches!(
            parse_head(folded),
            Err(NetError::Protocol("obs-fold continuation line"))
        ));
        let bad = b"HTTP/2 200 OK\r\n\r\n";
        assert!(matches!(
            parse_head(bad),
            Err(NetError::Protocol("unsupported HTTP version"))
        ));
    }
}
