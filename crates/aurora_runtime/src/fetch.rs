//! The fetch entry point of the engine (§4.3: the runtime owns the
//! "fetch-API"; §5.15.5 grows the web-facing API on top of this).
//!
//! Scheme dispatch per §6.9 item 1: `http`/`https` through the network
//! stack, `file`, `data`, and `about:blank` locally; anything else is
//! `NetError::UnsupportedScheme` (the future error page, §6.9 item 1).

use aurora_encoding::{Encoding, decode, encoding_for_label};
use aurora_net::{CancelToken, Method, NetError, Request};
use aurora_url::Url;

/// The outcome of one successful fetch, scheme-independent.
#[derive(Clone, Debug)]
pub struct FetchOutcome {
    pub status: u16,
    pub reason: String,
    /// Header (name, value) pairs, names lowercased.
    pub headers: Vec<(String, String)>,
    /// The final URL after redirects (absolute, serialized).
    pub final_url: String,
    /// The redirect chain, oldest first (§5.2.4).
    pub redirected_from: Vec<String>,
    /// The raw (content-decoded) body bytes.
    pub body: Vec<u8>,
    /// The charset label from Content-Type, if any, for text decoding.
    pub charset: Option<String>,
}

impl FetchOutcome {
    /// Decodes the body as text using the Content-Type charset when
    /// recognized, UTF-8 otherwise (§7.3: M1 text extraction is "print
    /// decoded body" — no HTML semantics).
    #[must_use]
    pub fn text(&self) -> String {
        let encoding = self
            .charset
            .as_deref()
            .and_then(encoding_for_label)
            .unwrap_or(Encoding::Utf8);
        decode(&self.body, encoding)
    }
}

/// Fetches `url` (absolute) and returns the outcome.
///
/// # Errors
/// [`NetError`] for URL failures, unsupported schemes, and network errors.
pub fn fetch(url: &str, cancel: &CancelToken) -> Result<FetchOutcome, NetError> {
    let parsed = Url::parse(url).map_err(|_| NetError::Protocol("invalid URL"))?;
    match parsed.scheme() {
        "http" | "https" => net_fetch(parsed, cancel),
        "file" => file_fetch(&parsed),
        "data" => data_fetch(&parsed),
        "about" if parsed.path() == "blank" => Ok(empty_outcome(&parsed)),
        _ => Err(NetError::UnsupportedScheme),
    }
}

fn net_fetch(url: Url, cancel: &CancelToken) -> Result<FetchOutcome, NetError> {
    let response = aurora_net::fetch_once(&Request::get(url), cancel)?;
    let headers = response
        .headers
        .iter()
        .map(|(name, value)| (name.to_owned(), value.to_owned()))
        .collect();
    let charset = response
        .headers
        .get("content-type")
        .and_then(|ct| ct.split(';').nth(1))
        .and_then(|param| param.split_once('='))
        .filter(|(name, _)| name.trim().eq_ignore_ascii_case("charset"))
        .map(|(_, value)| value.trim().trim_matches('"').to_owned());
    Ok(FetchOutcome {
        status: response.status,
        reason: response.reason,
        headers,
        final_url: response.url.serialize(false),
        redirected_from: response
            .redirected_from
            .iter()
            .map(|url| url.serialize(false))
            .collect(),
        body: response.body,
        charset,
    })
}

fn file_fetch(url: &Url) -> Result<FetchOutcome, NetError> {
    // file://host/path — only the empty host (or localhost) is local.
    let host_ok = match url.host() {
        Some(aurora_url::Host::Domain(domain)) => domain.is_empty() || domain == "localhost",
        _ => false,
    };
    if !host_ok {
        return Err(NetError::UnsupportedScheme);
    }
    let path = {
        let decoded = aurora_url::percent_decode(&url.path());
        String::from_utf8(decoded).map_err(|_| NetError::Protocol("file path is not UTF-8"))?
    };
    let body = std::fs::read(&path).map_err(|_| NetError::Protocol("file not readable"))?;
    Ok(FetchOutcome {
        status: 200,
        reason: "OK".into(),
        headers: Vec::new(),
        final_url: url.serialize(false),
        redirected_from: Vec::new(),
        body,
        charset: None,
    })
}

fn data_fetch(url: &Url) -> Result<FetchOutcome, NetError> {
    // data:[mediatype][;base64],payload (RFC 2397; §6.9 item 1).
    let opaque = url.path();
    let Some((metadata, payload)) = opaque.split_once(',') else {
        return Err(NetError::Protocol("data URL without comma"));
    };
    let (mediatype, is_base64) = match metadata.strip_suffix(";base64") {
        Some(rest) => (rest, true),
        None => (metadata, false),
    };
    let mediatype = if mediatype.is_empty() {
        "text/plain;charset=US-ASCII"
    } else {
        mediatype
    };
    let body = if is_base64 {
        base64_decode(&payload.replace(['\n', '\r', ' ', '\t'], ""))
            .ok_or(NetError::Protocol("bad base64 data URL"))?
    } else {
        aurora_url::percent_decode(payload)
    };
    let charset = mediatype
        .split(';')
        .nth(1)
        .and_then(|param| param.split_once('='))
        .filter(|(name, _)| name.trim().eq_ignore_ascii_case("charset"))
        .map(|(_, value)| value.trim().to_owned());
    Ok(FetchOutcome {
        status: 200,
        reason: "OK".into(),
        headers: Vec::new(),
        final_url: url.serialize(false),
        redirected_from: Vec::new(),
        body,
        charset,
    })
}

fn empty_outcome(url: &Url) -> FetchOutcome {
    FetchOutcome {
        status: 200,
        reason: "OK".into(),
        headers: Vec::new(),
        final_url: url.serialize(false),
        redirected_from: Vec::new(),
        body: Vec::new(),
        charset: None,
    }
}

/// Base64 decoding (RFC 4648, no padding requirements enforced) — owned
/// code; no approved base64 crate exists on the §3.3 list.
fn base64_decode(input: &str) -> Option<Vec<u8>> {
    fn value_of(byte: u8) -> Option<u8> {
        match byte {
            b'A'..=b'Z' => Some(byte - b'A'),
            b'a'..=b'z' => Some(byte - b'a' + 26),
            b'0'..=b'9' => Some(byte - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let bytes: Vec<u8> = input
        .bytes()
        .filter(|b| *b != b'=')
        .map(value_of)
        .collect::<Option<Vec<_>>>()?;
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    for chunk in bytes.chunks(4) {
        let mut word = 0u32;
        for (shift, value) in chunk.iter().enumerate() {
            word |= u32::from(*value) << (18 - 6 * shift);
        }
        // A 2- or 3-character tail carries only its full bytes.
        let usable = match chunk.len() {
            4 => 3,
            3 => 2,
            2 => 1,
            _ => return None,
        };
        for idx in 0..usable {
            out.push(((word >> (16 - 8 * idx)) & 0xFF) as u8);
        }
    }
    Some(out)
}

const _: Method = Method::Get; // keep the Method import used for future POST paths
