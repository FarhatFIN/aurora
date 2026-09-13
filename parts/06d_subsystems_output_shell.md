### §5.16 Painting and compositing

**Standard basis:** CSS 2.1 Appendix E (stacking contexts), CSS Position 3,
CSS Backgrounds/Borders 3, CSS Color Adjust/Color 4, CSS Transforms 2 (2D only),
CSS Overflow, CSS Will Change/Contain (as hints).
**Position:** fragment tree → display list → software raster → surface.

**Public API sketch**

```rust
pub enum DisplayItem { SolidColor(Rect, Color), Border { rect, widths: SideRect, styles: SideStyles, radius: CornerRadii },
                       Background { rect, kind: BgKind /* color | image{...} | gradient{...} */, radius },
                       TextRun { run: ShapedRunRef, origin, color, decorations: TextDecorations },
                       Image { frame: Arc<DecodedFrame>, dest: Rect, source: Rect, transform: Transform2D,
                               filters: FilterSet, blend: BlendMode },
                       Shadow(ShadowKind), PushClip(ClipId), PopClip, PushStackingContext(StackingContext), Pop,
                       IframePlaceholder(Rect, frame_id), Scrollbar(ScrollbarKind, Rect, ScrollState) }
pub struct DisplayList { pub items: Vec<DisplayItem>, pub clips: Vec<Clip>, pub epoch: u64 }
pub trait Surface { fn begin(&mut self, dirty: Option<Rect>); fn fill_path(&mut self, path: &Path, paint: &Paint);
                    fn stroke_path(&mut self, path: &Path, paint: &Paint, style: &StrokeStyle);
                    fn blit_image(&mut self, frame: &DecodedFrame, dst: &Rect, transform: &Transform2D, alpha: f32);
                    fn glyphs(&mut self, run: &ShapedRun, origin: Point, paint: &Paint);
                    fn push_clip(&mut self, clip: &Clip); fn pop_clip(&mut self);
                    fn end(&mut self) -> RasterStats; }
```

**Core behavior**

1. **Display list construction:** walk the fragment tree in paint order per CSS 2.1
   Appendix E — per stacking context: background/borders of the establishing box →
   negative-z-index children → in-flow block backgrounds → floats → inline content →
   positioned/z-index≥0/auto descendants → outlines; stacking context triggers:
   root, positioned with z-index ≠ auto, `opacity < 1`, transforms/filters/masks,
   `isolation: isolate`, `will-change` of those, `mix-blend-mode ≠ normal`,
   flex/grid items with z-index, `contain: paint`. Each item records its clip
   (rounded-rect capable) and its `PointerEvents` relevance for hit testing.
2. **Rasterizer (owned, software):** scanline polygon fill with 4×4 supersampled
   coverage (deterministic!), analytic AA edges for axis-aligned rects (the 99%
   case — fast path), border rendering as four trapezoid/ring paths with per-side
   styles (solid/dashed/dotted/double/groove/ridge/inset/outset) and elliptical
   corner joins, image blits with bilinear resampling and the transform's scale
   choice, glyph rasterization cached per (font, size, glyph, transform-bucket)
   with subpixel positioning quantized to 1/4 px, gradients (linear/radial/
   conic) via dithered scanline interpolation, blend modes (the separable set +
   `multiply/screen/overlay/darken/lighten/color-dodge/burn/hard/soft-light/
   difference/exclusion/hue/saturation/color/luminosity`) in premultiplied space,
   dirty-rect scissored rendering (an item outside the dirty rect is skipped).
3. **Scrolling:** scrollable overflow regions from layout; compositor scrolls
   (shell-owned pans over a cached display list with the raster re-run only on
   content change) — MVP: scroll = full repaint of the scrolled container's clip
   with `will-change: scroll-position` enabling the cached-raster optimization.
   Scrollbars are painted by the shell from `ScrollbarState` (styling `::-webkit-scrollbar`
   is out of scope; `scrollbar-width/color` parse and map to thickness/color).
4. **Hit testing:** the display list doubles as the hit-test index (reverse paint
   order, respecting clips and `pointer-events`); returns the deepest element plus
   the event coordinates mapped into its local space for input (§5.19.3).
5. **Determinism:** raster output is byte-identical for identical display lists
   across runs and platforms (fixed-point accumulation in AA, no SIMD-dependent
   reduction order) — the pixel test suite (§8.5) depends on this.

**Invariants:** the display list is immutable once built; paint reads DOM only
through the fragment tree (no element queries during paint); every clip pushed is
popped (balanced by construction, asserted in debug); memory of the display list
and glyph cache is within the Part 10 budget, measured per frame in DevTools.

**Definition of done:** the pixel test corpus (§8.5) — backgrounds/borders/radius
(30), text (15), images/transforms (10), blends/filters (10), stacking (15) —
matches golden images exactly; the showcase page rasterizes at the §10.1 rate;
`fuzz_paint` (random display lists) never panics or unbalances clips.

### §5.17 Storage and persistence

**Standard basis:** RFC 6265bis (cookies), HTML §11 (Web Storage), RFC 9111 (HTTP
caching), the File System layout of this document (we define it; no standard).
**Position:** engine services behind `aurora_storage`, used by `aurora_net`
(cookies, cache) and the runtime (`localStorage`/`sessionStorage`).

**Public API sketch**

```rust
pub struct CookieJar { /* per eTLD+1 partitioning, persistent */ }
impl CookieJar { fn set_for_url(&mut self, url: &Url, set_cookie: &str, now: Timestamp, https: bool) -> CookieOutcome;
                 fn header_for_url(&mut self, url: &Url, https: bool, cross_site: bool) -> Option<String>; }
pub struct WebStorage { /* localStorage: per-origin persistent; sessionStorage: per-tab, per-origin */ }
pub struct HttpDiskCache { /* RFC 9111-ish: keyed by (method GET/HEAD, url, vary-values) */ }
```

**Behavior**

- **Cookies:** full RFC 6265bis parse (name/value, `Domain`, `Path`, `Expires`/
  `Max-Age`, `Secure`, `HttpOnly`, `SameSite=Lax|Strict|None|default`,
  `Partitioned` accepted and partitioned by top-level site), host-only vs domain
  cookies, path-match, the sort order (longer paths first, earlier creation first),
  expiry (session cookies live with the browser session; persistent with the
  store), the `__Secure-`/`__Host-` prefix rules, secure-only transmission,
  cross-site rules per SameSite (top-level-site computation via the public-suffix
  list file checked into the repo, refreshed per release). JS access via
  `document.cookie` honors `HttpOnly` invisibility.
- **Web Storage:** `localStorage` per origin (5 MB quota in UTF-16 code units,
  `QuotaExceededError` on overflow, `storage` events to same-origin *other* tabs
  — cross-tab notification uses the IPC channel), `sessionStorage` per tab,
  both synchronous maps with string keys/values, persisted atomically
  (write-temp-rename, versioned file format, crash-safe).
- **HTTP disk cache:** entries keyed by normalized request (URL + `Vary` header
  values + method class); stores the full header set + body + `Date` +
  request time; freshness per RFC 9111 §4 (Age calculation, heuristic freshness
  when no explicit freshness and status is heuristically cacheable, `must-revalidate`,
  `no-store` never stored, `no-cache` stored but always revalidated); size-capped
  (Part 10) with LRU eviction on whole entries; transparently feeds §5.2.6.
- **Layout on disk:** `~/.aurora/<profile>/` with `cookies.db` (simple append-only
  log + index rebuilt on open), `storage/` per-origin files, `cache/` sharded by
  URL hash (2-hex-char directories), `prefs.toml` (shell), `hsts.db`. All formats
  carry a version byte (§4.8). A `--profile <dir>` flag selects it; tests use a
  temp profile always.

**Invariants:** every read path validates checksums (per-record CRC32) and
recovers by dropping the corrupt record, not the store; concurrent access from
loader threads is serialized by the storage writer thread; the cookie jar never
blocks the UI thread (reads are pre-computed per navigation on the document
thread).

**Definition of done:** cookie spec suite (the RFC's own examples + the WPT
cookies subset) green; storage quota and `storage` event tests; cache: a mock
server scenario matrix (fresh/stale/revalidate/no-store/vary/age) all green;
crash-simulation test (kill a process mid-write; reopen; store loads).

### §5.18 The security model

**Standard basis:** HTML §7 (origin/agents/SOP), Fetch (CORS), CSP Level 3 (the
directives below), Mixed Content, Referrer Policy, Secure Contexts (subset).
**Position:** `aurora_security` + enforcement hooks at each boundary.

**Behavior**

1. **Origins:** computed at parse (§5.1) and stored on documents/window/host
   handles. Same-origin for tuple origins = scheme+host+port equality; opaque
   origins equal only themselves. Every cross-object access in the runtime —
   `window.parent`, `frames`, `document.domain` (parsed, supported, deprecation
   warnings included) — checks it; a failure throws `SecurityError`.
2. **Document isolation:** each navigation runs in a fresh document context;
   `postMessage` (with origin/targetOrigin checks), `history.pushState` (same
   document), and named targeting are the only cross-document bridges we ship.
3. **CORS:** simple requests pass; preflights (`OPTIONS` with
   `Access-Control-Request-Method/-Headers`) issued when required; response
   checks (`Access-Control-Allow-Origin` incl. wildcard-with-credentials rules,
   `-Allow-Headers`, `-Allow-Methods`, `-Expose-Headers` limiting header reads,
   `-Max-Age` cache). A CORS failure is a network error — the page sees a
   generic `TypeError` on fetch, never the server's reason.
4. **CSP:** parse the header; enforce in the MVP: `default-src`, `script-src`
   (inline blocking incl. hashes `sha256-…` and nonces; `unsafe-inline`/
   `unsafe-eval` honored), `style-src` (same), `img-src`, `connect-src`,
   `font-src`, `frame-src`, `object-src`, `base-uri`, `form-action`;
   `report-uri`/`report-to` POST a JSON report to the endpoint. Violations fire
   the `SecurityPolicyViolationEvent` and block per the directive's rules.
5. **Mixed content:** blockable content (scripts, XHR/fetch, iframes of the
   blockable kind) on `https://` pages from `http://` is blocked with a console
   report; optionally-blockable (images) is allowed but reported (upgrade-insecure-
   requests honored when the CSP directive appears).
6. **Script authority:** script runs with exactly the authority the DOM API
   surface grants: no `fs` access, no process spawning, network only through
   `fetch`/XHR/`WebSocket`/`EventSource`/media-URLs through the loader with all
   §5.18.3 checks. `file:` documents get an opaque-per-directory origin policy:
   each file is its own opaque origin (strict), `file:` fetches to the same
   directory tree are allowed with a console warning (documented deviation, ADR).
7. **`:visited` partitioning:** visited-link styling uses the global history
   store but matches `:visited` styles to a *fixed* set of paint-only properties
   (color, background-color, border-*-color, etc.), never layout-affecting ones —
   the classic timing-attack surface stays closed.
8. **Sandbox flags:** the `sandbox` iframe attribute set (forms, scripts,
   same-origin, popups, top-navigation, pointer-lock, storage-access) enforced at
   the boundary checks above; `Content-Security-Policy: sandbox` honored as the
   document-level equivalent.

**Invariants:** security checks live at the boundary functions (the ~12 choke
points enumerated in `aurora_security::checkpoints`), not scattered — review
greps that list; no security decision depends on unvalidated remote data; a
fuzzed header set cannot enable cross-origin reads.

**Definition of done:** the adopted WPT security subset green; an adversarial
test page (checked into `tests/security/`) exercising each violation class shows
the documented blocked behavior; CSP report payloads validated against the spec's
JSON shape.

### §5.19 Browser shell and DevTools

**Position:** `aurora_shell` — the only consumer of the engine's public API.

**Behavior**

1. **Chrome:** tab strip (drag reorder, close, overflow scroll, favicons, load
   progress bar), omnibox (unified URL/search entry, autocomplete from history +
   bookmarks with keyboard navigation, paste-and-go, security indicator: lock
   icon for valid TLS, info for http, warning for cert-unsupported), navigation
   buttons (back/forward with long-press history, reload/stop), menu with zoom,
   find-in-page (bar with match count, wraps), full-screen toggle, downloads
   popover, downloads page (`aurora://downloads`), history page, bookmarks page
   + bookmark bar toggle, settings page (search engine, homepage, fonts sizes,
   site data clearing, do-not-track toggle which sets the header flag),
   keyboard shortcuts per Appendix F, `Ctrl+T/W/Shift+T` tab lifecycle.
2. **Navigation controller:** per-tab session history (entries with URL, title,
   scroll state, form state MVP: none, favicon), back/forward deltas, reload
   (bypass-cache variant), stop, navigation interception for `target=_blank`
   (new tab) and downloads (`Content-Disposition: attachment` → download manager
   with progress, pause/resume, path picker via platform).
3. **Input routing:** the shell converts platform events → engine input messages:
   mouse (move/enter/leave/down/up with buttons/modifiers), wheel (with
   scroll-chaining into the engine's scroller resolution), keyboard (Appendix F
   maps codes; shortcuts are shell-first, then engine), text input via IME
   composition events, drag-and-drop of files onto the window (creates a
   `file://` navigation or a drop event in-page), hit-test result from the engine
   decides hover/cursor/tooltip.
4. **Rendering integration:** the shell owns the window surface; on
   `FrameReady` it blits the engine raster and draws chrome on top with the same
   rasterizer (chrome is just more display lists). Viewport sync (resize, DPI
   change, zoom via `Ctrl+/-` → page zoom factor in 10% steps) flows down.
5. **DevTools:** `F12` opens a docked panel: **Console** (buffered messages,
   input line with a one-shot script eval in the page realm, object preview as
   structured text), **Inspector** (DOM tree synced on mutations, box-model
   view of the selected node, computed styles, matched rules with stylesheet
   locations), **Network** (request log: method, status, type, size, time,
   waterfall from loader events), **Performance** (task timeline with long-task
   marks, GC marks, frame times). The panel is a *browser page* — built with
   the engine, driven by a local protocol (a JSON-RPC over a Unix socket/
   named pipe, the messages listed in `docs/devtools-protocol.md`) so that
   DevTools tests are engine-level tests, not UI-only.
6. **Accessibility scaffolding:** DOM exposes role/name computation from
   semantics (`<h1>`→heading, `<button>`→button, `alt`, `aria-label`/`role`
   parsed and stored); the platform glue exposes it where a native AT API is
   trivially available (Linux AT-SPI via the platform crate is a stretch); the
   in-engine requirement is the computed tree and the tests of it.

**Definition of done:** every §5.19.1 surface works in the M11 demo; the
keyboard-only walkthrough (Appendix F's list) completes every core task;
DevTools' four panels pass their protocol tests; the shell never reads engine
crates other than `aurora` (enforced by a CI dependency check script).

### §5.19.7 Chrome widget specifications

The chrome's own widgets, drawn with the engine's rasterizer from their own
display lists; each has a defined interaction contract so tests can drive it
synthetically (§8.4):

- **Tab strip:** min/max tab widths with the shrinking algorithm (equal share
  of the available width, pinned tabs excluded), drag-reorder with the 8 px
  threshold, close-button hit box of at least 24×24 CSS px, overflow chevron
  opening the tab list popup, middle-click close, Ctrl+digit activation.
- **Omnibox:** unified entry — on focus, full-text selection; typing opens the
  dropdown with up to 6 rows: URL match (bolded match ranges), search
  suggestion, history with visit count and recency scoring; Enter navigates
  the highlighted row (none highlighted → the raw text, URL if it parses with
  a scheme or dot, else the configured search engine); Escape restores the
  pre-edit text; paste-and-go via the context menu; the security indicator
  (lock/info/warning per §5.19.1) sits left, the reload/stop button right.
- **Find bar:** input + match count + prev/next + close; matches are found on
  the document thread, highlighted via a paint overlay (not DOM mutation),
  `Enter`/`Shift+Enter` cycle, `F3`/`Shift+F3` alias, Escape closes and clears.
- **Downloads popover/page:** rows with filename, size, progress, pause/
  resume/cancel, open-folder via the platform; the popover attaches to the
  toolbar button; the page is `aurora://downloads`.
- **Context menus:** document (back/forward/reload/save/view-source/inspect),
  editable (undo/redo/cut/copy/paste/select-all — wired to §6.11 clipboard
  events), link (open in new tab/copy address), image (open/copy/save).
- **Keyboard focus rings:** every chrome widget has a visible focus ring in
  the same style as page `:focus-visible`; Tab order covers all chrome and
  never traps (§5.19 keyboard-only walkthrough).
