//! Host parsing and serialization (URL Standard §3 "hosts": host parser,
//! ends-in-a-number checker, IPv4/IPv6 parsers and serializers).
//!
//! Steps follow the standard's numbered algorithms one for one; pure
//! validation errors (non-fatal) are not collected yet — the parse-error
//! report arrives with `DevTools` (WBS §6.2). Spec: WHATWG URL Standard §3.

use crate::percent::{self, OPAQUE_HOST};

/// The host of a URL: a domain (possibly the empty host for `file:`), an
/// IPv4 or IPv6 address, or an opaque host (non-special schemes only).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Host {
    /// A domain, lowercased ASCII (IDNA placeholder: see PROGRESS.md).
    Domain(String),
    /// An IPv4 address.
    Ipv4([u8; 4]),
    /// An IPv6 address, as its 8 16-bit pieces.
    Ipv6([u16; 8]),
    /// An opaque host for non-special schemes, percent-encoded.
    Opaque(String),
}

/// Host parse failures (URL Standard §3 failure causes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum HostError {
    /// A forbidden host code point in an opaque host.
    InvalidHostCharacter,
    /// A forbidden domain code point in a domain.
    InvalidDomainCharacter,
    /// A special scheme requires a non-empty host.
    EmptyHost,
    /// The IPv4 parser rejected a host that ends in a number.
    InvalidIpv4,
    /// The IPv6 parser rejected bracketed input.
    InvalidIpv6,
    /// Bracketed input did not end with `]`.
    Ipv6Unclosed,
    /// IDNA (domain-to-ASCII) unsupported: non-ASCII domains are a
    /// recorded placeholder (PROGRESS.md, standing placeholders).
    IdnaUnsupported,
}

/// The forbidden host code points (URL Standard §2.1).
fn is_forbidden_host(cp: char) -> bool {
    matches!(
        cp,
        '\0' | '\t'
            | '\n'
            | '\r'
            | ' '
            | '#'
            | '/'
            | ':'
            | '<'
            | '>'
            | '?'
            | '@'
            | '['
            | '\\'
            | ']'
            | '^'
            | '|'
    )
}

/// The forbidden domain code points: forbidden host code points plus C0
/// controls, `%`, and DEL (URL Standard §2.1).
fn is_forbidden_domain(cp: char) -> bool {
    is_forbidden_host(cp) || cp.is_control() || cp == '%' || cp == '\u{007F}'
}

/// The host parser (URL Standard §3 "host parser", steps 1–8).
///
/// `is_opaque` is true for non-special schemes.
pub fn parse(input: &str, is_opaque: bool) -> Result<Host, HostError> {
    if let Some(rest) = input.strip_prefix('[') {
        // Step 1: bracketed input goes to the IPv6 parser.
        let inner = rest.strip_suffix(']').ok_or(HostError::Ipv6Unclosed)?;
        return parse_ipv6(inner).map(Host::Ipv6);
    }
    if is_opaque {
        return parse_opaque_host(input);
    }
    parse_domain(input)
}

/// Opaque-host parser (URL Standard §3): reject forbidden host code points,
/// then percent-encode with the opaque-host set.
fn parse_opaque_host(input: &str) -> Result<Host, HostError> {
    for cp in input.chars() {
        if is_forbidden_host(cp) {
            return Err(HostError::InvalidHostCharacter);
        }
    }
    Ok(Host::Opaque(percent::percent_encode(input, &OPAQUE_HOST)))
}

/// Domain path of the host parser: percent-decode, ASCII-ify, IPv4 dispatch.
fn parse_domain(input: &str) -> Result<Host, HostError> {
    // IDNA placeholder: full UTS-46 domain-to-ASCII is not implemented; ASCII
    // input passes through lowercased, non-ASCII fails (PROGRESS.md).
    if input.is_empty() {
        // The host parser asserts non-empty input on the domain path; the
        // URL parser guarantees it. Defend anyway (no panics, §4.6).
        return Err(HostError::EmptyHost);
    }
    let decoded = String::from_utf8(percent::percent_decode(input))
        .map_err(|_| HostError::InvalidDomainCharacter)?;
    if !decoded.is_ascii() {
        return Err(HostError::IdnaUnsupported);
    }
    let ascii_domain = decoded.to_ascii_lowercase();
    for cp in ascii_domain.chars() {
        if is_forbidden_domain(cp) {
            return Err(HostError::InvalidDomainCharacter);
        }
    }
    if ends_in_a_number(&ascii_domain) {
        return parse_ipv4(&ascii_domain).map(Host::Ipv4);
    }
    Ok(Host::Domain(ascii_domain))
}

/// The ends-in-a-number checker (URL Standard §3).
fn ends_in_a_number(input: &str) -> bool {
    let mut parts: Vec<&str> = input.split('.').collect();
    if parts.last() == Some(&"") {
        parts.pop();
    }
    let Some(last) = parts.last() else {
        return false;
    };
    if !last.is_empty() && last.bytes().all(|b| b.is_ascii_digit()) {
        return true;
    }
    parse_ipv4_number(last).is_some()
}

/// The IPv4 number parser (URL Standard §3): returns the value, or None on
/// failure. Hex (`0x`) and octal (leading `0`) prefixes shift the radix.
fn parse_ipv4_number(input: &str) -> Option<u64> {
    if input.is_empty() {
        return None;
    }
    let (radix, digits) = if input.len() >= 2 && matches!(&input[..2], "0x" | "0X") {
        (16, &input[2..])
    } else if input.len() >= 2 && input.starts_with('0') {
        (8, &input[1..])
    } else {
        (10, input)
    };
    if digits.is_empty() {
        return Some(0);
    }
    if !match radix {
        16 => digits.bytes().all(|b| b.is_ascii_hexdigit()),
        8 => digits.bytes().all(|b| (b'0'..=b'7').contains(&b)),
        _ => digits.bytes().all(|b| b.is_ascii_digit()),
    } {
        return None;
    }
    // "Mathematical integer": overflow means failure downstream either way.
    u64::from_str_radix(digits, radix).ok()
}

/// The IPv4 parser (URL Standard §3). Fewer than four parts is a validation
/// error only — `1.2.3` parses as `1.2.0.3` (the spec text is explicit).
fn parse_ipv4(input: &str) -> Result<[u8; 4], HostError> {
    let mut parts: Vec<&str> = input.split('.').collect();
    if parts.last() == Some(&"") && parts.len() > 1 {
        parts.pop();
    }
    if parts.len() > 4 {
        return Err(HostError::InvalidIpv4);
    }
    let mut numbers: Vec<u64> = Vec::with_capacity(parts.len());
    for part in &parts {
        let value = parse_ipv4_number(part).ok_or(HostError::InvalidIpv4)?;
        numbers.push(value);
    }
    // "If any but the last item in numbers is greater than 255, return failure."
    if numbers[..numbers.len() - 1].iter().any(|n| *n > 255) {
        return Err(HostError::InvalidIpv4);
    }
    let last = numbers[numbers.len() - 1];
    #[allow(clippy::cast_possible_truncation)] // parts count is 1..=4 (checked above)
    if last >= 256u64.pow(u32::try_from(5 - numbers.len()).unwrap_or(5)) {
        return Err(HostError::InvalidIpv4);
    }
    let mut ipv4 = last;
    // The standard iterates the remaining numbers in order, scaling by
    // 256^(3 − counter) — the first number is the most significant.
    for (counter, n) in numbers[..numbers.len() - 1].iter().enumerate() {
        ipv4 += n * 256u64.pow(u32::try_from(3 - counter).unwrap_or(0));
    }
    // The total is bounded by 256^4 − 1 (checked above), so the masked
    // byte extracts are exact narrowing conversions (§9.8).
    Ok([
        ((ipv4 >> 24) & 0xFF) as u8,
        ((ipv4 >> 16) & 0xFF) as u8,
        ((ipv4 >> 8) & 0xFF) as u8,
        (ipv4 & 0xFF) as u8,
    ])
}

/// The IPv6 parser (URL Standard §3), on the input inside the brackets.
#[allow(clippy::too_many_lines)] // one translation of one spec algorithm; splitting it would scatter the numbered steps
pub fn parse_ipv6(input: &str) -> Result<[u16; 8], HostError> {
    let bytes = input.as_bytes();
    let mut address = [0u16; 8];
    let mut piece_pointer = 0usize;
    let mut compress: Option<usize> = None;
    let mut i = 0usize;

    if bytes.first() == Some(&b':') {
        if bytes.get(1) != Some(&b':') {
            return Err(HostError::InvalidIpv6);
        }
        i = 2;
        piece_pointer += 1;
        compress = Some(piece_pointer);
    }
    while i < bytes.len() {
        if piece_pointer == 8 {
            return Err(HostError::InvalidIpv6);
        }
        if bytes[i] == b':' {
            if compress.is_some() {
                return Err(HostError::InvalidIpv6);
            }
            i += 1;
            piece_pointer += 1;
            compress = Some(piece_pointer);
            continue;
        }
        let mut value: u32 = 0;
        let mut length = 0usize;
        while length < 4 && i < bytes.len() && bytes[i].is_ascii_hexdigit() {
            let hex = u32::from(bytes[i].to_ascii_uppercase());
            value = value * 0x10 + (hex - if bytes[i].is_ascii_digit() { 48 } else { 55 });
            i += 1;
            length += 1;
        }
        if i < bytes.len() && bytes[i] == b'.' {
            if length == 0 {
                return Err(HostError::InvalidIpv6);
            }
            i -= length;
            if piece_pointer > 6 {
                return Err(HostError::InvalidIpv6);
            }
            // IPv4-in-IPv6: four dot-separated decimal numbers fill two pieces.
            let mut numbers_seen = 0usize;
            while i < bytes.len() {
                let mut ipv4_piece: Option<u8> = None;
                if numbers_seen > 0 {
                    if bytes[i] == b'.' && numbers_seen < 4 {
                        i += 1;
                    } else {
                        return Err(HostError::InvalidIpv6);
                    }
                }
                if i >= bytes.len() || !bytes[i].is_ascii_digit() {
                    return Err(HostError::InvalidIpv6);
                }
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    let digit = u32::from(bytes[i] - b'0');
                    match ipv4_piece {
                        None => ipv4_piece = Some(u8::try_from(digit).unwrap_or(0)),
                        Some(0) => return Err(HostError::InvalidIpv6),
                        Some(current) => {
                            let next = u32::from(current) * 10 + digit;
                            if next > 255 {
                                return Err(HostError::InvalidIpv6);
                            }
                            // Bounded by the check above.
                            ipv4_piece = Some(u8::try_from(next).unwrap_or(255));
                        }
                    }
                    i += 1;
                }
                let piece = &mut address[piece_pointer];
                *piece = (*piece) * 0x100 + u16::from(ipv4_piece.unwrap_or(0));
                numbers_seen += 1;
                if numbers_seen == 2 || numbers_seen == 4 {
                    piece_pointer += 1;
                }
                if numbers_seen == 4 {
                    break;
                }
            }
            if numbers_seen != 4 {
                return Err(HostError::InvalidIpv6);
            }
            break;
        }
        if i < bytes.len() && bytes[i] == b':' {
            i += 1;
            if i >= bytes.len() {
                return Err(HostError::InvalidIpv6);
            }
        } else if i < bytes.len() {
            return Err(HostError::InvalidIpv6);
        }
        // value comes from at most four hex digits, so it fits u16 exactly.
        address[piece_pointer] = u16::try_from(value).map_err(|_| HostError::InvalidIpv6)?;
        piece_pointer += 1;
    }

    if let Some(compress_at) = compress {
        let mut swaps = piece_pointer - compress_at;
        let mut dest = 7usize;
        while dest != 0 && swaps > 0 {
            address.swap(dest, compress_at + swaps - 1);
            dest -= 1;
            swaps -= 1;
        }
    } else if piece_pointer != 8 {
        return Err(HostError::InvalidIpv6);
    }
    Ok(address)
}

/// The IPv4 serializer (URL Standard §3): dotted decimal.
#[must_use]
pub fn serialize_ipv4(address: [u8; 4]) -> String {
    format!(
        "{}.{}.{}.{}",
        address[0], address[1], address[2], address[3]
    )
}

/// Finds the IPv6 address compressed piece index (URL Standard §3): the
/// first longest zero run longer than one piece, if any.
fn ipv6_compressed_piece_index(pieces: &[u16; 8]) -> Option<usize> {
    let mut longest_index: Option<usize> = None;
    let mut longest_size = 1usize;
    let mut found_index: Option<usize> = None;
    let mut found_size = 0usize;
    for (piece_index, piece) in pieces.iter().enumerate() {
        if *piece != 0 {
            if found_size > longest_size {
                longest_index = found_index;
                longest_size = found_size;
            }
            found_index = None;
            found_size = 0;
        } else {
            if found_index.is_none() {
                found_index = Some(piece_index);
            }
            found_size += 1;
            if found_size > longest_size {
                return found_index;
            }
        }
    }
    longest_index
}

/// The IPv6 serializer (URL Standard §3): longest zero run as `::`,
/// lowercase hex pieces. (The current standard text has no IPv4-mapped
/// special case; `::ffff:127.0.0.1` serializes as `::ffff:7f00:1`.)
#[must_use]
pub fn serialize_ipv6(pieces: &[u16; 8]) -> String {
    let compress = ipv6_compressed_piece_index(pieces);
    let mut output = String::new();
    let mut ignore0 = false;
    for (piece_index, piece) in pieces.iter().enumerate() {
        if ignore0 && *piece == 0 {
            continue;
        }
        if ignore0 {
            ignore0 = false;
        }
        if compress == Some(piece_index) {
            output.push_str(if piece_index == 0 { "::" } else { ":" });
            ignore0 = true;
            continue;
        }
        // Writing to a String cannot fail (std::fmt::Write).
        let _ = core::fmt::Write::write_fmt(&mut output, format_args!("{piece:x}"));
        if piece_index != 7 {
            output.push(':');
        }
    }
    output
}

/// The host serializer (URL Standard §3).
#[must_use]
pub fn serialize(host: &Host) -> String {
    match host {
        Host::Ipv4(address) => serialize_ipv4(*address),
        Host::Ipv6(pieces) => format!("[{}]", serialize_ipv6(pieces)),
        // Domain and opaque hosts are stored already serialized; the empty
        // host serializes to the empty string.
        Host::Domain(domain) => domain.clone(),
        Host::Opaque(opaque) => opaque.clone(),
    }
}

impl Host {
    /// Serializes the host (URL Standard §3 "host serializer").
    #[must_use]
    pub fn serialize(&self) -> String {
        serialize(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4_accepts_decimal_hex_octal_and_trailing_dot() {
        assert_eq!(parse_ipv4("127.0.0.1"), Ok([127, 0, 0, 1]));
        assert_eq!(parse_ipv4("0x7f.1"), Ok([127, 0, 0, 1]));
        assert_eq!(parse_ipv4("0x7f000001"), Ok([127, 0, 0, 1]));
        assert_eq!(parse_ipv4("0177.0.0.1"), Ok([127, 0, 0, 1]));
        assert_eq!(parse_ipv4("1.2.3"), Ok([1, 2, 0, 3]));
        assert_eq!(parse_ipv4("1.1.1.1."), Ok([1, 1, 1, 1]));
        assert_eq!(parse_ipv4("0x"), Ok([0, 0, 0, 0]));
        // "09": octal prefix `0` with the non-octal digit 9 → failure
        // (matches the standard's parse-serialize roundtrip table).
        assert_eq!(parse_ipv4("09"), Err(HostError::InvalidIpv4));
    }

    #[test]
    fn ipv4_rejects_overflow_and_bad_parts() {
        assert_eq!(parse_ipv4("256.1.1.1"), Err(HostError::InvalidIpv4));
        assert_eq!(parse_ipv4("1.1.1.1.1"), Err(HostError::InvalidIpv4));
        assert_eq!(parse_ipv4("example.255"), Err(HostError::InvalidIpv4));
        assert_eq!(parse_ipv4("1..1"), Err(HostError::InvalidIpv4));
        assert_eq!(parse_ipv4("4294967296"), Err(HostError::InvalidIpv4));
    }

    #[test]
    fn ipv6_parses_compression_and_v4_tail() {
        assert_eq!(parse_ipv6("::1"), Ok([0, 0, 0, 0, 0, 0, 0, 1]));
        assert_eq!(
            parse_ipv6("2001:db8::1"),
            Ok([0x2001, 0x0db8, 0, 0, 0, 0, 0, 1])
        );
        assert_eq!(
            parse_ipv6("::ffff:127.0.0.1"),
            Ok([0, 0, 0, 0, 0, 0xffff, 0x7f00, 1])
        );
        assert_eq!(parse_ipv6("1:2:3:4:5:6:7:8"), Ok([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(parse_ipv6("1::"), Ok([1, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(parse_ipv6("1::2::3"), Err(HostError::InvalidIpv6));
        assert_eq!(parse_ipv6(":1"), Err(HostError::InvalidIpv6));
        assert_eq!(parse_ipv6("::ffff:1.2.3.04"), Err(HostError::InvalidIpv6));
        assert_eq!(parse_ipv6("0:0::1"), Ok([0, 0, 0, 0, 0, 0, 0, 1]));
    }

    #[test]
    fn ipv6_serializes_longest_zero_run() {
        assert_eq!(serialize_ipv6(&[0, 0, 0, 0, 0, 0, 0, 1]), "::1");
        assert_eq!(serialize_ipv6(&[1, 0, 0, 0, 0, 0, 0, 0]), "1::");
        assert_eq!(
            serialize_ipv6(&parse_ipv6("2001:db8::1").unwrap()),
            "2001:db8::1"
        );
        assert_eq!(
            serialize_ipv6(&parse_ipv6("::ffff:127.0.0.1").unwrap()),
            "::ffff:7f00:1"
        );
        // 0:f:0:0:f:f:0:0 → the second zero run compresses (spec note).
        assert_eq!(
            serialize_ipv6(&[0, 0xf, 0, 0, 0xf, 0xf, 0, 0]),
            "0:f::f:f:0:0"
        );
    }

    #[test]
    fn host_parse_dispatches_by_opacity() {
        assert_eq!(
            parse("EXAMPLE.com", false),
            Ok(Host::Domain("example.com".into()))
        );
        assert_eq!(
            parse("EXAMPLE.com", true),
            Ok(Host::Opaque("EXAMPLE.com".into()))
        );
        assert_eq!(
            parse("[::1]", false),
            Ok(Host::Ipv6([0, 0, 0, 0, 0, 0, 0, 1]))
        );
        assert_eq!(parse("a b", false), Err(HostError::InvalidDomainCharacter));
        assert_eq!(parse("a^b", true), Err(HostError::InvalidHostCharacter));
        assert_eq!(parse("münchen", false), Err(HostError::IdnaUnsupported));
    }
}
