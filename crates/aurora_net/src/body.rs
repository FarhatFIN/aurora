//! Body framing and reading (§5.2.3): content-length, chunked, until-close
//! as last resort; content-coding decode (gzip/deflate via the approved
//! `flate2` until the owned inflate lands, §6.9 item 7).

use crate::chunked;
use crate::error::{CancelToken, NetError};
use crate::headers::HeaderMap;
use std::io::{Read, Write};

/// How the response body is framed (RFC 9112 §6).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Framing {
    /// `Content-Length: n` bytes follow the head.
    Length(u64),
    /// Chunked transfer coding; the connection stays usable afterwards.
    Chunked,
    /// Until connection close (last resort; the connection is not reusable).
    UntilClose,
    /// No body (HEAD responses; 1xx/204/304).
    None,
}

/// Determines the framing from method, status, and headers (RFC 9112 §6.3).
#[must_use]
pub fn framing_for(method: crate::message::Method, status: u16, headers: &HeaderMap) -> Framing {
    if method == crate::message::Method::Head
        || status == 204
        || status == 304
        || (100..200).contains(&status)
    {
        return Framing::None;
    }
    if headers
        .get("transfer-encoding")
        .is_some_and(|te| te.contains("chunked"))
    {
        // Chunked + content-length together is an error (§5.2 pitfalls);
        // framing follows Transfer-Encoding per RFC 9112 §6.1.
        return Framing::Chunked;
    }
    if let Some(len) = headers
        .get("content-length")
        .and_then(|v| v.trim().parse().ok())
    {
        return Framing::Length(len);
    }
    Framing::UntilClose
}

/// A readable stream the body comes from (TCP or TLS).
pub(crate) trait Stream: Read {
    /// Reads with the idle timeout already configured; cancellation is
    /// checked by the caller between reads.
    fn fill(&mut self, buf: &mut [u8]) -> Result<usize, NetError>;
}

pub(crate) struct PlainStream(pub std::net::TcpStream);

impl Read for PlainStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl Stream for PlainStream {
    fn fill(&mut self, buf: &mut [u8]) -> Result<usize, NetError> {
        self.0.read(buf).map_err(|error| io_error(&error))
    }
}

pub(crate) struct TlsStream(pub rustls::StreamOwned<rustls::ClientConnection, std::net::TcpStream>);

impl Read for TlsStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl Stream for TlsStream {
    fn fill(&mut self, buf: &mut [u8]) -> Result<usize, NetError> {
        self.0.read(buf).map_err(|error| io_error(&error))
    }
}

pub(crate) fn io_error(error: &std::io::Error) -> NetError {
    match error.kind() {
        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut => NetError::Timeout,
        std::io::ErrorKind::ConnectionAborted | std::io::ErrorKind::ConnectionReset => {
            NetError::Connect
        }
        _ => NetError::Io,
    }
}

/// Reads the whole body for `framing` from `stream`, which already sits
/// past the response head. `leftover` carries bytes read ahead of the body.
///
/// # Errors
/// [`NetError`] for abort, timeout, and protocol violations.
pub fn read_body(
    stream: &mut dyn Stream,
    framing: Framing,
    leftover: &mut Vec<u8>,
    cancel: &CancelToken,
) -> Result<Vec<u8>, NetError> {
    let mut body = Vec::new();
    match framing {
        Framing::None => Ok(body),
        Framing::Length(total) => {
            let total = usize::try_from(total)
                .map_err(|_| NetError::Protocol("content-length overflow"))?;
            body.extend_from_slice(leftover.drain(..).as_slice());
            while body.len() < total {
                cancel.check()?;
                let want = (total - body.len()).min(16 * 1024);
                let mut buf = vec![0u8; want];
                let read = stream.fill(&mut buf)?;
                if read == 0 {
                    return Err(NetError::Protocol(
                        "connection closed before content-length",
                    ));
                }
                body.extend_from_slice(&buf[..read]);
            }
            // Bytes past the body belong to the next response (keep-alive).
            if body.len() > total {
                leftover.extend_from_slice(&body[total..]);
                body.truncate(total);
            }
            Ok(body)
        }
        Framing::Chunked => {
            // Accumulate until the chunked decoder sees the terminator.
            loop {
                cancel.check()?;
                if let Ok((decoded, used)) = chunked::decode(leftover) {
                    leftover.drain(..used);
                    return Ok(decoded);
                }
                let mut buf = [0u8; 16 * 1024];
                let read = stream.fill(&mut buf)?;
                if read == 0 {
                    return Err(NetError::Protocol("connection closed mid-chunked-body"));
                }
                leftover.extend_from_slice(&buf[..read]);
            }
        }
        Framing::UntilClose => {
            body.extend_from_slice(leftover.drain(..).as_slice());
            loop {
                cancel.check()?;
                let mut buf = [0u8; 16 * 1024];
                let read = stream.fill(&mut buf)?;
                if read == 0 {
                    return Ok(body);
                }
                body.extend_from_slice(&buf[..read]);
            }
        }
    }
}

/// Decodes the `Content-Encoding` of a complete body (gzip, deflate, or
/// identity; `br` is a non-goal — §1.4).
///
/// # Errors
/// [`NetError::Protocol`] when the coding is unknown or the payload is
/// malformed.
pub fn decode_content_coding(encoding: Option<&str>, body: Vec<u8>) -> Result<Vec<u8>, NetError> {
    match encoding
        .map(str::trim)
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        None | Some("" | "identity") => Ok(body),
        Some("gzip") => {
            let mut decoder = flate2::read::MultiGzDecoder::new(body.as_slice());
            let mut out = Vec::new();
            decoder
                .read_to_end(&mut out)
                .map_err(|_| NetError::Protocol("bad gzip body"))?;
            Ok(out)
        }
        Some("deflate") => {
            let mut out = Vec::new();
            // RFC 9110: deflate is zlib-wrapped in practice; tolerate raw
            // streams by retrying (server quirks).
            if flate2::read::ZlibDecoder::new(body.as_slice())
                .read_to_end(&mut out)
                .is_ok()
            {
                return Ok(out);
            }
            out.clear();
            flate2::read::DeflateDecoder::new(body.as_slice())
                .read_to_end(&mut out)
                .map_err(|_| NetError::Protocol("bad deflate body"))?;
            Ok(out)
        }
        Some(_) => Err(NetError::Protocol("unsupported content-coding")),
    }
}

impl Write for PlainStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl Write for TlsStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
