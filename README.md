# AURORA

A web browser engine and browser built from first principles, in Rust.

AURORA is two products in one repository:

1. **AURORA Engine** — a library that turns bytes into pixels: URL parsing,
   HTTP/1.1, an HTML parser, a CSS engine, a DOM, layout, a software
   rasterizer, image decoders, a JavaScript interpreter with a garbage
   collector, an event loop, storage, and the security model.
2. **AURORA Browser** — an application over the engine: window, tabs, omnibox,
   history, bookmarks, downloads, and DevTools.

Nothing at the web-platform tier is imported: every parser, algorithm, and
decoder is written here (§1.2 of the master specification). Generic
infrastructure (TLS, windowing, system fonts) comes from the approved Tier 1
list (§3.3).

## Status

**M0 — Bootstrap and toolchain** (Part 7 of the master specification): the
workspace, all crates, the lint set, the scripts, and the CI matrix exist;
no web behavior is implemented yet. The living ledger is
[PROGRESS.md](PROGRESS.md); the full plan and work breakdown structure
(2,680 tracked items) is [BROWSER_ENGINE_PROMPT.md](BROWSER_ENGINE_PROMPT.md).

## Build, test, run

```sh
cargo build --workspace          # everything
scripts/test-fast.sh             # fmt + clippy -D warnings + tests (~the pre-commit gate)
scripts/test-all.sh              # full tier: fast tier + dep policy + WBS asserts
cargo run -p aurora_shell -- --version
```

The engine builds warning-free in debug and release (`cargo clippy --workspace
--all-targets -- -D warnings` is part of the fast tier). Toolchain: Rust
stable 1.95.0 (pinned in `rust-toolchain.toml`), edition 2024.

## Crates

| Crate | Owns |
|---|---|
| `aurora_url` | URL parse/serialize/join, origins (§5.1) |
| `aurora_net` | HTTP/1.1, redirects, sniffing (§5.2) |
| `aurora_encoding` | UTF-8/16, windows-1252, detection |
| `aurora_html` | HTML tokenizer + tree constructor (§5.4–§5.5) |
| `aurora_dom` | DOM core, events, ranges (§5.6) |
| `aurora_css` | CSS tokenizer/parser, selectors AST, values (§5.7) |
| `aurora_style` | Matching, cascade, computed values (§5.8) |
| `aurora_text` | Fonts, shaping, line breaking, bidi (§5.11) |
| `aurora_image` | Owned PNG/JPEG/GIF/BMP/ICO decoders (§5.12) |
| `aurora_layout` | Box building, fragment tree, layout (§5.9–§5.10) |
| `aurora_paint` | Display list, software rasterizer (§5.16) |
| `aurora_js` | JS lexer/parser/interpreter/GC (§5.13–§5.14) |
| `aurora_runtime` | Event loop, timers, bindings, console (§5.15) |
| `aurora_storage` | Cookies, web storage, HTTP cache (§5.17) |
| `aurora_security` | Origins, SOP, CSP hooks (§5.18) |
| `aurora_ipc` | Cross-thread message types (§4.4) |
| `aurora_platform` | Window/present, clipboard, fonts, DPI (§3.5) |
| `aurora` | The `WebView` facade — the engine's only public API (§4.1) |
| `aurora_shell` ([apps/](apps/aurora_shell)) | The browser application (§5.19) |

The engine never imports the shell; the shell reaches the engine only through
`aurora::WebView`. `scripts/check-deps.sh` enforces both boundaries.

## Repository map

- `BROWSER_ENGINE_PROMPT.md` — master specification (constitution, subsystem
  specs, WBS, milestones, session protocol). The WBS section is generated from
  `tools/wbs_data_*.py` via `python3 tools/generate_wbs.py`; the whole document
  is assembled by `python3 tools/build_prompt.py` from `parts/` + `generated/`.
- `PROGRESS.md` — session ledger, the single source of truth for project state.
- `docs/` — architecture map, glossary, ADRs, spec-reading notes.
- `scripts/` — test tiers, dependency policy check, WBS progress report.
- `.github/workflows/ci.yml` — fast tier on Linux/Windows/macOS; full tier nightly.
