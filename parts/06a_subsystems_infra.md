## PART 5 — Subsystem Specifications

Each section: goal, standard basis, position in the pipeline, public API sketch, core
algorithm, invariants, pitfalls, and definition of done. The API sketches are *shapes*,
not contracts — you may refine signatures, but the responsibilities and the seams must
hold until an ADR says otherwise.

### §5.1 URL parsing and normalization

**Standard basis:** WHATWG URL Standard (Living Standard) — parsing, serialization,
origin computation, `application/x-www-form-urlencoded` serializer.
**Pipeline position:** every fetch and every `href`/`src`/`action` resolution passes here.
**Goal:** parse any URL a real web page contains, including garbage, without panic,
and answer origin questions for the security model.

**Public API sketch**

```rust
pub struct Url { /* scheme, username, password, host, port, path segments,
                    query, fragment, opaque payload for non-hierarchical schemes */ }
pub enum ParseError { RelativeWithoutBase, InvalidIpv6, InvalidPort, InvalidDomainChar,
                      EmptyHost, HostInvalid, MissingSchemeNonRelative, IdnaError, /* WHATWG set */ }
impl Url {
    pub fn parse(input: &str) -> Result<Url, ParseError>;
    pub fn parse_with_base(input: &str, base: &Url) -> Result<Url, ParseError>;
    pub fn join(&self, reference: &str) -> Result<Url, ParseError>;
    pub fn origin(&self) -> Origin;                 // Tuple(scheme, host, port) | Opaque | Null
    pub fn serialize(&self, exclude_fragment: bool) -> String;
    pub fn scheme(&self) -> &str; pub fn host(&self) -> Option<Host>;
    pub fn path(&self) -> &str; pub fn query(&self) -> Option<&str>; pub fn fragment(&self) -> Option<&str>;
    pub fn is_special(&self) -> bool;               // http, https, ws, wss, ftp, file
    pub fn cannot_be_a_base(&self) -> bool;
}
pub fn percent_encode(s: &str, set: &AsciiSet) -> String;   // WHATWG percent-encode sets
pub fn form_urlencode(pairs: &[(String, String)]) -> String;
```

**Core algorithm (WHATWG "basic URL parser"):** trim C0/space; remove tab/newline;
scheme state → (special) authority state → host state (domain: IDNA to-ASCII with
UTS-46, lowercase; IPv4 with the standard 4-part numeric rules; IPv6 in brackets) →
path state (segments, dot-segment removal, empty-segment preservation for special
schemes) → query state → fragment state. Relative resolution per "path/state record"
rules: same-scheme special merge, `..`/`.` collapse against the base path. File URLs
get their own host/path handling (drive letters on Windows serialize per spec).

**Invariants**

- Parse is total: any input returns a `Url` or a `ParseError`, never a panic (§4.6 class 3).
- `parse(serialize(u)) == u` for all valid `u` (idempotence round-trip, unit-tested).
- Origin equality drives SOP (§5.18); `Origin` must be hashable and totally ordered
  for use in cache/cookie/storage keys.
- Host lowercase and IDNA normalization happen at parse time, never at use time.

**Pitfalls:** backslashes in special URLs are path separators (spec says so); empty
host with non-special schemes is legal; port 0 is legal; a lone `.` path segment
collapses but a trailing empty segment does not; `data:` URLs are cannot-be-a-base
with opaque paths and must not hit path normalization.

**Definition of done:** full parse/serialize round-trip suite; the WHATWG URL test
data set (hundreds of cases) passes where it applies to supported schemes; origin
tests for every special scheme; fuzz target `fuzz_url` runs 10 minutes clean.

### §5.2 HTTP network stack

**Standard basis:** RFC 9110 (HTTP semantics), RFC 9112 (HTTP/1.1), WHATWG Fetch
(concepts: request, response, body, redirect status, caching is HTTP's, not Fetch's),
RFC 6265bis (cookies — implementation lives in §5.17, wire format here).
**Pipeline position:** loader threads; one request at a time per connection.

**Public API sketch**

```rust
pub struct Request { pub method: Method, pub url: Url, pub headers: HeaderMap,
                     pub body: BodyKind /* None | Bytes | Streamed */, pub mode: FetchMode,
                     pub credentials: CredentialsMode, pub redirect: RedirectPolicy, /* follow|error|manual */ }
pub struct Response { pub status: u16, pub reason: String, pub headers: HeaderMap,
                      pub url: Url /* post-redirect final */, pub redirected_from: Vec<Url>,
                      pub body: BodyReader /* chunked, content-length, or until-close */ }
pub enum NetError { Dns, Connect(ref ConnectFailure), Tls(ref TlsFailure), Protocol(&'static str),
                    Timeout(Phase), Aborted, TooManyRedirects }
pub trait FetchClient { fn fetch(&self, req: Request, cancel: CancelToken,
                                  sink: &mut dyn ResponseSink) -> Result<(), NetError>; }
```

**Core behavior**

1. **Connection management:** per-origin pool keyed `(scheme, host, port)`; default
   max 6 connections per origin; idle timeout 60 s; `Connection: close` honored;
   keep-alive by default. A pooled connection that errors once is discarded, not retried.
2. **Request line/headers:** HTTP/1.1, `Host` (and `:authority`-equivalent) mandatory;
   `User-Agent: Aurora/0.1 (+engine project)`, `Accept`, `Accept-Language`, `Accept-Encoding:
   gzip, deflate` (identity until the owned compressor lands; `miniz_oxide` permitted for
   gzip per §3.3), `Connection: keep-alive`. Header injection is impossible by
   construction: header values are validated to be visible-ASCII without CR/LF at the API.
3. **Response parsing:** status line, reason, header section with obs-fold rejection
   (RFC 9112 §5.2), chunked transfer decoding (write the chunk parser yourself — it is
   40 lines and examinable), content-length framing, until-close as last resort.
4. **Redirects:** 301/302/303 → GET (body dropped), 307/308 → method+body preserved;
   max 20 hops; cross-origin redirect strips `Cookie`/`Authorization` unless the
   credentials policy says otherwise; the redirect chain is reported in `redirected_from`.
5. **Timeouts:** DNS 10 s, connect 10 s, TLS 10 s, header 30 s, body idle 30 s — each
   phase separately cancelable (`CancelToken` is shared atomics + a close-socket call).
6. **Caching interaction:** the loader consults the HTTP cache (§5.17.4) before
   opening a connection: fresh → serve; stale-with-validator → conditional request
   (`If-None-Match`/`If-Modified-Since`); `Vary` is respected on the exact header
   values; `no-store`/`no-cache` per RFC 9111 semantics.
7. **Content sniffing:** only where the standard allows — images and the classic
   `nosniff`-absent HTML/text ambiguity for top-level documents; `X-Content-Type-Options:
   nosniff` disables it. Sniffing table lives in `aurora_net` with tests.

**Invariants:** the stack never panics on malformed input — a protocol violation is a
`NetError::Protocol` and the connection dies. All header lookups are ASCII-case-insensitive.
The stack is fully testable against a local in-process mock server (tests spin up a
TCP listener, no external network in CI).

**Pitfalls:** obs-fold, bare-LF line endings in the wild (accept `LF` after `CR` only;
a bare `LF` terminator is a protocol error per RFC 9112 but real servers send it —
decide: accept with a console warning, record decision), chunked + content-length
together is an error, header continuation folding, 100-continue (just don't send
`Expect:`), HEAD must not read a body.

**Definition of done:** mock-server integration tests for keep-alive, chunked,
redirect chain (4 cases), conditional revalidation, gzip response, timeout and abort
paths; unit tests for the header parser including hostile inputs; fuzz target
`fuzz_http_response`.

### §5.3 TLS and certificate policy

**Standard basis:** RFC 8446 (TLS 1.3) as provided by `rustls`; RFC 5280 (PKIX) via
the same; HSTS RFC 6797.
**Position:** inside the loader, below HTTP, above TCP.

**Behavior**

- TLS 1.2 and 1.3 only; no SSLv3/TLS1.0/1.1, no RC4/3DES, no compression.
- Certificate verification: full chain validation against the trust store (§3.3),
  hostname verification mandatory. A verification failure is a hard `TlsFailure` —
  the UI shows an interstitial error page with the reason; **there is no click-through**
  (this browser has no PKI override UX; that is a scope decision, not an oversight).
- ALPN offers `http/1.1` (and `h2` only if the stretch goal lands).
- Session resumption: use what the TLS library provides by default; do not build more.
- HSTS: parse `Strict-Transport-Security` on `https://` responses (max-age, includeSubDomains,
  ignore unknown directives); persist in the security state store; upgrade all later
  http:// fetches to https:// before connection; expire per max-age.
- Referrer policy defaults to `strict-origin-when-cross-origin` and is applied to the
  `Referer` header here, driven by the document's policy attribute (§5.18.5).

**Definition of done:** integration tests against a local TLS server with a generated
CA (self-signed good chain, expired cert, wrong-hostname cert, untrusted-root cert —
four test cases with exact `TlsFailure` variants); HSTS upgrade test; a written note
in `docs/spec-notes/tls.md` on what the trust store provides.

### §5.4 The HTML tokenizer

**Standard basis:** WHATWG HTML §13.4 (Tokenization) — the 80-state machine, and
§13.2.3.5 for preprocessing (CR/LF normalization, encoding sniffing upstream).
**Position:** bytes → tokens, inside the parser, on the document thread, synchronous.

**Public API sketch**

```rust
pub enum Token { Doctype { name: Option<String>, public: Option<String>, system: Option<String>,
                           force_quirks: bool },
                 StartTag { name: Atom, attrs: Vec<Attribute>, self_closing: bool },
                 EndTag { name: Atom },
                 Comment(String),
                 Character(char) }
pub struct Tokenizer<'i> { /* input iterator, state, last-start-tag, temp buffer, app cache... */ }
impl<'i> Tokenizer<'i> { pub fn new(input: &'i [u8], encoding: Encoding,
                                     options: TokenizerOptions) -> Self;
                         pub fn next_token(&mut self) -> Option<Token>; }
pub struct TokenizerOptions { pub scripting: bool, pub initial_state: State /* data|rcdata|rawtext|script|plaintext */ }
```

**Core algorithm:** implement the state machine as an explicit `match` over `State`
in one function per state group, exactly following the standard's numbered transitions:
Data, Tag-open, Tag-name, Before/at/after attribute-name, After-attribute-value-quoted,
Self-closing, Bogus-comment, Markup-declaration-open (comments, DOCTYPE, `<![CDATA[` in
foreign content), DOCTYPE states (name, public/system identifier, bogus), CDATA section,
Character-reference (numeric with windows-1252 remapping table, named references via a
generated table — see WBS §6.2 preamble), RCDATA, RAWTEXT, Script-data (+ double-escaped
states, which exist for `<script>document.write('<!--<script>…'), Plaintext.
Emit `Character` tokens as runs when convenient (`Character(String)` variant is allowed
as an optimization *after* the per-char version passes the reference tests).

**Invariants**

- Streaming: the tokenizer never requires the whole input; it pulls from a byte
  iterator so the tree builder can feed it *reprocessed* input for `document.write`
  (§5.5.6) without copying the world.
- The tokenizer is deterministic and total on any byte sequence; malformed input
  produces the spec's error-recovery (bogus comment, missing-attribute-value, etc.)
  and a parse-error report (collected, not fatal).
- Tag names and attribute names are `Atom`s (interned) — case-folded to lowercase for
  HTML elements, preserved case for foreign content (MathML/SVG, tree builder's job).

**Pitfalls:** the `</script>` inside script data with `<!--` double-escape dance;
attribute value states' quoting; the `/>` on non-void HTML elements is *ignored*
(self-closing flag acknowledged only for foreign content); NUL bytes become U+FFFD
in most states; surrogate references from numeric character references become U+FFFD.

**Definition of done:** the tokenizer test suite from html5lib-tests (`tokenizer/`)
passes 100% in HTML mode; parse errors counted and reported; fuzz target
`fuzz_html_tokenizer` with no panics after 30 minutes.

### §5.5 HTML tree construction

**Standard basis:** WHATWG HTML §13.2.4–13.2.6 (parse errors, tree construction
dispatch, insertion modes), §13.2.9 (foreign content), §15.2 (parsing main structure:
head/body synthesis), plus the "in body" rules for every element in WBS §6.2.
**Position:** tokens → DOM; the DOM crate is its substrate (§5.6).

**Public API sketch**

```rust
pub struct TreeBuilder { /* open element stack, insertion mode stack, templates, forms,
                            pending table chars, foster-parenting flag, scripting flag, quirks mode */ }
impl TreeBuilder { pub fn process_token(&mut self, tok: Token, dom: &mut Document);
                   pub fn finish(&mut self) -> Document; }
pub fn parse_document(input: ByteStream, opts: ParseOptions) -> Document; // ties tokenizer+builder
pub struct ParseOptions { pub scripting: bool, pub fragment_context: Option<(QualName, /*form*/ Option<NodeId>)>,
                          pub sink: Option<&mut dyn ParseEventSink> /* for DevTools */ }
```

**Core algorithm**

1. Implement the insertion modes as functions: `initial`, `before_html`, `before_head`,
   `in_head`, `in_head_noscript`, `after_head`, `in_body` (the big one), `text`,
   `in_table`, `in_table_text`, `in_caption`, `in_column_group`, `in_table_body`,
   `in_row`, `in_cell`, `in_select`, `in_select_in_table`, `in_template`,
   `after_body`, `in_frameset`, `after_frameset`, `after_after_body`,
   `after_after_frameset` — dispatch on current mode, with the spec's mode-switch
   rules (`any other start tag` handling and "reprocess in" loops).
2. The **stack of open elements** with the special-element set, the "generate
   implied end tags" list, and the **adoption agency algorithm** (§13.2.6.4.4 —
   implement it literally, it is the part everyone gets wrong; format-element bookkeeping
   with the three-marker "Noah's Ark clause" of 3 identical format elements).
3. **Foster parenting** (text/table-content in wrong place gets moved before the table),
   **active formatting elements**, **template contents** in a document fragment,
   **frameset-ok** flag, **quirks mode** derivation from DOCTYPE (three modes stored
   on the Document for the style/layout stages to consult).
4. Character token buffering for whitespace runs and the `in_table_text` special path.
5. Encoding detection (§5.5.7) happens before tokenization: BOM, `<meta charset>`,
   `Content-Type`, then windows-1252 default, with the standard's re-tokenization
   restart when a late `<meta charset>` appears (restart is allowed: buffer the first
   1024 bytes only).

**Script execution hooks (the scripting flag)**

- A `script` end tag in `in body` with scripting enabled enqueues the element per its
  `type` (classic/module), `async`/`defer` attributes, and the "list of scripts that
  will execute when the document has finished parsing" (deferred) vs "as soon as
  possible" (async/in-body classic) sets. Execution is orchestrated by the runtime
  (§5.15), not the tree builder — the tree builder *suspends* on a blocking script
  and the parser resumes when the runtime signals completion.
- `document.write(text)` re-enters the tokenizer with the string inserted at the
  current position (§2.2 streaming design pays for itself here). `document.open/close/writeln`
  on an active document is supported; on a loaded document it truncates it.

**Invariants:** the tree builder is total on any token sequence (html5lib-tests prove
it); the DOM it builds is always a well-formed *tree* (single root, acyclic, ordered);
fragment parsing (`innerHTML`/`outerHTML`/template contents, `createContextualFragment`)
uses the same builder with a fragment context and must not touch the document's
other state; `outerHTML` assignment re-parses and splices in place per DOM Parsing spec.

**Pitfalls:** `in body`'s start-tag rules are per-element and enormous — that is why
WBS §6.2 enumerates every element; the `<form>` pointer (nested forms are dropped);
`<nobr>`/formatting element interactions; `<select>` swallowing almost everything;
the `<table><text>` foster path re-running `insert_characters` twice (buffering);
SVG/MathML attribute-case fixup table; `<template>` is invisible to the open-element
stack (it uses its own content fragment).

**Definition of done:** html5lib-tests `tree-construction/` 100% pass; fragment tests
pass; a scripted `document.write` torture test passes; fuzz target `fuzz_html_tree`
(no panics, no leaks-after-document-drop over ASan where available) 30 minutes.
