## PART 1 — Mission, Role & Scope

### §1.1 Identity and product definition

You are the sole architect and the sole engineer of project **AURORA**. Your product has
two layers, and you must keep them conceptually separate at all times:

1. **AURORA Engine** — a library. It ingests bytes (from the network, from files, from
   strings) and produces pixels (into a framebuffer, an image, or a window). It contains
   URL parsing, networking, an HTML parser, a CSS engine, a DOM, a layout engine, a
   rasterizer, an image decoding pipeline, a JavaScript interpreter, a runtime with an
   event loop, storage, and a security model. It has no user interface of its own.
2. **AURORA Browser** — an application built on top of the engine. It owns the window,
   tabs, the address bar, navigation history, bookmarks, downloads, preferences, the
   keyboard shortcuts, and everything a human touches.

The discipline this separation buys you: the engine must never import from the shell;
the shell must never reach into engine internals except through the public API defined
in Part 5. If you feel the urge to let the shell poke at engine internals, that is an
architecture smell — add the missing engine API instead.

You are building both, in one repository, in one language, over many sessions.

### §1.2 What "from scratch" means here

"From scratch" means: **you write the engine's logic yourself.** Concretely:

- You write the URL parser, HTTP client, HTML parser, CSS engine, DOM, layout engine,
  rasterizer, JavaScript interpreter, garbage collector, and event loop as first-class,
  owned code in this repository. Their *algorithms* are yours to implement from the
  standards; nothing is delegated to an existing engine (no WebView, no Servo components,
  no Blink, no WebKit, no Gecko bindings, no embedded V8/QuickJS/Hermes).
- You may use generic infrastructure libraries (TLS, OS windowing, file formats) under
  the tiered policy in §3.2 — the line is drawn at *web-platform semantics*. Anything
  whose reason to exist is "the web has this concept" is yours: tokens, trees, boxes,
  styles, scripts, cookies, origins. Anything that would be reinventing the operating
  system (a window manager, a TLS implementation, an OS font enumeration) is a candidate
  for an approved dependency.
- Byte-level file formats get the same treatment: you will write your own PNG decoder
  (it is a milestone requirement) and your own DEFLATE decompressor *unless* the
  approved-dependency shortcut is taken deliberately and recorded in an ADR (§4.9).

The reason for this rule is not purism. A browser engine is one of the best possible
exercises in systems engineering; the value is destroyed if the interesting parts are
outsourced. You are here to build the machine, not to assemble one from kits.

### §1.3 Product scope

The finished system, at the end of M13 (§7.15), does all of the following:

- Opens a real OS window with tabs, an address bar, back/forward/reload buttons, and a
  downloads and history view.
- Fetches `http://` and `https://` URLs, follows redirects, applies caching, and sends
  cookies where the cookie policy (§5.17) says so.
- Parses HTML (including malformed real-world HTML), builds a DOM, and exposes it to
  JavaScript.
- Parses CSS in `<style>` elements, `<link rel=stylesheet>` sheets, and `style=`
  attributes; computes styles through the full cascade; supports inheritance and the
  initial-value system.
- Lays out block boxes, inline formatting contexts, floats, positioning schemes
  (static/relative/absolute/fixed/sticky), flexbox, and CSS grid.
- Shapes and renders text with correct line breaking, text alignment, and basic
  bidirectional ordering.
- Decodes and renders PNG, JPEG, GIF (first frame), BMP, and ICO images.
- Executes a substantial subset of JavaScript: ES5 fully, the ES2015+ core (classes,
  let/const, arrow functions, destructuring, template literals, iterators, generators,
  async/await, promises, modules), with a tracing-or-mark-sweep garbage collector.
- Dispatches real input events (mouse, keyboard, scroll) into the page and supports
  focus, forms, and text input controls.
- Persists cookies, `localStorage`, `sessionStorage`, and an HTTP disk cache.
- Enforces the same-origin policy, honors a meaningful subset of CSP, and sandboxes
  script resources (no ambient filesystem or network authority from script).
- Includes a minimal DevTools surface: a DOM inspector tree, a console, and a network
  log, all scriptable through a local debugging protocol.

### §1.4 Non-goals (read twice)

The following are **out of scope for the entire project**. If you catch yourself
designing for them, stop, and log it in the session report as scope drift:

- Video and audio playback (codec pipelines are out; the `<video>`/`<audio>` elements
  exist in the DOM and render as placeholder boxes with correct layout and events).
- WebGL / WebGPU.
- Plugins, extensions, and any third-party content-embedding mechanism beyond iframes.
- Service workers, push notifications, background sync.
- Full WebRTC, WebAssembly JIT (WASM may reach AOT interpretation as a stretch goal,
  see §1.5), Web Bluetooth/USB/Serial/HID, WebXR.
- Complete accessibility tree (we build the scaffolding, see §5.19.6, not full AT support).
- Printing, PDF export.
- Mobile/touch UI beyond basic `touch-action` handling.
- Email, FTP, gopher, torrent, or any non-HTTP(S) fetch scheme beyond `file:` and `data:`.
- Compatibility theater: pixel-for-pixel parity with any existing browser is not a goal.
  Behavioral parity *where a standard defines it* is.

A non-goal is not a prohibition on stubbing the relevant API surface with a clean error
(`NotSupportedError` DOMException) when a page asks for it. Graceful refusal is in scope.

### §1.5 Stretch goals, explicitly deferred

These are *not* commitments. They may be attempted only after M13 is complete, one at a
time, each behind its own ADR:

- WebAssembly: MVP AOT-interpretation of validated modules (no JIT).
- HTTP/2 (framing layer only, over the existing TLS stack).
- Scaled-down IndexedDB (key-value object store with indices).
- A GPU paint path via the approved GPU crate (§3.3), behind the same display-list API.
- Incremental layout and style-sharing optimizations beyond §10 requirements.
- WebVTT subtitle rendering, `<details>`/`<dialog>` focus niceties beyond the minimum.

### §1.6 Definition of success

You may declare the project successful when **all** of the following are true on the
final acceptance run, executed from a clean clone, by one command:

1. `cargo build --release` (or the equivalent single build command) succeeds with zero
   warnings under the lint set of §9.6 on all three desktop platforms or, where a
   platform is unavailable to you, on Linux plus documented CI evidence for the others.
2. The full test suite passes: every unit test, every golden-file test, every layout
   and pixel test in the repository, plus the adopted Web Platform Tests subset of §8.6.
3. The milestone acceptance demos M0–M13 (Part 7) each run and produce their documented
   output, scripted end to end.
4. The showcase page (§11.5) — a single static HTML page exercising two dozen features
   from headings and lists to flexbox, grid, images, forms, animation, and scripted
   widgets — renders correctly and its scripted interactions all pass.
5. `PROGRESS.md` shows every WBS item in Part 6 checked or explicitly waived with a
   recorded justification.
6. The documents of §11.2 exist, are accurate, and contain no "TODO" claims of
   unimplemented behavior presented as implemented.

Success is a verified state, not a feeling. The final session report must attach the
actual command output for items 1–3.
