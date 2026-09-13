//! Header storage and validation (§5.2.2: header values are validated to be
//! visible ASCII without CR/LF at the API — header injection is impossible
//! by construction). Lookups are ASCII-case-insensitive (§5.2 invariant).

/// An ordered header map: duplicates preserved in insertion order, lookups
/// case-insensitive.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HeaderMap {
    entries: Vec<(String, String)>,
}

/// A header name or value that would break the wire format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidHeader;

impl HeaderMap {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a header. Names/values must be visible ASCII (space and tab
    /// allowed in values); CR and LF are rejected outright (§5.2.2).
    ///
    /// # Errors
    /// [`InvalidHeader`] when the name is empty, non-ASCII, contains CTLs,
    /// or the value contains CR/LF or other control characters.
    pub fn append(&mut self, name: &str, value: &str) -> Result<(), InvalidHeader> {
        validate_name(name)?;
        validate_value(value)?;
        self.entries
            .push((name.to_ascii_lowercase(), value.to_owned()));
        Ok(())
    }

    /// The first value for `name`, or None.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&str> {
        let lowered = name.to_ascii_lowercase();
        self.entries
            .iter()
            .find(|(n, _)| *n == lowered)
            .map(|(_, v)| v.as_str())
    }

    /// All values for `name` in order.
    #[must_use]
    pub fn get_all(&self, name: &str) -> Vec<&str> {
        let lowered = name.to_ascii_lowercase();
        self.entries
            .iter()
            .filter(|(n, _)| *n == lowered)
            .map(|(_, v)| v.as_str())
            .collect()
    }

    /// Removes every entry for `name` (used to strip credentials on
    /// cross-origin redirects, §5.2.4).
    pub fn remove_all(&mut self, name: &str) {
        let lowered = name.to_ascii_lowercase();
        self.entries.retain(|(n, _)| *n != lowered);
    }

    /// Iterates (name, value) pairs, names lowercased as stored.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries.iter().map(|(n, v)| (n.as_str(), v.as_str()))
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

fn validate_name(name: &str) -> Result<(), InvalidHeader> {
    // RFC 9110 field-name: token characters only; be conservative.
    let valid = !name.is_empty()
        && name.bytes().all(|b| {
            b.is_ascii_alphanumeric()
                || matches!(
                    b,
                    b'!' | b'#'
                        | b'$'
                        | b'%'
                        | b'&'
                        | b'\''
                        | b'*'
                        | b'+'
                        | b'-'
                        | b'.'
                        | b'^'
                        | b'_'
                        | b'`'
                        | b'|'
                        | b'~'
                )
        });
    if valid { Ok(()) } else { Err(InvalidHeader) }
}

fn validate_value(value: &str) -> Result<(), InvalidHeader> {
    // field-value: visible ASCII plus space/tab; no CR, LF, or NUL.
    let valid = value
        .bytes()
        .all(|b| (0x20..=0x7E).contains(&b) || b == b'\t');
    if valid { Ok(()) } else { Err(InvalidHeader) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookups_are_case_insensitive() {
        let mut headers = HeaderMap::new();
        headers.append("Content-Type", "text/html").unwrap();
        assert_eq!(headers.get("content-type"), Some("text/html"));
        assert_eq!(headers.get("CONTENT-TYPE"), Some("text/html"));
        assert_eq!(headers.get("accept"), None);
    }

    #[test]
    fn duplicates_preserved_in_order() {
        let mut headers = HeaderMap::new();
        headers.append("Set-Cookie", "a=1").unwrap();
        headers.append("set-cookie", "b=2").unwrap();
        assert_eq!(headers.get_all("Set-Cookie"), vec!["a=1", "b=2"]);
    }

    #[test]
    fn injection_is_rejected() {
        let mut headers = HeaderMap::new();
        assert_eq!(
            headers.append("X-A", "v\r\nInjected: 1"),
            Err(InvalidHeader)
        );
        assert_eq!(headers.append("X-A", "v\nInjected: 1"), Err(InvalidHeader));
        assert_eq!(headers.append("X-A\n", "v"), Err(InvalidHeader));
        assert_eq!(headers.append("", "v"), Err(InvalidHeader));
        assert!(headers.is_empty());
    }
}
