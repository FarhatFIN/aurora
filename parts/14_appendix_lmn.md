## APPENDIX L — URL Parser State Index

The WHATWG basic URL parser states (§5.1), indexed for navigation like
Appendix K. Implementation: one `match` arm per state in
`aurora_url::parser`; the WHATWG URL test data (adopted per §8.6) drives every
transition.

| State | Purpose (one line) |
|---|---|
| Scheme start / scheme | Accumulate the scheme; ASCII alpha start, then alnum + `+ - .`; `:` completes; else no-scheme failure. |
| No scheme | Relative reference against the base, or failure. |
| File / file slash / file host / file path states | The four file-URL states: drive-letter handling (Windows), host (empty allowed), path semantics. |
| Special relative or authority | `//` → authority; else relative-path state for special schemes. |
| Special authority slashes / special authority ignore slashes | Consume any number of `/` or `\` (special schemes treat `\` as separator), then authority. |
| Authority state | Credentials split on the last `@`; host/port follow. |
| Host / hostname | Domain (IDNA to-ASCII, lowercase), IPv4 (four numeric parts with the standard's overflow rules), or IPv6 in brackets. |
| Port | Digits until `/`, `?`, `#`, or EOF; empty port allowed; validity checked against the scheme. |
| Path or authority | For non-special schemes: `//`-prefix decides authority. |
| Path start | Decide empty-path vs absolute-path; drive-letter logic for file. |
| Path | Segment accumulation, dot-segment collapse, empty-segment rules. |
| Cannot-be-a-base path | Opaque path for non-hierarchical schemes (`data:`, `mailto:`). |
| Query | Percent-encode with the query set (special vs non-special encode sets differ). |
| Fragment | Percent-encode with the fragment set; terminates parsing. |

Serialization invariants (§5.1): scheme lowercase; special schemes always
serialize an authority; password omitted when empty; port omitted when it is
the scheme default; file URLs serialize per the platform convention chosen in
the ADR (§4.9) — decide once, test everywhere.

## APPENDIX M — DOMException Table

The exceptions the engine throws at script-facing boundaries (§4.6 class 3,
§9.3). The binding layer maps internal error enums to exactly these; anything
not in this table is a bug. Name, message pattern, and typical trigger:

| Name | Thrown when |
|---|---|
| `IndexSizeError` | Index negative or greater than the allowed count (ranges, canvas). |
| `HierarchyRequestError` | Tree request violates the hierarchy (ancestor as descendant, wrong parent). |
| `WrongDocumentError` | Node from another document used where the current one is required (legacy paths). |
| `InvalidCharacterError` | String contains characters the context forbids (element names, `createElement`). |
| `NoModificationAllowedError` | Object cannot be modified (readonly nodes, non-contentEditable mutation). |
| `NotFoundError` | Object not found where it must be (insertBefore child, removeChild). |
| `NotSupportedError` | Operation not supported by this engine — the §1.4 non-goal surface's standard answer. |
| `InUseAttributeError` | Adopting an attribute still attached to an element. |
| `InvalidStateError` | Object in a state where the operation is unavailable (response body consumed, port closed). |
| `SyntaxError` | Malformed string in an API (selector strings, `querySelector`, invalid regex flags, JSON in some contexts). |
| `InvalidModificationError` | Mutation would produce invalid content (doctype inside elements). |
| `NamespaceError` | Malformed namespace operations (`createElementNS` qname/prefix mismatch). |
| `InvalidAccessError` | Object/function no longer supported, or unsupported argument combination. |
| `TypeMismatchError` | Legacy name of `TypeError` for IDL type mismatches in some interfaces. |
| `SecurityError` | §5.18 boundary check failures: cross-origin access, storage denial, forbidden operation. |
| `NetworkError` | Fetch/network failures surfaced as DOM errors (WebSocket connect failure, form submission fetch). |
| `AbortError` | Operation aborted (fetch abort via §6.6 `AbortSignal`, navigation interrupted). |
| `URLMismatchError` | `window.open`/history target origin mismatch. |
| `QuotaExceededError` | Storage quota exceeded (§6.10). |
| `TimeoutError` | Operation timed out (signals, locking stretch goals). |
| `InvalidNodeTypeError` | Wrong node type for the operation (ranges, mutation observer init). |
| `DataCloneError` | `structuredClone`/`postMessage` value not cloneable (functions, DOM nodes). |

Every binding test (§13.3 runbook) asserts both the name and that the message
follows the documented pattern — messages are not API, but their shape is.

## APPENDIX N — Implicit ARIA Roles

The accessibility scaffolding of §5.19.6 computes roles from markup. Table:
element → implicit role (per HTML-AAM; the computed-tree test corpus
`tests/corpus/a11y/` asserts one row per line). `aria-*` attributes and
explicit `role` always override the implicit value.

| Element(s) | Implicit role |
|---|---|
| `a[href]` | `link` |
| `address` | `group` |
| `article` | `article` |
| `aside` | `complementary` |
| `button` | `button` |
| `datalist` | `listbox` |
| `details` | `group` |
| `summary` (first child of details) | (no role; acts as disclosure) |
| `dialog[open]` | `dialog` |
| `fieldset` | `group` |
| `figure` | `figure` |
| `footer` (not in sectioning content) | `contentinfo` |
| `form` | `form` |
| `h1`–`h6` | `heading` (level from the element) |
| `header` (not in sectioning content) | `banner` |
| `hr` | `separator` |
| `img[alt=""]` | `presentation` |
| `img[alt]` | `img` |
| `input[type=button/submit/reset]` | `button` |
| `input[type=checkbox]` | `checkbox` |
| `input[type=number]` | `spinbutton` |
| `input[type=radio]` | `radio` |
| `input[type=range]` | `slider` |
| `input[type=text/search/tel/url/email/password]` | `textbox` |
| `li` | `listitem` |
| `main` | `main` |
| `menu` | `list` |
| `nav` | `navigation` |
| `ol`, `ul` | `list` |
| `option` | `option` |
| `output` | `status` |
| `progress` | `progressbar` |
| `section` | `region` (named) / `generic` |
| `select` | `combobox` (single) / `listbox` (multiple) |
| `table` | `table` |
| `tbody/thead/tfoot` | `rowgroup` |
| `td` | `cell` |
| `textarea` | `textbox` (multiline) |
| `th` | `columnheader` / `rowheader` per scope |
| `tr` | `row` |

Name computation (subset): `aria-label` → `aria-labelledby` target text →
native source (`alt`, `label[for]`, `legend`, `caption`, `title`, value) →
text content → nothing. The computed tree is exposed through the DevTools
inspector's a11y view and the platform layer where available (§5.19.6); its
tests are pure engine tests of the computed tree, not of any AT integration.
