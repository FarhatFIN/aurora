//! The `application/x-www-form-urlencoded` serializer (URL Standard §5).

use crate::percent;

/// Serializes name–value pairs as `application/x-www-form-urlencoded`
/// (URL Standard §5 "urlencoded serializer"): space becomes `+`, bytes are
/// percent-encoded with the urlencoded set, pairs joined with `&`.
#[must_use]
pub fn form_urlencode(pairs: &[(String, String)]) -> String {
    let mut output = String::new();
    for (name, value) in pairs {
        if !output.is_empty() {
            output.push('&');
        }
        encode_pair_into(&mut output, name);
        output.push('=');
        encode_pair_into(&mut output, value);
    }
    output
}

fn encode_pair_into(output: &mut String, text: &str) {
    for byte in text.as_bytes() {
        match byte {
            b' ' => output.push('+'),
            b if percent::FORM_URLENCODED.contains(*b) || *b >= 128 => {
                output.push('%');
                // Writing to a String cannot fail (std::fmt::Write).
                let _ = core::fmt::Write::write_fmt(output, format_args!("{b:02X}"));
            }
            // ASCII byte: exact char conversion.
            b => output.push(char::from(*b)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_pairs_with_plus_for_space() {
        assert_eq!(form_urlencode(&[("a b".into(), "c&d".into())]), "a+b=c%26d");
        assert_eq!(
            form_urlencode(&[("k1".into(), "v1".into()), ("k2".into(), String::new())]),
            "k1=v1&k2="
        );
        assert_eq!(form_urlencode(&[]), "");
    }

    #[test]
    fn encodes_non_ascii_as_utf8() {
        assert_eq!(form_urlencode(&[("q".into(), "é".into())]), "q=%C3%A9");
    }
}
