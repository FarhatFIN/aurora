//! Chunked transfer decoding (RFC 9112 §7.1), written here — per §5.2.3 it
//! is "40 lines and examinable". Grammar: `chunk-size [ ; ext ] CRLF
//! chunk-data CRLF`, terminated by a zero chunk and an optional trailer
//! section. Bare-LF terminators inside chunked framing are protocol errors
//! (the RFC is strict; §5.2's bare-LF tolerance applies to header blocks
//! only — recorded decision, PROGRESS.md).

use crate::error::NetError;

/// Decodes one chunked body from `input` (already fully buffered).
///
/// Returns the decoded bytes and the number of input bytes consumed, so a
/// keep-alive connection can locate the next response.
///
/// # Errors
/// [`NetError::Protocol`] for every malformed framing case (bad hex,
/// missing CRLF, truncated input, chunk-size overflow).
pub fn decode(input: &[u8]) -> Result<(Vec<u8>, usize), NetError> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    loop {
        let line_end = find_crlf(input, pos).ok_or(NetError::Protocol("chunked: missing CRLF"))?;
        let size_line = std::str::from_utf8(&input[pos..line_end])
            .map_err(|_| NetError::Protocol("chunked: non-ASCII size line"))?;
        let size_hex = size_line.split(';').next().unwrap_or("").trim();
        let size = usize::from_str_radix(size_hex, 16)
            .map_err(|_| NetError::Protocol("chunked: bad chunk size"))?;
        pos = line_end + 2;
        if size == 0 {
            // Trailer section: header lines until an empty line.
            loop {
                let end = find_crlf(input, pos)
                    .ok_or(NetError::Protocol("chunked: missing trailer terminator"))?;
                let line_empty = end == pos;
                pos = end + 2;
                if line_empty {
                    return Ok((out, pos));
                }
            }
        }
        if pos + size + 2 > input.len() {
            return Err(NetError::Protocol("chunked: truncated chunk data"));
        }
        out.extend_from_slice(&input[pos..pos + size]);
        pos += size;
        if &input[pos..pos + 2] != b"\r\n" {
            return Err(NetError::Protocol("chunked: missing chunk-data CRLF"));
        }
        pos += 2;
    }
}

/// Finds a CRLF at or after `from`; a bare LF inside the scan window makes
/// the framing invalid (strict per RFC 9112 §7.1).
fn find_crlf(input: &[u8], from: usize) -> Option<usize> {
    let mut i = from;
    while i + 1 < input.len() {
        if input[i] == b'\n' {
            return None; // bare LF inside chunked framing
        }
        if input[i] == b'\r' && input[i + 1] == b'\n' {
            return Some(i);
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_single_and_multiple_chunks() {
        let (body, used) = decode(b"5\r\nhello\r\n0\r\n\r\n").unwrap();
        assert_eq!(body, b"hello".to_vec());
        assert_eq!(used, 15);

        let (body, used) = decode(b"3\r\nabc\r\n2\r\nde\r\n0\r\n\r\n").unwrap();
        assert_eq!(body, b"abcde".to_vec());
        assert_eq!(used, 20);
    }

    #[test]
    fn skips_extensions_and_trailers() {
        let input = b"4;name=value\r\nabcd\r\n0\r\nX-Trailer: t\r\n\r\nrest".to_vec();
        let (body, used) = decode(&input).unwrap();
        assert_eq!(body, b"abcd".to_vec());
        assert_eq!(used, input.len() - 4);
    }

    #[test]
    fn protocol_errors_on_garbage() {
        assert_eq!(
            decode(b"zz\r\nabc\r\n0\r\n\r\n"),
            Err(NetError::Protocol("chunked: bad chunk size"))
        );
        assert_eq!(
            decode(b"5\r\nhello"),
            Err(NetError::Protocol("chunked: truncated chunk data"))
        );
        assert_eq!(
            decode(b"5\nhello\r\n0\r\n\r\n"),
            Err(NetError::Protocol("chunked: missing CRLF"))
        );
        assert_eq!(
            decode(b"5\r\nhel"),
            Err(NetError::Protocol("chunked: truncated chunk data"))
        );
    }
}
