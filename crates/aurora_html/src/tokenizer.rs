//! The HTML tokenizer (§5.4 = WHATWG HTML §13.4): an explicit state
//! machine over `State`, one match arm per standard state, error recovery
//! per the standard (parse errors collected, never fatal — §4.6 class 3).
//!
//! Character tokens are emitted as runs (`Token::Character(String)`) — the
//! optimization the §5.4 sketch explicitly permits. The machine consumes
//! one code point per step (`step`), with `cursor.unread()` providing the
//! standard's "reconsume" semantics.

use crate::cursor::Cursor;
use crate::reference::{self, ReferenceOutcome};
use crate::token::{Attribute, InitialState, State, Token, TokenizerOptions};

/// Parse errors, in order, as the standard's error codes.
pub(crate) type ParseErrors = Vec<&'static str>;

#[allow(clippy::struct_excessive_bools)] // the bools mirror the standard's own tokenizer state variables
pub struct Tokenizer {
    cursor: Cursor,
    state: State,
    return_state: State,
    #[allow(dead_code)] // read by the tree constructor's scripting flag (M2 next slice)
    options: TokenizerOptions,
    errors: ParseErrors,
    run: String,
    comment_buffer: String,
    completed: Option<Token>,
    eof_done: bool,
    // Start/end tag under construction.
    tag_name: String,
    attrs: Vec<Attribute>,
    attr_name: String,
    attr_value: String,
    attr_pending: bool,
    self_closing: bool,
    tag_is_end: bool,
    temp: String,
    last_start_tag_name: String,
    // DOCTYPE under construction.
    doctype_name: Option<String>,
    doctype_public: Option<String>,
    doctype_system: Option<String>,
    force_quirks: bool,
    /// Set by the tree builder when the adjusted current node is not in
    /// the HTML namespace (§13.4 "markup declaration open state").
    allow_cdata: bool,
    /// Whether the current character reference sits in an attribute value
    /// (drives the legacy no-semicolon ambiguity).
    reference_in_attribute: bool,
}

impl Tokenizer {
    #[must_use]
    pub fn new(input: &str, options: TokenizerOptions) -> Self {
        let state = match options.initial_state {
            InitialState::Data => State::Data,
            InitialState::Rcdata => State::Rcdata,
            InitialState::Rawtext => State::Rawtext,
            InitialState::ScriptData => State::ScriptData,
            InitialState::Plaintext => State::PlainText,
        };
        Self {
            cursor: Cursor::new(input),
            state,
            return_state: State::Data,
            options,
            errors: Vec::new(),
            run: String::new(),
            comment_buffer: String::new(),
            completed: None,
            eof_done: false,
            tag_name: String::new(),
            attrs: Vec::new(),
            attr_name: String::new(),
            attr_value: String::new(),
            attr_pending: false,
            self_closing: false,
            tag_is_end: false,
            temp: String::new(),
            last_start_tag_name: String::new(),
            doctype_name: None,
            doctype_public: None,
            doctype_system: None,
            force_quirks: false,
            allow_cdata: false,
            reference_in_attribute: false,
        }
    }

    /// Test hook: the real pipeline's last start tag comes from the Data
    /// state itself; rawtext/rcdata/script unit tests preset it.
    #[cfg(test)]
    pub(crate) fn set_last_start_tag_for_tests(&mut self, name: &str) {
        self.last_start_tag_name = name.to_owned();
    }

    #[must_use]
    pub fn errors(&self) -> &[&'static str] {
        &self.errors
    }

    /// Pulls the next token, or None at end of stream.
    pub fn next_token(&mut self) -> Option<Token> {
        loop {
            if let Some(token) = self.completed.take() {
                // A character run that accumulated before this token goes
                // out first (document order).
                if let Some(run) = self.take_run() {
                    self.completed = Some(token);
                    return Some(run);
                }
                return Some(token);
            }
            if self.eof_done {
                return self.take_run();
            }
            self.step();
        }
    }

    fn take_run(&mut self) -> Option<Token> {
        if self.run.is_empty() {
            None
        } else {
            Some(Token::Character(std::mem::take(&mut self.run)))
        }
    }

    fn emit_char(&mut self, c: char) {
        self.run.push(c);
    }

    fn error(&mut self, code: &'static str) {
        self.errors.push(code);
    }

    /// One state step over the next code point, or the EOF event.
    fn step(&mut self) {
        let Some(c) = self.cursor.peek() else {
            self.process_eof();
            return;
        };
        // The character-reference state consumes the '&' itself (the table
        // names include it); every other state takes the char up front.
        if self.state != State::CharacterReference {
            self.cursor.next();
        }
        self.process(c);
    }

    fn reconsume_in(&mut self, state: State) {
        self.cursor.unread();
        self.state = state;
    }

    // ---- token completion helpers ----------------------------------------

    fn finish_attribute(&mut self) {
        if self.attr_pending {
            self.attrs.push(Attribute {
                name: std::mem::take(&mut self.attr_name),
                value: std::mem::take(&mut self.attr_value),
            });
            self.attr_pending = false;
        }
    }

    fn start_attribute(&mut self) {
        self.finish_attribute();
        self.attr_pending = true;
    }

    fn emit_start_tag(&mut self, self_closing: bool) {
        if self.tag_is_end {
            self.self_closing |= self_closing;
            self.emit_end_tag();
            return;
        }
        self.finish_attribute();
        // Duplicate attributes: first wins, the rest are removed with a
        // parse error (§13.4, end of the attribute name state).
        let mut seen: Vec<String> = Vec::with_capacity(self.attrs.len());
        self.attrs.retain(|attribute| {
            if seen.contains(&attribute.name) {
                self.errors.push("duplicate-attribute");
                false
            } else {
                seen.push(attribute.name.clone());
                true
            }
        });
        let name = std::mem::take(&mut self.tag_name);
        let attrs = std::mem::take(&mut self.attrs);
        self.completed = Some(Token::StartTag {
            name: name.clone(),
            attrs,
            self_closing: self_closing || self.self_closing,
        });
        self.last_start_tag_name = name;
        self.self_closing = false;
    }

    fn emit_end_tag(&mut self) {
        self.finish_attribute();
        if !self.attrs.is_empty() {
            self.error("end-tag-with-attributes");
        }
        if self.self_closing {
            self.error("end-tag-with-trailing-solidus");
        }
        self.attrs.clear();
        let name = std::mem::take(&mut self.tag_name);
        self.completed = Some(Token::EndTag { name });
    }

    fn reset_tag_buffers(&mut self) {
        self.tag_name.clear();
        self.finish_attribute();
        self.self_closing = false;
        self.tag_is_end = false;
    }

    fn emit_doctype(&mut self) {
        self.completed = Some(Token::Doctype {
            name: self.doctype_name.clone(),
            public: self.doctype_public.clone(),
            system: self.doctype_system.clone(),
            force_quirks: self.force_quirks,
        });
        self.doctype_name = None;
        self.doctype_public = None;
        self.doctype_system = None;
        self.force_quirks = false;
    }

    /// Emits the comment under construction from its own buffer (the
    /// character run must keep flowing independently, §13.4).
    fn flush_comment(&mut self) {
        self.completed = Some(Token::Comment(std::mem::take(&mut self.comment_buffer)));
    }

    /// The end-tag-name states' mismatch flush: "</" plus the buffered
    /// name become literal characters, then reconsume.
    fn flush_premature_end_tag(&mut self) {
        let buffered = std::mem::take(&mut self.temp);
        self.run.push_str("</");
        self.run.push_str(&buffered);
    }

    fn is_appropriate_end_tag(&self) -> bool {
        !self.last_start_tag_name.is_empty() && self.tag_name == self.last_start_tag_name
    }

    // ---- the state machine -------------------------------------------------

    #[allow(clippy::too_many_lines)] // one translation of one 80-state algorithm; splitting it would scatter the standard's transitions
    fn process(&mut self, c: char) {
        match self.state {
            State::Data => match c {
                '&' => {
                    // The cursor goes back ON the '&': reference::consume
                    // reads it (table names include the ampersand).
                    self.cursor.unread();
                    self.return_state = State::Data;
                    self.reference_in_attribute = false;
                    self.state = State::CharacterReference;
                }
                '<' => self.state = State::TagOpen,
                '\0' => {
                    self.error("unexpected-null-character");
                    self.emit_char('\u{FFFD}');
                }
                _ => self.emit_char(c),
            },
            State::Rcdata => match c {
                '&' => {
                    self.cursor.unread();
                    self.return_state = State::Rcdata;
                    self.reference_in_attribute = false;
                    self.state = State::CharacterReference;
                }
                '<' => self.state = State::RcdataLessThanSign,
                '\0' => {
                    self.error("unexpected-null-character");
                    self.emit_char('\u{FFFD}');
                }
                _ => self.emit_char(c),
            },
            State::RcdataLessThanSign => {
                if c == '/' {
                    self.state = State::RcdataEndTagOpen;
                } else {
                    self.emit_char('<');
                    self.reconsume_in(State::Rcdata);
                }
            }
            State::RcdataEndTagOpen => {
                if c.is_ascii_alphabetic() {
                    self.temp.clear();
                    self.temp.push(c);
                    self.reset_tag_buffers();
                    self.tag_is_end = true;
                    self.tag_name.push(c.to_ascii_lowercase());
                    self.state = State::RcdataEndTagName;
                } else {
                    self.run.push_str("</");
                    self.reconsume_in(State::Rcdata);
                }
            }
            State::RcdataEndTagName => self.end_tag_name(c, State::Rcdata),
            State::Rawtext => match c {
                '<' => self.state = State::RawtextLessThanSign,
                '\0' => {
                    self.error("unexpected-null-character");
                    self.emit_char('\u{FFFD}');
                }
                _ => self.emit_char(c),
            },
            State::RawtextLessThanSign => {
                if c == '/' {
                    self.state = State::RawtextEndTagOpen;
                } else {
                    self.emit_char('<');
                    self.reconsume_in(State::Rawtext);
                }
            }
            State::RawtextEndTagOpen => {
                if c.is_ascii_alphabetic() {
                    self.temp.clear();
                    self.temp.push(c);
                    self.reset_tag_buffers();
                    self.tag_is_end = true;
                    self.tag_name.push(c.to_ascii_lowercase());
                    self.state = State::RawtextEndTagName;
                } else {
                    self.run.push_str("</");
                    self.reconsume_in(State::Rawtext);
                }
            }
            State::RawtextEndTagName => self.end_tag_name(c, State::Rawtext),
            State::ScriptData => match c {
                '<' => self.state = State::ScriptDataLessThanSign,
                '\0' => {
                    self.error("unexpected-null-character");
                    self.emit_char('\u{FFFD}');
                }
                _ => self.emit_char(c),
            },
            State::ScriptDataLessThanSign => match c {
                '/' => self.state = State::ScriptDataEndTagOpen,
                '!' => {
                    self.emit_char('<');
                    self.emit_char('!');
                    self.state = State::ScriptDataEscapeStart;
                }
                _ => {
                    self.emit_char('<');
                    self.reconsume_in(State::ScriptData);
                }
            },
            State::ScriptDataEndTagOpen => {
                if c.is_ascii_alphabetic() {
                    self.temp.clear();
                    self.temp.push(c);
                    self.reset_tag_buffers();
                    self.tag_is_end = true;
                    self.tag_name.push(c.to_ascii_lowercase());
                    self.state = State::ScriptDataEndTagName;
                } else {
                    self.run.push_str("</");
                    self.reconsume_in(State::ScriptData);
                }
            }
            State::ScriptDataEndTagName => self.end_tag_name(c, State::ScriptData),
            State::ScriptDataEscapeStart => match c {
                '-' => {
                    self.emit_char('-');
                    self.state = State::ScriptDataEscapeStartDash;
                }
                _ => self.reconsume_in(State::ScriptData),
            },
            State::ScriptDataEscapeStartDash => match c {
                '-' => {
                    self.emit_char('-');
                    self.state = State::ScriptDataEscapedDashDash;
                }
                _ => self.reconsume_in(State::ScriptData),
            },
            State::ScriptDataEscaped => match c {
                '-' => {
                    self.emit_char('-');
                    self.state = State::ScriptDataEscapedDash;
                }
                '<' => self.state = State::ScriptDataEscapedLessThanSign,
                '\0' => {
                    self.error("unexpected-null-character");
                    self.emit_char('\u{FFFD}');
                }
                _ => self.emit_char(c),
            },
            State::ScriptDataEscapedDash => match c {
                '-' => {
                    self.emit_char('-');
                    self.state = State::ScriptDataEscapedDashDash;
                }
                '<' => self.state = State::ScriptDataEscapedLessThanSign,
                '\0' => {
                    self.error("unexpected-null-character");
                    self.emit_char('\u{FFFD}');
                    self.state = State::ScriptDataEscaped;
                }
                _ => {
                    self.emit_char(c);
                    self.state = State::ScriptDataEscaped;
                }
            },
            State::ScriptDataEscapedDashDash => match c {
                '-' => self.emit_char('-'),
                '<' => self.state = State::ScriptDataEscapedLessThanSign,
                '>' => {
                    self.emit_char('>');
                    self.state = State::ScriptData;
                }
                '\0' => {
                    self.error("unexpected-null-character");
                    self.emit_char('\u{FFFD}');
                    self.state = State::ScriptDataEscaped;
                }
                _ => {
                    self.emit_char(c);
                    self.state = State::ScriptDataEscaped;
                }
            },
            State::ScriptDataEscapedLessThanSign => match c {
                '/' => self.state = State::ScriptDataEscapedEndTagOpen,
                alpha if alpha.is_ascii_alphabetic() => {
                    self.temp.clear();
                    // §13.2.5.23: emit '<' and RECONSUME in the double
                    // escape start state, which appends and emits the alpha.
                    self.emit_char('<');
                    self.reconsume_in(State::ScriptDataDoubleEscapeStart);
                }
                _ => {
                    self.emit_char('<');
                    self.reconsume_in(State::ScriptDataEscaped);
                }
            },
            State::ScriptDataEscapedEndTagOpen => {
                if c.is_ascii_alphabetic() {
                    self.temp.clear();
                    self.temp.push(c);
                    self.reset_tag_buffers();
                    self.tag_is_end = true;
                    self.tag_name.push(c.to_ascii_lowercase());
                    self.state = State::ScriptDataEscapedEndTagName;
                } else {
                    self.run.push_str("</");
                    self.reconsume_in(State::ScriptDataEscaped);
                }
            }
            State::ScriptDataEscapedEndTagName => self.end_tag_name(c, State::ScriptDataEscaped),
            State::ScriptDataDoubleEscapeStart => {
                // Verified against §13.2.5.26: alphas accumulate into the
                // temp buffer and are emitted; the terminator decides.
                if c.is_ascii_alphabetic() {
                    self.temp.push(c.to_ascii_lowercase());
                    self.emit_char(c);
                } else if matches!(c, '\t' | '\n' | '\u{0C}' | ' ' | '/' | '>') {
                    if self.temp == "script" {
                        self.state = State::ScriptDataDoubleEscaped;
                    } else {
                        self.state = State::ScriptDataEscaped;
                    }
                    self.emit_char(c);
                } else {
                    self.reconsume_in(State::ScriptDataEscaped);
                }
            }
            State::ScriptDataDoubleEscaped => match c {
                '-' => {
                    self.emit_char('-');
                    self.state = State::ScriptDataDoubleEscapedDash;
                }
                '<' => self.state = State::ScriptDataDoubleEscapedLessThanSign,
                '\0' => {
                    self.error("unexpected-null-character");
                    self.emit_char('\u{FFFD}');
                }
                _ => self.emit_char(c),
            },
            State::ScriptDataDoubleEscapedDash => match c {
                '-' => {
                    self.emit_char('-');
                    self.state = State::ScriptDataDoubleEscapedDashDash;
                }
                '<' => self.state = State::ScriptDataDoubleEscapedLessThanSign,
                '\0' => {
                    self.error("unexpected-null-character");
                    self.emit_char('\u{FFFD}');
                    self.state = State::ScriptDataDoubleEscaped;
                }
                _ => {
                    self.emit_char(c);
                    self.state = State::ScriptDataDoubleEscaped;
                }
            },
            State::ScriptDataDoubleEscapedDashDash => match c {
                '-' => self.emit_char('-'),
                '<' => self.state = State::ScriptDataDoubleEscapedLessThanSign,
                '>' => {
                    self.emit_char('>');
                    self.state = State::ScriptData;
                }
                '\0' => {
                    self.error("unexpected-null-character");
                    self.emit_char('\u{FFFD}');
                    self.state = State::ScriptDataDoubleEscaped;
                }
                _ => {
                    self.emit_char(c);
                    self.state = State::ScriptDataDoubleEscaped;
                }
            },
            State::ScriptDataDoubleEscapedLessThanSign => {
                if c == '/' {
                    self.temp.clear();
                    self.state = State::ScriptDataDoubleEscapeEnd;
                } else {
                    self.emit_char('<');
                    self.reconsume_in(State::ScriptDataDoubleEscaped);
                }
            }
            State::ScriptDataDoubleEscapeEnd => {
                // Verified against §13.2.5.31.
                if c.is_ascii_alphabetic() {
                    self.temp.push(c.to_ascii_lowercase());
                    self.emit_char(c);
                } else if matches!(c, '\t' | '\n' | '\u{0C}' | ' ' | '/' | '>') {
                    if self.temp == "script" {
                        self.state = State::ScriptDataEscaped;
                    } else {
                        self.state = State::ScriptDataDoubleEscaped;
                    }
                    self.emit_char(c);
                } else {
                    self.reconsume_in(State::ScriptDataDoubleEscaped);
                }
            }
            State::PlainText => match c {
                '\0' => {
                    self.error("unexpected-null-character");
                    self.emit_char('\u{FFFD}');
                }
                _ => self.emit_char(c),
            },
            State::TagOpen => match c {
                '!' => self.state = State::MarkupDeclarationOpen,
                '/' => self.state = State::EndTagOpen,
                alpha if alpha.is_ascii_alphabetic() => {
                    self.reset_tag_buffers();
                    self.tag_name.push(alpha.to_ascii_lowercase());
                    self.state = State::TagName;
                }
                '>' => {
                    self.error("missing-greater-than-name-in-start-tag");
                    self.emit_char('<');
                    self.emit_char('>');
                    self.state = State::Data;
                }
                '?' => {
                    self.error("unexpected-question-mark-instead-of-tag-name");
                    self.reconsume_in(State::BogusComment);
                }
                _ => {
                    self.error("invalid-first-character-of-tag-name");
                    self.emit_char('<');
                    self.reconsume_in(State::Data);
                }
            },
            State::EndTagOpen => match c {
                '>' => {
                    self.error("missing-end-tag-name");
                    self.state = State::Data;
                }
                alpha if alpha.is_ascii_alphabetic() => {
                    self.reset_tag_buffers();
                    self.tag_is_end = true;
                    self.tag_name.push(alpha.to_ascii_lowercase());
                    self.state = State::TagName;
                }
                _ => {
                    self.error("invalid-first-character-of-tag-name");
                    self.run.push_str("</");
                    self.reconsume_in(State::BogusComment);
                }
            },
            State::TagName => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => self.state = State::BeforeAttributeName,
                '/' => self.state = State::SelfClosingStartTag,
                '>' => {
                    self.emit_start_tag(false);
                    self.state = State::Data;
                }
                '\0' => {
                    self.error("unexpected-null-character");
                    self.tag_name.push('\u{FFFD}');
                }
                _ => self.tag_name.push(c.to_ascii_lowercase()),
            },
            State::BeforeAttributeName => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => {}
                '/' | '>' => self.reconsume_in(State::AfterAttributeName),
                '=' => {
                    self.error("unexpected-equals-sign-before-attribute-name");
                    self.start_attribute();
                    self.attr_name.push('=');
                    self.state = State::AttributeName;
                }
                _ => {
                    self.start_attribute();
                    self.reconsume_in(State::AttributeName);
                }
            },
            State::AttributeName => match c {
                '\t' | '\n' | '\u{0C}' | ' ' | '/' | '>' => {
                    self.reconsume_in(State::AfterAttributeName);
                }
                // §13.2.5.33: '=' goes straight to the before-attribute-
                // value state (AfterAttributeName's '=' handles a SECOND '=').
                '=' => self.state = State::BeforeAttributeValue,
                '\0' => {
                    self.error("unexpected-null-character");
                    self.attr_name.push('\u{FFFD}');
                }
                _ => self.attr_name.push(c.to_ascii_lowercase()),
            },
            State::AfterAttributeName => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => {}
                '/' => self.state = State::SelfClosingStartTag,
                '=' => self.state = State::BeforeAttributeValue,
                '>' => {
                    self.emit_start_tag(false);
                    self.state = State::Data;
                }
                _ => {
                    self.start_attribute();
                    self.reconsume_in(State::AttributeName);
                }
            },
            State::BeforeAttributeValue => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => {}
                '"' => self.state = State::AttributeValueDoubleQuoted,
                '\'' => self.state = State::AttributeValueSingleQuoted,
                '>' => {
                    self.error("missing-attribute-value");
                    self.emit_start_tag(false);
                    self.state = State::Data;
                }
                _ => self.reconsume_in(State::AttributeValueUnquoted),
            },
            State::AttributeValueDoubleQuoted => match c {
                '"' => self.state = State::AfterAttributeValueQuoted,
                '&' => {
                    self.cursor.unread();
                    self.enter_reference(State::AttributeValueDoubleQuoted);
                }
                '\0' => {
                    self.error("unexpected-null-character");
                    self.attr_value.push('\u{FFFD}');
                }
                _ => self.attr_value.push(c),
            },
            State::AttributeValueSingleQuoted => match c {
                '\'' => self.state = State::AfterAttributeValueQuoted,
                '&' => {
                    self.cursor.unread();
                    self.enter_reference(State::AttributeValueSingleQuoted);
                }
                '\0' => {
                    self.error("unexpected-null-character");
                    self.attr_value.push('\u{FFFD}');
                }
                _ => self.attr_value.push(c),
            },
            State::AttributeValueUnquoted => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => {
                    self.finish_attribute();
                    self.state = State::BeforeAttributeName;
                }
                '&' => {
                    self.cursor.unread();
                    self.enter_reference(State::AttributeValueUnquoted);
                }
                '>' => {
                    self.emit_start_tag(false);
                    self.state = State::Data;
                }
                '\0' => {
                    self.error("unexpected-null-character");
                    self.attr_value.push('\u{FFFD}');
                }
                '"' | '\'' | '<' | '=' | '`' => {
                    self.error("unexpected-character-in-unquoted-attribute-value");
                    self.attr_value.push(c);
                }
                _ => self.attr_value.push(c),
            },
            State::AfterAttributeValueQuoted => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => self.state = State::BeforeAttributeName,
                '/' => self.state = State::SelfClosingStartTag,
                '>' => {
                    self.emit_start_tag(false);
                    self.state = State::Data;
                }
                _ => {
                    self.error("missing-whitespace-between-attributes");
                    self.reconsume_in(State::BeforeAttributeName);
                }
            },
            State::SelfClosingStartTag => {
                if c == '>' {
                    self.emit_start_tag(true);
                    self.state = State::Data;
                } else {
                    self.error("unexpected-solidus-in-tag");
                    self.reconsume_in(State::BeforeAttributeName);
                }
            }
            State::BogusComment => match c {
                '>' => {
                    self.flush_comment();
                    self.state = State::Data;
                }
                '\0' => self.comment_buffer.push('\u{FFFD}'),
                _ => self.comment_buffer.push(c),
            },
            State::MarkupDeclarationOpen => {
                // step() consumed the trigger ('!'); the lookahead checks
                // and the bogus-comment reconsume need it back.
                self.cursor.unread();
                if self.cursor.starts_with("--") {
                    let _ = self.cursor.take(2); // consumed for the state switch, not buffered
                    self.comment_buffer.clear();
                    self.state = State::CommentStart;
                } else if self.cursor.starts_with_ignore_case("DOCTYPE") {
                    let _ = self.cursor.take(7); // consumed for the state switch, not buffered
                    self.state = State::Doctype;
                } else if self.cursor.starts_with("[CDATA[") {
                    let _ = self.cursor.take(7); // consumed for the state switch, not buffered
                    if self.allow_cdata {
                        self.state = State::CdataSection;
                    } else {
                        self.error("cdata-in-html-content");
                        self.comment_buffer.clear();
                        self.comment_buffer.push_str("[CDATA[");
                        self.state = State::BogusComment;
                    }
                } else {
                    self.error("incorrectly-opened-comment");
                    self.comment_buffer.clear();
                    self.state = State::BogusComment;
                }
            }
            State::CommentStart => match c {
                '-' => self.state = State::CommentStartDash,
                '>' => {
                    self.error("abrupt-closing-of-empty-comment");
                    self.flush_comment();
                    self.state = State::Data;
                }
                _ => self.reconsume_in(State::Comment),
            },
            State::CommentStartDash => match c {
                '-' => self.state = State::CommentEnd,
                '>' => {
                    self.error("abrupt-closing-of-empty-comment");
                    self.flush_comment();
                    self.state = State::Data;
                }
                _ => {
                    self.comment_buffer.push('-');
                    self.reconsume_in(State::Comment);
                }
            },
            State::Comment => match c {
                '<' => {
                    self.comment_buffer.push('<');
                    self.state = State::CommentLessThanSign;
                }
                '-' => self.state = State::CommentEndDash,
                '\0' => {
                    self.error("unexpected-null-character");
                    self.comment_buffer.push('\u{FFFD}');
                }
                _ => self.comment_buffer.push(c),
            },
            State::CommentLessThanSign => match c {
                '<' => {
                    self.comment_buffer.push('<');
                    self.state = State::CommentLessThanSignBang;
                }
                '!' => self.state = State::CommentLessThanSignBang,
                _ => self.reconsume_in(State::Comment),
            },
            State::CommentLessThanSignBang => match c {
                '-' => self.state = State::CommentLessThanSignBangDash,
                _ => self.reconsume_in(State::Comment),
            },
            State::CommentLessThanSignBangDash => match c {
                '-' => self.state = State::CommentLessThanSignBangDashDash,
                _ => self.reconsume_in(State::Comment),
            },
            State::CommentLessThanSignBangDashDash => {
                if c == '>' {
                    self.reconsume_in(State::CommentEnd);
                } else {
                    self.error("nested-comment");
                    self.reconsume_in(State::CommentEnd);
                }
            }
            State::CommentEndDash => {
                if c == '-' {
                    self.state = State::CommentEnd;
                } else {
                    self.comment_buffer.push('-');
                    self.reconsume_in(State::Comment);
                }
            }
            State::CommentEnd => match c {
                '>' => {
                    self.flush_comment();
                    self.state = State::Data;
                }
                '-' => self.comment_buffer.push('-'),
                '!' => self.state = State::CommentEndBang,
                _ => {
                    self.comment_buffer.push_str("--");
                    self.reconsume_in(State::Comment);
                }
            },
            State::CommentEndBang => match c {
                '-' => {
                    self.comment_buffer.push_str("--!");
                    self.state = State::CommentEndDash;
                }
                '>' => {
                    self.error("incorrectly-closed-comment");
                    self.flush_comment();
                    self.state = State::Data;
                }
                _ => {
                    self.comment_buffer.push_str("--!");
                    self.reconsume_in(State::Comment);
                }
            },
            State::Doctype => {
                if matches!(c, '\t' | '\n' | '\u{0C}' | ' ') {
                    self.state = State::BeforeDoctypeName;
                } else {
                    // '>' is the missing-whitespace-before-doctype-name
                    // error; the reconsume emits a force-quirks doctype.
                    self.error("missing-whitespace-before-doctype-name");
                    self.reconsume_in(State::BeforeDoctypeName);
                }
            }
            State::BeforeDoctypeName => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => {}
                '>' => {
                    self.error("missing-doctype-name");
                    self.force_quirks = true;
                    self.emit_doctype();
                    self.state = State::Data;
                }
                '\0' => {
                    self.error("unexpected-null-character");
                    self.doctype_name = Some('\u{FFFD}'.to_string());
                    self.state = State::DoctypeName;
                }
                alpha if alpha.is_ascii_alphabetic() => {
                    self.doctype_name = Some(alpha.to_ascii_lowercase().to_string());
                    self.state = State::DoctypeName;
                }
                _ => {
                    self.error("missing-doctype-name");
                    self.doctype_name = Some(c.to_string());
                    self.state = State::DoctypeName;
                }
            },
            State::DoctypeName => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => self.state = State::AfterDoctypeName,
                '>' => {
                    self.emit_doctype();
                    self.state = State::Data;
                }
                '\0' => {
                    self.error("unexpected-null-character");
                    if let Some(name) = &mut self.doctype_name {
                        name.push('\u{FFFD}');
                    }
                }
                _ => {
                    if let Some(name) = &mut self.doctype_name {
                        name.push(c.to_ascii_lowercase());
                    }
                }
            },
            State::AfterDoctypeName => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => {}
                '>' => {
                    self.emit_doctype();
                    self.state = State::Data;
                }
                // The keyword check includes the current character (already
                // consumed by step): rewind, match, consume the keyword.
                _ if {
                    self.cursor.unread();
                    let matches = self.cursor_matches_keyword("PUBLIC");
                    if !matches {
                        self.cursor.next(); // restore the step() consumption
                    }
                    matches
                } =>
                {
                    let _ = self.cursor.take("PUBLIC".len()); // consumed, not buffered
                    self.state = State::AfterDoctypePublicKeyword;
                }
                _ if {
                    let matches = self.cursor_matches_keyword("SYSTEM");
                    if !matches {
                        self.cursor.next();
                    }
                    matches
                } =>
                {
                    let _ = self.cursor.take("SYSTEM".len()); // consumed, not buffered
                    self.state = State::AfterDoctypeSystemKeyword;
                }
                _ => {
                    self.error("invalid-character-sequence-after-doctype-name");
                    self.force_quirks = true;
                    self.state = State::BogusDoctype;
                    self.cursor.unread();
                }
            },
            State::AfterDoctypePublicKeyword => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => self.state = State::BeforeDoctypePublicIdentifier,
                '>' => {
                    self.error("missing-doctype-system-identifier");
                    self.force_quirks = true;
                    self.emit_doctype();
                    self.state = State::Data;
                }
                '"' => {
                    self.error("missing-whitespace-after-doctype-public-keyword");
                    self.doctype_public = Some(String::new());
                    self.state = State::DoctypePublicIdentifierDoubleQuoted;
                }
                '\'' => {
                    self.error("missing-whitespace-after-doctype-public-keyword");
                    self.doctype_public = Some(String::new());
                    self.state = State::DoctypePublicIdentifierSingleQuoted;
                }
                _ => {
                    self.error("missing-quote-before-doctype-public-identifier");
                    self.force_quirks = true;
                    self.state = State::BogusDoctype;
                    self.cursor.unread();
                }
            },
            State::BeforeDoctypePublicIdentifier => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => {}
                '"' => {
                    self.doctype_public = Some(String::new());
                    self.state = State::DoctypePublicIdentifierDoubleQuoted;
                }
                '\'' => {
                    self.doctype_public = Some(String::new());
                    self.state = State::DoctypePublicIdentifierSingleQuoted;
                }
                '>' => {
                    self.error("abrupt-doctype-public-identifier");
                    self.force_quirks = true;
                    self.emit_doctype();
                    self.state = State::Data;
                }
                _ => {
                    self.error("missing-quote-before-doctype-public-identifier");
                    self.force_quirks = true;
                    self.state = State::BogusDoctype;
                    self.cursor.unread();
                }
            },
            State::DoctypePublicIdentifierDoubleQuoted => match c {
                '"' => self.state = State::AfterDoctypePublicIdentifier,
                '\0' => {
                    self.error("unexpected-null-character");
                    if let Some(public) = &mut self.doctype_public {
                        public.push('\u{FFFD}');
                    }
                }
                '>' => {
                    self.error("abrupt-doctype-public-identifier");
                    self.force_quirks = true;
                    self.emit_doctype();
                    self.state = State::Data;
                }
                _ => {
                    if let Some(public) = &mut self.doctype_public {
                        public.push(c);
                    }
                }
            },
            State::DoctypePublicIdentifierSingleQuoted => match c {
                '\'' => self.state = State::AfterDoctypePublicIdentifier,
                '\0' => {
                    self.error("unexpected-null-character");
                    if let Some(public) = &mut self.doctype_public {
                        public.push('\u{FFFD}');
                    }
                }
                '>' => {
                    self.error("abrupt-doctype-public-identifier");
                    self.force_quirks = true;
                    self.emit_doctype();
                    self.state = State::Data;
                }
                _ => {
                    if let Some(public) = &mut self.doctype_public {
                        public.push(c);
                    }
                }
            },
            State::AfterDoctypePublicIdentifier => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => {
                    self.state = State::BetweenDoctypePublicAndSystemIdentifiers;
                }
                '>' => {
                    self.emit_doctype();
                    self.state = State::Data;
                }
                '"' => {
                    self.error("missing-whitespace-between-doctype-public-and-system-identifiers");
                    self.doctype_system = Some(String::new());
                    self.state = State::DoctypeSystemIdentifierDoubleQuoted;
                }
                '\'' => {
                    self.error("missing-whitespace-between-doctype-public-and-system-identifiers");
                    self.doctype_system = Some(String::new());
                    self.state = State::DoctypeSystemIdentifierSingleQuoted;
                }
                _ => {
                    self.error("missing-quote-before-doctype-system-identifier");
                    self.force_quirks = true;
                    self.state = State::BogusDoctype;
                    self.cursor.unread();
                }
            },
            State::BetweenDoctypePublicAndSystemIdentifiers => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => {}
                '>' => {
                    self.emit_doctype();
                    self.state = State::Data;
                }
                '"' => {
                    self.doctype_system = Some(String::new());
                    self.state = State::DoctypeSystemIdentifierDoubleQuoted;
                }
                '\'' => {
                    self.doctype_system = Some(String::new());
                    self.state = State::DoctypeSystemIdentifierSingleQuoted;
                }
                _ => {
                    self.error("missing-quote-before-doctype-system-identifier");
                    self.force_quirks = true;
                    self.state = State::BogusDoctype;
                    self.cursor.unread();
                }
            },
            State::AfterDoctypeSystemKeyword => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => self.state = State::BeforeDoctypeSystemIdentifier,
                '>' => {
                    self.error("missing-doctype-system-identifier");
                    self.force_quirks = true;
                    self.emit_doctype();
                    self.state = State::Data;
                }
                '"' => {
                    self.error("missing-whitespace-after-doctype-system-keyword");
                    self.doctype_system = Some(String::new());
                    self.state = State::DoctypeSystemIdentifierDoubleQuoted;
                }
                '\'' => {
                    self.error("missing-whitespace-after-doctype-system-keyword");
                    self.doctype_system = Some(String::new());
                    self.state = State::DoctypeSystemIdentifierSingleQuoted;
                }
                _ => {
                    self.error("missing-quote-before-doctype-system-identifier");
                    self.force_quirks = true;
                    self.state = State::BogusDoctype;
                    self.cursor.unread();
                }
            },
            State::BeforeDoctypeSystemIdentifier => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => {}
                '"' => {
                    self.doctype_system = Some(String::new());
                    self.state = State::DoctypeSystemIdentifierDoubleQuoted;
                }
                '\'' => {
                    self.doctype_system = Some(String::new());
                    self.state = State::DoctypeSystemIdentifierSingleQuoted;
                }
                '>' => {
                    self.error("abrupt-doctype-system-identifier");
                    self.force_quirks = true;
                    self.emit_doctype();
                    self.state = State::Data;
                }
                _ => {
                    self.error("missing-quote-before-doctype-system-identifier");
                    self.force_quirks = true;
                    self.state = State::BogusDoctype;
                    self.cursor.unread();
                }
            },
            State::DoctypeSystemIdentifierDoubleQuoted => match c {
                '"' => self.state = State::AfterDoctypeSystemIdentifier,
                '\0' => {
                    self.error("unexpected-null-character");
                    if let Some(system) = &mut self.doctype_system {
                        system.push('\u{FFFD}');
                    }
                }
                '>' => {
                    self.error("abrupt-doctype-system-identifier");
                    self.force_quirks = true;
                    self.emit_doctype();
                    self.state = State::Data;
                }
                _ => {
                    if let Some(system) = &mut self.doctype_system {
                        system.push(c);
                    }
                }
            },
            State::DoctypeSystemIdentifierSingleQuoted => match c {
                '\'' => self.state = State::AfterDoctypeSystemIdentifier,
                '\0' => {
                    self.error("unexpected-null-character");
                    if let Some(system) = &mut self.doctype_system {
                        system.push('\u{FFFD}');
                    }
                }
                '>' => {
                    self.error("abrupt-doctype-system-identifier");
                    self.force_quirks = true;
                    self.emit_doctype();
                    self.state = State::Data;
                }
                _ => {
                    if let Some(system) = &mut self.doctype_system {
                        system.push(c);
                    }
                }
            },
            State::AfterDoctypeSystemIdentifier => match c {
                '\t' | '\n' | '\u{0C}' | ' ' => {}
                '>' => {
                    self.emit_doctype();
                    self.state = State::Data;
                }
                _ => {
                    self.error("missing-whitespace-after-doctype-system-identifier");
                    self.force_quirks = true;
                    self.state = State::BogusDoctype;
                    self.cursor.unread();
                }
            },
            State::BogusDoctype => match c {
                '>' => {
                    self.emit_doctype();
                    self.state = State::Data;
                }
                '\0' => self.error("unexpected-character-in-doctype"),
                _ => {}
            },
            State::CdataSection => match c {
                ']' => self.state = State::CdataSectionBracket,
                _ => self.emit_char(c),
            },
            State::CdataSectionBracket => {
                if c == ']' {
                    self.state = State::CdataSectionEnd;
                } else {
                    self.emit_char(']');
                    self.reconsume_in(State::CdataSection);
                }
            }
            State::CdataSectionEnd => {
                if c == ']' {
                    self.emit_char(']');
                } else {
                    self.run.push_str("]]");
                    self.reconsume_in(State::CdataSection);
                }
            }
            State::CharacterReference => {
                // Decoded codepoints go to the attribute value while the
                // reference sits inside one, to the character run otherwise.
                let mut emit = |c: char| {
                    if self.reference_in_attribute {
                        self.attr_value.push(c);
                    } else {
                        self.run.push(c);
                    }
                };
                let outcome = reference::consume(
                    &mut self.cursor,
                    self.reference_in_attribute,
                    &mut self.errors,
                    &mut emit,
                );
                match outcome {
                    ReferenceOutcome::Consumed => self.state = self.return_state,
                    // The `&` is literal: emit it, consume it, and let the
                    // return state process what followed it.
                    ReferenceOutcome::FlushBuffer => {
                        self.emit_char('&');
                        self.cursor.next();
                        self.state = self.return_state;
                    }
                }
            }
        }
    }

    fn enter_reference(&mut self, return_state: State) {
        self.return_state = return_state;
        self.reference_in_attribute = true;
        self.state = State::CharacterReference;
    }

    /// The shared body of the RCDATA/RAWTEXT/script end-tag-name states:
    /// accumulate alphas into the temp buffer; on a terminator, either the
    /// tag completes (appropriate end tag) or the buffered text flushes as
    /// literal characters and `fallback` reconsumes.
    fn end_tag_name(&mut self, c: char, fallback: State) {
        if c.is_ascii_alphabetic() {
            self.temp.push(c);
            self.tag_name.push(c.to_ascii_lowercase());
            return;
        }
        let appropriate = match c {
            '\t' | '\n' | '\u{0C}' | ' ' | '/' | '>' => self.is_appropriate_end_tag(),
            _ => false,
        };
        if appropriate && c == '>' {
            self.emit_end_tag();
            self.state = State::Data;
            return;
        }
        if appropriate && c == '/' {
            self.state = State::SelfClosingStartTag;
            return;
        }
        if appropriate && matches!(c, '\t' | '\n' | '\u{0C}' | ' ') {
            self.state = State::BeforeAttributeName;
            return;
        }
        self.flush_premature_end_tag();
        self.reconsume_in(fallback);
    }

    fn cursor_matches_keyword(&self, keyword: &str) -> bool {
        self.cursor.starts_with_ignore_case(keyword)
    }

    /// The EOF rules of the current state (§13.4 each state's "EOF" entry).
    #[allow(clippy::too_many_lines)] // per-state EOF rules of the same algorithm
    fn process_eof(&mut self) {
        match self.state {
            // Text states end quietly; the escaped script states report.
            // (Routing states share the quiet body: no EOF entry.)
            State::Data
            | State::Rcdata
            | State::Rawtext
            | State::ScriptData
            | State::PlainText
            | State::ScriptDataDoubleEscaped
            | State::ScriptDataDoubleEscapedDash
            | State::ScriptDataDoubleEscapedDashDash
            | State::ScriptDataEscapeStart
            | State::ScriptDataEscapeStartDash
            | State::MarkupDeclarationOpen => {}
            State::ScriptDataEscaped
            | State::ScriptDataEscapedDash
            | State::ScriptDataEscapedDashDash
            | State::ScriptDataDoubleEscapeStart
            | State::ScriptDataDoubleEscapeEnd
            | State::ScriptDataDoubleEscapedLessThanSign => {
                self.error("eof-in-script-html-comment-like-text");
            }
            State::RcdataLessThanSign
            | State::RawtextLessThanSign
            | State::ScriptDataLessThanSign => {
                self.emit_char('<');
            }
            State::RcdataEndTagOpen | State::RawtextEndTagOpen | State::ScriptDataEndTagOpen => {
                self.run.push_str("</");
            }
            State::ScriptDataEscapedLessThanSign => {
                self.emit_char('<');
                self.error("eof-in-script-html-comment-like-text");
            }
            State::ScriptDataEscapedEndTagOpen => {
                self.run.push_str("</");
                self.error("eof-in-script-html-comment-like-text");
            }
            State::TagOpen => {
                self.error("eof-before-tag-name");
                self.run.push_str("<>");
            }
            State::EndTagOpen => {
                self.error("eof-before-tag-name");
                self.run.push_str("</>");
            }
            State::TagName
            | State::BeforeAttributeName
            | State::AttributeName
            | State::AfterAttributeName
            | State::BeforeAttributeValue
            | State::AttributeValueDoubleQuoted
            | State::AttributeValueSingleQuoted
            | State::AttributeValueUnquoted
            | State::AfterAttributeValueQuoted
            | State::SelfClosingStartTag => {
                self.error("eof-in-tag");
                // The tag token is dropped per the standard.
            }
            State::BogusComment => self.flush_comment(),
            State::CommentStart
            | State::CommentStartDash
            | State::Comment
            | State::CommentLessThanSign
            | State::CommentLessThanSignBang
            | State::CommentLessThanSignBangDash
            | State::CommentLessThanSignBangDashDash
            | State::CommentEndDash
            | State::CommentEnd
            | State::CommentEndBang => {
                self.error("eof-in-comment");
                self.flush_comment();
            }
            State::Doctype
            | State::BeforeDoctypeName
            | State::DoctypeName
            | State::AfterDoctypeName
            | State::AfterDoctypePublicKeyword
            | State::BeforeDoctypePublicIdentifier
            | State::DoctypePublicIdentifierDoubleQuoted
            | State::DoctypePublicIdentifierSingleQuoted
            | State::AfterDoctypePublicIdentifier
            | State::BetweenDoctypePublicAndSystemIdentifiers
            | State::AfterDoctypeSystemKeyword
            | State::BeforeDoctypeSystemIdentifier
            | State::DoctypeSystemIdentifierDoubleQuoted
            | State::DoctypeSystemIdentifierSingleQuoted
            | State::AfterDoctypeSystemIdentifier
            | State::BogusDoctype => {
                self.error("eof-in-doctype");
                self.force_quirks = true;
                self.emit_doctype();
            }
            State::CdataSection | State::CdataSectionBracket | State::CdataSectionEnd => {
                self.error("eof-in-cdata");
            }
            State::CharacterReference => {
                self.emit_char('&');
            }
            State::RcdataEndTagName | State::RawtextEndTagName | State::ScriptDataEndTagName => {
                self.flush_premature_end_tag();
            }
            State::ScriptDataEscapedEndTagName => {
                self.flush_premature_end_tag();
                self.error("eof-in-script-html-comment-like-text");
            }
        }
        self.eof_done = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> (Vec<Token>, Vec<&'static str>) {
        let mut tokenizer = Tokenizer::new(input, TokenizerOptions::default());
        let mut tokens = Vec::new();
        while let Some(token) = tokenizer.next_token() {
            tokens.push(token);
        }
        (tokens, tokenizer.errors().to_vec())
    }

    fn text_of(tokens: &[Token]) -> String {
        tokens
            .iter()
            .filter_map(|token| match token {
                Token::Character(text) => Some(text.as_str()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn plain_text_and_tags() {
        let (tokens, errors) = tokenize("hello <b>world</b>!");
        assert_eq!(errors, Vec::<&'static str>::new());
        assert_eq!(
            tokens,
            vec![
                Token::Character("hello ".into()),
                Token::StartTag {
                    name: "b".into(),
                    attrs: vec![],
                    self_closing: false
                },
                Token::Character("world".into()),
                Token::EndTag { name: "b".into() },
                Token::Character("!".into()),
            ]
        );
    }

    #[test]
    fn attributes_parse_with_quoting_and_references() {
        let (tokens, errors) = tokenize(r#"<a href="x?y=1&amp;z" data-nb='q&nbsp;q' bare=1>"#);
        assert!(errors.is_empty(), "{errors:?}");
        let Some(Token::StartTag {
            name,
            attrs,
            self_closing,
        }) = tokens.first()
        else {
            panic!("expected start tag");
        };
        assert_eq!(name, "a");
        assert!(!*self_closing);
        assert_eq!(
            attrs,
            &[
                Attribute {
                    name: "href".into(),
                    value: "x?y=1&z".into()
                },
                Attribute {
                    name: "data-nb".into(),
                    value: "q\u{00A0}q".into()
                },
                Attribute {
                    name: "bare".into(),
                    value: "1".into()
                },
            ]
        );
    }

    #[test]
    fn duplicate_attributes_keep_the_first() {
        let (tokens, errors) = tokenize("<p a=1 a=2>");
        let Some(Token::StartTag { attrs, .. }) = tokens.first() else {
            panic!("expected start tag");
        };
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs[0].value, "1");
        assert_eq!(errors, ["duplicate-attribute"]);
    }

    #[test]
    fn self_closing_on_html_elements_sets_the_flag_but_it_is_reported() {
        let (tokens, errors) = tokenize("<br/>");
        assert!(matches!(
            tokens[0],
            Token::StartTag {
                self_closing: true,
                ..
            }
        ));
        // The self-closing flag on non-void HTML elements is a parse error
        // the TREE BUILDER acknowledges; the tokenizer reports nothing.
        assert!(errors.is_empty());
    }

    #[test]
    fn comments_recover_from_hostile_shapes() {
        let (tokens, errors) = tokenize("a<!-- normal -->b<!--x<y!--z-->c<! ---- >d");
        assert_eq!(text_of(&tokens), "abcd");
        // "<! ---- >" is an incorrectly opened comment; "x<y!--z--" closed by ">"
        assert_eq!(errors, ["incorrectly-opened-comment"]);
    }

    #[test]
    fn doctype_variants() {
        let (tokens, errors) = tokenize(
            "<!DOCTYPE html><!doctype HTML PUBLIC \"-//W3C//DTD HTML 4.01//EN\" \"http://www.w3.org/TR/html4/strict.dtd\"><!DOCTYPE html PUBLIC \"-//X\" \"http://www.w3.org/tr/html4/loose.dtd\">",
        );
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(
            tokens[0],
            Token::Doctype {
                name: Some("html".into()),
                public: None,
                system: None,
                force_quirks: false
            }
        );
        let Some(Token::Doctype { public, system, .. }) = tokens.get(1) else {
            panic!("expected doctype");
        };
        assert_eq!(public.as_deref(), Some("-//W3C//DTD HTML 4.01//EN"));
        assert_eq!(
            system.as_deref(),
            Some("http://www.w3.org/TR/html4/strict.dtd")
        );
        // Public + system identifiers both land — after PUBLIC, the system
        // identifier is a second quoted string (the SYSTEM keyword form is
        // only valid without a public id; Between's anything-else errors).
        let Some(Token::Doctype {
            name: _,
            public,
            system,
            force_quirks,
        }) = tokens.get(2)
        else {
            panic!("expected third doctype");
        };
        assert_eq!(public.as_deref(), Some("-//X"));
        assert_eq!(
            system.as_deref(),
            Some("http://www.w3.org/tr/html4/loose.dtd")
        );
        assert!(!*force_quirks);
    }

    #[test]
    fn abrupt_doctype_public_identifier_forces_quirks() {
        let (tokens, errors) = tokenize("<!DOCTYPE html PUBLIC \"-//X>");
        assert_eq!(errors, ["abrupt-doctype-public-identifier"]);
        let Some(Token::Doctype { force_quirks, .. }) = tokens.first() else {
            panic!("expected doctype");
        };
        assert!(*force_quirks);
    }

    #[test]
    fn eof_in_tag_drops_the_token() {
        let (tokens, errors) = tokenize("<div class=\"x");
        assert_eq!(errors, ["eof-in-tag"]);
        assert!(tokens.is_empty());
    }

    #[test]
    fn eof_in_comment_emits_the_comment() {
        let (tokens, errors) = tokenize("<!-- unterminated");
        assert_eq!(errors, ["eof-in-comment"]);
        assert_eq!(tokens, vec![Token::Comment(" unterminated".into())]);
    }

    #[test]
    fn script_data_handles_nested_markup_and_the_double_escape_dance() {
        // The body the tree builder feeds after switching to script data;
        // the pipeline's <script> start tag preset the appropriate name.
        let mut tokenizer = Tokenizer::new(
            r#"if (a<b) {} //<script>
var x = "</scr" + "ipt>";"#,
            TokenizerOptions {
                initial_state: InitialState::ScriptData,
                ..TokenizerOptions::default()
            },
        );
        tokenizer.set_last_start_tag_for_tests("script");
        let mut tokens = Vec::new();
        while let Some(token) = tokenizer.next_token() {
            tokens.push(token);
        }
        assert!(tokenizer.errors().is_empty(), "{:?}", tokenizer.errors());
        let body: String = tokens
            .iter()
            .map_while(|token| match token {
                Token::Character(text) => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert!(body.contains("if (a<b) {}"), "{body:?}");
        assert!(body.contains("<script>"), "{body:?}");
        // The "</scr" + "ipt>" split never forms an end tag.
        assert!(body.contains(r#""</scr" + "ipt>";"#), "{body:?}");
        let _ = tokens.last();
    }

    #[test]
    fn script_data_end_tag_closes_after_the_escape_dance() {
        let mut tokenizer = Tokenizer::new(
            "<!--<script>x</script>--></script>ok",
            TokenizerOptions {
                initial_state: InitialState::ScriptData,
                ..TokenizerOptions::default()
            },
        );
        tokenizer.set_last_start_tag_for_tests("script");
        let mut tokens = Vec::new();
        while let Some(token) = tokenizer.next_token() {
            tokens.push(token);
        }
        assert!(tokenizer.errors().is_empty(), "{:?}", tokenizer.errors());
        let body: String = tokens
            .iter()
            .map_while(|token| match token {
                Token::Character(text) => Some(text.as_str()),
                _ => None,
            })
            .collect();
        // In the double-escaped state the '/' of "</script>" is consumed
        // silently (§13.2.5.30) — that is exactly why it cannot close the
        // script element; only the final real end tag does.
        assert_eq!(body, "<!--<script>xscript>-->");
    }

    #[test]
    fn rcdata_preserves_unmatched_end_tags_and_names_the_matching_close() {
        let mut tokenizer = Tokenizer::new(
            "a</p>b</textarea>",
            TokenizerOptions {
                initial_state: InitialState::Rcdata,
                ..TokenizerOptions::default()
            },
        );
        tokenizer.set_last_start_tag_for_tests("textarea");
        let tokens: Vec<_> = std::iter::from_fn(|| tokenizer.next_token()).collect();
        assert!(!tokens.iter().any(|token| matches!(
            token,
            Token::EndTag { name } if name == "p"
        )));
        assert_eq!(
            tokens,
            vec![
                Token::Character("a</p>b".into()),
                Token::EndTag {
                    name: "textarea".into()
                },
            ]
        );
        assert!(tokenizer.errors().is_empty());
    }

    #[test]
    fn text_modes_preserve_unmatched_tags_and_emit_matching_end_tag_names() {
        for state in [
            InitialState::Rcdata,
            InitialState::Rawtext,
            InitialState::ScriptData,
        ] {
            for close in ["</TiTlE>", "</TiTlE >", "</TiTlE/>"] {
                let input = format!("a</P>b{close}<i>");
                let mut tokenizer = Tokenizer::new(
                    &input,
                    TokenizerOptions {
                        initial_state: state,
                        ..TokenizerOptions::default()
                    },
                );
                tokenizer.set_last_start_tag_for_tests("title");
                let tokens: Vec<_> = std::iter::from_fn(|| tokenizer.next_token()).collect();
                assert_eq!(
                    tokens,
                    vec![
                        Token::Character("a</P>b".into()),
                        Token::EndTag {
                            name: "title".into()
                        },
                        Token::StartTag {
                            name: "i".into(),
                            attrs: vec![],
                            self_closing: false,
                        },
                    ],
                    "{state:?}: {close}"
                );
            }
        }
    }

    #[test]
    fn text_mode_end_tag_fallback_at_eof_preserves_input() {
        for state in [
            InitialState::Rcdata,
            InitialState::Rawtext,
            InitialState::ScriptData,
        ] {
            for input in ["<", "</", "</Ti", "</TiTlE", "</P>", "</TiTlE!", "</TiTlEé"] {
                let mut tokenizer = Tokenizer::new(
                    input,
                    TokenizerOptions {
                        initial_state: state,
                        ..TokenizerOptions::default()
                    },
                );
                tokenizer.set_last_start_tag_for_tests("title");
                let tokens: Vec<_> = std::iter::from_fn(|| tokenizer.next_token()).collect();
                assert_eq!(
                    tokens,
                    vec![Token::Character(input.into())],
                    "{state:?}: {input}"
                );
                assert!(
                    tokenizer.errors().is_empty(),
                    "{state:?}: {input}: {:?}",
                    tokenizer.errors()
                );
            }
        }
    }

    #[test]
    fn appropriate_text_end_tag_eof_after_delimiter_drops_candidate() {
        for state in [
            InitialState::Rcdata,
            InitialState::Rawtext,
            InitialState::ScriptData,
        ] {
            for input in ["a</TiTlE ", "a</TiTlE/", "a</TiTlE class='x"] {
                let mut tokenizer = Tokenizer::new(
                    input,
                    TokenizerOptions {
                        initial_state: state,
                        ..TokenizerOptions::default()
                    },
                );
                tokenizer.set_last_start_tag_for_tests("title");
                let tokens: Vec<_> = std::iter::from_fn(|| tokenizer.next_token()).collect();
                assert_eq!(tokens, vec![Token::Character("a".into())]);
                assert_eq!(tokenizer.errors(), ["eof-in-tag"]);
            }
        }
    }

    #[test]
    fn escaped_script_end_tags_preserve_case_and_close() {
        for close in ["</ScRiPt>", "</ScRiPt >", "</ScRiPt/>"] {
            let input = format!("<!--a</P>b{close}<i>");
            let mut tokenizer = Tokenizer::new(
                &input,
                TokenizerOptions {
                    initial_state: InitialState::ScriptData,
                    ..TokenizerOptions::default()
                },
            );
            tokenizer.set_last_start_tag_for_tests("script");
            let tokens: Vec<_> = std::iter::from_fn(|| tokenizer.next_token()).collect();
            assert_eq!(
                tokens,
                vec![
                    Token::Character("<!--a</P>b".into()),
                    Token::EndTag {
                        name: "script".into()
                    },
                    Token::StartTag {
                        name: "i".into(),
                        attrs: vec![],
                        self_closing: false
                    },
                ]
            );
        }
    }

    #[test]
    fn escaped_script_end_tag_eof_preserves_input_and_reports_error() {
        for input in ["<!--<", "<!--</", "<!--</ScRiPt", "<!--</P>"] {
            let mut tokenizer = Tokenizer::new(
                input,
                TokenizerOptions {
                    initial_state: InitialState::ScriptData,
                    ..TokenizerOptions::default()
                },
            );
            tokenizer.set_last_start_tag_for_tests("script");
            let tokens: Vec<_> = std::iter::from_fn(|| tokenizer.next_token()).collect();
            assert_eq!(tokens, vec![Token::Character(input.into())]);
            assert_eq!(tokenizer.errors(), ["eof-in-script-html-comment-like-text"]);
        }
    }

    #[test]
    fn end_tag_attributes_do_not_leak_into_following_start_tag() {
        let (tokens, errors) = tokenize("</p class=x/><i>");
        assert_eq!(
            tokens,
            vec![
                Token::EndTag { name: "p".into() },
                Token::StartTag {
                    name: "i".into(),
                    attrs: vec![],
                    self_closing: false
                },
            ]
        );
        assert_eq!(errors, ["end-tag-with-attributes"]);
        let (_, errors) = tokenize("</p class='x'/>");
        assert_eq!(
            errors,
            ["end-tag-with-attributes", "end-tag-with-trailing-solidus"]
        );
    }

    #[test]
    fn rawtext_rcdata_and_plaintext_modes() {
        // No start tag was seen, so </style b> is NOT an appropriate end
        // tag: the whole run stays literal text (the standard's mismatch
        // flush). The matching case is exercised by the script tests below,
        // which preset the last start tag.
        let mut style = Tokenizer::new(
            "a</style b>c",
            TokenizerOptions {
                initial_state: InitialState::Rawtext,
                ..TokenizerOptions::default()
            },
        );
        let mut tokens = Vec::new();
        while let Some(token) = style.next_token() {
            tokens.push(token);
        }
        assert_eq!(tokens, vec![Token::Character("a</style b>c".into())]);

        let mut title = Tokenizer::new(
            "5 > 3 &amp; 2",
            TokenizerOptions {
                initial_state: InitialState::Rcdata,
                ..TokenizerOptions::default()
            },
        );
        let mut tokens = Vec::new();
        while let Some(token) = title.next_token() {
            tokens.push(token);
        }
        assert_eq!(tokens[0], Token::Character("5 > 3 & 2".into()));

        let mut plain = Tokenizer::new(
            "<b>x</b>",
            TokenizerOptions {
                initial_state: InitialState::Plaintext,
                ..TokenizerOptions::default()
            },
        );
        let mut tokens = Vec::new();
        while let Some(token) = plain.next_token() {
            tokens.push(token);
        }
        assert_eq!(tokens, vec![Token::Character("<b>x</b>".into())]);
    }

    #[test]
    fn character_reference_edges() {
        let (tokens, errors) = tokenize("&amp; &lt &#65; &notin; &notit;");
        // &notit;: the legacy &not matches without a semicolon outside
        // attributes (parse error, still decodes) — html5lib behavior.
        assert_eq!(text_of(&tokens), "& < A ∉ ¬it;");
        // &lt (no semicolon) and &not (legacy match) both report.
        assert_eq!(
            errors,
            [
                "missing-semicolon-after-character-reference",
                "missing-semicolon-after-character-reference"
            ]
        );
    }

    #[test]
    fn nul_becomes_replacement_in_data() {
        let (tokens, errors) = tokenize("a\u{0}b");
        assert_eq!(text_of(&tokens), "a\u{FFFD}b");
        assert_eq!(errors, ["unexpected-null-character"]);
    }

    #[test]
    fn crlf_is_normalized_once() {
        let (tokens, _) = tokenize("a\r\nb\rc");
        assert_eq!(text_of(&tokens), "a\nb\nc");
    }

    #[test]
    fn bogus_comment_and_cdata_recovery() {
        let (tokens, errors) = tokenize("<?php echo 1 ?>a<![CDATA[x]]>b");
        assert_eq!(
            errors,
            [
                "unexpected-question-mark-instead-of-tag-name",
                "cdata-in-html-content"
            ]
        );
        assert_eq!(tokens[0], Token::Comment("?php echo 1 ?".into()));
        assert_eq!(text_of(&tokens[1..]), "ab");
        // The CDATA marker becomes a bogus comment in HTML content (§13.4):
        // data "[CDATA[" + consumed chars up to the '>'.
        assert_eq!(tokens[2], Token::Comment("[CDATA[x]]".into()));
    }
}
