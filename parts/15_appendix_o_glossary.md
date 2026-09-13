## APPENDIX O — Glossary of Engine Terms

The seed for `docs/GLOSSARY.md` (§9.1 mandates abbreviations be registered
here); extend it in the same session that coins a term.

| Term | Meaning in this codebase |
|---|---|
| AA | Anti-aliasing; the rasterizer's analytic coverage (§5.16.2). |
| Adoption agency | The HTML algorithm that repairs mis-nested formatting elements (§5.5). |
| ADR | Architecture Decision Record (§4.9). |
| an+b | The nth-child step/offset grammar (§6.5). |
| BFC | Block formatting context (§5.9.2). |
| Binding | The script↔native bridge implementing a DOM interface (§5.15.2). |
| Blink | Reference engine (Chrome); never a dependency — the word appears here only in tests' naming of behaviors. |
| Box tree | The display/style-driven structure before layout (§5.9.1). |
| Chunked | HTTP/1.1 transfer coding framing (§5.2.3). |
| Clip chain | The ordered stack of clips applied to display items (§5.16.3). |
| Computed value | The per-longhand resolved style after cascade and defaulting (§5.8.3). |
| CSPRNG | Cryptographically secure random source used by `crypto.getRandomValues` (§6.6 Crypto). |
| Document thread | The §4.4 thread owning a document's DOM, script, and pipeline. |
| DOMException | The script-visible exception taxonomy (Appendix M). |
| DPR | Device pixel ratio; shell→engine scale factor (§5.19.4). |
| Facade | A crate's single re-export surface (§4.10). |
| Foster parenting | Moving out-of-place table content before the table (§5.5). |
| Fragment | A laid-out piece of a box: the geometry product (§5.9). |
| GC heap | The JavaScript engine's traced object heap (§5.14.6). |
| Hit test | Display-list reverse lookup of the element under a point (§5.16.4). |
| IDL | WebIDL; the interface definition language of the DOM surface (§5.15.2). |
| IFC | Inline formatting context (§5.9.3). |
| Insertion mode | The tree constructor's state (§5.5). |
| Interning | Deduplicating strings/selectors into shared tables (§4.5). |
| InvalidAtComputedValueTime | Custom-property failure mode resolving to unset (§5.8.3). |
| Live collection | A DOM list that recomputes against the current tree (§5.6). |
| Loader thread | The §4.4 thread running DNS/TCP/TLS/HTTP. |
| NaN-boxing | The JS value encoding (§5.14.1). |
| Noah's Ark clause | Maximum of 3 identical active formatting elements (§5.5). |
| Origin | The security principal: tuple or opaque (§5.18.1). |
| Paint order | CSS 2.1 Appendix E ordering of display items (§5.16.1). |
| Presentational hint | Attribute-sourced style at the lowest author tier (§5.8.2). |
| Quirks mode | Legacy compatibility mode selected by DOCTYPE (§5.5.3). |
| Revalidation | Conditional request re-checking a cached entry (§5.2.6). |
| Safepoint | A point where the interpreter may pause for GC or interrupts (§5.14.6). |
| SameSite | The cookie cross-site sending policy (§5.17.1). |
| Scoping root | The `@scope`/`querySelector`-context root element (§6.5). |
| Shape | The JS hidden-class record of an object's properties (§5.14.2). |
| Shaping | Turning characters into positioned glyphs (§5.11.2). |
| Sniffing | Content-type inference from bytes (§5.2.7). |
| Spec-first | The §2.2 rule: read the standard before writing the code. |
| SOP | Same-origin policy (§5.18). |
| Stack | The tree constructor's stack of open elements (§5.5). |
| Strut | The imaginary zero-content line box ensuring minimum line height (§5.9.3). |
| Stacking context | The atomic painting subtree boundary (§5.16.1). |
| Task / microtask | Event-loop scheduling units and their ordering rules (§5.15.1). |
| TDZ | Temporal dead zone for let/const bindings (§5.14.3). |
| Tier 0/1/2 | The §3.2 dependency classes. |
| Tombstone | A logically removed arena node kept for generation safety (§4.5). |
| UA origin | The user-agent stylesheet's cascade origin (§5.8.2). |
| Vertical slice | A milestone cut that works end to end (§2.3). |
| WBS | The work breakdown structure (Part 6). |
| WPT | Web Platform Tests (§8.6). |
| test262 | The ECMAScript conformance suite (§8.6). |
