## PART 7 — Milestones and Acceptance

### §7.1 Milestone discipline

A milestone is **done** when its exit criteria are all verified in the session
report, its demo runs from one command, and `PROGRESS.md` records the milestone's
WBS completion percentage. Do not start milestone M+1 with M's exit criteria
unverified, and do not expand a milestone's scope mid-flight — new ideas go to the
WBS backlog, not into the current milestone. Each milestone section below lists:
Objective, Scope (in/out), Demo, Exit criteria, and Risks.

### §7.2 M0 — Bootstrap and toolchain

**Objective:** an empty-but-real repository where `cargo build && cargo test` runs
a workspace with every crate of §4.3 present, each with a smoke test.
**In scope:** workspace + crates skeletons, lint/fmt configs, CI workflow file,
`PROGRESS.md` + `docs/` scaffolding, the two-platform CI matrix, `test:fast`
script (§8.8), the ADR directory, the dependency-check script (§5.19).
**Out of scope:** any web behavior.
**Demo:** `cargo run -p aurora_shell -- --version` prints name/version/commit;
CI runs the fast tier on push.
**Exit criteria:** build clean with `-D warnings`; every crate has a passing
placeholder test; docs of §11.2 exist as stubs with correct structure; the
dependency check script runs and passes (no engine crate depends on shell).
**Risks:** none worth writing down — this milestone exists to force the plumbing
questions early.

### §7.3 M1 — Fetch and render text

**Objective:** `aurora --url http://example.com` prints the response bytes
(then just the text) to stdout. End-to-end: URL → DNS → TCP → TLS → HTTP → bytes.
**In scope:** §5.1 URL, §5.2 HTTP/1.1 core (GET, redirects, chunked, gzip),
§5.3 TLS, `file://` and `data:` schemes, the loader thread, a `--dump-bytes` /
`--dump-text` CLI, a mock-server test harness.
**Out of scope:** HTML semantics (text extraction is "print decoded body").
**Demo:** three commands: fetch `http://example.com` over the real network (or
mock in CI), fetch `https://` counterpart, `data:text/html,<h1>hi</h1>`.
**Exit criteria:** §5.1–§5.3 definitions of done; mock-server suite green;
`--dump-bytes` byte-identical to `curl -s` output on the five test URLs.
**Risks:** TLS trust store friction on the three platforms — mitigated by
`webpki-roots` default and a documented `--cert-bundle` override.

### §7.4 M2 — HTML to DOM

**Objective:** `--dump-dom` prints a serialized DOM tree for any real page.
**In scope:** §5.4 tokenizer, §5.5 tree construction (all insertion modes),
encoding detection, the DOM core of §5.6 without script, serialization.
**Out of scope:** scripting, styling.
**Demo:** `--dump-dom http://example.com` then a diff of the output against a
golden file; `--dump-dom --errors` shows parse-error table.
**Exit criteria:** html5lib-tests tokenizer + tree-construction 100%;
`document.write` torture test; the top-50-URLs corpus (§8.6) parses without panic.
**Risks:** the adoption agency algorithm's test cases are subtle — budget a full
session for it alone and land its 40 cases as a first-class test file.

### §7.5 M3 — CSS and the style system

**Objective:** `--dump-style <selector>` prints computed styles per element.
**In scope:** §5.7 parser, §5.8 cascade/computed values for the WBS §6.3
properties marked `[M3]`, stylesheet loading (inline + `<link>`), `@import`,
media query parsing (screen/print/width), the UA stylesheet v1 (needed by every
later milestone — write it carefully, it is a product artifact).
**Out of scope:** layout-dependent values, selectors needing layout state.
**Demo:** a page with three stylesheets, imports, specificity conflicts, and
!important — dump shows the winner per element with source locations.
**Exit criteria:** §5.7/§5.8 definitions of done; cascade torture matrix;
css-parsing-tests green; UA stylesheet covered by style tests.
**Risks:** custom property substitution semantics — implement the spec's
substitution *before* shortcuts, it contaminates everything later.

### §7.6 M4 — Block layout

**Objective:** `--render ascii out.txt` renders the page as an ASCII/box-drawing
approximation of the layout (blocks as boxes with sizes, text as ¶ placeholders
with wrap counts), and `--layout-debug` prints the fragment tree.
**In scope:** §5.9 items 1–5 (boxes, BFC, margin collapse, inline *skeleton*
without real text — fixed-width metrics placeholder recorded as a standing
placeholder until M7, floats, positioning), the box builder.
**Out of scope:** painting, real text, images.
**Demo:** the CSS 2.1 test-page subset renders; the margin-collapse suite prints
expected geometries.
**Exit criteria:** §5.9's golden-file suite (except text cases) green; fragment
tree dump matches expected geometry files.
**Risks:** percentage-height resolution chains — cover with 10 dedicated cases
before building anything on top.

### §7.7 M5 — Painting to pixels

**Objective:** `--render png out.png` writes a real raster of the page.
**In scope:** §5.16 display list + software rasterizer (fills, borders, radius,
clips), background properties, border properties, color; PPM/PNG output (PNG via
the owned encoder or approved crate *for output only* — input decoding stays M8).
**Out of scope:** text, images, scrolling.
**Demo:** the "CSS backgrounds & borders" showcase page renders to PNG; pixel
test harness lands with the first 30 golden images.
**Exit criteria:** §5.16 definition of done minus text; pixel tests byte-stable
across 3 consecutive runs and 2 machines.
**Risks:** AA determinism — settle the fixed-point design in the first week of
the milestone; retrofitting determinism later is a disaster.

### §7.8 M6 — The first window

**Objective:** a window opens showing the rendered page; scrolling works.
**In scope:** §3.6 surface presentation, `aurora_platform` glue, viewport/resize,
wheel scrolling, dirty-rect painting, basic hit testing for cursor changes,
a minimal chrome: address bar + reload only.
**Out of scope:** tabs, real text (still placeholder), script.
**Demo:** navigate `file://` and `http://` pages in the window, resize, scroll.
**Exit criteria:** 60 FPS scroll on the reference page at reference size (§10.1);
frame budget telemetry printed on `--debug-frames`; no input lag over 2 frames.
**Risks:** present/vsync behavior differences across platforms — abstract behind
`Surface` and keep a "present immediately" mode for tests.

### §7.9 M7 — Text and inline layout

**Objective:** real text everywhere: fonts, shaping, line breaking, inline layout.
**In scope:** §5.11 all, §5.9.3 inline layout full, `line-height`, `vertical-align`,
`text-*` properties, the strut, bidi MVP, the UA stylesheet's typography block.
**Out of scope:** auto-hyphenation, vertical writing.
**Demo:** a typography specimen page (serif/sans/mono, weights, sizes, CJK +
Arabic + Cyrillic samples, justified paragraph) renders to PNG and window.
**Exit criteria:** §5.11 definition of done; the pixel tests for text; the
placeholder from M4 is removed (§2.3 standing-placeholder ledger zero for text).
**Risks:** font enumeration differences across platforms — pin the reference
fonts in the test profile; shaping crate integration — wrap it at `TextShaper`
so the owned shaper can grow underneath.

### §7.10 M8 — Images

**Objective:** images render; `alt` text on error; layout reserves space.
**In scope:** §5.12 all four decoders, `<img>`/`<picture>`/`srcset`, intrinsic
sizes + `object-fit`, background-image, image caching, EXIF orientation.
**Out of scope:** SVG, animated GIF playback polish (first frame + timer-driven
advance are in).
**Demo:** an image gallery page with every supported format, a broken image, and
a lazy-loaded image scrolling into view.
**Exit criteria:** §5.12 definition of done; fuzz targets run clean; the gallery
golden image matches.
**Risks:** JPEG decoder schedule — baseline first and ship M8 with it if
progressive slips (record as standing placeholder; progressive lands in M13).

### §7.11 M9 — The JavaScript engine

**Objective:** `--eval "script"` runs real programs; the engine passes its
test262 subset.
**In scope:** §5.13 lexer/parser, §5.14 interpreter, GC, builtins through the
WBS §6.7 `[M9]` set (Object/Array/String/Number/Math/JSON/RegExp/Promise/Map/Set/
Date/Error/TypedArrays), the host API surface (`HostObject` trait) with a
filesystem-free `console` and timers, modules with a `file://` loader for tests.
**Out of scope:** DOM bindings (M10), Intl.
**Demo:** the milestone's scripted demo (a pure-JS todo-list logic module with
tests) runs; test262 subset report printed.
**Exit criteria:** §5.13/§5.14 definitions of done; GC stress suite; benchmark
triple within §10.1.
**Risks:** this is the largest single milestone; the WBS granularity for §6.7
exists precisely to let you slice it — run M9 as three internal checkpoints
(parser / interpreter / builtins).

### §7.12 M10 — Scriptable DOM and events

**Objective:** pages come alive: `document`, `querySelector`, event handlers,
dynamic DOM, forms responding to input.
**In scope:** the binding layer (§5.15.2) for the WBS §6.6 `[M10]` interfaces,
the event system wired to real input (§5.19.3), focus management, `innerHTML`,
MutationObserver, ResizeObserver/IntersectionObserver MVP, rAF loop, form
controls' activation behavior (checkbox/radio/select/button/input-text),
`requestAnimationFrame`-driven paint.
**Out of scope:** shadow DOM (lands M12), workers (non-goal §1.4).
**Demo:** the interactive showcase page: a counter, a tab widget, a form with
validation and dynamic rows, an animated box (rAF + style mutation), and a
MutationObserver-driven logger — all working in the window.
**Exit criteria:** WPT DOM/events adopted subset green; the interaction test
script (§8.4's scripted input driver) passes on the showcase page; soak test
stable.
**Risks:** GC↔DOM reference cycles (script wrapper holds DOM node, node holds
script handler) — the binding registry's root registration rules (§5.14.6) must
be proven with a cycle stress test in this milestone, not discovered later.

### §7.13 M11 — The browser shell

**Objective:** it is a browser: tabs, omnibox, history, bookmarks, downloads,
find-in-page, settings, keyboard shortcuts, DevTools v1 (console + inspector).
**In scope:** §5.19.1, §5.19.2, §5.19.5's Console and Inspector panels, zoom,
`about:blank`/`aurora://` pages, the session history, crash-safe tab restore.
**Out of scope:** cookies UI (M12), Network/Performance panels (M13).
**Demo:** the recorded keyboard-only walkthrough (§5.19) completes: open tab,
navigate three sites, bookmark one, find text, inspect a node in DevTools,
close tabs, restore session.
**Exit criteria:** §5.19 definition of done minus Network/Performance panels;
the shell dependency check stays green; window resize/tab-switch keeps frame
budget.
**Risks:** chrome rendering stealing complexity from the page pipeline —
chrome is *just* display lists; resist any shell-specific rendering path.

### §7.14 M12 — Storage, cookies, and polish

**Objective:** the browser remembers: cookies, localStorage/sessionStorage, HTTP
disk cache, HSTS, form-less persistence; shadow DOM MVP; network panel.
**In scope:** §5.17 all, §5.18 HSTS, shadow DOM MVP (§5.6), CSP enforcement,
`document.cookie`, storage events, the Network DevTools panel, profile
management (`--profile`), the "clear browsing data" settings page.
**Out of scope:** IndexedDB, service workers.
**Demo:** a login-ish page (cookies set via JS, session visible in settings),
a cross-restart localStorage demo, a second visit served from cache shown in
the Network panel, a shadow-DOM component page.
**Exit criteria:** §5.17/§5.18 definitions of done; crash-simulation storage
test; cache hit rate ≥ 90% on the repeat-visit test corpus.
**Risks:** cookie correctness vs real sites' expectations — the SameSite
default (Lax) plus the prefix rules cover the standard cases; log deviations.

### §7.15 M13 — Hardening, performance, release

**Objective:** make it shippable: budgets met, fuzzers run long, the corpus pass
rate is final, docs complete, v0.1 tagged.
**In scope:** §10 budgets measured and met (or ADR'd exceptions), all fuzz
targets × 24 h cumulative, the WPT corpus final report, DevTools Performance
panel, the JPEG progressive decoder if deferred, the zero-shortcut audit
(§2.7), the final documentation pass (§11.2), the release checklist of §12.7.
**Demo:** the full showcase + the top-100-corpus render gallery, generated as a
site anyone can open; the benchmark report table.
**Exit criteria:** §1.6's definition of success verified literally; `v0.1.0`
tagged; release notes published in `CHANGELOG.md`.
**Risks:** schedule pressure inventing shortcuts — the §2.7 ledger makes them
visible; do not trade the zero-shortcut audit for anything.
