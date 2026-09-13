//! Character references (§13.4.5.1 "consuming a character reference" and
//! the numeric remapping table of §13.2.5.2).

use crate::cursor::Cursor;
use crate::tables::NAMED_REFERENCES;

/// What the reference consumer decided; the tokenizer reprocesses on
/// `FlushBuffer` (the `&` is literal text).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReferenceOutcome {
    /// Codepoints were appended to the character run.
    Consumed,
    /// The `&` is literal: the caller reprocesses the input in its return
    /// state with the buffer flushed as-is.
    FlushBuffer,
}

/// The §13.2.5.2 numeric-reference windows-1252 remapping table.
const WINDOWS_1252_REMAP: [u32; 32] = [
    0x20AC, 0x81, 0x201A, 0x0192, 0x201E, 0x2026, 0x2020, 0x2021, 0x02C6, 0x2030, 0x0160, 0x2039,
    0x0152, 0x8D, 0x017D, 0x8F, 0x90, 0x2018, 0x2019, 0x201C, 0x201D, 0x2022, 0x2013, 0x2014,
    0x02DC, 0x2122, 0x0161, 0x203A, 0x0153, 0x9D, 0x017E, 0x0178,
];

/// The longest name in the table bounds the candidate scan.
const MAX_NAME_LEN: usize = 32;

/// Consumes a character reference, with the cursor positioned **at** the
/// `&` (table names include it). Appends decoded codepoints via `emit`.
///
/// On `FlushBuffer` the cursor is restored to the `&` itself: the caller
/// emits the literal `&`, consumes it, and reprocesses the rest in its
/// return state.
pub(crate) fn consume(
    cursor: &mut Cursor,
    in_attribute: bool,
    errors: &mut Vec<&'static str>,
    emit: &mut impl FnMut(char),
) -> ReferenceOutcome {
    let entry = cursor.position_of();
    let second = cursor.peek_at(1);
    if second.is_some_and(|c| c.is_ascii_alphanumeric()) {
        consume_named(cursor, in_attribute, errors, emit)
    } else if second == Some('#') {
        if consume_numeric(cursor, errors, emit) == ReferenceOutcome::Consumed {
            ReferenceOutcome::Consumed
        } else {
            cursor.restore_to(entry);
            ReferenceOutcome::FlushBuffer
        }
    } else {
        ReferenceOutcome::FlushBuffer
    }
}

fn consume_named(
    cursor: &mut Cursor,
    in_attribute: bool,
    errors: &mut Vec<&'static str>,
    emit: &mut impl FnMut(char),
) -> ReferenceOutcome {
    // The table match is atomic (§13.4.5.1 "named character reference
    // state": consume the maximal match over the buffer, which includes
    // the leading `&`). Snapshot so failed or ambiguous matches restore.
    let entry = cursor.position_of();
    let candidate = cursor.take(MAX_NAME_LEN + 1);
    let mut best: Option<(&str, &'static [char])> = None;
    for (name, points) in NAMED_REFERENCES {
        if candidate.starts_with(name) {
            best = Some((name, points));
        }
    }
    let Some((name, points)) = best else {
        cursor.restore_to(entry);
        return ReferenceOutcome::FlushBuffer;
    };
    let ended_with_semicolon = name.ends_with(';');
    let rest_after_match = &candidate[name.len()..];
    if !ended_with_semicolon
        && in_attribute
        && rest_after_match
            .chars()
            .next()
            .is_some_and(|next| next == '=' || next.is_ascii_alphanumeric())
    {
        // Historic attribute ambiguity: not a reference after all.
        cursor.restore_to(entry);
        return ReferenceOutcome::FlushBuffer;
    }
    if !ended_with_semicolon {
        errors.push("missing-semicolon-after-character-reference");
    }
    for c in points {
        emit(*c);
    }
    // Consume exactly the matched name (the take() above over-read).
    cursor.restore_to(entry + name.chars().count());
    ReferenceOutcome::Consumed
}

fn consume_numeric(
    cursor: &mut Cursor,
    errors: &mut Vec<&'static str>,
    emit: &mut impl FnMut(char),
) -> ReferenceOutcome {
    cursor.next(); // the '&' (the cursor is positioned on it)
    cursor.next(); // the '#'
    let radix = match cursor.peek() {
        Some('x' | 'X') => {
            cursor.next();
            16u32
        }
        _ => 10,
    };
    let mut value: u64 = 0;
    let mut any_digit = false;
    while let Some(c) = cursor.peek() {
        let digit = match (radix, c.to_digit(radix)) {
            (_, Some(d)) => u64::from(d),
            (16, _) if c.is_ascii_hexdigit() => {
                u64::from(c.to_ascii_uppercase().to_digit(16).unwrap_or(0))
            }
            _ => break,
        };
        // Overflow clamps to the FFFD path below.
        value = value.saturating_mul(u64::from(radix)).saturating_add(digit);
        any_digit = true;
        cursor.next();
    }
    if !any_digit {
        errors.push("absence-of-digits-in-numeric-character-reference");
        return ReferenceOutcome::FlushBuffer;
    }
    match cursor.peek() {
        Some(';') => {
            cursor.next();
        }
        _ => errors.push("missing-semicolon-after-character-reference"),
    }
    emit(numeric_remap(value));
    ReferenceOutcome::Consumed
}

/// The numeric reference mapping (§13.2.5.2): NUL, surrogates, and
/// out-of-range become U+FFFD; 0x80–0x9F map through windows-1252.
#[must_use]
pub(crate) fn numeric_remap(value: u64) -> char {
    let in_block = |low: u64, high: u64| (low..=high).contains(&value);
    if value == 0 || in_block(0xD800, 0xDFFF) || value > 0x0010_FFFF {
        return '\u{FFFD}';
    }
    if in_block(0x0080, 0x009F) {
        let mapped = WINDOWS_1252_REMAP[usize::try_from(value - 0x80).unwrap_or(31)];
        return char::from_u32(mapped).unwrap_or('\u{FFFD}');
    }
    char::from_u32(u32::try_from(value).unwrap_or(0x10_FFFF + 1)).unwrap_or('\u{FFFD}')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode(input: &str, in_attribute: bool) -> (String, Vec<&'static str>) {
        let (out, rest, errors) = decode_with_rest(input, in_attribute);
        assert_eq!(rest, "", "fully consumed: {rest}");
        (out, errors)
    }

    fn decode_with_rest(input: &str, in_attribute: bool) -> (String, String, Vec<&'static str>) {
        let mut cursor = Cursor::new(input);
        let mut errors = Vec::new();
        let mut out = String::new();
        let outcome = {
            let emit = &mut |c: char| out.push(c);
            consume(&mut cursor, in_attribute, &mut errors, emit)
        };
        assert_eq!(
            outcome,
            ReferenceOutcome::Consumed,
            "rest after: {}",
            cursor_rest(&mut cursor)
        );
        let rest = cursor_rest(&mut cursor);
        (out, rest, errors)
    }

    fn cursor_rest(cursor: &mut Cursor) -> String {
        std::iter::from_fn(|| cursor.next()).collect()
    }

    #[test]
    fn named_references_decode_longest_match() {
        // &amp; (with semicolon) beats the legacy &amp prefix.
        let (text, rest, errors) = decode_with_rest("&amp;c", false);
        assert_eq!(text, "&");
        assert_eq!(rest, "c");
        assert!(errors.is_empty());
        let (text, rest, _) = decode_with_rest("&lt;gt;", false);
        assert_eq!(text, "<");
        assert_eq!(rest, "gt;");
        // &not is a real legacy reference: the longest match wins even
        // when the remainder makes the name look fake (html5lib case).
        let (text, rest, errors) = decode_with_rest("&notarealentity;", false);
        assert_eq!(text, "\u{00AC}");
        assert_eq!(rest, "arealentity;");
        assert_eq!(errors, ["missing-semicolon-after-character-reference"]);
    }

    #[test]
    fn legacy_without_semicolon_matches_outside_attributes() {
        // &amp without the semicolon still decodes to & outside attributes;
        // only the reference itself is consumed.
        let (text, rest, errors) = decode_with_rest("&amp x", false);
        assert_eq!(text, "&");
        assert_eq!(rest, " x");
        assert_eq!(errors, ["missing-semicolon-after-character-reference"]);
    }

    #[test]
    fn legacy_without_semicolon_before_equals_stays_literal_in_attributes() {
        let mut cursor = Cursor::new("&amp=x");
        let mut errors = Vec::new();
        let mut out = String::new();
        let outcome = {
            let emit = &mut |c: char| out.push(c);
            consume(&mut cursor, true, &mut errors, emit)
        };
        assert_eq!(outcome, ReferenceOutcome::FlushBuffer);
        assert!(out.is_empty());
        // The cursor sits ON the '&': the caller emits it, consumes it, and
        // the return state reprocesses the rest.
        assert_eq!(cursor.next(), Some('&'));
        assert_eq!(cursor.next(), Some('a'));
    }

    #[test]
    fn numeric_references_decode_and_remap() {
        let (text, _) = decode("&#65;", false);
        assert_eq!(text, "A");
        let (text, _) = decode("&#x41;", false);
        assert_eq!(text, "A");
        let (text, errors) = decode("&#151;", false); // windows-1252 remap
        assert_eq!(text, "\u{2014}");
        assert!(errors.is_empty());
        let (text, errors) = decode("&#0;", false);
        assert_eq!(text, "\u{FFFD}");
        assert!(errors.is_empty());
        let (text, errors) = decode("&#1114112;", false); // > U+10FFFF
        assert_eq!(text, "\u{FFFD}");
        assert!(errors.is_empty());
        let (text, errors) = decode("&#65", false); // missing semicolon
        assert_eq!(text, "A");
        assert_eq!(errors, ["missing-semicolon-after-character-reference"]);
    }

    #[test]
    fn numeric_without_digits_flushes() {
        let mut cursor = Cursor::new("&#x;");
        let mut errors = Vec::new();
        let mut out = String::new();
        let outcome = {
            let emit = &mut |c: char| out.push(c);
            consume(&mut cursor, false, &mut errors, emit)
        };
        assert_eq!(outcome, ReferenceOutcome::FlushBuffer);
        assert_eq!(errors, ["absence-of-digits-in-numeric-character-reference"]);
    }
}
