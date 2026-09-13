//! Chunked transfer decoding (RFC 9112 §7.1), written here — per §5.2.3 it
//! is "40 lines and examinable". Grammar: `chunk-size [ ; ext ] CRLF
//! chunk-data CRLF`, terminated by a zero chunk and an optional trailer
//! section. Bare-LF terminators inside chunked framing are protocol errors
//! (the RFC is strict; §5.2's bare-LF tolerance applies to header blocks
//! only — recorded decision, PROGRESS.md).

use crate::error::NetError;

/// Decodes one chunked body from `input` (a growing read-ahead buffer).
///
/// Returns `Ok(None)` when the input ends inside an incomplete frame — the
/// caller reads more bytes and retries — and `Err` for genuinely invalid
/// framing (bad hex, bare LF, misaligned chunk-data CRLF).
///
/// On success: the decoded bytes plus the number of input bytes consumed,
/// so a keep-alive connection can locate the next response.
///
/// # Errors
/// [`NetError::Protocol`] for malformed framing.
pub fn decode(input: &[u8]) -> Result<Option<(Vec<u8>, usize)>, NetError> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    loop {
        let Some(line_end) = find_crlf(input, pos)? else {
            return Ok(None);
        };
        let size_line = std::str::from_utf8(&input[pos..line_end])
            .map_err(|_| NetError::Protocol("chunked: non-ASCII size line"))?;
        let size_hex = size_line.split(';').next().unwrap_or("").trim();
        let size = usize::from_str_radix(size_hex, 16)
            .map_err(|_| NetError::Protocol("chunked: bad chunk size"))?;
        pos = line_end + 2;
        if size == 0 {
            // Trailer section: header lines until an empty line.
            loop {
                let Some(end) = find_crlf(input, pos)? else {
                    return Ok(None);
                };
                let line_empty = end == pos;
                pos = end + 2;
                if line_empty {
                    return Ok(Some((out, pos)));
                }
            }
        }
        if pos + size > input.len() {
            return Ok(None); // chunk data not fully arrived yet
        }
        out.extend_from_slice(&input[pos..pos + size]);
        pos += size;
        if pos + 2 > input.len() {
            return Ok(None); // chunk-data terminator not arrived yet
        }
        if &input[pos..pos + 2] != b"\r\n" {
            return Err(NetError::Protocol("chunked: missing chunk-data CRLF"));
        }
        pos += 2;
    }
}

/// Scans for a CRLF at or after `from`.
///
/// `Ok(None)` when no complete CRLF exists yet; `Err` when a bare LF is
/// encountered first (invalid inside chunked framing) or a CR is followed
/// by something other than LF.
fn find_crlf(input: &[u8], from: usize) -> Result<Option<usize>, NetError> {
    let mut i = from;
    while i < input.len() {
        if input[i] == b'\n' {
            return Err(NetError::Protocol("chunked: bare LF in framing"));
        }
        if input[i] == b'\r' {
            return match input.get(i + 1) {
                Some(b'\n') => Ok(Some(i)),
                Some(_) => Err(NetError::Protocol("chunked: bare CR in framing")),
                None => Ok(None), // CR at the end: wait for its pair
            };
        }
        i += 1;
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_single_and_multiple_chunks() {
        let (body, used) = decode(b"5\r\nhello\r\n0\r\n\r\n").unwrap().unwrap();
        assert_eq!(body, b"hello".to_vec());
        assert_eq!(used, 15);

        let (body, used) = decode(b"3\r\nabc\r\n2\r\nde\r\n0\r\n\r\n")
            .unwrap()
            .unwrap();
        assert_eq!(body, b"abcde".to_vec());
        assert_eq!(used, 20);
    }

    #[test]
    fn skips_extensions_and_trailers() {
        let input = b"4;name=value\r\nabcd\r\n0\r\nX-Trailer: t\r\n\r\nrest".to_vec();
        let (body, used) = decode(&input).unwrap().unwrap();
        assert_eq!(body, b"abcd".to_vec());
        assert_eq!(used, input.len() - 4);
    }

    #[test]
    fn incomplete_input_is_not_an_error() {
        // Every truncation point reports "need more bytes", never failure.
        assert_eq!(decode(b""), Ok(None));
        assert_eq!(decode(b"5\r"), Ok(None));
        assert_eq!(decode(b"5\r\nhel"), Ok(None));
        assert_eq!(decode(b"5\r\nhello\r\n0\r\n"), Ok(None));
        // A trailing CR alone is incomplete, not an error.
        assert_eq!(decode(b"5\r\nhello\r"), Ok(None));
    }

    #[test]
    fn protocol_errors_on_garbage() {
        assert_eq!(
            decode(b"zz\r\nabc\r\n0\r\n\r\n"),
            Err(NetError::Protocol("chunked: bad chunk size"))
        );
        assert_eq!(
            decode(b"5\nhello\r\n0\r\n\r\n"),
            Err(NetError::Protocol("chunked: bare LF in framing"))
        );
        assert_eq!(
            decode(b"5\rXhello\r\n0\r\n\r\n"),
            Err(NetError::Protocol("chunked: bare CR in framing"))
        );
    }
}
