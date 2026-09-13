### §5.6 DOM core and events

**Standard basis:** WHATWG DOM Standard (Living Standard), UI Events, DOM Parsing and
Serialization, W3C DOM4 ranges/traversals.
**Position:** the substrate everything reads and script pokes; lives on the document thread.

**Public API sketch**

```rust
pub struct NodeId(u64); // index into Document.arena, upper bits = generation
pub enum NodeData { Document, DocumentType { name, public_id, system_id },
                    ShadowRoot { mode }, Element(ElementData), Text { contents: RefCell<String> },
                    Comment { contents: String }, ProcessingInstruction { target, data },
                    DocumentFragment, Attr { name, value } }
pub struct Node { pub id: NodeId, pub parent: Option<NodeId>, pub first_child: Option<NodeId>,
                  pub last_child: Option<NodeId>, pub prev_sibling: Option<NodeId>,
                  pub next_sibling: Option<NodeId>, pub data: NodeData }
pub struct Document { pub arena: Vec<Node>, pub root: NodeId, pub doctype: DoctypeInfo,
                      pub quirks: QuirksMode, pub base_url: Url, pub origin: Origin,
                      pub style_sheets: StyleSheetSet, pub scripts: ScriptSet,
                      pub event_dispatcher: EventRegistry, /* ... */ }
```

**Core surface (implemented in `aurora_dom`, bound to script in §5.15):** the full
`Node` operations (`insert`, `remove`, `replace`, `clone_node(deep)`,
`contains`, `compare_document_position`); `ParentNode` (`append`, `prepend`,
`query_selector`, `query_selector_all`, `children`, `first_element_child`, …);
`Element` (attribute map with the namespace model — HTML/MathML/SVG/XLink/XML namespaces
with `xmlns` fixups; `class_list` as a live `DOMTokenList`; `dataset`; `attach_shadow`
with open/closed modes and slot assignment); `Text` (`split_text`, `data` normalization
on parent operations); live vs static collections (`HTMLCollection` is **live** and
recomputes on access against the current tree; `NodeList` from `querySelectorAll` is static);
`Range` (boundary points with document-order comparison, `extract_contents`,
`clone_contents`, `insert_node`, `surround_contents`); `TreeWalker`/`NodeIterator`
with the four-filter states; `DOMParser`, `XMLSerializer` (attribute escaping table:
`&`, `<`, `>`, `"`, NBSP in attributes; text nodes escape `&<>` only).

**Events**

- `EventTarget` dispatch: capture (root→target), target, bubble (target→root, unless
  `composed: false` stops at shadow boundary); `stop_propagation`, `stop_immediate_propagation`,
  `prevent_default` (and `passive` listeners that make `preventDefault` a no-op with a
  console warning); `once` listeners; the *legacy-pre-activation*/*activation* behavior
  for `click` on links, checkboxes, radio groups, buttons, labels, form submission.
- Event construction: the typed events of WBS §6.8 with their standard init dictionaries.
- MutationObserver: queue records per observer until the microtask checkpoint
  (§5.15.3), with `childList`/`attributes`/`characterData` + `subtree`/`oldValue`.
- ResizeObserver and IntersectionObserver at the fidelity of §1.3 (single viewport,
  root = document, threshold list) — enough for common lazy-loading and responsiveness
  patterns.

**Shadow DOM (MVP):** attach, slot assignment (flat tree traversal for style/layout),
`::slotted()` styling, `Event.composedPath()`. Declarative shadow templates `<template
shadowrootmode>` are a stretch goal.

**Invariants:** no cross-thread DOM access, ever — the DOM is single-threaded by design
(§4.4); all mutation goes through `Document` methods so mutation observers and the
style/layout invalidation hooks (§5.8.6) cannot be bypassed; `clone_node` copies data,
not live state (no copying of event listeners, style, or script results).

**Pitfalls:** live collections are the classic infinite-loop generator (`while (n =
list.item(0)) n.remove()` terminates; naive caching does not); `compare_document_position`
bit table; text node merging after `normalize()` and after `split_text`; attribute
order is insertion order and matters for serialization equality tests.

**Definition of done:** the DOM test corpus (WPT `dom/` subset adopted per §8.6)
passes; every mutation API fires the right observers; serialization round-trips
`XMLSerializer.parse` for a 200-node torture tree; zero panics under
`fuzz_dom_api` (random legal call sequences from a grammar of DOM operations).

### §5.7 CSS tokenizer and parser

**Standard basis:** CSS Syntax Level 3 (tokenization + parsing into at-rules/
qualified rules/declarations), CSS Values and Units 4, CSS Color 4 (the color syntaxes
of WBS §6.3 preamble).
**Position:** stylesheets and inline styles, parsed once, stored interned (§4.5).

**Public API sketch**

```rust
pub enum CSSToken { Ident(Atom), Function(Atom), AtKeyword(Atom), Hash { value: Atom, unitless: bool },
                    String(String), BadString, Url(String), BadUrl, Delim(char),
                    Number { value: f64, int: bool, sign: Option<char> },
                    Percentage(f64), Dimension { value: f64, unit: Atom },
                    Whitespace, Colon, Semicolon, Comma, Delims([char; 4]) /* [] () { } */,
                    CDO, CDC, EOF }
pub struct Stylesheet { pub rules: Vec<CSSRule>, pub origin: Origin /* UA|User|Author */,
                        pub base_url: Url, pub media: Option<MediaQueryList> }
pub enum CSSRule { Style(StyleRule), Import(ImportRule), Media(MediaRule), Supports(SupportsRule),
                   FontFace(FontFaceRule), Keyframes(KeyframesRule), Page(PageRule),
                   Namespace(NamespaceRule), Charset(String), Layer(LayerRule), Unknown(AtRuleBlock) }
pub struct StyleRule { pub selectors: Vec<Selector>, pub declarations: Vec<Declaration>,
                       pub specificity: Specificity, pub source_location: SourceLocation }
pub struct Declaration { pub name: Atom, pub value: Vec<CSSToken>, pub important: bool,
                         pub custom: bool /* name starts with -- */, pub location: SourceLocation }
```

**Core algorithm**

- **Tokenizer:** per CSS Syntax §4.1 with the standard's consumed/comment-stripped
  input stream; escapes (`\` + 1–6 hex + optional whitespace, literal-escape for
  non-ident chars); numeric parsing including the `-0` and exponent cases; URL token
  with its own escapes; bad-string/bad-url recovery. No regexes anywhere in this path.
- **Parser:** a top-down recursive parser producing the rule tree, implementing
  error recovery exactly ("parse error, consume the remnants of a declaration /
  block, return nothing") — malformed declarations are dropped individually;
  malformed rules drop to the enclosing block boundary. Custom properties (`--*`)
  are token-preserving (stored as token lists, substituted at computed-value time).
- **Values:** each property's value grammar is implemented by the property registry
  (§5.8.4) using shared value parsers: lengths/percentages (with number/length
  distinction — a number is *not* a length except where the property says so, e.g.
  `line-height`/`flex-grow`), colors (hex 3/4/6/8, `rgb()/rgba()`, `hsl()/hsla()`,
  `hwb()`, `lab()/lch()/oklab()/oklch()` parse; `color()` with `srgb`/`display-p3`/
  `srgb-linear`/`xyz-d65`, `system-color` keywords, the 148 named colors of Appendix A,
  `transparent`/`currentColor`, `color-mix()`), angles, times, resolutions, `calc()`
  (parse into a typed op tree; evaluate at computed-value time with unit algebra),
  keywords per property, and shorthands expanding into longhands with the standard's
  exact expansion rules (including `border` resetting `border-image`).
- **At-rules:** the WBS §6.4 set; unknown at-rules are *preserved* as `Unknown` blocks
  (needed for `@supports`-like forward-compat behavior and DevTools display).

**Invariants:** the parser is total on any byte string; parse errors are recoverable
and reported with line/column for DevTools; interning makes `StyleRule` equality
cheap; a stylesheet is immutable after parse (media-condition changes create new
matching state, they do not mutate the sheet).

**Pitfalls:** CDO/CDC tokens *inside* qualified rules are just tokens; `!important`
placement (space-separated after value, before semicolon); `@media` nested blocks
are rules, not declarations; unitless zero is a length, unitless nonzero is not;
dimension tokens with weird units (`12apples`) are valid tokens, invalid values —
that distinction happens at the property registry, not the tokenizer.

**Definition of done:** CSS Syntax test corpus from css-parsing-tests passes
(component value lists, declarations, rules, color, an+b); every WBS §6.3 property's
grammar parses and round-trips through `serialize`; fuzz `fuzz_css` 30 min no panics.

### §5.8 Selectors, cascade, and computed values

**Standard basis:** Selectors Level 4, CSS Cascading and Inheritance 4/5, CSS Conditional
(media/supports), CSSOM (style attributes, `getComputedStyle` shape).
**Position:** DOM + stylesheets → computed styles per element.

**Public API sketch**

```rust
pub struct Selector { pub compound: Vec<SimpleSelector>, pub combinators: Vec<Combinator>, /* right-to-left plan */ }
pub enum SimpleSelector { LocalName(Atom), Namespace(Option<Atom>), ID(Atom), Class(Atom),
                          Attribute { ns, name, op: AttrSelectorOperator /* =,~=,|=,^=,$=,*= */, value },
                          PseudoClass(PseudoClassKind), PseudoElement(PseudoElementKind),
                          Scope, Nesting, Is(Vec<Selector>), Not(Vec<Selector>), Where(Vec<Selector>),
                          Has(Vec<Selector>) /* plan-driven, see below */, NthChild { a, b, of: Option<...> } }
pub struct CascadeData { pub rules: Vec<RuleHash>, pub element_index: SelectorMap,
                         pub inheritance_roots: /* … */ }
pub struct ComputedValues { /* one typed struct per longhand group, e.g.: */
    pub color: Color, pub font: FontValues, pub box_: BoxValues, pub background: BackgroundValues, /* ... */ }
```

**Core algorithm**

1. **Selector matching:** compile selectors into a right-to-left matching plan at
   parse time. Match against the DOM via a `MatchingContext` that walks parent links.
   Pseudo-classes: structural (`:nth-child/an+b` parser per css-syntax an+b tests,
   `:first/last/only-child`, `:nth-of-type` family), link-state (`:link`, `:visited`
   — always styled as unvisited; see §5.18.6 for why), user-state (`:hover` from the
   input system, `:focus`/`:focus-visible`/`:focus-within`, `:active`, `:disabled`,
   `:checked`, `:placeholder-shown`), direction (`:dir(ltr|rtl)` from the bidi state),
   language (`:lang()` from `lang` attribute walk-up), resource-state (`:defined`,
   `:fullscreen` false, `:modal` for dialog), logical (`:is()/:not()/:where()/:has()`
   — `:has()` is implemented as a cached descendant/sibling scan invalidated by the
   invalidation sets of §5.8.6; if it cannot meet budget, it is disabled by ADR with
   the fallback documented).
   Pseudo-elements: `::before`, `::after` (with `content` values: strings, `attr()`,
   counters, `open-quote/close-quote/no-*/normal/none`), `::placeholder`, `::selection`
   (paint stage), `::marker` (list items), `::first-line`/`::first-letter` (block-level
   approximation per the spec's loosened model).
2. **Cascade:** collect matching declarations from UA → user → author sheets plus
   `style` attribute (author-level, highest of author except `!important` inversion),
   animation declarations, and presentation attributes (lowest author tier, below
   author `*` rules). Sort by: origin+importance order per CSS Cascade §6, then
   `@layer` order, then specificity, then source order (stable, last wins).
   `revert`/`revert-layer` roll back to the earlier origin/layer.
3. **Computed values:** resolve every registered longhand: `inherit` (parent's
   computed), `initial` (the per-property initial from WBS §6.3), `unset`
   (inherit-if-inherited else initial), keywords→concrete, relative lengths
   (`em/ex/rem/ch` against font metrics, `vh/vw/vmin/vmax` against the viewport,
   `%` stays percentage where it is percentage-resolved at layout), `calc()` algebra,
   custom property substitution (with cycle detection → the standard's
   invalid-at-computed-value-time → `unset` behavior), `currentColor` resolution,
   defaulting: inherited properties inherit, the rest initial.
4. **The style system cache:** rule nodes deduplicate cascade paths; sibling style
   sharing (elements with identical "relevant link visited" state, tag name, class/id
   sets, attribute sets, and non-matching of "sensitive" selectors share computed
   values until anything invalidates) is implemented behind the same `ComputedValues`
   lookup so disabling it is one flag (correctness first, then measure).

**Invalidation protocol (how changes reach style):** `Document` mutations record
cheap dependency bits: `ElementDescendantsMayMatch::{Class,Id,Attr,State}` sets
maintained from selectors at parse time. A mutation enqueues invalidation of the
narrowest applicable set; unknown selectors (e.g., `:has()`) invalidate the whole
subtree — correct and slow until proven hot by profiling.

**Invariants:** cascade is a pure function of (element position in tree, its
attributes/state, stylesheets, style attribute) — no hidden inputs; computed values
never contain `em`/`%` where the spec says absolute (layout never sees an unresolved
relative unit that was resolvable at style time); `getComputedStyle` returns the
computed values *and* resolved used values for the layout-dependent set
(`width`, `height`, margins, `line-height` as used px, …) by consulting the last
fragment tree, falling back to computed when not laid out.

**Definition of done:** adopted WPT selectors + cascade subset green; a cascade
torture test (origin/importance/layer matrix, 60 cases, hand-written expected
values); style-sharing flag flip changes nothing in any pixel test; restyle of a
100k-element page (generated) meets the §10.1 budget.

### §5.9 Layout I — boxes, block, and inline formatting

**Standard basis:** CSS 2.1 §8–10 (box model, positioning schemes, block/inline),
CSS Display 3 (display ↔ formatting contexts, anonymous boxes), CSS Box Sizing 4,
CSS Overflow 3, CSS Position 3, CSS Multicol 1 (columns), CSS Logical 1.
**Position:** style tree → fragment tree (geometry), on the document thread.

**Public API sketch**

```rust
pub struct Box { pub style: StyleRef, pub children: LayoutBoxChildren /* Block-level | Inline-level */,
                 pub kind: BoxKind /* Block | Inline | FlexItem | GridItem | TableWrapper ... */,
                 pub contents: BoxContents /* Text runs | Replaced { image, intrinsic } | Subtree */ }
pub struct Fragment { pub rect: LogicalRect /* position + size in the containing block */,
                      pub style: StyleRef, pub children: Vec<Fragment>, pub clip: ClipChainId,
                      pub kind: FragmentKind /* same taxonomy as boxes + text runs */ }
pub trait LayoutFlow { fn layout(&self, ctx: &LayoutContext, containing: &ContainingBlock,
                                 layout_in: &LayoutInput) -> LayoutResult; }
```

**Core algorithm**

1. **Box construction:** from the DOM+styles, generate boxes per CSS Display —
   `display:none` prunes; inline/block mixtures produce anonymous boxes; `display:
   contents` is transparent; replaced elements (images, form controls, canvas,
   iframe placeholder) become atomic boxes with intrinsic sizes from their content;
   `::before/::after` generate boxes per `content`.
2. **Block formatting context (BFC):** children stacked vertically; margins collapse
   (the full adjoining-margins algorithm: adjacent siblings, parent/first+last child,
   through zero-height boxes; never through BFC-establishing boxes, floats, or
   `overflow: non-visible` roots); height: `auto` = content, percentage heights
   resolve against the containing block's definite height, `min-/max-` clamped.
   BFC establishment: root, floats, absolutely positioned, `overflow≠visible`, flex/
   grid items, `display: flow-root`, cells/captions.
3. **Inline formatting context:** per CSS 2.1 §10.8 + CSS Text/Inline 3 — line boxes
   built from inline runs: text runs shaped by §5.11, replaced content, inline-blocks
   as atomic inlines; baseline alignment per `vertical-align` (baseline, sub/super,
   `length`, `middle`, `top/bottom` of line box); line height = leading distribution;
   the strut; text wrapping with the §5.11.3 line breaker; `white-space` variants
   (`pre`, `nowrap`, `pre-wrap`, `pre-line`, `break-spaces`); `text-align` including
   `justify` (expand inter-word spaces, disable after forced breaks).
4. **Floats and clearance:** the float placement algorithm (top of current line/last
   float, move down until it fits, offset per `clear`), the float *intrusion* model
   for line boxes (left/right edge shrinking), BFC roots placed beside floats per
   CSS 2.1 §9.5. Floats participate in the parent BFC and are *not* part of line
   box content that flows around them — lines shorten, boxes are moved.
5. **Positioning:** `relative` (offset the placed fragment, keep original space);
   `absolute` (containing block = nearest positioned ancestor's padding box; static
   position fallback for unspecified insets; shrink-to-fit width per §10.3.7 equation);
   `fixed` (viewport); `sticky` (constrained rectangle between containing-block edges
   and its static-position box — computed against scroll state each frame).
   Stacking/containment interactions: `position`/`float`/`display` normalization per
   CSS 2.1 §9.7's table.
6. **Overflow and clipping:** `overflow` per box (scrollable overflow region;
   scrollbars on the shell's scrollable ancestors per `overflow: auto`), `clip-path`,
   `border-radius` clipping pushed to the clip chain for paint (§5.16.3).
7. **Fragmentation (columns):** `columns`/`column-*` — split content into column
   boxes of equal height, `column-fill: balance` default; `break-*` honored at
   block boundaries only in the MVP.

**Invariants:** layout is a pure function of (box tree, computed styles, containing
block, available space, viewport state) — no clocks, no randomness, no DOM reads
outside the passed context (enables the incremental design and tests); percentages
that cannot resolve (indefinite containing block height) become `auto` per spec;
layout never mutates the DOM (no `LayoutNode` writes back) — even where the spec
allows it, style recomputation is the write path.

**Definition of done:** the layout golden-file suite (§8.4) covers margin collapsing
(20 cases), float wrap (15), absolute positioning (12), inline line breaking and
alignment (20), overflow/scroll regions (8); the §7.6/§7.9 milestone demos render
their reference pages with no pixel-diff regressions; `layout` meets §10.1 budgets.

### §5.10 Layout II — flexbox and grid

**Standard basis:** CSS Flexible Box 1, CSS Grid 2 (through subgrid: parse it, treat
as regular grid in the MVP), CSS Box Alignment 3.
**Position:** layout stage; flex/grid containers establish BFC-external contexts.

**Flexbox algorithm (the ordered machine, CSS Flexbox §9):** generate flex items
(including anonymous items from raw text runs); resolve flexible lengths:
1. Determine the base size per item (`flex-basis` → `width/height` → `max-content`
   via intrinsic sizing passes; `box-sizing` honored) with hypothetical main size
   after `min-/max-` clamping.
2. Collect inflexible items (`flex-grow=0` and `flex-shrink=0`, or fixed basis) —
   they freeze.
3. Main-axis distribution: if free space > 0, distribute by `flex-grow` weighted
   (scaled flex shrink factor per spec when freezing); if negative, shrink by
   `flex-shrink` weighted by scaled outer size; iterate freeze- unfreeze rounds per
   §9.7 ("resolve flexible lengths" with the min-content floor honored via
   `min-width: auto` = min-content for items with `overflow: visible`).
4. Cross-axis: items stretched (`align-self: stretch`) or sized; container cross
   size per `align-content` when the container's cross size is `auto`.
5. Line packing (`flex-wrap: wrap`): items grouped by hypothetical main size,
   lines sized by max cross size, `align-content` distributes lines.
6. Main-axis alignment: `justify-content` including `space-between/around/evenly`;
   `gap` applies between items/lines as fixed spacing.
7. Absolute children: static position from the container's content box per alignment
   properties.

**Grid algorithm (CSS Grid §7–12):** parse track lists (`<track-list>` with
`minmax()`, `fit-content()`, repeat notations incl. `auto-fill/auto-fit`); place
items (auto-placement cursor with the dense/sparse algorithms, definite placement by
line numbers/named areas); compute track sizes: base sizes via intrinsic contributions
(min/max-content of the items in the track), then the space-distribution loop:
grow base sizes to match growth limits, then distribute remaining free space per
`fr` factors (find-the-size-of-an-unspecified-fr algorithm, §12.7); align (`justify-*`,
`align-*` per axis, `gap`); build item fragments from the resolved track positions.
Nested/subgrid: parse `subgrid` and implement it as "inherit the parent's tracks in
that axis" if budget allows, else ADR-defer with a parse-only acceptance.

**Invariants:** both algorithms are deterministic and terminate (the flex
freezing loop and the grid distribution loop each have documented maximum rounds and
assert them in debug builds); intrinsic sizing passes (`min-content`,
`max-content`) are implemented as layout requests into the same machinery with an
"available space = infinite" mode — no second code path.

**Definition of done:** flexbox torture suite (the 40 hand-written cases from the
flexbox WPT corpus subset adopted in §8.6) green; grid: the 30-case suite covering
auto-placement, `fr` distribution, named areas, spanning items, alignment; the
§7.13 showcase page's flex/grid sections pixel-match their golden files.
