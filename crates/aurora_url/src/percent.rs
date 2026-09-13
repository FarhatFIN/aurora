//! Percent-encoding and the WHATWG percent-encode sets (URL Standard §2.3,
//! "percent-encoded bytes").
//!
//! An [`AsciiSet`] is a 128-bit table of ASCII code points to encode. The
//! standard's sets are exposed as constants and are built at compile time;
//! encoding itself is total and allocation-only (no panic paths).

/// A set of ASCII code points to percent-encode (URL Standard §2.3).
///
/// Bytes at or above U+0080 are always encoded regardless of the set, per
/// the standard (encode sets only ever enumerate ASCII).
#[derive(Clone, Copy, Debug)]
pub struct AsciiSet {
    bits: [u64; 2],
}

impl AsciiSet {
    /// The empty set.
    #[must_use]
    pub const fn empty() -> Self {
        Self { bits: [0, 0] }
    }

    /// Adds one ASCII code point to the set (no-op for non-ASCII input).
    #[must_use]
    pub const fn insert(mut self, byte: u8) -> Self {
        if byte < 128 {
            let word = if byte >= 64 { 1 } else { 0 };
            self.bits[word] |= 1 << (byte % 64);
        }
        self
    }

    /// Adds every code point in the inclusive ASCII range (clamped to 0..=127).
    #[must_use]
    pub const fn insert_range(mut self, lo: u8, hi: u8) -> Self {
        let hi = if hi > 127 { 127 } else { hi };
        let mut b = lo;
        while b <= hi {
            self = self.insert(b);
            if b == 127 {
                break;
            }
            b += 1;
        }
        self
    }

    #[must_use]
    pub fn contains(&self, byte: u8) -> bool {
        byte < 128 && self.bits[usize::from(byte >= 64)] & (1 << (byte % 64)) != 0
    }
}

/// C0 controls and U+007F (URL Standard §2.3 "C0 control percent-encode set").
pub const C0_CONTROL: AsciiSet = AsciiSet::empty().insert_range(0x00, 0x1F).insert(0x7F);

/// Fragment set: C0 set plus space, `"`, `<`, `>`, `` ` ``.
pub const FRAGMENT: AsciiSet = C0_CONTROL
    .insert(b' ')
    .insert(b'"')
    .insert(b'<')
    .insert(b'>')
    .insert(b'`');

/// Query set: C0 set plus space, `"`, `#`, `<`, `>`.
pub const QUERY: AsciiSet = C0_CONTROL
    .insert(b' ')
    .insert(b'"')
    .insert(b'#')
    .insert(b'<')
    .insert(b'>');

/// Special-query set: query set plus `'` (special schemes only).
pub const SPECIAL_QUERY: AsciiSet = QUERY.insert(b'\'');

/// Path set: query set plus `? ^ ` ` { }` (URL Standard §1.3, current text).
pub const PATH: AsciiSet = QUERY
    .insert(b'?')
    .insert(b'^')
    .insert(b'`')
    .insert(b'{')
    .insert(b'}');

/// Userinfo set: path set plus `/ : ; = @ [ \ ] |`.
pub const USERINFO: AsciiSet = PATH
    .insert(b'/')
    .insert(b':')
    .insert(b';')
    .insert(b'=')
    .insert(b'@')
    .insert(b'[')
    .insert(b'\\')
    .insert(b']')
    .insert(b'|');

/// Opaque-host encode set: C0 set plus the forbidden host code points
/// (tab, LF, CR, space, `# / : < > ? @ [ \ ] ^ |`).
pub const OPAQUE_HOST: AsciiSet = C0_CONTROL
    .insert(b'\t')
    .insert(b'\n')
    .insert(b'\r')
    .insert(b' ')
    .insert(b'#')
    .insert(b'/')
    .insert(b':')
    .insert(b'<')
    .insert(b'>')
    .insert(b'?')
    .insert(b'@')
    .insert(b'[')
    .insert(b'\\')
    .insert(b']')
    .insert(b'^')
    .insert(b'|');

/// The `application/x-www-form-urlencoded` percent-encode set: everything
/// except ASCII alphanumerics and `*-._` (URL Standard §5.2).
pub const FORM_URLENCODED: AsciiSet = {
    let mut set = AsciiSet::empty();
    let mut b = 0u8;
    while b < 128 {
        let keep = b.is_ascii_alphanumeric() || matches!(b, b'*' | b'-' | b'.' | b'_');
        if !keep {
            set = set.insert(b);
        }
        b += 1;
    }
    set
};

const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";

/// Percent-encodes `input`, encoding every byte in `set` plus all non-ASCII
/// bytes (URL Standard "UTF-8 percent-encode").
#[must_use]
pub fn percent_encode(input: &str, set: &AsciiSet) -> String {
    let mut out = String::with_capacity(input.len());
    for &byte in input.as_bytes() {
        if byte >= 128 || set.contains(byte) {
            out.push('%');
            out.push(HEX_UPPER[usize::from(byte >> 4)] as char);
            out.push(HEX_UPPER[usize::from(byte & 0x0F)] as char);
        } else {
            out.push(byte as char);
        }
    }
    out
}

/// Percent-decodes `input` to bytes; invalid `%` sequences decode literally
/// (URL Standard "percent-decode" is total).
#[must_use]
pub fn percent_decode(input: &str) -> Vec<u8> {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && let (Some(&hi), Some(&lo)) = (bytes.get(i + 1), bytes.get(i + 2))
            && let (Some(h), Some(l)) = (hex_byte(hi), hex_byte(lo))
        {
            out.push((h << 4) | l);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    out
}

fn hex_byte(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_encodes_set_members_and_multibyte() {
        assert_eq!(percent_encode("a b", &QUERY), "a%20b");
        assert_eq!(percent_encode("café", &C0_CONTROL), "caf%C3%A9");
        assert_eq!(percent_encode("<>`", &FRAGMENT), "%3C%3E%60");
    }

    #[test]
    fn encode_leaves_unreserved_alone() {
        assert_eq!(
            percent_encode("aA1-_~.!$'()*+,;=:@/?#[]", &C0_CONTROL),
            "aA1-_~.!$'()*+,;=:@/?#[]"
        );
    }

    #[test]
    fn decode_is_total_on_garbage() {
        assert_eq!(percent_decode("%41%42"), b"AB".to_vec());
        assert_eq!(percent_decode("%4"), b"%4".to_vec());
        assert_eq!(percent_decode("%zz"), b"%zz".to_vec());
        assert_eq!(percent_decode("caf%C3%A9"), "café".as_bytes().to_vec());
    }

    #[test]
    fn form_set_keeps_alnum_and_special_five() {
        // The set itself encodes space as %20; the `+` mapping for space is
        // the x-www-form-urlencoded serializer's (see crate::form).
        assert_eq!(
            percent_encode("a b*c-d.e_f~g+", &FORM_URLENCODED),
            "a%20b*c-d.e_f%7Eg%2B"
        );
    }
}
