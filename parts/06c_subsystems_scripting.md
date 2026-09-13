### §5.11 Text, fonts, and shaping

**Standard basis:** CSS Fonts 4, CSS Text 3/4, CSS Writing Modes 4 (horizontal only
in the MVP; `vertical-rl` is a stretch), UAX #14 (line breaking), UAX #9 (bidi),
UAX #29 (grapheme/word segmentation), OpenType shaping via the approved shaper (§3.3).
**Position:** the layout stage consults `aurora_text` for every text run.

**Public API sketch**

```rust
pub struct FontDatabase { /* system fonts + @font-face entries, cached descriptors */ }
impl FontDatabase { pub fn match_font(&self, family_chain: &[FamilySpecified], style: FontStyle,
                     weight: FontWeight, stretch: FontStretch) -> FontHandle; }
pub struct ShapedRun { pub glyphs: Vec<ShapedGlyph>, pub advance: f32, pub font: FontHandle,
                       pub script: Script, pub direction: Direction, pub language: Language }
pub struct ShapedGlyph { pub id: u16, pub cluster: Range<usize>, pub offset: Point, pub advance: f32 }
pub trait TextShaper { fn shape(&self, text: &str, font: &FontHandle, script: Script,
                         language: Option<Language>, features: &FeatureSet) -> ShapedRun; }
pub struct LineBreaker<'a> { /* UAX#14 with CSS white-space overrides */ }
```

**Core behavior**

1. **Font selection:** walk the `font-family` chain (generic `serif/sans-serif/
   monospace/cursive/fantasy/system-ui/ui-serif/...` mapped per platform config);
   match per CSS Fonts §5.2 (weight distance rules, style, stretch). `@font-face`
   (WBS §6.4) contributes local/remote fonts with `font-display` phases
   (block ≤ 3 s, swap, fallback) — loading happens async; text paints with the
   fallback until the face arrives, then relayouts.
2. **Shaping:** Latin and simple scripts shape through the owned shaper (cmap +
   kern + basic ligatures); complex scripts (Arabic, Indic, Hangul jamo, Thai)
   delegate to the approved HarfBuzz-based shaper. Shaping results are cached
   (§4.5) keyed by (text, font, script, features) with an LRU byte budget.
3. **Line breaking:** UAX #14 opportunities, overridden by `white-space`,
   `word-break`, `overflow-wrap` (break-word), `hyphens` (manual `-` only in MVP;
   auto-hyphenation is a stretch), `line-break`, and CJK rules (`line-break: strict`
   table for forbidden-start/end characters). Breaking produces *break points*, the
   inline layout (§5.9.3) chooses among them per line width.
4. **Bidi:** UAX #9 on each block: paragraph direction from `dir`/`direction` +
   first-strong heuristic, embedding levels, reordering at paint time; resolved
   per-inline-run level array stored on the fragment for paint ordering.
5. **Metrics:** the em-box, ascent/descent/line-gap from the font (usWin*/typo*
   per CSS Fonts §4.4 rules), the strut per line box, `letter/word-spacing`
   distribution, `tab-size` expansion, `text-indent` (including `each-line`,
   `hanging`), soft-wrap around shaped runs only at cluster boundaries.

**Invariants:** all measurement goes through shaped runs — never font-size × count
heuristics outside the M4 pre-text placeholder; shaping is cached and immutable;
the same input produces the same run (deterministic shaper settings, no variation
selectors surprises after normalization which is *not* applied — text is measured
as-is per platform conventions).

**Definition of done:** line-break test corpus (100 strings × `white-space` matrix)
matches expected break points; bidi reordering tests (30 mixed-direction cases)
reorder per UAX #9; font fallback chain renders the Unicode sample page without
tofu for Latin/Cyrillic/Greek/Arabic/CJK on the reference platform; shaping cache
respects its budget (test by oversizing the cache and checking eviction).

### §5.12 Images and decoders

**Standard basis:** PNG (RFC 2083 + extensions), JPEG (ITU T.81 baseline + progressive),
GIF89a, BMP, ICO/CUR; CSS `image-*` properties for orientation/sizing; `<img>`
/`<picture>`/`srcset` behaviors from WHATWG HTML §4.8.
**Position:** network/cache → decoder → frame cache → layout intrinsic size → paint.

**Public API sketch**

```rust
pub trait ImageDecoder { fn sniff(&self, bytes: &[u8]) -> bool;
                         fn dimensions(&mut self, bytes: &[u8]) -> Result<(u32, u32), DecodeError>;
                         fn frames(&mut self, bytes: &[u8], limits: &DecodeLimits)
                                   -> Result<Vec<DecodedFrame>, DecodeError>; }
pub struct DecodedFrame { pub width: u32, pub height: u32, pub format: PixelFormat /* RGBA8 */,
                          pub delay_ms: Option<u32>, pub dispose: DisposalMethod,
                          pub blend: BlendMethod, pub data: Vec<u8> }
pub struct ImageCache { /* key: Url (+Vary), states: pending→decoded(err|frames), LRU by bytes */ }
```

**Decoders (each its own module, each with its own fuzz target)**

- **PNG:** signature, chunk walker with CRC checks, IHDR constraints, palette
  (`PLTE`/`tRNS`), grayscale/RGB/palette/gray+alpha/RGBA at 1/2/4/8/16 bits,
  filters 0–4 per row, interlacing (Adam7 pass extraction), DEFLATE via the owned
  inflate (or approved crate until it lands), gamma/sRGB-aware conversion is
  clamp-to-sRGB (color management is a stretch), `iCCP` ignored with a console note.
- **JPEG:** baseline sequential DCT first (huffman tables — including the standard
  tables for omission, quantization tables, IDCT (you may use a fixed-point AAN
  implementation — cite it), upsampling of 4:2:0/4:2:2, YCbCr→RGB, grayscale);
  progressive then restart-marker robustness.
- **GIF:** LZW decompression (write it; ~150 lines), interlace, frame composition
  with disposal/blend methods, delay parsing; animation advances on the compositor
  clock (§5.16.6).
- **BMP:** the common header variants (BITMAPINFOHEADER 1/4/8/24/32bpp,
  uncompressed + BI_RGB/BITFIELDS), RLE4/RLE8, bottom-up/top-down, ICO directory +
  embedded BMP/PNG resolution selection.

**Behaviors:** `<img>` fires `load`/`error` correctly with `alt` text rendering on
error; intrinsic size (with EXIF orientation applied — parse the TIFF orientation
tag from JPEG APP1 and `image-orientation: from-image` default per CSS Images 4);
`srcset`/`sizes` selection (density and width descriptors, DPR from the shell);
`loading=lazy` via IntersectionObserver (§5.6); broken-image rendering per the
HTML spec's alt-text rules; SVG images are **not** decoded (an `<svg>` in an `<img>`
renders the placeholder box; recorded as a scope decision in §1.3).

**Invariants:** decoders are total on arbitrary bytes (`DecodeError` or frames,
never a panic — this is the primary fuzz surface); a decoded frame is immutable and
`Arc`-shared; the cache bound is in bytes (Part 10) with LRU eviction and a
"recently requested cannot evict" pin for the current frame.

**Definition of done:** each decoder has a corpus test (the project's own generated
reference set + the public test images checked into `tests/corpus/images/`) with
hash-verified outputs; fuzz targets `fuzz_png`, `fuzz_jpeg`, `fuzz_gif`, `fuzz_bmp`
run clean 30 min each; malformed-image error events verified end to end in the
shell.

### §5.13 JavaScript — lexer and parser

**Standard basis:** ECMA-262 (Living Standard) lexical grammar §11–12 and syntactic
grammar §13–15; the parser produces the standard's node taxonomy.
**Position:** string → AST; the interpreter (§5.14) consumes it.

**Public API sketch**

```rust
pub enum TokenKind { Identifier(Atom), PrivateName(Atom), Keyword(Keyword), Punctuator(Punct),
                     NumericLiteral(NumericValue /* decimal|hex|octal|binary|bigint */),
                     StringLiteral(String), TemplateString { cooked: Option<String>, raw: String },
                     RegexLiteral { pattern: String, flags: String }, NoSubstitutionTemplate, /* etc. */ }
pub struct Lexer<'s> { /* unicode mode, goal symbols (InputElementHashOrRegex Div) */ }
pub struct Parser { options: ParseOptions /* target: script|module, strict default */ }
impl Parser { pub fn parse_script(&mut self, src: &JsString) -> Result<ScriptAst, SyntaxError>;
              pub fn parse_module(&mut self, src: &JsString) -> Result<ModuleAst, SyntaxError>; }
pub enum Expr { Literal, Identifier, Array, Object, Function, Arrow, Class, Call { callee, args, optional_chain },
                Member { obj, prop, computed, optional_chain }, New, Template, TaggedTemplate,
                Conditional, Assign { op, lhs, rhs }, Binary { op, l, r }, Unary, Update,
                Yield, Await, Spread, Sequence, ... }
pub enum Stmt { Var, Function, If, While, DoWhile, For, ForIn, ForOf, Switch, Try, Throw,
                Block, Labeled, Return, Break, Continue, Debugger, Expression, With, Empty, ... }
```

**Core behavior**

- **Lexer:** automatic semicolon insertion (ASI) implemented as the standard's three
  rules applied at parse positions, not as a pass; unicode identifiers (ID_Start/
  ID_Continue via the Unicode crates of §3.3); numeric literals including separators
  (`1_000`), bigint (`10n`), legacy octal only in sloppy non-strict strings; template
  lexing with the substitution-state machine; regex literal vs division disambiguation
  per the goal-symbol rules; strict-mode early errors collected during parse.
- **Parser:** recursive descent with the standard's productions, producing a
  red-green-style AST (trivia-free, with source spans for DevTools and error messages).
  Early errors enforced: duplicate `__proto__` in object literals (lexical vs non),
  strict-mode restrictions (with, octal, delete of unqualified, duplicate params,
  `eval`/`arguments` assignment), `let` disambiguation ([no LineTerminator here]
  rules), private-name scope resolution at parse time, cover grammars
  (`CoverParenthesizedExpressionAndArrowParameterList`, `CoverInitializedName`) —
  implemented exactly, reinterpreted after the lookahead decision.
- **Modules:** `import`/`export` (named, default, namespace, string, attributes),
  the module record with its dependency graph, cycles resolved per spec (function
  hoisting across cycles works), top-level `await` allowed.
- The AST is Arena-allocated; nodes carry spans; the parser is total on any string
  (`SyntaxError` with line/column, never a panic).

**Definition of done:** adopted test262 subset for lexer/parser (`language/`
lexical and early-error tests, §8.6) passes; ASI torture cases (the classic 30)
parse exactly as the standard says; fuzz `fuzz_js_parser` no panics 30 min;
parse of the 1 MB synthetic script meets the §10.1 budget.

### §5.14 JavaScript — interpreter, GC, and builtins

**Standard basis:** ECMA-262 execution semantics (§8–10 execution contexts, §27
control abstractions), Built-ins §18–26, ECMAScript job queue integration from
HTML §8.1.7 (§5.15), Well-Known Symbols.
**Position:** everything script-visible flows through here; the DOM is reached only
via host objects (§5.15.2).

**Public API sketch**

```rust
pub struct JsValue(Inner); // NaN-boxed: double | i32-ish int | pointer to heap cell | special (undefined/null/bool)
pub enum HeapCell { Object(GcObject), String(JsString), Symbol, BigInt, Double, Module, Private, ... }
pub struct GcObject { pub prototype: GcRef<HeapCell>, pub extensible: bool,
                      pub properties: PropertyMap /* shapes + ordered dict for index props */,
                      pub kind: ObjectKind /* Ordinary | Function(FunctionFlags) | Array |
                       exotic: String|Arguments|TypedArray|Proxy|BoundFunction|ModuleNamespace | Host(HostObjectRef) */ }
pub struct Interpreter { heap: GcHeap, realm: Realm, frames: Vec<CallFrame>, /* ... */ }
impl Interpreter { pub fn run_script(&mut self, ast: ScriptAst) -> Result<JsValue, ThrownValue>;
                   pub fn call(&mut self, f: JsValue, this: JsValue, args: &[JsValue]) -> Result<JsValue, ThrownValue>; }
pub enum ThrownValue { Js(JsValue) /* the thrown object */, Host(HostError) /* stack overflow, OOM, interrupt */ }
```

**Core design**

1. **Values:** NaN-boxing in 64 bits; strings are rope-and-slice with a flattening
   threshold and UTF-16 semantics (the spec's string indexing is UTF-16; store
   Latin-1-optimized and UTF-16, convert on demand); symbols and bigints are heap
   cells; integers stay int-tagged until double semantics are required (the
   "int fallback" in `ToNumber` paths must remain invisible — all arithmetic goes
   through spec operations).
2. **Objects & property lookup:** shapes (hidden classes) for ordinary objects —
   transitions on add, dictionary mode after a delete storm; indexed elements in a
   packed/dictionary pair; exotic behaviors implemented per spec: array index
   fast path with `length` invariants, `arguments` mapping, string index reads,
   typed array bounds/canonical-numeric-index-string rules, `Proxy` traps invoked
   in spec order, bound functions. Prototype chains and `[[Get]]`/`[[Set]]`
   exactly per spec (setter walk, receiver identity, `super` homes).
3. **Interpreter:** a tree-walking interpreter over the AST for M9 (with in-frame
   local slots, name resolution compiled to environment-chain depth at parse time —
   do a "bytecode-ish" annotation pass), upgraded to a register bytecode VM as a
   stretch (ADR first). Closures capture environments, not variables; `var` hoists
   into function environments; `let/const` live in block environments with the TDZ
   enforced by an uninitialized marker that throws exactly `ReferenceError`.
4. **Functions:** ordinary (sloppy/strict), arrow (`this` lexical, no
   `arguments`), methods, generators (`yield` suspends the frame into a resumable
   state machine — implemented as an explicit frame save, not OS threads),
   async functions (return promises; `await` parks the frame on the job queue),
   async generators, class constructors with `[[Call]]` vs `[[Construct]]`
   separation and `new.target`, private fields/methods with brand checks.
5. **Promises & jobs:** `Promise` states/reactions per spec; reaction jobs and
   microtasks drain at the host's microtask checkpoint (§5.15.3); `queueMicrotask`
   supported; unhandled-rejection tracking (fires `unhandledrejection` on the
   window after the checkpoint).
6. **GC:** mark-sweep over `HeapCell`s: roots = realm globals, running frames'
   handles, host-registered roots (DOM nodes reachable from script via the binding
   layer register their script-visible wrappers as roots), microtask payloads.
   Incremental marking begins when the heap exceeds the budget of Part 10; a
   write barrier on object field stores keeps incremental marking sound. `WeakRef`
   and `FinalizationRegistry` hook the sweep phase. GC runs on the document
   thread between tasks; a GC can be interrupted at safepoints (loop back-edges,
   calls) by the *interrupt* mechanism that also serves stop-the-script (§5.15.6).
7. **Builtins:** the WBS §6.7 inventory — implemented with the same interpreter
   primitives; `Function.prototype.toString` returns a source slice (real
   implementation, not "[native code]", because we have spans); `Intl` provides
   only the `Intl` object with `NotSupportedError`-style graceful stubs.

**Invariants:** spec algorithms are implemented as spec-named functions
(`ordinary_get`, `array_species_create`, …) so test262 failures map to code;
host objects can do nothing the interpreter cannot observe (no direct heap writes);
a thrown script value never crosses into host code except as an explicit
`Result::Err(ThrownValue)`; the GC is never entered re-entrantly from a builtin
(mid-builtin safepoints defer GC to the next check).

**Definition of done:** the adopted test262 subset (§8.6 scope) passes at the
recorded percentage *per section* with failures listed in `PROGRESS.md`; the GC
stress test (allocation storm under `--gc-stress` collecting at every allocation)
passes the suite; `fuzz_js_runtime` (generated AST mutator) runs without panics
or unsoundness reports 30 min; the todo/fib/JSON benchmark triple meets §10.1.

### §5.15 Runtime — event loop, timers, and bindings

**Standard basis:** HTML §8.1.7 (event loop), §8.1.4 (tasks/microtasks), WebIDL
(bindings semantics), Console Standard, Encoding Standard, Fetch API subset.
**Position:** the document thread's heart; owns the script realm and the DOM.

**Public API sketch**

```rust
pub struct EventLoop { task_queues: VecDeque<Task>, microtask_queue: VecDeque<Job>,
                       timers: TimerWheel, render_scheduler: FrameTicker, /* ... */ }
impl EventLoop { pub fn spin(&mut self, deadline: Instant) -> LoopOutcome;
                 pub fn enqueue_task(&mut self, source: TaskSource, t: Task);
                 pub fn enqueue_microtask(&mut self, j: Job); }
pub struct ScriptRuntime { pub interpreter: Interpreter, pub realm: Realm,
                           pub bindings: BindingRegistry, pub console: ConsoleBuffer, /* ... */ }
```

**Core behavior**

1. **Event loop:** per document thread. Sources (task queues): DOM manipulation,
   user interaction, networking (response hand-off), history/navigation, timers
   (implemented as a timer wheel, 4 ms clamping applies to nested `setTimeout` per
   HTML §8.1.4.2), rendering updates. Ordering: select the oldest runnable task
   (per-source FIFOs with the rendering source's special cadence below), run it,
   run the **microtask checkpoint**, run the rendering update if scheduled
   (rAF callbacks → style → layout → paint → observer callbacks), repeat.
   Idle deadlines feed `requestIdleCallback` (non-breaking best effort).
2. **Bindings (WebIDL subset):** each WBS §6.6 interface gets a generated-by-hand
   binding module: prototype chains with the correct `Symbol.toStringTag`,
   `instanceof` via prototype walking (plus brand checks for typed host objects),
   attribute getters/setters (with `[LegacyUnforgeable]` behavior where specified,
   e.g. `Node.node*`, `window.location`), method overloads resolved by WebIDL
   argument-count/types, optional/defaulted args, dictionary inits, enum/union
   conversions, `DOMException` mapping (§4.6 class 3), callback invocations from
   host to script wrapped in try/into-thrown. String conversions: IDL
   `DOMString`/`USVString`/`ByteString` semantics, including lone-surrogate
   replacement for `USVString`.
3. **Timers:** `setTimeout/setInterval/clearTimeout/clearInterval`, clamping,
   pass-through args, `this` binding rules (window in sloppy), timeout cookie
   invalidation on navigation.
4. **Console:** the Console Standard's formatting (`%s/%d/%f/%o/%c`-aware message
   formatting kept simple: concatenation + JSON for plain objects), buffer with
   the DevTools ring size, counters/timers/groups, assertion.
5. **Fetch API:** `fetch()` with `Request/Response/Headers/URLSearchParams/AbortController`
   per the Fetch standard subset: same-origin and CORS-simple requests (see §5.18.3),
   body streaming as full-buffer MVP, `AbortSignal` cancellation honored in the
   loader channel, response `.json()/.text()/.blob()` implemented.
6. **Script governance:** the interrupt handle (a shared atomic the shell/GC/
   debugger can set) is checked at loop back-edges and call entries; a runaway
   loop can be stopped for navigation, GC, or a DevTools pause. Long-task
   reporting (any task > 50 ms) feeds the DevTools performance view.

**Invariants:** the event loop is the only thing that runs script — parser-suspended
script, tasks, microtasks, and observers all funnel through `spin()`; the
microtask checkpoint happens after *every* task and after script-tasks embedded in
the parser, before rendering (this ordering is observable and tested); no task
captures non-rooted GC handles across a GC point.

**Definition of done:** WPT event-loop/ordering subset green; timer clamping test;
binding round-trip suite (every §6.6 interface constructible, every method
callable with correct exceptions on wrong argument types); a 10-minute soak test
(the showcase page with scripted timers/observers) with stable RSS within budget.
