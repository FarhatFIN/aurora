## PART 4 — System Architecture

### §4.1 The ten-thousand-foot view

```
                    ┌──────────────────────────── Browser shell (UI thread) ─────────────┐
                    │  tabs, omnibox, chrome widgets, downloads, prefs, DevTools UI      │
                    └───────▲────────────────────────────────────────────▲───────────────┘
                            │ pixels / input events                     │ engine API
┌───────────────────────────┴──────────── Document thread (per tab) ──┴──────────────────────────┐
│                                                                                                │
│  event loop (§5.15) ── timers, tasks, microtasks, input, rendering updates                     │
│      │                                                                                         │
│      ▼                                                                                         │
│  loader ──► net stack ──► cache ──► HTML parser ──► DOM ◄── CSS parser ◄── stylesheets         │
│  (§5.1-5.3)              (§5.17)      (§5.4-5.5)    │             (§5.7)                        │
│                                                      ▼                                         │
│                                              style system (§5.8)                               │
│                                                      ▼                                         │
│                                              layout (§5.9-5.10)                                │
│                                                      ▼                                         │
│                                        display list ──► rasterizer (§5.16) ──► surface          │
│                                                      ▲                                         │
│  script runtime: JS lexer/parser/interp/GC/builtins (§5.13-5.14) ── bindings to DOM ───────────┤
│  storage: cookies, localStorage/sessionStorage (§5.17) ── security: SOP/CSP (§5.18) ───────────┤
└────────────────────────────────────────────────────────────────────────────────────────────────┘
```

The engine is a library of crates; the shell is a binary. There is exactly one public
entry point the shell uses (`aurora::WebView`), exactly one pixel channel (frames),
and exactly one input channel (events). Everything else is internal to the engine.

### §4.2 The pipeline: data flow from URL to pixels

The load of one document, in order, with the artifact each stage produces:

1. **Resolve.** `URL → URL` (redirects, base URL, `about:`, `data:`, `file:` handling).
   Artifact: an absolute `Url`.
2. **Fetch.** `Url → Response { status, headers, body-stream }` through the cache and
   cookie jar. Artifact: bytes plus metadata.
3. **Sniff.** Content-Type + community sniffing (§5.2.7). Artifact: a document type
   decision (HTML / XHTML-XML / plain text / image / error page).
4. **Tokenize.** Byte stream → token stream, with encoding detection upstream.
   Artifact: `Token`s (start tags with attributes, end tags, characters, comments, DOCTYPE).
5. **Tree-construct.** Token stream → DOM tree, applying insertion modes, foster
   parenting, and the scripting flag. Artifact: `Document` (owned by the document thread).
6. **Load subresources.** Stylesheets (re-enter at step 2), images, scripts (deferred
   by fetch/defer/async semantics, §5.5.6).
7. **Compute styles.** Matched rules → cascade → computed values per element.
   Artifact: the style tree (computed values stored on DOM nodes).
8. **Build boxes.** `display` and friends → box tree (block/inline/flex/grid/table).
   Artifact: the box tree with styles resolved to concrete values.
9. **Layout.** Box tree → fragment tree with positions and sizes; text shaping cached
   per run. Artifact: positioned fragments.
10. **Paint.** Fragments → display list (z-ordered, clipped). Artifact: display list.
11. **Rasterize.** Display list → pixels in the `Surface` (dirty-rect aware).
    Artifact: a frame.
12. **Present.** Frame → shell via the frame channel; shell blits and swaps.

Invalidation flows backwards with the narrowest legal scope: a style-affecting change
restyles (7), a geometry-affecting change relayouts (9), a paint-only change repaints
(11). Each stage has a *dirty* protocol defined in its Part 5 section; the default is
conservative (full-stage rebuild) with narrow optimizations added only when a budget
in Part 10 demands them and a benchmark exists to prove the win.

### §4.3 Workspace and crate map

One Cargo workspace. Crate boundaries are load-bearing: they are the future process
boundaries (§4.7) and the review units (§9). Members and their hard rules:

| Crate | Owns | May depend on | Must never |
|---|---|---|---|
| `aurora_url` | URL parse/serialize/join, origins | — | know about network or DOM |
| `aurora_net` | HTTP/1.1, redirects, cookies interface, sniffing | url, platform, tls-approved-crate | parse HTML; run JS |
| `aurora_encoding` | UTF-8/16, windows-1252, encoding detection | — | — |
| `aurora_html` | Tokenizer + tree constructor | dom, encoding | style or layout |
| `aurora_dom` | Node/Document/Element core, events, ranges, selectors runtime data | — | parse HTML/CSS (it hosts, others fill) |
| `aurora_css` | Tokenizer, parser, stylesheet objects, selectors AST, values | — | touch DOM; resolve cascade |
| `aurora_style` | Matching, cascade, computed values | css, dom | layout |
| `aurora_text` | Font DB, shaping, line breaking, bidi | platform, shaping-crate | layout boxes |
| `aurora_image` | Decoders: PNG, JPEG, GIF, BMP, ICO; `ImageCache` | — | network (cache feeds it) |
| `aurora_layout` | Box building, fragment tree, layout algorithms | style, text, dom | paint |
| `aurora_paint` | Display list build, rasterizer, compositing | layout, text, image | DOM |
| `aurora_js` | Lexer, parser, AST, interpreter, GC | — | know about DOM (see `aurora_js_api` bridge below) |
| `aurora_runtime` | Event loop, timers, microtasks, host objects/bindings, console, fetch-API | js, dom, net, style, layout, paint | own UI |
| `aurora_storage` | Cookies, localStorage, sessionStorage, HTTP cache | net, platform | DOM/JS |
| `aurora_security` | Origins, SOP checks, CSP parser/enforcement hooks | url | decide policy for the shell |
| `aurora_ipc` | Message types crossing threads (future: processes) | — | contain logic |
| `aurora_platform` | Window/present glue, clipboard, fonts, DPI, clock | winit/softbuffer | web semantics |
| `aurora_shell` | Tabs, omnibox, chrome, downloads, prefs UI, DevTools UI | runtime, paint, platform, ipc | be imported by any engine crate |
| `aurora` | The `WebView` facade; the only public API of the engine | runtime | contain logic |

Bridge rule for script↔DOM: `aurora_dom` and `aurora_js` must not depend on each
other. `aurora_runtime` defines the binding layer: it implements `aurora_js`'s
`HostObject` trait for DOM node handles and translates calls into document-thread
operations. This one-way mediation keeps the JS engine testable without a DOM.

Crate count is fixed. Creating a new crate requires an ADR; as a rule, code goes in
the existing crate whose table row describes it, or the ADR explains why the map was
wrong.

### §4.4 Threading and synchronization model

Implements §3.7. Concrete rules:

- Thread inventory: **UI thread** (owns window + shell), **loader threads** (pool,
  default size 1, cap 4), **document threads** (one per tab with a live document),
  **storage writer** (single, serializes storage writes), nothing else.
- Message types live in `aurora_ipc` and are exhaustive, ordered, and documented.
  Channel directions:
  - UI → document: `InputEvent`, `ViewportChange`, `NavigationCommand`, `ScriptCommand`.
  - document → UI: `FrameReady(frame_id, dirty_rect)`, `TitleChanged`,
    `FaviconChanged`, `CursorChange`, `TooltipChange`, `NavigationStateChange`,
    `ConsoleMessage`, `RequestPermission` (rare, UI decides).
  - document → loader: `Fetch(url, request_params, cache_mode)`.
  - loader → document: `FetchResponse(id, meta, body_chunk|done|error)`.
  - document → storage: read requests with one-shot reply channels; storage → document
    replies. Writes are fire-and-forget with an optional completion ping.
  - shell → engine and engine → shell via `aurora::WebView` only.
- Backpressure: every bounded channel documents its bound and its policy when full
  (loader request queue: unbounded, but capped at 64 in-flight with `ERR_TOO_MANY`;
  frame channel: latest-wins slot — the UI always wants only the newest frame).
- Cancellation: navigations carry a monotonic `navigation_id`; stale messages are
  dropped by id. Timers and fetches take a generation counter, not a bare flag, so a
  rapid navigate away cannot un-cancel a newer load.
- The UI thread's frame budget is 16 ms; the document thread pre-rasters the next
  frame while the UI presents the current one only after M10, and only if §10 numbers
  say it is needed.

### §4.5 Memory strategy and ownership across the pipeline

- **DOM:** arena-allocated nodes with stable generational indices (`NodeId(u64)`,
  generation bits to catch stale handles), parent/first-child/next-sibling links.
  The arena is owned by the document. Removal is logical (tombstone + generation
  bump); physical reclamation is a document-lifetime decision. This makes reentrancy
  from script (a deleted node being touched mid-iteration) explicit instead of UB.
- **CSS:** stylesheets parse once into immutable, interned structures. Selectors,
  property names, and non-inline string values are interned in a per-document or
  global (immutable-after-startup) string table. Computed values are plain structs
  on DOM nodes; inherited longhands share `Rc` to parent values until overridden.
- **Layout:** the box tree is rebuilt from the style tree when structure changes;
  the fragment tree is rebuilt on relayout. Caching (text run metrics, intrinsic
  sizes) is keyed by content hash + generation, never by pointer identity.
- **JS values:** all script-visible objects live in the GC heap (§5.14.5). Handles
  into the GC heap are root-registered stack handles; a handle that outlives its
  scope without rooting is a build error (type-system enforced, §5.14).
- **Buffers:** decoded image frames and raster surfaces are `Vec<u8>` in
  premultiplied BGRA or RGBA — pick **RGBA8 non-premultiplied** for surfaces and
  premultiplied for compositing internals; conversions are explicit helpers, never
  ad-hoc per-pixel math. Image frames are `Arc`-shared with the cache; a frame is
  never mutated after publish.
- **Budgets:** the numbers in Part 10 are the memory contract; every cache has a
  byte-based limit and an eviction policy (LRU unless stated), and reports its
  footprint into the DevTools memory view (§5.19.5).

### §4.6 Error taxonomy

Three error classes, never mixed:

1. **Resource errors** (network failed, file missing, decoder hit corrupt data):
   modeled as typed enums per crate (`net::Error`, `image::DecodeError`, …),
   convertible into DOM-facing outcomes per the fetch/HTML specs (network error =
   abort the fetch algorithm; image error = fire `error` event). They are *values*,
   expected and handled, not exceptional.
2. **Programming errors** (invariant violated, unreachable state, index out of
   bounds): `panic!` with a message that names the invariant and the section of this
   document or the standard that established it. Panics are bugs; the test suite
   must make each first occurrence a fixed bug, not a tolerated flake.
3. **Boundary errors** (bad input from the outside world — malformed HTML/CSS/JS,
   invalid URLs, bad API arguments from script): per web platform rules, parsers
   *recover* per spec error-recovery rules; script-facing APIs throw
   `DOMException`s from the standard table (SyntaxError, TypeError,
   HierarchyRequestError, NotSupportedError, InvalidStateError, SecurityError…).
   The engine never panics on external input. Fuzzing (§8.7) enforces this.

Every crate's public functions are annotated which class they return (class 1 via
`Result`, class 3 via DOMException at the binding boundary, class 2 never returned).

### §4.7 Process model roadmap

- **Phase 1 (M0–M11):** everything in one process, threads as in §4.4.
- **Phase 2 (M12+ prep, post-M13 execution):** split the document thread into a
  **content process** and the shell into a **browser process**; `aurora_ipc` messages
  become length-prefixed frames over OS pipes. The message types must not change —
  that invariant is why §4.4's table is written in terms of directions and ids.
- **Phase 3 (stretch):** a network service process. The loader pool API already
  isolates this.
The ADR for each phase lands *before* the code that needs it. If Phase 2 messages
need a change to stay serializable, that is a design bug to fix immediately, not to
work around with process-specific forks in message handling.

### §4.8 Compatibility and versioning policy

- The engine's *public* API (`aurora` crate) is versioned semantically from the first
  release of M13. Inside the workspace, crates move freely; the WBS is the tracker.
- Web-platform compatibility is defined by the adopted test corpus (§8.6), not by
  opinions. The corpus and the engine's pass rate are recorded in `PROGRESS.md`
  per milestone; the number only moves up (a regression in the corpus pass rate is a
  release blocker at M13).
- Data formats on disk (cache entries, cookies, prefs, storage) carry a version byte
  from day one. Migrating forward is supported; reading never mutates without a
  successful migration to current.

### §4.9 Architecture Decision Records (ADRs)

Any decision that constrains future work and is not already fixed by this document
gets an ADR: `docs/adr/NNNN-title.md`, numbered, append-only. Format: **Status**
(proposed/accepted/superseded by NNNN), **Context** (what forced the decision),
**Decision** (what was chosen, concretely), **Consequences** (what becomes easier,
what becomes harder, what is now forbidden). ADRs are cross-referenced from the
session report that made them. Typical ADR triggers: new dependency, new thread, new
crate, changing a §4.4 channel contract, deviating from a standard's algorithm for
performance, dropping or deferring a WBS item.

### §4.10 Crate skeleton template

Every crate starts from the same skeleton (M0), so structure is never a
per-crate improvisation:

```
crates/<name>/
├── Cargo.toml          # tier annotations per §3.2; features: default set only
├── src/
│   ├── lib.rs          # crate doc: responsibility, thread, neighbors (§9.5)
│   ├── facade.rs       # pub use of the crate's actual public API — the only re-export surface
│   ├── <modules>/      # per §9.2 seams; tables.rs for generated constant data
│   └── tests/          # #[cfg(test)] units live beside code; here: integration-only
├── tests/              # cross-module integration tests (§8)
└── benches/            # criterion benchmarks named per the §10.1 table
```

```rust
// lib.rs minimum shape:
//! <one-paragraph responsibility statement from §4.3's table>
//! Thread: <which §4.4 thread(s) own this>. Neighbors: <in / out>.

#![warn(clippy::all, clippy::pedantic)]
#![deny(/* the §9.6 deny set */)]

mod facade; // pub use facade::*;
// …
```

Two rules this template enforces: the public API is visible in one file
(`facade.rs`), and no crate ever publishes more than that surface even
internally — future process boundaries (§4.7) depend on it.
