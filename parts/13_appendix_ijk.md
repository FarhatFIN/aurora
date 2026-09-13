## APPENDIX I — HTTP Status Code Reference

Engine behavior per status (§5.2). Codes are grouped; unlisted codes in a known
class follow the class default. `→` marks what the pipeline does after the
response completes.

| Code | Meaning | Engine behavior |
|---|---|---|
| 100 | Continue | Not requested by us; if received, skip and read the real response. |
| 101 | Switching Protocols | Only valid for the WebSocket upgrade (§6.9); otherwise protocol error. |
| 200 | OK | Success → deliver body. |
| 201 | Created | Success → deliver body. |
| 204 | No Content | Success, no body; keep connection; headers apply. |
| 206 | Partial Content | Only meaningful for ranged media requests (we do not send Range in MVP): treat as 200. |
| 301 | Moved Permanently | Redirect → GET, cacheable navigation redirect. |
| 302 | Found | Redirect → GET. |
| 303 | See Other | Redirect → GET regardless of method. |
| 304 | Not Modified | Cache revalidation hit → serve stored entry (§5.17.4). |
| 307 | Temporary Redirect | Redirect preserving method and body. |
| 308 | Permanent Redirect | Redirect preserving method and body; cacheable. |
| 400 | Bad Request | Error document; render response body if HTML, else generated error page. |
| 401 | Unauthorized | Error page; WWW-Authenticate parsed and shown in DevTools (no credential prompt, §5.2). |
| 403 | Forbidden | Error page (server body rendered). |
| 404 | Not Found | Error page (server body rendered; error status recorded for the page). |
| 405 | Method Not Allowed | Error page; Allow header surfaced in DevTools. |
| 408 | Request Timeout | Network error, retryable at the loader's discretion (once). |
| 410 | Gone | Like 404 with the stronger cache note. |
| 413 | Content Too Large | Error page. |
| 418 | I'm a teapot | Rendered as any other 4xx body (no special case). |
| 429 | Too Many Requests | Error page; Retry-After parsed, honored for that origin's next request scheduling. |
| 500–599 | Server errors | Error page (server body rendered); 502/504 marked as retryable in the network panel. |
| 3xx with Location missing | Malformed redirect | Treated as a normal response of its class with a console warning. |
| Unknown 1xx | Informational | Skip and continue reading (interim responses). |
| Unknown 2xx/3xx | — | Treat as 200 / redirect-class-without-location per class default. |
| Unknown 4xx/5xx | — | Error-page path of the class. |

## APPENDIX J — windows-1252 Remapping Table

The character-reference state (§5.4) remaps numeric references in 0x80–0x9F to
these codepoints (HTML §13.2.5.2). Implement as a 32-entry `const` table with a
serialization test over the full range.

| Byte | CP | Byte | CP |
|---|---|---|---|
| 0x80 | U+20AC (€) | 0x90 | U+FFFD |
| 0x81 | U+FFFD | 0x91 | U+2018 |
| 0x82 | U+201A | 0x92 | U+2019 |
| 0x83 | U+0192 | 0x93 | U+201C |
| 0x84 | U+201E | 0x94 | U+201D |
| 0x85 | U+2026 | 0x95 | U+2022 |
| 0x86 | U+2020 | 0x96 | U+2013 |
| 0x87 | U+2021 | 0x97 | U+2014 |
| 0x88 | U+02C6 | 0x98 | U+02DC |
| 0x89 | U+2030 | 0x99 | U+2122 |
| 0x8A | U+0160 | 0x9A | U+0161 |
| 0x8B | U+2039 | 0x9B | U+203A |
| 0x8C | U+0152 | 0x9C | U+0153 |
| 0x8D | U+FFFD | 0x9D | U+FFFD |
| 0x8E | U+017D | 0x9E | U+017E |
| 0x8F | U+FFFD | 0x9F | U+0178 |

All other positions: identity mapping into Unicode. The windows-1252 decoder
(aurora_encoding) uses the same table plus the remaining high-byte mappings
from the Encoding Standard's index file, regenerated per release with the
full 256-entry test.

## APPENDIX K — HTML Tokenizer State Index

The state machine of §5.4, indexed for navigation. Each state's implementation
lives in one function (`aurora_html::tokenizer::states`); the tests under
`tests/vendor/html5lib/tokenizer/` exercise every transition.

| State | Group | Purpose (one line) |
|---|---|---|
| Data | character | Default: characters pass through; `<` opens tags. |
| Tag open | markup dispatch | `<` + letter → tag name; `!` → markup declaration; `/` → end tag; `?` → bogus comment. |
| End tag open | markup dispatch | Letters → end-tag name; `>` error → data; EOF error. |
| Tag name | tag | Accumulate the name; whitespace/`/`/`>` transitions. |
| Before attribute name | attribute | Skip whitespace; detect attribute or end-of-tag. |
| Attribute name | attribute | Accumulate; handle duplicate names (first wins). |
| After attribute name | attribute | Route to value, next attribute, or tag end. |
| Before attribute value | attribute | Quoted or unquoted value start. |
| Attribute value (double/single/unquoted) | attribute | Accumulate with character-reference expansion. |
| After attribute value (quoted) | attribute | Whitespace/`/`/`>` routing; anything else is a parse error and reprocesses. |
| Self-closing start tag state | tag | Acknowledge `/>` (HTML: ignored with error unless foreign). |
| Bogus comment | comment | Consume until `>`; everything becomes comment data. |
| Markup declaration open | markup dispatch | `--` → comment; DOCTYPE → doctype; `[CDATA[` → cdata (foreign only). |
| Comment start / start dash / comment / comment end dash / comment end | comment | The five-state comment machine with error recoveries. |
| DOCTYPE states | doctype | Name, after-name, before/at/after public and system identifier, bogus DOCTYPE — the standard's thirteen sub-states. |
| CDATA section states | cdata | Section body + end bracket matching, foreign content only. |
| Character reference states | character | Ampersand → named table lookup; `#` numeric with windows-1252 remap (Appendix J); missing-semicolon recoveries. |
| RCDATA | character | Like Data but `<` opens RCDATA-end; character references active. |
| RAWTEXT | character | Like RCDATA without character references. |
| Script data | script | RAWTEXT rules plus the `<!--` double-escape machinery below. |
| Script data less-than sign / end tag name / escape start / escaped / escaped dash / escaped dash dash / escaped less-than sign / double escaped / double escaped dash / double escaped dash dash | script | The fourteen script-data sub-states implementing `</script>` detection and the comment double-escape dance. |
| Plaintext | character | Everything is text; no transitions; the end of recovery. |

Tree-builder-driven switches: the tree constructor sets the tokenizer's state
for `title`/`textarea` (RCDATA), `style`/`xmp`/`iframe`/`noembed`/`noframes`
(RAWTEXT), `script` (script data), and `plaintext` — the tokenizer exposes
`set_state()` for exactly these re-entrancies (§5.4 streaming invariant).
