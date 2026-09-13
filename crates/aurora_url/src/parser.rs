//! The basic URL parser (URL Standard §4.4 "basic URL parser").
//!
//! Each state below translates the standard's numbered steps one for one,
//! keeping the standard's state names. Pure validation errors (non-fatal
//! per the standard) are not collected yet — the parse-error report arrives
//! with `DevTools` (WBS §6.2). Pointer model: a state processes the code point
//! at the pointer; "decrease pointer by k" rewinds so the following state
//! reprocesses that input; the machine exits when a run leaves the pointer
//! at the EOF code point.
//!
//! States are grouped into methods by their spec neighborhood (§9.2's state
//! seams): scheme handling, slash tolerance, authority/host/port, file,
//! path, query/fragment, relative.

use crate::host::{self, Host, HostError};
use crate::percent::{self, C0_CONTROL, FRAGMENT, PATH, QUERY, SPECIAL_QUERY, USERINFO};
use crate::url::{Path, Url, default_port};

/// Parse failures (URL Standard §4.4 failure causes, §5.1's names).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
    /// Relative input with no base URL (missing-scheme-non-relative-URL).
    RelativeWithoutBase,
    /// The host parser rejected a domain or IPv4 literal.
    InvalidDomainChar,
    /// The host parser rejected an IPv6 literal.
    InvalidIpv6,
    /// Bracketed host input did not end with `]`.
    InvalidIpv6Unclosed,
    /// A non-special scheme's host contained a forbidden code point.
    HostInvalid,
    /// The port is out of range or contains non-digits.
    InvalidPort,
    /// A special URL has a missing or empty host (host-missing).
    EmptyHost,
    /// Non-ASCII domains are unsupported until IDNA lands.
    IdnaError,
    /// An absolute URL needs a scheme (missing-scheme-non-relative-URL).
    MissingSchemeNonRelative,
}

/// Parser states, named after the standard's states (§4.4).
#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    SchemeStart,
    Scheme,
    NoScheme,
    SpecialRelativeOrAuthority,
    PathOrAuthority,
    SpecialAuthoritySlashes,
    SpecialAuthorityIgnoreSlashes,
    Authority,
    Host,
    Port,
    File,
    FileSlash,
    FileHost,
    PathStart,
    Path,
    OpaquePath,
    Query,
    Fragment,
    Relative,
    RelativeSlash,
}

struct Parser<'a> {
    input: Vec<char>,
    base: Option<&'a Url>,
    pointer: isize,
    buffer: String,
    at_sign_seen: bool,
    inside_brackets: bool,
    password_token_seen: bool,
    url: Url,
}

/// Runs the basic URL parser. `input` must already be trimmed of leading and
/// trailing C0 controls and spaces, with ASCII tab and newline removed (§4.4
/// steps 2–5; the caller applies them).
pub fn run(input: &str, base: Option<&Url>) -> Result<Url, ParseError> {
    let trimmed = input.trim_matches(|c: char| c <= ' ');
    let mut cleaned = String::with_capacity(trimmed.len());
    cleaned.extend(trimmed.chars().filter(|c| !matches!(c, '\t' | '\n' | '\r')));
    let parser = Parser {
        input: cleaned.chars().collect(),
        base,
        pointer: 0,
        buffer: String::new(),
        at_sign_seen: false,
        inside_brackets: false,
        password_token_seen: false,
        url: Url::empty_record(),
    };
    parser.loop_states()
}

impl<'a> Parser<'a> {
    /// The state machine driver (§4.4: run a state, exit at EOF, else
    /// advance the pointer by one code point).
    fn loop_states(mut self) -> Result<Url, ParseError> {
        let mut state = State::SchemeStart;
        loop {
            let c = self.peek();
            state = self.run_state(state, c)?;
            if self.pointer >= self.input_len() {
                break;
            }
            self.pointer += 1;
        }
        Ok(self.url)
    }

    fn run_state(&mut self, state: State, c: Option<char>) -> Result<State, ParseError> {
        match state {
            State::SchemeStart | State::Scheme | State::NoScheme => self.scheme_group(state, c),
            State::SpecialRelativeOrAuthority | State::PathOrAuthority => {
                self.special_or_authority_group(state, c)
            }
            State::SpecialAuthoritySlashes | State::SpecialAuthorityIgnoreSlashes => {
                self.slash_tolerance_group(state, c)
            }
            State::Authority | State::Host | State::Port => self.authority_group(state, c),
            State::File | State::FileSlash | State::FileHost => self.file_group(state, c),
            State::PathStart | State::Path | State::OpaquePath => self.path_group(state, c),
            State::Query | State::Fragment => self.query_fragment_group(state, c),
            State::Relative | State::RelativeSlash => self.relative_group(state, c),
        }
    }

    // ---- pointer helpers -------------------------------------------------

    fn peek(&self) -> Option<char> {
        self.input.get(self.cursor()).copied()
    }

    fn input_len(&self) -> isize {
        isize::try_from(self.input.len()).unwrap_or(isize::MAX)
    }

    /// The pointer clamped into a valid index; it is only ever negative
    /// transiently, before the loop's advance step lands it back on 0.
    fn cursor(&self) -> usize {
        usize::try_from(self.pointer.max(0)).unwrap_or(0)
    }

    /// Whether the code points starting `offset` past the current one match
    /// `pattern` ("remaining starts with …" in the standard).
    fn remaining_starts_with(&self, offset: isize, pattern: &str) -> bool {
        let start = usize::try_from((self.pointer + offset).max(0)).unwrap_or(0);
        let rest = &self.input[start..];
        rest.len() >= pattern.chars().count()
            && rest
                .iter()
                .copied()
                .zip(pattern.chars())
                .all(|(a, b)| a == b)
    }

    /// The code point substring from the pointer to the end of input,
    /// including the current code point (used by the file states).
    fn rest_from_pointer(&self) -> String {
        self.input[self.cursor()..].iter().collect()
    }

    // ---- shared steps ----------------------------------------------------

    /// "Shorten a url's path" (§4.2): keep a lone normalized Windows drive
    /// letter of a file URL; otherwise remove the last segment, if any.
    fn shorten_path(&mut self) {
        if self.url.scheme == "file"
            && let Path::Segments(segments) = &self.url.path
            && segments.len() == 1
            && is_normalized_windows_drive_letter(&segments[0])
        {
            return;
        }
        if let Path::Segments(segments) = &mut self.url.path {
            segments.pop();
        }
    }

    fn host_error(error: HostError) -> ParseError {
        match error {
            HostError::EmptyHost => ParseError::EmptyHost,
            HostError::InvalidIpv4 | HostError::InvalidDomainCharacter => {
                ParseError::InvalidDomainChar
            }
            HostError::InvalidIpv6 => ParseError::InvalidIpv6,
            HostError::Ipv6Unclosed => ParseError::InvalidIpv6Unclosed,
            HostError::InvalidHostCharacter => ParseError::HostInvalid,
            HostError::IdnaUnsupported => ParseError::IdnaError,
        }
    }

    /// Host-parses `self.buffer` with `isOpaque = url is not special`.
    fn parse_host_buffer(&mut self) -> Result<Host, ParseError> {
        let is_opaque = !self.url.is_special();
        host::parse(&self.buffer, is_opaque).map_err(Self::host_error)
    }

    /// Whether `c` ends the authority/host/port scan: EOF, `/`, `?`, `#`,
    /// or — for special URLs — `\`.
    fn is_component_terminator(&self, c: Option<char>) -> bool {
        match c {
            None => true,
            Some(ch) => matches!(ch, '/' | '?' | '#') || (self.url.is_special() && ch == '\\'),
        }
    }

    /// Invariant helper: the `Relative` states are only entered via `NoScheme`
    /// with a non-null, hierarchical base.
    fn parse_base(&self) -> Result<&'a Url, ParseError> {
        self.base.ok_or(ParseError::RelativeWithoutBase)
    }

    /// Applies the buffered port digits (port state's terminator step).
    fn flush_port(&mut self) -> Result<(), ParseError> {
        if !self.buffer.is_empty() {
            let port: u32 = self.buffer.parse().map_err(|_| ParseError::InvalidPort)?;
            let port = u16::try_from(port).map_err(|_| ParseError::InvalidPort)?;
            self.url.port = if Some(port) == default_port(&self.url.scheme) {
                None
            } else {
                Some(port)
            };
            self.buffer.clear();
        }
        Ok(())
    }

    /// Applies the buffered query (query state's terminator step): percent-
    /// encode with the special-query set for special URLs, query set else.
    fn flush_query(&mut self) {
        let set = if self.url.is_special() {
            &SPECIAL_QUERY
        } else {
            &QUERY
        };
        self.url.query = Some(percent::percent_encode(&self.buffer, set));
        self.buffer.clear();
    }

    // ---- state groups ----------------------------------------------------

    fn scheme_group(&mut self, state: State, c: Option<char>) -> Result<State, ParseError> {
        match state {
            State::SchemeStart => match c {
                Some(alpha) if alpha.is_ascii_alphabetic() => {
                    self.buffer.push(alpha.to_ascii_lowercase());
                    Ok(State::Scheme)
                }
                _ => {
                    self.pointer -= 1;
                    Ok(State::NoScheme)
                }
            },
            State::Scheme => match c {
                Some(ch) if ch.is_ascii_alphanumeric() || matches!(ch, '+' | '-' | '.') => {
                    self.buffer.push(ch.to_ascii_lowercase());
                    Ok(State::Scheme)
                }
                Some(':') => {
                    self.url.scheme = std::mem::take(&mut self.buffer);
                    if self.url.scheme == "file" {
                        // A missing "//" is a validation error only.
                        Ok(State::File)
                    } else if self.url.is_special()
                        && self.base.is_some_and(|base| base.scheme == self.url.scheme)
                    {
                        Ok(State::SpecialRelativeOrAuthority)
                    } else if self.url.is_special() {
                        Ok(State::SpecialAuthoritySlashes)
                    } else if self.remaining_starts_with(1, "/") {
                        self.pointer += 1;
                        Ok(State::PathOrAuthority)
                    } else {
                        self.url.path = Path::Opaque(String::new());
                        Ok(State::OpaquePath)
                    }
                }
                _ => {
                    // Not a scheme after all: restart from input[0] in the
                    // no-scheme state ("start over"); the loop's advance
                    // lands the pointer back on the first code point.
                    self.buffer.clear();
                    self.pointer = -1;
                    Ok(State::NoScheme)
                }
            },
            State::NoScheme => {
                let Some(base) = self.base else {
                    return Err(ParseError::RelativeWithoutBase);
                };
                if base.cannot_be_a_base() && c != Some('#') {
                    return Err(ParseError::MissingSchemeNonRelative);
                }
                if base.cannot_be_a_base() {
                    self.url.scheme.clone_from(&base.scheme);
                    self.url.path = base.path.clone();
                    self.url.query = base.query.clone();
                    self.url.fragment = Some(String::new());
                    Ok(State::Fragment)
                } else if base.scheme != "file" {
                    self.pointer -= 1;
                    Ok(State::Relative)
                } else {
                    self.pointer -= 1;
                    Ok(State::File)
                }
            }
            _ => Err(group_routing_bug(state)),
        }
    }

    fn special_or_authority_group(
        &mut self,
        state: State,
        c: Option<char>,
    ) -> Result<State, ParseError> {
        match state {
            State::SpecialRelativeOrAuthority => {
                if c == Some('/') && self.remaining_starts_with(1, "/") {
                    self.pointer += 1;
                    Ok(State::SpecialAuthorityIgnoreSlashes)
                } else {
                    // Validation error; the input is treated as relative.
                    self.pointer -= 1;
                    Ok(State::Relative)
                }
            }
            State::PathOrAuthority => {
                if c == Some('/') {
                    Ok(State::Authority)
                } else {
                    self.pointer -= 1;
                    Ok(State::Path)
                }
            }
            _ => Err(group_routing_bug(state)),
        }
    }

    fn slash_tolerance_group(
        &mut self,
        state: State,
        c: Option<char>,
    ) -> Result<State, ParseError> {
        match state {
            State::SpecialAuthoritySlashes => {
                if c == Some('/') && self.remaining_starts_with(1, "/") {
                    self.pointer += 1;
                    Ok(State::SpecialAuthorityIgnoreSlashes)
                } else {
                    self.pointer -= 1;
                    Ok(State::SpecialAuthorityIgnoreSlashes)
                }
            }
            State::SpecialAuthorityIgnoreSlashes => {
                if matches!(c, Some('/' | '\\')) {
                    // Extra slashes are validation errors; consume them.
                    Ok(State::SpecialAuthorityIgnoreSlashes)
                } else {
                    self.pointer -= 1;
                    Ok(State::Authority)
                }
            }
            _ => Err(group_routing_bug(state)),
        }
    }

    fn authority_group(&mut self, state: State, c: Option<char>) -> Result<State, ParseError> {
        match state {
            State::Authority => {
                if c == Some('@') {
                    if self.at_sign_seen {
                        self.buffer.insert_str(0, "%40");
                    }
                    self.at_sign_seen = true;
                    for cp in self.buffer.chars() {
                        if cp == ':' && !self.password_token_seen {
                            self.password_token_seen = true;
                            continue;
                        }
                        let encoded = percent::percent_encode(&cp.to_string(), &USERINFO);
                        if self.password_token_seen {
                            self.url.password.push_str(&encoded);
                        } else {
                            self.url.username.push_str(&encoded);
                        }
                    }
                    self.buffer.clear();
                    Ok(State::Authority)
                } else if self.is_component_terminator(c) {
                    if self.at_sign_seen && self.buffer.is_empty() {
                        return Err(ParseError::EmptyHost);
                    }
                    // Rewind so the host state re-reads the buffer.
                    let rewind = isize::try_from(self.buffer.chars().count()).unwrap_or(0) + 1;
                    self.pointer -= rewind;
                    self.buffer.clear();
                    Ok(State::Host)
                } else if let Some(ch) = c {
                    self.buffer.push(ch);
                    Ok(State::Authority)
                } else {
                    Ok(State::Authority)
                }
            }
            State::Host => {
                if c == Some(':') && !self.inside_brackets {
                    if self.buffer.is_empty() {
                        return Err(ParseError::EmptyHost);
                    }
                    let host = self.parse_host_buffer()?;
                    self.url.host = Some(host);
                    self.buffer.clear();
                    Ok(State::Port)
                } else if self.is_component_terminator(c) {
                    if self.url.is_special() && self.buffer.is_empty() {
                        return Err(ParseError::EmptyHost);
                    }
                    let host = self.parse_host_buffer()?;
                    self.url.host = Some(host);
                    self.buffer.clear();
                    self.pointer -= 1;
                    Ok(State::PathStart)
                } else if let Some(ch) = c {
                    if ch == '[' {
                        self.inside_brackets = true;
                    }
                    if ch == ']' {
                        self.inside_brackets = false;
                    }
                    self.buffer.push(ch);
                    Ok(State::Host)
                } else {
                    Ok(State::Host)
                }
            }
            State::Port => {
                if let Some(digit) = c.filter(char::is_ascii_digit) {
                    self.buffer.push(digit);
                    Ok(State::Port)
                } else if self.is_component_terminator(c) {
                    self.flush_port()?;
                    self.pointer -= 1;
                    Ok(State::PathStart)
                } else {
                    Err(ParseError::InvalidPort)
                }
            }
            _ => Err(group_routing_bug(state)),
        }
    }

    fn file_group(&mut self, state: State, c: Option<char>) -> Result<State, ParseError> {
        match state {
            State::File => {
                self.url.scheme = "file".into();
                self.url.host = Some(Host::Domain(String::new()));
                if matches!(c, Some('/' | '\\')) {
                    // '\' is a validation error; treated as '/'.
                    Ok(State::FileSlash)
                } else if let Some(base) = self.base.filter(|base| base.scheme == "file") {
                    self.url.host = base.host.clone();
                    self.url.path = base.path.clone();
                    self.url.query = base.query.clone();
                    match c {
                        Some('?') => {
                            self.url.query = Some(String::new());
                            Ok(State::Query)
                        }
                        Some('#') => {
                            self.url.fragment = Some(String::new());
                            Ok(State::Fragment)
                        }
                        _ => {
                            if c.is_some() {
                                self.url.query = None;
                                if starts_with_windows_drive_letter(&self.rest_from_pointer()) {
                                    // Windows drive letter quirk: reset the
                                    // path to empty (validation error).
                                    self.url.path = Path::Segments(Vec::new());
                                } else {
                                    self.shorten_path();
                                }
                            }
                            self.pointer -= 1;
                            Ok(State::Path)
                        }
                    }
                } else {
                    self.pointer -= 1;
                    Ok(State::Path)
                }
            }
            State::FileSlash => {
                if matches!(c, Some('/' | '\\')) {
                    Ok(State::FileHost)
                } else {
                    if let Some(base) = self.base.filter(|base| base.scheme == "file") {
                        self.url.host = base.host.clone();
                        // Windows drive letter quirk: keep the base's drive
                        // segment when the input starts none.
                        if !starts_with_windows_drive_letter(&self.rest_from_pointer())
                            && let Path::Segments(base_segments) = &base.path
                            && let Some(first) = base_segments.first()
                            && is_normalized_windows_drive_letter(first)
                            && let Path::Segments(segments) = &mut self.url.path
                        {
                            segments.push(first.clone());
                        }
                    }
                    self.pointer -= 1;
                    Ok(State::Path)
                }
            }
            State::FileHost => {
                let is_terminator = match c {
                    Some(ch) => matches!(ch, '/' | '\\' | '?' | '#'),
                    None => true,
                };
                if is_terminator {
                    self.pointer -= 1;
                    if buffer_is_windows_drive_letter(&self.buffer) {
                        // Validation error; the buffer is kept for the path
                        // state, which normalizes the `|` to `:`.
                        Ok(State::Path)
                    } else if self.buffer.is_empty() {
                        self.url.host = Some(Host::Domain(String::new()));
                        Ok(State::PathStart)
                    } else {
                        let host = self.parse_host_buffer()?;
                        let host = match host {
                            Host::Domain(d) if d == "localhost" => Host::Domain(String::new()),
                            other => other,
                        };
                        self.url.host = Some(host);
                        self.buffer.clear();
                        Ok(State::PathStart)
                    }
                } else if let Some(ch) = c {
                    self.buffer.push(ch);
                    Ok(State::FileHost)
                } else {
                    Ok(State::FileHost)
                }
            }
            _ => Err(group_routing_bug(state)),
        }
    }

    fn path_group(&mut self, state: State, c: Option<char>) -> Result<State, ParseError> {
        match state {
            State::PathStart => {
                if self.url.is_special() {
                    // '\' is a validation error; treated as '/'.
                    if !matches!(c, Some('/' | '\\')) {
                        self.pointer -= 1;
                    }
                    Ok(State::Path)
                } else if c == Some('?') {
                    self.url.query = Some(String::new());
                    Ok(State::Query)
                } else if c == Some('#') {
                    self.url.fragment = Some(String::new());
                    Ok(State::Fragment)
                } else if c.is_some() {
                    if c != Some('/') {
                        self.pointer -= 1;
                    }
                    Ok(State::Path)
                } else {
                    // EOF with no state override: the path stays as it is.
                    Ok(State::PathStart)
                }
            }
            State::Path => {
                let is_terminator = matches!(c, None | Some('/' | '?' | '#'))
                    || (self.url.is_special() && c == Some('\\'));
                if is_terminator {
                    self.flush_path_segment(c);
                    match c {
                        Some('?') => {
                            self.url.query = Some(String::new());
                            Ok(State::Query)
                        }
                        Some('#') => {
                            self.url.fragment = Some(String::new());
                            Ok(State::Fragment)
                        }
                        _ => Ok(State::Path),
                    }
                } else if let Some(ch) = c {
                    self.buffer
                        .push_str(&percent::percent_encode(&ch.to_string(), &PATH));
                    Ok(State::Path)
                } else {
                    Ok(State::Path)
                }
            }
            State::OpaquePath => match c {
                Some('?') => {
                    self.url.query = Some(String::new());
                    Ok(State::Query)
                }
                Some('#') => {
                    self.url.fragment = Some(String::new());
                    Ok(State::Fragment)
                }
                Some(' ') => {
                    let next_is_delim = self
                        .input
                        .get(self.cursor() + 1)
                        .is_some_and(|next| matches!(next, '?' | '#'));
                    if let Path::Opaque(opaque) = &mut self.url.path {
                        if next_is_delim {
                            opaque.push_str("%20");
                        } else {
                            opaque.push(' ');
                        }
                    }
                    Ok(State::OpaquePath)
                }
                Some(ch) => {
                    if let Path::Opaque(opaque) = &mut self.url.path {
                        opaque.push_str(&percent::percent_encode(&ch.to_string(), &C0_CONTROL));
                    }
                    Ok(State::OpaquePath)
                }
                None => Ok(State::OpaquePath),
            },
            _ => Err(group_routing_bug(state)),
        }
    }

    /// The path state's terminator steps: dot-segment handling and the
    /// Windows drive letter quirk, then the buffer flush.
    fn flush_path_segment(&mut self, c: Option<char>) {
        let boundary_is_slash =
            matches!(c, Some('/')) || (self.url.is_special() && c == Some('\\'));
        if is_double_dot(&self.buffer) {
            self.shorten_path();
            if !boundary_is_slash && let Path::Segments(segments) = &mut self.url.path {
                segments.push(String::new());
            }
        } else if is_single_dot(&self.buffer) {
            if !boundary_is_slash && let Path::Segments(segments) = &mut self.url.path {
                segments.push(String::new());
            }
        } else if self.url.scheme == "file"
            && buffer_is_windows_drive_letter(&self.buffer)
            && matches!(&self.url.path, Path::Segments(s) if s.is_empty())
        {
            // Windows drive letter quirk: `c|` → `c:`.
            self.buffer.replace_range(1..2, ":");
            if let Path::Segments(segments) = &mut self.url.path {
                segments.push(std::mem::take(&mut self.buffer));
            }
        } else if let Path::Segments(segments) = &mut self.url.path {
            segments.push(std::mem::take(&mut self.buffer));
        }
        self.buffer.clear();
    }

    fn query_fragment_group(&mut self, state: State, c: Option<char>) -> Result<State, ParseError> {
        match state {
            State::Query => {
                if matches!(c, Some('#') | None) {
                    self.flush_query();
                    if c == Some('#') {
                        self.url.fragment = Some(String::new());
                        return Ok(State::Fragment);
                    }
                } else if let Some(ch) = c {
                    self.buffer.push(ch);
                }
                Ok(State::Query)
            }
            State::Fragment => {
                if let Some(ch) = c
                    && let Some(fragment) = &mut self.url.fragment
                {
                    fragment.push_str(&percent::percent_encode(&ch.to_string(), &FRAGMENT));
                }
                Ok(State::Fragment)
            }
            _ => Err(group_routing_bug(state)),
        }
    }

    fn relative_group(&mut self, state: State, c: Option<char>) -> Result<State, ParseError> {
        match state {
            State::Relative => {
                let base = self.parse_base()?;
                self.url.scheme.clone_from(&base.scheme);
                if c == Some('/') || (self.url.is_special() && c == Some('\\')) {
                    Ok(State::RelativeSlash)
                } else {
                    self.url.username.clone_from(&base.username);
                    self.url.password.clone_from(&base.password);
                    self.url.host.clone_from(&base.host);
                    self.url.port = base.port;
                    self.url.path = base.path.clone();
                    self.url.query = base.query.clone();
                    match c {
                        Some('?') => {
                            self.url.query = Some(String::new());
                            Ok(State::Query)
                        }
                        Some('#') => {
                            self.url.fragment = Some(String::new());
                            Ok(State::Fragment)
                        }
                        None => Ok(State::Relative),
                        _ => {
                            self.url.query = None;
                            self.shorten_path();
                            self.pointer -= 1;
                            Ok(State::Path)
                        }
                    }
                }
            }
            State::RelativeSlash => {
                let base = self.parse_base()?;
                if self.url.is_special() && matches!(c, Some('/' | '\\')) {
                    Ok(State::SpecialAuthorityIgnoreSlashes)
                } else if c == Some('/') {
                    Ok(State::Authority)
                } else {
                    self.url.username.clone_from(&base.username);
                    self.url.password.clone_from(&base.password);
                    self.url.host.clone_from(&base.host);
                    self.url.port = base.port;
                    self.pointer -= 1;
                    Ok(State::Path)
                }
            }
            _ => Err(group_routing_bug(state)),
        }
    }
}

// Reaching a group's fallback arm means a state was routed to the wrong
// group — a programming error (§4.6 class 2), surfaced as a parse failure
// because the parser's functions return Result (no panics in result fns).
fn group_routing_bug(state: State) -> ParseError {
    let _ = state;
    ParseError::RelativeWithoutBase
}

/// Whether `segment` is a single-dot URL path segment (§4.1): `.` or `%2e`,
/// ASCII case-insensitive.
fn is_single_dot(segment: &str) -> bool {
    segment == "." || segment.eq_ignore_ascii_case("%2e")
}

/// Whether `segment` is a double-dot URL path segment (§4.1): `..`,
/// `.%2e`, `%2e.`, or `%2e%2e`, ASCII case-insensitive.
fn is_double_dot(segment: &str) -> bool {
    segment == ".."
        || segment.eq_ignore_ascii_case(".%2e")
        || segment.eq_ignore_ascii_case("%2e.")
        || segment.eq_ignore_ascii_case("%2e%2e")
}

/// A Windows drive letter: ASCII alpha followed by `:` or `|` (§4.1).
fn buffer_is_windows_drive_letter(buffer: &str) -> bool {
    let bytes = buffer.as_bytes();
    bytes.len() == 2 && bytes[0].is_ascii_alphabetic() && (bytes[1] == b':' || bytes[1] == b'|')
}

/// A normalized Windows drive letter: ASCII alpha followed by `:` (§4.1).
fn is_normalized_windows_drive_letter(segment: &str) -> bool {
    let bytes = segment.as_bytes();
    bytes.len() == 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

/// "Starts with a Windows drive letter" (§4.1): the first two code points
/// are a drive letter and the string ends there or the third code point is
/// `/`, `\`, `?`, or `#`.
fn starts_with_windows_drive_letter(rest: &str) -> bool {
    let mut chars = rest.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    let Some(second) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() && (second == ':' || second == '|')) {
        return false;
    }
    match chars.next() {
        None => true,
        Some(third) => matches!(third, '/' | '\\' | '?' | '#'),
    }
}
