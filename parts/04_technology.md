## PART 3 — Technology Policy

### §3.1 Primary language and rationale

The engine and the shell are written in **Rust** (edition 2021 or newer, stable
toolchain). Rationale, recorded here so it is never relitigated without an ADR:

- Memory safety without a tracing GC in the *host* language. The JavaScript engine
  needs a precise GC for *script* objects; the last thing the project needs is a second
  GC (the language's) fighting it for control. Rust's ownership model also documents
  the pipeline's ownership story (§4.5) in the type system.
- Cargo gives the workspace, test harness, benchmarking, fuzzing harness, and docs in
  one toolchain — the project's process overhead is small enough for an autonomous agent.
- First-class C interop exists for the two places we genuinely need C libraries
  (windowing, system font access) via the approved crates of §3.3.

**Fallback rule:** if a hard blocker appears (a required approved dependency is broken
on a target platform, and no alternative exists), the language does not change — the
dependency tier does (§3.2), or the platform target is dropped with an ADR. Changing
implementation language mid-project is prohibited outright.

C++20 is the designated *alternative* language for this prompt: if you are executing
this prompt in an environment where Rust is genuinely unavailable, port every Rust
sketch in this document to C++20 with the mapping table in §3.1.1, and record the
substitution in `PROGRESS.md`. All architecture, scope, process, and quality rules
apply unchanged.

#### §3.1.1 C++20 mapping table (only if Rust is unavailable)

| Rust concept | C++20 equivalent | Notes |
|---|---|---|
| `enum` with data | `std::variant` + constexpr tags | Prefer a sealed class hierarchy for recursive ASTs |
| `Result<T, E>` | `std::expected<T, E>` | Or a project-local `Result`; no exceptions for control flow |
| `Option<T>` | `std::optional<T>` | |
| `Box<T>` / `T` in `Vec` | `std::unique_ptr<T>` / `std::vector<T>` | |
| `Rc<RefCell<T>>` (DOM) | intrusive refcount + arena, or `std::shared_ptr` | GC-managed JS values are their own system (§5.14) |
| `cargo test` | CTest + doctest/Catch2 | One framework, chosen once |
| `cargo fmt`/`clippy` | `clang-format`/`clang-tidy` | Config checked in on day one |
| `unsafe` | raw pointer code | Same audit rules as §9.4 |
| traits | concepts / CRTP / virtual | Choose virtual for the two marked seams only (§2.6) |

### §3.2 The tiered dependency policy

Dependencies are classified into three tiers. The classification is recorded in the
workspace `Cargo.toml` comments and enforced in review (§9.7).

- **Tier 0 — owned.** All web-platform semantics: URL parsing, HTTP/1.1, HTML/CSS/JS
  tokenization and parsing, DOM, style cascade, layout, display lists, rasterization
  (CPU), image decoders (PNG/JPEG/GIF/BMP/ICO), the JavaScript lexer/parser/interpreter,
  the GC, the event loop, cookies, storage engines. Zero third-party code at this tier,
  ever. This is the product.
- **Tier 1 — approved infrastructure.** Libraries that replace OS- or format-level
  plumbing whose semantics are not part of the web platform. Each requires a one-line
  justification in `Cargo.toml`. The approved list is §3.3. Adding a Tier 1 dependency
  not on the list requires an ADR with: what it does, what it pulls in transitively,
  why no approved option works, and the migration path if it must be dropped.
- **Tier 2 — exceptional.** Anything else (crypto beyond TLS, compression you decline
  to own, GPU). Requires an ADR *and* a milestone-deferred plan to remove it if the
  corresponding owned implementation is on the WBS. Tier 2 dependencies must never be
  reachable from the engine's public API types.

Rule of thumb: if the library's name would appear in a web platform spec, it is Tier 0
and you must write it. If it would appear in an OS manual, it is Tier 1 candidate.

### §3.3 Approved dependency list

These are pre-approved. Everything not listed is Tier 2 until an ADR approves it.

| Crate (Rust) | Purpose | Tier | Constraint |
|---|---|---|---|
| `winit` | OS window + input events | 1 | Shell only, never the engine |
| `softbuffer` | Present a CPU raster to a window | 1 | The M5/M6 paint path |
| `wgpu` | GPU backend (stretch, §1.5) | 2 | Behind the same display-list API |
| `rustls` + `rustls-pemfile` | TLS 1.2/1.3 client | 1 | Engine networking uses it; you implement the HTTP on top |
| `rustls-native-certs` / `webpki-roots` | Trust store | 1 | Prefer `webpki-roots` for hermetic tests |
| `harfbuzz` (via `harfbuzz-sys`) or `rustybuzz` | Text shaping | 1 | §5.11: own shaper for Latin first; integrate for complex scripts |
| `fontdb` / `fontconfig` binding | System font enumeration | 1 | Cache results; no runtime re-scan |
| `memmap2` | mmap for font/cache files | 1 | |
| `image` | **Reference only** in tests for pixel comparison | 1 | Never linked into the engine binary — the engine ships its own decoders |
| `png` crate | Same: reference-only test oracle | 1 | Same constraint |
| `flate2` / `miniz_oxide` | DEFLATE for HTTP gzip and PNG unless owned | 1 | Owned inflate is a WBS item; switching must keep tests green |
| `libc`, `windows-sys`, `objc2` | Platform glue | 1 | Behind `aurora_platform` only (§4.3) |
| `unicode-bidi`, `unicode-segmentation`, `unicode-normalization` | Unicode algorithms | 1 | These are UCD-derived algorithm crates, permitted; write your own only if an ADR says so |
| `rayon` | Test-time parallelism only | 1 | Never in the engine pipeline |
| `serde` | DevTools protocol and prefs files only | 1 | Not in engine hot paths |
| `criterion` | Benchmarks | 1 | Dev-dependency |
| `cargo-fuzz`/`libFuzzer` | Fuzzing harnesses (§8.7) | 1 | Dev-dependency |

Explicitly **not approved** at any tier: any HTML/CSS/JS/DOM/layout/parser library
(`html5ever`, `scraper`, `selectors`, `quick-xml` for web documents, `swc`, `boa`,
`quickjs`, `deno_core`, `tao`-based toolkits, GTK/Qt/wx/egui for the shell UI — the
shell draws its own chrome with the engine's rasterizer; that is the point).

### §3.4 Build system and toolchain

- One Cargo workspace (§4.3 shows the member list). One command builds everything:
  `cargo build --workspace`. One command tests everything: `cargo test --workspace`.
- `rustfmt.toml` and `clippy.toml` are committed in the **first session**, before the
  first crate, with the lint set of §9.6. `cargo fmt --check` and `cargo clippy
  --workspace -- -D warnings` are part of the fast test tier and must be clean at every
  commit.
- Feature flags: each optional subsystem (TLS, GPU, fuzzing, devtools server) is a
  workspace feature. The default feature set must be sufficient to build and run every
  milestone demo through M13.
- MSRV (minimum supported Rust version): the current stable at project start, pinned
  in `rust-toolchain.toml`. Do not chase new nightly features; this project's value is
  portability and longevity.
- Builds must be warning-free in release *and* debug. If a warning cannot be fixed
  immediately, it is fixed before the commit that introduced it — there is no
  `#[allow]` without a written reason (§9.6).

### §3.5 Platform targets

Tier-1 platforms (must work, must have CI): **Linux (X11 via XCB through winit),
Windows 10/11, macOS 13+**. Tier-2 (builds, not guaranteed): FreeBSD, Linux/Wayland.

The platform-specific surface is confined to `aurora_platform` (§4.3): window and
event pump (via winit anyway), clipboard, file dialogs, font enumeration, high-res
clocks, screen DPI. Any `#[cfg(target_os)]` outside `aurora_platform` fails the
self-review checklist. If a platform is unavailable in your environment, that fact is
recorded once in `PROGRESS.md` and the other platforms carry the acceptance burden,
per §1.6.

### §3.6 Graphics backend strategy

The engine paints to an abstract `Surface` trait (§5.16): a 2D pixel buffer with the
raster operations the paint stage needs (fill, path fill/stroke, image blit with
transforms, glyph runs, clips). Two backends:

1. **Software rasterizer (owned, Tier 0)** — the default and the acceptance path. You
   write scanline polygon fill, Bresenham/AA line drawing, image resampling
   (bilinear/bicubic), glyph rasterization from font outlines, and dirty-rect
   composition. This is the path every milestone is validated against, because it is
   deterministic, debuggable, and dependency-free.
2. **GPU backend (stretch)** — later, behind the same `Surface` trait, via `wgpu`.
   Pixel tests always run against the software backend; the GPU path gets structural
   tests only (same display list, same geometry, toleranced pixels).

Rasterization is deterministic: fixed-point or carefully-ordered float math, no
multithreaded tile ordering that changes output, no platform differences. Golden pixel
tests (§8.5) depend on this and it is load-bearing for the whole test strategy.

### §3.7 Concurrency model

Chosen model: **static pipeline threads + one UI thread, message-passing between
them, no async runtime** (§2.6). Specifically:

- The **main/UI thread** owns the window, the event pump, and the shell. It never
  blocks on I/O and never runs layout.
- The **loader thread pool** (start with a single loader thread; grow when profiling
  demands, not before) performs DNS, connect, TLS, and HTTP exchange, delivering
  decoded bytes to the document thread.
- The **document thread** (one per active document, plus one per hidden-but-alive
  tab) runs the parse → style → layout → paint pipeline, the script runtime, and the
  event loop (§5.15). The DOM lives exclusively on its document thread.
- Cross-thread traffic goes through bounded MPSC channels with typed messages defined
  in `aurora_ipc` (§4.3). Every message variant is documented with the thread that may
  send it and the thread that must handle it.
- Shared mutable state across threads: none, with the single exception of atomics for
  liveness flags (`AtomicBool` cancels). If you find another, that is an ADR.

This model is chosen because it is debuggable (thread-per-document mirrors the future
process-per-document of §4.7) and because an autonomous agent can hold it in its head.
The future multi-process split moves channel sends to serialized IPC with zero
message-semantics changes; that is the test of the design.
