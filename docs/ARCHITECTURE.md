# ARCHITECTURE

The authoritative architecture is Part 4 of `BROWSER_ENGINE_PROMPT.md`; this
file is the working map kept accurate by every ADR that changes a diagram or
table there (§11.2). State below reflects what is *built*, not only planned.

## Pipeline (§4.2)

```
URL → fetch → sniff → tokenize → tree-construct → subresources
    → style → boxes → layout → display list → rasterize → present
```

Invalidation flows backwards with the narrowest legal scope. Each stage's
dirty protocol is defined in its Part 5 section; the default is a conservative
full-stage rebuild until a Part 10 budget demands and proves better.

## Crate map (§4.3)

All 19 crates of the §4.3 table exist as skeletons since M0, in one Cargo
workspace: `crates/aurora_url` … `crates/aurora` plus `apps/aurora_shell`.
Load-bearing rules:

- Crate boundaries are the future process boundaries (§4.7) — the public API
  of each crate lives in exactly one file (`src/facade.rs`).
- The dependency graph is restricted by the §4.3 "may depend on" column and
  machine-checked by `scripts/check-deps.sh` on every full-tier run.
- Bridge rule: `aurora_dom` and `aurora_js` never depend on each other;
  `aurora_runtime` mediates through `aurora_js`'s `HostObject` trait.
- The shell is never imported by an engine crate; the engine reaches the
  shell through the single `aurora::WebView` entry point.

At M0 every crate carries zero dependencies (§3.2 Tier 0) and only skeleton
code; the dependency edges appear with the milestones that need them.

## Threading model (§3.7, §4.4)

Static pipeline threads + one UI thread, message-passing only, no async
runtime: **UI thread** (window, shell) · **loader pool** (default 1, cap 4) ·
**document threads** (one per live document; the DOM exists only here) ·
**storage writer** (single, serializing). Message types live in `aurora_ipc`;
cross-thread shared mutable state is forbidden except atomics for liveness.

## Error taxonomy (§4.6)

Resource errors are typed `Result` values; programming errors panic with the
invariant named; boundary errors recover per spec (parsers) or throw standard
`DOMException`s (script-facing). The engine never panics on external input.

## Decisions

Append-only ADRs in `docs/adr/`:

- ADR-0001 — bootstrap toolchain: Rust 1.95.0, edition 2024, the §9.6 lint set
  via `[workspace.lints]`, three-platform CI.
- ADR-0002 — the prompt source (`parts/`, `generated/`, `tools/`) is vendored
  in the repository so the §6.12 regeneration discipline is executable.
