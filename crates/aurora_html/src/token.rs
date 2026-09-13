//! Token model (§5.4's sketch). Character runs: the standard emits
//! per-character tokens; the sketch explicitly permits the run variant
//! (`Character(String)`) as the optimization path — the tree builder and
//! the html5lib reference expectations are insensitive to run splits.

/// One tokenizer output token (§5.4 sketch).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Doctype {
        name: Option<String>,
        public: Option<String>,
        system: Option<String>,
        force_quirks: bool,
    },
    StartTag {
        name: String,
        attrs: Vec<Attribute>,
        self_closing: bool,
    },
    EndTag {
        name: String,
    },
    Comment(String),
    /// A run of consecutive character data.
    Character(String),
}

/// `name="value"`, name ASCII-lowercased, order preserved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

/// The tokenizer's initial state — decided by the tree builder when
/// switching (§5.5.2) or by `TokenizerOptions` (§5.4 sketch).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InitialState {
    Data,
    Rcdata,
    Rawtext,
    ScriptData,
    Plaintext,
}

/// Tokenizer configuration (§5.4's `TokenizerOptions`).
#[derive(Clone, Copy, Debug)]
pub struct TokenizerOptions {
    /// Drives the noscript-adjacent behaviors and the tree builder's
    /// scripting flag (§5.5).
    pub scripting: bool,
    pub initial_state: InitialState,
}

impl Default for TokenizerOptions {
    fn default() -> Self {
        Self {
            scripting: true,
            initial_state: InitialState::Data,
        }
    }
}

/// Tokenizer states (§13.4), named after the standard's states.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum State {
    Data,
    Rcdata,
    RcdataLessThanSign,
    RcdataEndTagOpen,
    RcdataEndTagName,
    Rawtext,
    RawtextLessThanSign,
    RawtextEndTagOpen,
    RawtextEndTagName,
    ScriptData,
    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    ScriptDataEscapeStart,
    ScriptDataEscapeStartDash,
    ScriptDataEscaped,
    ScriptDataEscapedDash,
    ScriptDataEscapedDashDash,
    ScriptDataEscapedLessThanSign,
    ScriptDataEscapedEndTagOpen,
    ScriptDataEscapedEndTagName,
    ScriptDataDoubleEscapeStart,
    ScriptDataDoubleEscaped,
    ScriptDataDoubleEscapedDash,
    ScriptDataDoubleEscapedLessThanSign,
    ScriptDataDoubleEscapeEnd,
    ScriptDataDoubleEscapedDashDash,
    PlainText,
    TagOpen,
    EndTagOpen,
    TagName,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    BogusComment,
    MarkupDeclarationOpen,
    CommentStart,
    CommentStartDash,
    Comment,
    CommentLessThanSign,
    CommentLessThanSignBang,
    CommentLessThanSignBangDash,
    CommentLessThanSignBangDashDash,
    CommentEndDash,
    CommentEnd,
    CommentEndBang,
    Doctype,
    BeforeDoctypeName,
    DoctypeName,
    AfterDoctypeName,
    AfterDoctypePublicKeyword,
    BeforeDoctypePublicIdentifier,
    DoctypePublicIdentifierDoubleQuoted,
    DoctypePublicIdentifierSingleQuoted,
    AfterDoctypePublicIdentifier,
    BetweenDoctypePublicAndSystemIdentifiers,
    AfterDoctypeSystemKeyword,
    BeforeDoctypeSystemIdentifier,
    DoctypeSystemIdentifierDoubleQuoted,
    DoctypeSystemIdentifierSingleQuoted,
    AfterDoctypeSystemIdentifier,
    BogusDoctype,
    CdataSection,
    CdataSectionBracket,
    CdataSectionEnd,
    CharacterReference,
}
