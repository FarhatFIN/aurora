//! The input cursor: preprocessed characters (§13.2.3.5: CR, CRLF → LF)
//! with push-back for reprocessing (§5.5.6's document.write feeds
//! reprocessed input; the tokenizer's own reprocessing uses it too).

/// A cursor over preprocessed input. M2 takes the whole input; streaming
/// chunks (with the trailing-CR subtlety) arrive with the loader pipe —
/// recorded in PROGRESS.md, not a silent shortcut.
pub(crate) struct Cursor {
    chars: Vec<char>,
    position: usize,
}

impl Cursor {
    /// Normalizes newlines once (equivalent to per-pull for whole inputs).
    #[must_use]
    pub fn new(input: &str) -> Self {
        let mut chars = Vec::with_capacity(input.len());
        let mut previous_was_cr = false;
        for c in input.chars() {
            match c {
                '\r' => {
                    chars.push('\n');
                    previous_was_cr = true;
                }
                '\n' if previous_was_cr => previous_was_cr = false,
                other => {
                    previous_was_cr = false;
                    chars.push(other);
                }
            }
        }
        Self { chars, position: 0 }
    }

    /// The next code point without consuming it; None at EOF.
    #[must_use]
    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    /// The code point `offset` positions ahead, without consuming.
    #[must_use]
    pub fn peek_at(&self, offset: usize) -> Option<char> {
        self.chars.get(self.position + offset).copied()
    }

    /// Consumes and returns the next code point.
    pub fn next(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.position += 1;
        }
        c
    }

    /// Pushes the most recently consumed code point back (reprocessing).
    pub fn unread(&mut self) {
        self.position = self.position.saturating_sub(1);
    }

    /// Whether the cursor is at EOF.
    #[must_use]
    pub fn at_eof(&self) -> bool {
        self.position >= self.chars.len()
    }

    /// ASCII-case-insensitive variant of [`Cursor::starts_with`].
    #[must_use]
    pub fn starts_with_ignore_case(&self, text: &str) -> bool {
        let mut index = self.position;
        for expected in text.chars() {
            match self.chars.get(index) {
                Some(&c) if c.eq_ignore_ascii_case(&expected) => index += 1,
                _ => return false,
            }
        }
        true
    }

    /// The absolute position, for save/restore around atomic matches.
    #[must_use]
    pub fn position_of(&self) -> usize {
        self.position
    }

    /// Restores a snapshot taken with [`Cursor::position_of`].
    pub fn restore_to(&mut self, position: usize) {
        self.position = position.min(self.chars.len());
    }

    /// Consumes up to `n` code points into a String (temp buffer use).
    #[must_use]
    pub fn take(&mut self, n: usize) -> String {
        let end = (self.position + n).min(self.chars.len());
        let text: String = self.chars[self.position..end].iter().collect();
        self.position = end;
        text
    }

    /// Whether the upcoming code points start with `text` (no consumption).
    #[must_use]
    pub fn starts_with(&self, text: &str) -> bool {
        let mut index = self.position;
        for expected in text.chars() {
            match self.chars.get(index) {
                Some(&c) if c == expected => index += 1,
                _ => return false,
            }
        }
        true
    }
}
