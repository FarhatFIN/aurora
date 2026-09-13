//! Text decoding (§5.4 upstream): UTF-8 and windows-1252, the M1 set
//! (the full encoding soup is an M2 concern, WBS §6.2).

use crate::tables::WINDOWS_1252_SPECIAL;

/// Labels understood by [`decode`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
    Windows1252,
}

/// Selects an encoding by label (subset of the Encoding Standard's labels).
#[must_use]
pub fn encoding_for_label(label: &str) -> Option<Encoding> {
    match label.trim().to_ascii_lowercase().as_str() {
        "utf-8" | "utf8" | "unicode-1-1-utf-8" => Some(Encoding::Utf8),
        "windows-1252" | "cp1252" | "x-cp1252" | "ansi_x3.4-1968" | "latin1" | "iso-8859-1" => {
            Some(Encoding::Windows1252)
        }
        _ => None,
    }
}

/// Decodes `bytes` with `encoding` per the Encoding Standard's decode
/// algorithm: invalid input becomes U+FFFD, never an error (§4.6 class 3).
#[must_use]
pub fn decode(bytes: &[u8], encoding: Encoding) -> String {
    match encoding {
        Encoding::Utf8 => String::from_utf8_lossy(bytes).into_owned(),
        Encoding::Windows1252 => bytes.iter().map(|b| windows1252_char(*b)).collect(),
    }
}

fn windows1252_char(byte: u8) -> char {
    match byte {
        0x80..=0x9F => {
            let mapped = WINDOWS_1252_SPECIAL[usize::from(byte - 0x80)];
            char::from_u32(u32::from(mapped)).unwrap_or('\u{FFFD}')
        }
        other => char::from(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_lossy_on_invalid_input() {
        assert_eq!(decode("héllo".as_bytes(), Encoding::Utf8), "héllo");
        assert_eq!(decode(&[b'a', 0xFF, b'b'], Encoding::Utf8), "a\u{FFFD}b");
    }

    #[test]
    fn windows1252_special_range_and_passthrough() {
        assert_eq!(decode(&[0x80], Encoding::Windows1252), "\u{20AC}"); // euro
        assert_eq!(decode(&[0x93], Encoding::Windows1252), "\u{201C}"); // left quote
        assert_eq!(decode(b"abc", Encoding::Windows1252), "abc");
    }

    #[test]
    fn labels_are_case_insensitive() {
        assert_eq!(encoding_for_label("UTF-8"), Some(Encoding::Utf8));
        assert_eq!(
            encoding_for_label(" Windows-1252 "),
            Some(Encoding::Windows1252)
        );
        assert_eq!(encoding_for_label("iso-2022-jp"), None);
    }
}
