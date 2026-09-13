## APPENDIX A — CSS Named Colors

The 148 CSS Color 4 named colors (case-insensitive keywords; the legacy
synonym pairs `gray`/`grey`, `aqua`/`cyan`, `magenta`/`fuchsia`, and the
`darkslategray`-style variants are distinct keywords mapping to equal values).
Table: keyword → RGB hex; implemented as a perfect-hash table in the
color parser (§5.7) with a serialization test per row.

| Keyword | Hex | Keyword | Hex |
|---|---|---|---|
| aliceblue | #f0f8ff | lightpink | #ffb6c1 |
| antiquewhite | #faebd7 | lightsalmon | #ffa07a |
| aqua | #00ffff | lightseagreen | #20b2aa |
| aquamarine | #7fffd4 | lightskyblue | #87cefa |
| azure | #f0ffff | lightslategray | #778899 |
| beige | #f5f5dc | lightslategrey | #778899 |
| bisque | #ffe4c4 | lightsteelblue | #b0c4de |
| black | #000000 | lightyellow | #ffffe0 |
| blanchedalmond | #ffebcd | lime | #00ff00 |
| blue | #0000ff | limegreen | #32cd32 |
| blueviolet | #8a2be2 | linen | #faf0e6 |
| brown | #a52a2a | magenta | #ff00ff |
| burlywood | #deb887 | maroon | #800000 |
| cadetblue | #5f9ea0 | mediumaquamarine | #66cdaa |
| chartreuse | #7fff00 | mediumblue | #0000cd |
| chocolate | #d2691e | mediumorchid | #ba55d3 |
| coral | #ff7f50 | mediumpurple | #9370db |
| cornflowerblue | #6495ed | mediumseagreen | #3cb371 |
| cornsilk | #fff8dc | mediumslateblue | #7b68ee |
| crimson | #dc143c | mediumspringgreen | #00fa9a |
| cyan | #00ffff | mediumturquoise | #48d1cc |
| darkblue | #00008b | mediumvioletred | #c71585 |
| darkcyan | #008b8b | midnightblue | #191970 |
| darkgoldenrod | #b8860b | mintcream | #f5fffa |
| darkgray | #a9a9a9 | mistyrose | #ffe4e1 |
| darkgreen | #006400 | moccasin | #ffe4b5 |
| darkgrey | #a9a9a9 | navajowhite | #ffdead |
| darkkhaki | #bdb76b | navy | #000080 |
| darkmagenta | #8b008b | oldlace | #fdf5e6 |
| darkolivegreen | #556b2f | olive | #808000 |
| darkorange | #ff8c00 | olivedrab | #6b8e23 |
| darkorchid | #9932cc | orange | #ffa500 |
| darkred | #8b0000 | orangered | #ff4500 |
| darksalmon | #e9967a | orchid | #da70d6 |
| darkseagreen | #8fbc8f | palegoldenrod | #eee8aa |
| darkslateblue | #483d8b | palegreen | #98fb98 |
| darkslategray | #2f4f4f | paleturquoise | #afeeee |
| darkslategrey | #2f4f4f | palevioletred | #db7093 |
| darkturquoise | #00ced1 | papayawhip | #ffefd5 |
| darkviolet | #9400d3 | peachpuff | #ffdab9 |
| deeppink | #ff1493 | peru | #cd853f |
| deepskyblue | #00bfff | pink | #ffc0cb |
| dimgray | #696969 | plum | #dda0dd |
| dimgrey | #696969 | powderblue | #b0e0e6 |
| dodgerblue | #1e90ff | purple | #800080 |
| firebrick | #b22222 | rebeccapurple | #663399 |
| floralwhite | #fffaf0 | red | #ff0000 |
| forestgreen | #228b22 | rosybrown | #bc8f8f |
| fuchsia | #ff00ff | royalblue | #4169e1 |
| gainsboro | #dcdcdc | saddlebrown | #8b4513 |
| ghostwhite | #f8f8ff | salmon | #fa8072 |
| gold | #ffd700 | sandybrown | #f4a460 |
| goldenrod | #daa520 | seagreen | #2e8b57 |
| gray | #808080 | seashell | #fff5ee |
| green | #008000 | sienna | #a0522d |
| greenyellow | #adff2f | silver | #c0c0c0 |
| grey | #808080 | skyblue | #87ceeb |
| honeydew | #f0fff0 | slateblue | #6a5acd |
| hotpink | #ff69b4 | slategray | #708090 |
| indianred | #cd5c5c | slategrey | #708090 |
| indigo | #4b0082 | snow | #fffafa |
| ivory | #fffff0 | springgreen | #00ff7f |
| khaki | #f0e68c | steelblue | #4682b4 |
| lavender | #e6e6fa | tan | #d2b48c |
| lavenderblush | #fff0f5 | teal | #008080 |
| lawngreen | #7cfc00 | thistle | #d8bfd8 |
| lemonchiffon | #fffacd | tomato | #ff6347 |
| lightblue | #add8e6 | turquoise | #40e0d0 |
| lightcoral | #f08080 | violet | #ee82ee |
| lightcyan | #e0ffff | wheat | #f5deb3 |
| lightgoldenrodyellow | #fafad2 | white | #ffffff |
| lightgray | #d3d3d3 | whitesmoke | #f5f5f5 |
| lightgreen | #90ee90 | yellow | #ffff00 |
| lightgrey | #d3d3d3 | yellowgreen | #9acd32 |

Plus: `transparent` = rgba(0,0,0,0); `currentcolor` resolves from the
`color` property at computed-value time (§5.8.3); system colors
(`canvastext`, `canvas`, `linktext`, `visitedtext`, `buttontext`,
`buttonface`, `buttonborder`, `field`, `fieldtext`, `highlight`,
`highlighttext`, `graytext`, `mark`, `marktext`) map to the default theme.

## APPENDIX B — HTML Named Character References (common subset)

Named references the tokenizer's character-reference state must resolve
(§5.4); the full standard table (~2,231 entries) is generated from the
entities JSON at build time — this subset must be hand-verified.

| Reference | Codepoint | Glyph note |
|---|---|---|
| &amp; | U+0026 | ampersand & |
| &lt; | U+003C | less-than < |
| &gt; | U+003E | greater-than > |
| &quot; | U+0022 | double quote " |
| &apos; | U+0027 | apostrophe ' |
| &nbsp; | U+00A0 | no-break space (non-ASCII in serialization) |
| &copy; | U+00A9 | copyright sign |
| &reg; | U+00AE | registered sign |
| &trade; | U+2122 | trademark sign |
| &hellip; | U+2026 | horizontal ellipsis |
| &mdash; | U+2014 | em dash |
| &ndash; | U+2013 | en dash |
| &lsquo; | U+2018 | left single quote |
| &rsquo; | U+2019 | right single quote |
| &ldquo; | U+201C | left double quote |
| &rdquo; | U+201D | right double quote |
| &sbquo; | U+201A | single low quote |
| &bdquo; | U+201E | double low quote |
| &laquo; | U+00AB | left guillemet |
| &raquo; | U+00BB | right guillemet |
| &lsaquo; | U+2039 | single left angle quote |
| &rsaquo; | U+203A | single right angle quote |
| &times; | U+00D7 | multiplication sign |
| &divide; | U+00F7 | division sign |
| &plusmn; | U+00B1 | plus-minus sign |
| &deg; | U+00B0 | degree sign |
| &middot; | U+00B7 | middle dot |
| &bull; | U+2022 | bullet |
| &dagger; | U+2020 | dagger |
| &Dagger; | U+2021 | double dagger |
| &permil; | U+2030 | per mille sign |
| &euro; | U+20AC | euro sign |
| &cent; | U+00A2 | cent sign |
| &pound; | U+00A3 | pound sign |
| &yen; | U+00A5 | yen sign |
| &sect; | U+00A7 | section sign |
| &para; | U+00B6 | pilcrow sign |
| &larr; | U+2190 | leftwards arrow |
| &uarr; | U+2191 | upwards arrow |
| &rarr; | U+2192 | rightwards arrow |
| &darr; | U+2193 | downwards arrow |
| &harr; | U+2194 | left-right arrow |
| &minus; | U+2212 | minus sign |
| &lowast; | U+2217 | asterisk operator |
| &radic; | U+221A | square root |
| &infin; | U+221E | infinity |
| &cap; | U+2229 | intersection |
| &cup; | U+222A | union |
| &int; | U+222B | integral |
| &asymp; | U+2248 | almost equal |
| &ne; | U+2260 | not equal |
| &le; | U+2264 | less-or-equal |
| &ge; | U+2265 | greater-or-equal |
| &alpha; | U+03B1 | greek small alpha |
| &beta; | U+03B2 | greek small beta |
| &gamma; | U+03B3 | greek small gamma |
| &pi; | U+03C0 | greek small pi |
| &Omega; | U+03A9 | greek capital omega |
| &sum; | U+2211 | n-ary summation |
| &prod; | U+220F | n-ary product |
| &part; | U+2202 | partial differential |
| &nabla; | U+2207 | nabla |
| &isin; | U+2208 | element of |
| &notin; | U+2209 | not an element of |
| &empty; | U+2205 | empty set |
| &forall; | U+2200 | for all |
| &exist; | U+2203 | there exists |
| &oplus; | U+2295 | circled plus |
| &otimes; | U+2297 | circled times |
| &perp; | U+22A5 | up tack |
| &sdot; | U+22C5 | dot operator |
| &lceil; | U+2308 | left ceiling |
| &rceil; | U+2309 | right ceiling |
| &lfloor; | U+230A | left floor |
| &rfloor; | U+230B | right floor |
| &lang; | U+27E8 | left angle bracket |
| &rang; | U+27E9 | right angle bracket |
| &loz; | U+25CA | lozenge |
| &spades; | U+2660 | black spade suit |
| &clubs; | U+2663 | black club suit |
| &hearts; | U+2665 | black heart suit |
| &diams; | U+2666 | black diamond suit |
| &shy; | U+00AD | soft hyphen (invisible) |
| &zwnj; | U+200C | zero-width non-joiner |
| &zwj; | U+200D | zero-width joiner |
| &lrm; | U+200E | left-to-right mark |
| &rlm; | U+200F | right-to-left mark |

Numeric references: decimal `&#NNN;` and hex `&#xHHH;`, with the
windows-1252 remapping for 0x80–0x9F and U+FFFD for surrogates and
out-of-range codepoints. Unmatched named references serialize literally.

## APPENDIX C — HTTP Header Field Reference

Engine behavior per header (§5.2); direction is the side the engine
sends or consumes it on. Unknown headers pass through untouched and
appear in the DevTools network view.

| Header | Direction | Engine behavior |
|---|---|---|
| `Host` | request | Mandatory for http/1.1 special schemes; validated non-empty. |
| `User-Agent` | request | `Aurora/0.1 (+project)`; single well-known string, no spoofing options. |
| `Accept` | request | `text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8` for documents; narrower for subresources. |
| `Accept-Language` | request | From shell prefs (`en-US,en;q=0.9` default); feeds :lang fallback matching. |
| `Accept-Encoding` | request | `gzip, deflate` (identity until compressor lands; §5.2.2). |
| `Connection` | both | `keep-alive` default; `close` honored both directions. |
| `Content-Length` | both | Framing for bodies; conflicts with chunked are protocol errors (§5.2.3). |
| `Transfer-Encoding` | response | `chunked` decoding implemented; other values are protocol errors. |
| `Content-Type` | both | Drives document-type sniffing (§5.2.7) and stylesheet/script/image load decisions; `nosniff` interplay. |
| `Cache-Control` | response | Freshness directives: max-age, no-store, no-cache, must-revalidate, immutable, s-maxage ignored (no shared cache) (§5.17.4). |
| `Expires` | response | Legacy freshness; superseded by Cache-Control when both present. |
| `Pragma` | response | `no-cache` legacy honored; otherwise ignored. |
| `Age` | response | Seconds since origin generation; feeds freshness math. |
| `Date` | response | Stored for heuristic freshness and Age computation. |
| `ETag` | response | Strong/weak validators stored; used for If-None-Match revalidation. |
| `If-None-Match` | request | Conditional GET on revalidation; 304 applies stored headers. |
| `If-Modified-Since` | request | Date-based revalidation fallback. |
| `Last-Modified` | response | Stored for If-Modified-Since. |
| `Vary` | response | Per-header-value cache keying; `*` disables reuse (§5.2.6). |
| `Location` | response | Redirect target resolution per §5.2.4 (relative allowed). |
| `Refresh` | response | Non-standard meta refresh equivalent: parse and schedule navigation with a console note. |
| `Content-Encoding` | response | `gzip`/`deflate`/`br`(unsupported → error) decoded before the pipeline. |
| `Content-Disposition` | response | `attachment` triggers the download manager with filename parsing (§5.19.2). |
| `Set-Cookie` | response | Full RFC 6265bis parse in §5.17.1; multiple headers supported. |
| `Cookie` | request | Jar lookup with path/domain/secure/samesite rules. |
| `Strict-Transport-Security` | response | HSTS policy parse and persist (§5.3). |
| `Access-Control-Allow-Origin` | response | CORS check; wildcard-with-credentials rules (§5.18.3). |
| `Access-Control-Allow-Methods` | response | Preflight method check. |
| `Access-Control-Allow-Headers` | response | Preflight header check (case-insensitive). |
| `Access-Control-Expose-Headers` | response | Restricts JS-visible response headers on CORS responses. |
| `Access-Control-Allow-Credentials` | response | true enables credentialed CORS. |
| `Access-Control-Max-Age` | response | Preflight result cache TTL. |
| `Access-Control-Request-Method` | request | Preflight request marker. |
| `Access-Control-Request-Headers` | request | Preflight header list. |
| `Origin` | request | Sent on CORS and non-GET same-site requests; feeds the CORS check. |
| `Referer` | request | Per referrer policy (§5.18.5); strip on downgrade. |
| `Referrer-Policy` | response | Updates the document's referrer policy (strict-origin-when-cross-origin default). |
| `Content-Security-Policy` | response | Parse and enforce per §5.18.4 (report-only variant reports only). |
| `X-Content-Type-Options` | response | `nosniff` disables sniffing (§5.2.7); blocks wrong-type scripts/styles. |
| `X-Frame-Options` | response | DENY/SAMEORIGIN honored for iframe embedding decisions. |
| `Retry-After` | response | Parse only (no auto-retry policy in MVP). |
| `Range/Accept-Ranges/Content-Range` | both | Range requests for media (MVP: not sent; parse and ignore). |
| `Allow` | response | 405 reporting only. |
| `WWW-Authenticate/Authorization` | both | Parsed; engine sends Basic only when the shell credential prompt (MVP: none) supplies it; otherwise 401 error page. |
| `Upgrade` | response | 101 for WebSocket handshake (§6.9); otherwise ignored. |
| `Sec-WebSocket-*` | both | RFC 6455 handshake headers generated/parsed by the WebSocket client. |
| `Server` | response | Logged in the network panel only. |
| `Alt-Svc` | response | Ignored (no HTTP/2 in MVP) with a DevTools note. |
| `Link` | response | Preload hints parsed as best-effort; stylesheet variant honored. |
| `Timing-Allow-Origin` | response | Marks cross-origin resources for high-res timing exposure. |
| `Cross-Origin-Opener-Policy / -Embedder-Policy / -Resource-Policy` | response | COOP: report-only in MVP; COEP: ignored with console note; CORP: enforced for no-cors subresource fetches. |

## APPENDIX D — MIME Type Table

Document-type decisions (§5.2.7, §5.5.5); parameters such as `charset`
are honored where the row says so.

| MIME type | Engine behavior |
|---|---|
| `text/html` | HTML document pipeline (§5.4-5.5); charset parameter honored. |
| `application/xhtml+xml` | XML-ish: parse well-formed-only; failure renders the error document (MVP lenient note). |
| `text/plain` | Plain text document; wrap in <pre>-like display document. |
| `text/css` | Stylesheet only when the embedding allows it (nosniff honored). |
| `text/javascript + application/javascript` | Classic/module script depending on the element's type attribute. |
| `application/json` | Fetchable; standalone navigation renders a JSON viewer (MVP: plain text). |
| `image/png, image/jpeg, image/gif, image/bmp` | Image decoders (§5.12); standalone navigation renders the image in a generated document. |
| `image/x-icon + image/vnd.microsoft.icon` | ICO decoder; favicon selection source. |
| `image/webp, image/avif, image/svg+xml` | Not decodable (§1.3): treated as unsupported image (placeholder + error event for svg-in-img; webp/avif error). |
| `application/pdf` | Download (non-goal to render). |
| `audio/*, video/*` | Media element stub accepts; no decode (§1.4). |
| `application/octet-stream` | Download. |
| `multipart/form-data` | Request-side encoding for form POST (§5.6 forms); response-side multipart/x-mixed-replace ignored. |
| `application/x-www-form-urlencoded` | Form GET/POST body encoding (§5.1). |
| `text/xml, application/xml` | Well-formedness-checking parse; used by DOMParser text/xml. |
| `font/woff2, font/woff, font/ttf, font/otf` | Font loading from @font-face src (§5.11); format hints validated. |
| `application/wasm` | Not supported (stretch goal §1.5): fetch error page. |
| `unknown/missing types` | Sniffing per §5.2.7 for images and top-level documents; otherwise download. |

## APPENDIX E — CSS Units

Resolution rules live in §5.8.3 (style time) and §5.9 (layout time).

| Unit | Kind | Notes |
|---|---|---|
| `px` | length | CSS reference pixel; the anchor all others resolve to at style time. |
| `em` | length | Relative to the element's own font-size (for font-size: the parent's). |
| `rem` | length | Root element font-size; resolves at style time to absolute px. |
| `ex` | length | x-height of the first available font. |
| `ch` | length | Advance of the 0 glyph (U+0030) of the first available font. |
| `cap` | length | Cap height of the first available font. |
| `ic` | length | Advance of the water ideograph (U+6C34). |
| `lh` | length | Line height of the element. |
| `rlh` | length | Root line height. |
| `vw` | length | 1% of viewport width. |
| `vh` | length | 1% of viewport height. |
| `vi` | length | 1% of viewport inline axis. |
| `vb` | length | 1% of viewport block axis. |
| `vmin` | length | 1% of the smaller viewport dimension. |
| `vmax` | length | 1% of the larger viewport dimension. |
| `sv* family` | length | Small-viewport units (svw/svh); MVP maps to vw/vh (ADR if used). |
| `lv* family` | length | Large-viewport units (lvw/lvh); MVP maps to vw/vh. |
| `cm` | length | 1cm = 96px/2.54. |
| `mm` | length | 1mm = 96px/25.4. |
| `q` | length | quarter-millimeter. |
| `in` | length | 1in = 96px. |
| `pt` | length | 1pt = 96px/72. |
| `pc` | length | 1pc = 12pt. |
| `%` | percentage | Resolution context is per-property (containing block, font, or own box); see the property registry. |
| `deg` | angle | 360 per turn. |
| `grad` | angle | 400 per turn. |
| `rad` | angle | 2pi per turn. |
| `turn` | angle | 1 turn. |
| `s` | time | Seconds; animation/transition longhands resolve to ms. |
| `ms` | time | Milliseconds. |
| `Hz` | frequency | Parse-only (voice/a11y stretch). |
| `kHz` | frequency | Parse-only. |
| `dpi` | resolution | Dots per inch; resolution media queries and image-set(). |
| `dpcm` | resolution | Dots per centimeter. |
| `dppx` | x | Dots per pixel unit (= 96dpi); resolution queries. |
| `fr` | flex | Grid free-space unit; only inside track lists (§5.10.2). |

## APPENDIX F — Keyboard Event Map

The `KeyboardEvent.code`/`key` wiring and shell shortcut table (§5.19.1,
§6.11). Platform layouts may remap `key`; `code` is positional and stable.

| Key / shortcut | Category | Default action note |
|---|---|---|
| `Backquote/Digit1..Digit0/Minus/Equal` | typing keys | Produce characters via the platform layout map |
| `KeyQ..KeyM (letters)` | typing keys | Key values per current layout; printable keys feed text input |
| `Space` | typing keys | Character U+0020; page-scrolls when focus is not editable |
| `Tab` | navigation | Moves focus in DOM order (Shift reverses); preventDefault gives raw key |
| `Enter` | activation | Activates focused control; submits single-input forms; text areas insert newline |
| `NumpadEnter` | activation | Same as Enter |
| `Backspace` | editing | Deletes backwards in editables; history-back only when shell policy allows |
| `Delete` | editing | Deletes forwards in editables |
| `Escape` | navigation | Closes dialogs/menus; exits fullscreen (no-op here); cancels IME |
| `ArrowLeft/ArrowRight` | navigation | Text caret movement in editables; horizontal scroll fallback |
| `ArrowUp/ArrowDown` | navigation | Line navigation in editables; option cycling in select; page scroll fallback |
| `Home/End` | navigation | Line start/end in editables; scroll to edges in pages |
| `PageUp/PageDown` | navigation | Scroll by page |
| `Insert` | editing | Toggle overwrite mode (MVP: no-op) |
| `F1..F12` | function keys | Shell shortcuts where mapped (F12 DevTools, F5 reload, F11 fullscreen toggle, F6 focus omnibox, F10 menu) |
| `Shift/Control/Alt/Meta (Left/Right)` | modifiers | Modifier state machinery for getModifierState and shortcuts |
| `CapsLock` | modifier | Toggle state surfaced to getModifierState |
| `NumLock/ScrollLock` | modifier | State surfaced; no default behavior |
| `ContextMenu` | menu | Opens the context menu at selection/caret |
| `PrintScreen` | misc | No engine behavior |
| `Ctrl+T / Ctrl+W / Ctrl+Shift+T` | shell | New tab / close tab / reopen closed tab |
| `Ctrl+L / Alt+D / F6` | shell | Focus the omnibox |
| `Ctrl+R / F5 / Ctrl+Shift+R` | shell | Reload / reload (bypass cache) |
| `Alt+Left / Alt+Right` | shell | Back / forward |
| `Ctrl+D` | shell | Bookmark current page |
| `Ctrl+F` | shell | Find in page |
| `Ctrl+Plus/Minus/Zero` | shell | Zoom in/out/reset (10% steps) |
| `Ctrl+U` | shell | View source (generated document from --dump-dom path) |
| `Ctrl+S` | shell | Save page (MHTML-lite MVP: single-file HTML) |
| `Ctrl+J` | shell | Open downloads |
| `Ctrl+H` | shell | Open history |
| `Ctrl+Shift+I / F12` | shell | Toggle DevTools |
| `Ctrl+Shift+C` | shell | Inspect element mode (pick on next click) |
| `Ctrl+Tab / Ctrl+Shift+Tab` | shell | Next/previous tab |
| `Ctrl+1..8 / Ctrl+9` | shell | Select tab by index / last tab |
| `Ctrl+P` | shell | Print (no-op with console note; non-goal §1.4) |

## APPENDIX H — Console Message Catalog

The engine's console-facing messages (§5.15.4). Every message has a
stable id, a level, and a documented trigger; message text may change,
ids may not (tests and DevTools filters key on them). Add new messages
to `tools/wbs_data_b.py` and regenerate.

| Id | Level | Trigger |
|---|---|---|
| `ParseError.html` | info | HTML parse error with line:col and the standard's error code (§5.4). |
| `ParseError.css` | info | CSS parse error with line:col and the recovered-at token (§5.7). |
| `Encoding.restarted` | info | A late <meta charset> forced re-tokenization (§5.5.5). |
| `Encoding.fallback` | warn | No encoding declaration found; windows-1252 assumed per spec. |
| `Net.protocolError` | error | HTTP/1.1 violation with the offending bytes summarized (§5.2.3). |
| `Net.badStatus` | error | Status line outside 100–599; treated as network error. |
| `Tls.verificationFailed` | error | Certificate failure with the concrete rustls alert (§5.3). |
| `HSTS.applied` | info | Navigation upgraded http→https by HSTS policy. |
| `CORS.blocked` | error | Fetch failed the CORS check; reason not exposed to script (§5.18.3). |
| `MixedContent.blocked` | error | Blockable mixed content refused on an https page (§5.18.5). |
| `MixedContent.allowed` | warn | Optionally-blockable mixed content loaded (images) and reported. |
| `CSP.violation` | error | Directive violated; the resource was blocked; event fired (§5.18.4). |
| `CSP.reportOnly` | warn | Directive violated in report-only mode; resource allowed. |
| `Storage.quotaExceeded` | error | setItem over quota; QuotaExceededError thrown to script (§6.10). |
| `Cookie.rejected` | info | Set-Cookie rejected with the RFC rule that rejected it (§5.17.1). |
| `Cache.revalidated` | info | Conditional request returned 304; served from cache. |
| `Dom.liveCollectionMutation` | warn | Live collection mutated during iteration (perf hint). |
| `Dom.passiveDefault` | warn | preventDefault called on a passive wheel/touch listener (§5.6). |
| `Js.unhandledRejection` | error | Promise rejected with no handler at the microtask checkpoint (§5.14.5). |
| `Js.longTask` | warn | Task exceeded 50 ms with its source (§5.15.6). |
| `Js.trapInvariant` | error | Proxy trap returned a spec-invariant-violating value (§6.7). |
| `Image.decodeError` | error | Image bytes failed decoding; error event fired (§5.12). |
| `Image.unsupportedFormat` | warn | Format outside the supported set (webp/avif/svg-in-img, §1.3). |
| `Font.loadFailed` | error | @font-face source failed; fallback face used (§5.11.1). |
| `Font.fallbackUsed` | info | Glyphs missing in the primary font; fallback chain applied. |
| `Shaper.complexDelegated` | info | Complex script shaped via the HarfBuzz-backed shaper (§5.11.2). |
| `Layout.percentageIndefinite` | info | Percentage height resolved as auto due to indefinite containing block (§5.9.2). |
| `Paint.clipUnbalanced` | error | Internal: display list pushed/popped clips unevenly (should be unreachable; a bug report). |
| `Shell.downloadStarted` | info | Attachment Content-Disposition routed to the download manager (§5.19.2). |
| `Shell.fileOrigin` | warn | file:// fetch permitted within the directory policy (§5.18.6). |
| `NotSupported.feature` | info | Script requested a non-goal feature (§1.4); NotSupportedError path taken. |

