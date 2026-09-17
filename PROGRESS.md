# PROGRESS

The single source of truth for where the project is (§12.1). Updated every
session, before the report ends. Reality wins over this file, then this file
is corrected.

## Current state

- Milestone: M2 — HTML to DOM (tokenizer hardening; tree construction pending)
- WBS completion: 8 of 2,680 items (0.30%; script rounds to 0%)
- Standing placeholders: 0
- Open AURORA-SHORTCUT tags: 0
- Bench deltas vs last milestone: none (benchmarks land with the §10.1 table)

## Open questions

None.

## Debts

- Tokenizer conformance is not complete: html5lib adoption and fuzz verification remain pending.
- Tree construction remains pending; DOCTYPE classification and guarded DOM
  insertion are available as a prerequisite, not a complete initial mode.
- The input cursor buffers the whole document; streaming remains pending.
- Earlier session reports mention configurable network deadlines and attribute
  namespaces as deferred work; their completion has not been re-audited here.

## Decision log

| Session | Decision | ADR |
|---|---|---|
| 1 | Rust 1.95.0 / edition 2024; §9.6 lint set via `[workspace.lints]` (mechanism equivalent of §4.10's per-crate attributes); three-platform CI per §3.5, superseding §7.2's "two-platform" wording | 0001 |
| 1 | Prompt source (`parts/`, `generated/`, `tools/`) vendored at repo root so WBS ticks follow the §6.12 data-file → regenerate → review discipline | 0002 |
| 1 | All 19 crates start with zero dependencies (§3.2 Tier 0); dependency edges appear with the milestone that needs them | — |
| 5 | License: Apache-2.0 (explicit patent grant, §3); MIT option dropped from the M0 placeholder `MIT OR Apache-2.0` | 0005 |
| 4 | `FOR-NEXT-AGENT.txt` at repo root: the durable handoff hint for successor sessions/models; CURRENT STATE block refreshed every session; PROGRESS.md stays the state truth | 0004 |

## Session log

(newest first, §11.4 template)

## Session 2026-09-18 — script fidelity and DOCTYPE prerequisite
- **Implemented:** double-escaped script preserves literal `<` and `/`, with
  EOF diagnostics and 66 matrix cases; pushed as ccdfca3 after 7364845.
  Added Doctype token conversion, force-quirks preservation, legacy public
  identifier classification, and guarded insertion into Document. Duplicate
  doctypes or an existing document element prevent insertion and mode changes.
- **Tested:** fast tier and dependency policy passed; aurora_html has 41 unit
  tests, two DOCTYPE integration tests, and one smoke test. Targeted DOCTYPE
  tests and clippy were rerun successfully before commit.
- **Scope:** this is a prerequisite of §5.5, not a complete tree constructor.
  No WBS ticks; html5lib conformance remains unverified.
- **Next task:** implement initial/before-html/before-head insertion modes
  and tokenizer/tree-builder coordination, then head/text modes.

## Session 2026-09-18-0000 — M2 slice 2 hardening: end-tag name fidelity, EOF recovery, attributes on end tags; 28 tests green
- **Milestone:** M2 — HTML to DOM   **WBS items touched:** none (tokenizer
  completes against html5lib, not yet adopted — nothing tickable)
- **Implemented:**
  - End-tag candidate fidelity in text modes: the temp buffer now keeps
    original case (the standard's temporary buffer), the lowercase name
    accumulates in `tag_name` from the first letter, and "appropriate"
    compares that tag name to the last start tag (empty last-start-tag
    never matches — the standard's missing precondition).
  - EOF handling rewritten to the standard's shapes: text-mode `<` and
    `</` at EOF emit themselves (no error); end-tag-name at EOF flushes
    `</` + original-case buffer and reports no tokenizer error; escaped
    script shapes additionally report
    `eof-in-script-html-comment-like-text`; but once an appropriate name
    has entered attribute/self-closing parsing, EOF is `eof-in-tag` and
    the candidate is dropped (no literal fallback) — a three-way
    distinction the old code flattened.
  - End tags can now carry attributes and a trailing solidus: both are
    reported (`end-tag-with-attributes`,
    `end-tag-with-trailing-solidus`) and discarded; attributes no longer
    leak into the following start tag.
  - Removed `Cursor::at_eof` (§9.10) after the dead branch left with it.
- **Tested:** fast tier — all green; aurora_html has 28 unit tests plus
  its smoke test. Preserved the two pre-existing uncommitted regression
  tests and added five more tests covering case preservation, EOF recovery,
  attribute isolation, and end-tag errors. Both original failures were
  reproduced before their fixes and rerun individually afterward. `cargo test -- --test-threads=1`
  also green. `check-deps` OK; WBS unchanged (8/2680).
  Note: a stale `aurora_net` test binary pointed at an old absolute path
  and failed `fs::read`; `cargo clean -p aurora_net` + rebuild resolved it
  (environment artifact, not a code fault).
- **Bench:** n/a
- **Decisions:** "appropriate" requires a non-empty last-start-tag
  (missing-precondition); end-tag errors are centralized in `emit_end_tag`
  (the flush path is single); EOF emission happens before the error report
  to keep token order.
- **Debts opened/closed:** opened — double-escaped script states still
  drop `<`/`/` in `</script>` text (see Debts); the old escape-dance test
  asserts the wrong behavior and must be corrected when fixing it.
- **Next task:** fix the double-escaped `<`/`/` emission with corrected
  expectations; then §5.5 tree construction (§12.3 rule b/c) and
  html5lib adoption.

## Session 2026-09-14-0130 — M2 slice 2: the §13.4 tokenizer complete, 21 tests green
- **Milestone:** M2 — HTML to DOM   **WBS items touched:** none tickable yet (html5lib adoption is the tick bar)
- **Implemented:**
  - `aurora_html::Tokenizer` (public per the §5.4 sketch): the full §13.4
    state machine — data/rcdata/rawtext/script-data/plaintext, tags,
    attributes, comments, DOCTYPEs, CDATA routing, character references.
    Parse errors collected by the standard's codes; token emission via
    character runs; start-tag attribute dedupe (first wins).
  - Verified-against-spec subtleties pinned by tests: AttributeName '='
    goes straight to the value state (§13.2.5.33 — my first draft had it
    wrong and the trace caught it); EscapedLessThanSign RECONSUMES the
    first alpha (§13.2.5.23); the '/' of a double-escaped </script> is
    consumed silently (§13.2.5.30) — why it cannot close the element;
    AfterDoctypePublicIdentifier takes a SECOND QUOTED STRING, no SYSTEM
    keyword; &notit; → ¬ + parse error outside attributes.
- **Tested:** fast tier — 124 passed / 0 failed (was 103). New: 21
  tokenizer/reference unit tests covering markup, hostile comments
  (nested `<!--` shapes), all doctype variants incl. abrupt identifiers,
  the double-escape dance, EOF recovery (eof-in-tag drops, eof-in-comment
  emits), NUL/CRLF handling, legacy references. clippy -D warnings clean.
- **Bench:** n/a
- **Decisions:** Character runs from the start (sketch-permitted);
  generated entities table committed for hermetic CI; Tokenizer made
  public per the sketch (was pub(crate) — dead-code in non-test builds
  forced the facade decision, which the sketch itself mandates).
- **Debts opened/closed:** `set_allow_cdata` removed as unused — re-add
  with the foreign-content insertion modes (noted in FOR-NEXT-AGENT.txt).
- **Next task:** §5.5 — tree construction (`tree_builder.rs`) over the
  Token stream into aurora_dom; then html5lib-tests adoption (§12.3 b/c).

## Session 2026-09-14-0030 — M2 slice 2 in flight: tokenizer prerequisites, entities table
- **Milestone:** M2 — HTML to DOM   **WBS items touched:** none ticked yet (tokenizer not complete)
- **Implemented:**
  - `tools/gen_entities.py` + generated `tables.rs`: the full named
    character-reference table (2,231 entries) from the WHATWG entities
    JSON, per the §6.2 preamble's "generated at build time" intent
    (committed artifact + regeneration command instead of build-time
    network fetch — hermetic CI).
  - WIP (untracked, documented in FOR-NEXT-AGENT.txt): `token.rs` (Token
    model + State enum), `cursor.rs` (CR/LF preprocessing + reprocess
    push-back), `reference.rs` (named/numeric references, windows-1252
    remap, attribute ambiguity, unit tests).
- **Tested:** fast tier — 103 passed / 0 failed (aurora_html table
  compiles clean; WIP modules are not yet wired into lib.rs, their tests
  run once the tokenizer lands).
- **Bench:** n/a
- **Decisions:** Character runs (`Character(String)`) adopted from the
  start — the §5.4 sketch explicitly permits it and html5lib expectations
  merge consecutive characters. Committed generated table over
  build-time generation for hermetic CI (§8.2: no network).
- **Debts opened/closed:** none new; WIP is tracked in the handoff file.
- **Next task:** finish `tokenizer.rs` (§13.4 state machine), wire the
  modules, then §5.5 tree construction (§12.3 rule b/c).

## Session 2026-09-13-2340 — M2 slice 1: aurora_dom node arena + serialization; handoff file
- **Milestone:** M2 — HTML to DOM   **WBS items touched:** none tickable yet (§6.6 interface items tick per-interface as the script surface lands)
- **Implemented:**
  - `FOR-NEXT-AGENT.txt` (ADR-0004): durable handoff hint for successor
    sessions/models — pointers to the three sources of truth, session
    protocol, environment notes, CURRENT STATE block refreshed each session.
  - `aurora_dom`: the node arena (§4.5) — generational indices with
    tombstones, logical removal, teardown Drop; the Document mutation
    surface (`append`/`insert_before`/`remove`/`set_attribute`, cycle
    prevention, `contains`, `children`, `last_descendant`); HTML
    serialization per the WHATWG algorithm — iterative (50k-deep tree
    test, no recursion), raw-text literals, standard escape sets.
- **Tested:** fast tier — 103 passed / 0 failed (was 95). New: DOM
  behavior suite (8: tree build + serialize round-trip, attributes with
  nbsp/quote escaping, raw-text literals, insert_before ordering,
  remove/reinsert, contains + cycle rejection, fragment/doctype, deep
  nesting) + the tombstone unit test.
- **Bench:** n/a
- **Decisions:** tombstoning applies to detached nodes only (DOM `remove`
  detaches; teardown/tombstone is the lifetime decision, §4.5). The
  serializer uses the WHATWG escape sets (&, nbsp, <, > text; &, nbsp, "
  attributes) — the §5.6 table's attribute column is broader than the
  standard; the standard wins (§0.4) — flagged as a spec-table
  discrepancy. RCDATA has no separate serializer set. `set_quirks`
  removed as unused until the tree constructor needs it (§9.10).
- **Debts opened/closed:** namespace model for attributes deferred to the
  foreign-content insertion modes (recorded in `ElementData` docs).
- **Next task:** §5.4 — the HTML tokenizer in `aurora_html`; states
  translated 1:1 from the standard; spec notes in
  `docs/spec-notes/html-tokenizer.md` first (§12.3 rule c).

## Session 2026-09-13-2230 — M1 hardening: byte-exactness, error injection, TLS matrix; exit criterion met
- **Milestone:** M1 — Fetch and render text   **WBS items touched:** §6.9 items 17, 18 ticked (evidence below); §5.3 DoD satisfied
- **Implemented:**
  - Byte-exactness: the emitted request head is compared byte-for-byte
    against `tests/golden/net/request-head.txt` (§8.3; `{port}` substituted).
  - Error injection (§6.9 items 17/24 subset): truncation mid-head,
    truncation mid-content-length, invalid chunk size, premature close
    mid-chunked — all surface typed `NetError::Protocol`, no panics; plus a
    scripted-delay test. The chunked decoder now distinguishes "incomplete"
    (`Ok(None)`) from "invalid" (`Err`) — a real conflation bug the
    truncation tests exposed and fixed.
  - TLS (§5.3 DoD): `Pool::with_root_certs` for test/`--cert-bundle` trust
    overrides; committed throwaway fixture PKI (`tests/fixtures/tls/`);
    in-process rustls TLS server; the four-case matrix — good chain serves,
    expired → `Tls("certificate expired")`, wrong host → `Tls("hostname
    mismatch")`, untrusted root → `Tls("untrusted certificate authority")`.
    Handshake driver rewritten (`read_tls`/`write_tls` + typed
    `process_new_packets`) so verification failures surface exactly.
- **Tested:** fast tier — 95 passed / 0 failed (was 83). New: golden
  byte-exactness (1), error injection + delay (5), TLS matrix (5), chunked
  incomplete-vs-invalid unit cases. clippy `-D warnings` clean.
  **Demo (§11.5, M1 exit criterion):** `aurora --dump-bytes` vs
  `curl -sL` — byte-identical (cmp) on: http://example.com/ (559 B),
  https://example.com/ (559 B), https://www.rfc-editor.org/rfc/rfc2324.txt
  (19 610 B), https://www.iana.org/help/example-domains (6 639 B),
  http://info.cern.ch/ (646 B). neverssl.com was down from this network
  (curl also failed) and was replaced by info.cern.ch — recorded per §11.5.
- **Bench:** n/a
- **Decisions:** chunked "incomplete vs invalid" split; trust-store override
  lives on `Pool` (the connection factory) rather than `Request`;
  test-only fixture PKI committed with regeneration notes
  (`docs/spec-notes/tls.md`).
- **Debts opened/closed:** TLS matrix debt CLOSED; byte-exactness exit
  criterion CLOSED; item-15 configurable-deadline debt OPEN (see Debts).
- **Next task:** §5.4/§5.5 — begin M2: the HTML tokenizer
  (`aurora_html`), the first subsystem of Part 5's document pipeline
  (§12.3 rule c; M1's remaining items — error page UI, about:srcdoc —
  need the document concept M2 builds).

## Session 2026-09-13-2100 — M1 network slice: URL parser + HTTP/1.1 + TLS, 6 WBS ticks
- **Milestone:** M1 — Fetch and render text   **WBS items touched:** §6.9 items 2–7 ticked; item 1 partial (about:srcdoc + error page remain)
- **Implemented:**
  - `aurora_url`: full WHATWG basic URL parser translated step-for-step from
    the living standard (states, host/IPv4/IPv6 parsers+serializers, file
    quirks, percent-encode sets), serialization, relative resolution, origins,
    form-urlencode. Parsing is total; round-trip suite green.
  - `aurora_net`: injection-proof `HeaderMap`; own chunked decoder;
    content-length/until-close framing; gzip/deflate via `flate2` (§6.9 item 7
    permits it); redirect chains (301–303 → GET, 307/308 preserved, 20-hop cap,
    cross-origin credential stripping); per-origin keep-alive pool (6 conns,
    60 s idle) with one clean retry on a stale pooled connection; per-phase
    timeouts; shared `CancelToken`. TLS 1.2/1.3 via `rustls`(ring) +
    `webpki-roots` (§3.3); ALPN http/1.1; no click-through (§5.3).
  - `aurora_encoding`: UTF-8 (lossy) + windows-1252 decode; charset labels.
  - `aurora_runtime`: fetch entry point with scheme dispatch (http/https/
    file/data/about:blank; unknown → UnsupportedScheme), owned base64.
  - `aurora` bin: `--url/--dump-bytes/--dump-text/--version` (§7.3 demo).
- **Tested:** fast tier — 83 passed / 0 failed (was 19). New: URL behavior +
  round-trip suites (18), host unit tests, mock-server suite (10: framing,
  gzip golden, redirect chain, keep-alive reuse, stale-connection retry,
  HEAD, obs-fold, hostile heads), scheme tests (7), chunked/headers unit
  tests. `check-deps` OK. Full tier green.
  Verified live: `aurora --dump-text https://example.com/` renders via the
  real DNS→TCP→TLS→HTTP path; data:/file:/about:/unknown schemes verified.
- **Bench:** n/a
- **Decisions:** bare-LF header terminators accepted (robustness), bare-LF in
  chunked framing rejected (RFC-strict) — recorded in code + PROGRESS.md.
  Response bodies buffered in M1 (framing reader is the streaming seam).
  ADR-0003: `aurora_runtime` gains `aurora_url`/`aurora_encoding` edges; the
  headless binary lives in the `aurora` package as a bin target using only
  the facade.
- **Debts opened/closed:** see Debts above (TLS matrix, 307/308 tests,
  console note, fuzz targets, byte-exactness curl criterion).
- **Next task:** §6.9 item 18 — byte-exactness request tests + the M1 exit
  criterion (`--dump-bytes` vs `curl -s` on five URLs), then mock-server
  delays/truncations (§6.9 item 17); §12.3 rule b (smallest unfinished M1
  exit-criteria item).

## Session 2026-09-13-1900 — M0 bootstrap: workspace, lint set, scripts, CI
- **Milestone:** M0 — Bootstrap and toolchain   **WBS items touched:** none (M0 is infrastructure only, §6.13)
- **Implemented:**
  - Cargo workspace with all 19 crates of the §4.3 map (18 in `crates/`, the
    `aurora_shell` application in `apps/`): each with the §4.10 skeleton
    (crate doc naming responsibility/thread/neighbors, `facade.rs` as the
    single re-export surface, an M0 smoke test), zero dependencies.
  - §9.6 lint set via `[workspace.lints]`; `rustfmt.toml`, `clippy.toml`,
    `rust-toolchain.toml` (MSRV 1.95.0) committed (ADR-0001).
  - Shell `--version` demo: prints name, workspace version, and git commit
    stamped by `build.rs`.
  - Scripts: `test-fast.sh` (fmt, clippy `-D warnings`, tests),
    `test-all.sh` (fast tier + dependency policy + WBS regeneration
    asserts), `check-deps.sh` (§4.3 allowed-deps map, §3.3 approved/forbidden
    lists, shell boundary), `wbs-progress.sh` (§6.14 report incl. `--strict`),
    `new-milestone.sh`, session-report template.
  - Prompt source vendored (`parts/`, `generated/`, `tools/`) with the
    byte-identical-regeneration assert in the full tier (ADR-0002).
  - Docs of §11.2 as stubs: README, ARCHITECTURE, GLOSSARY, UA-STYLESHEET,
    devtools-protocol, known-issues, spec-notes, ADR-0001/0002, CHANGELOG;
    tests/ inventory README.
  - CI: fast tier on Linux/Windows/macOS, full tier nightly.
- **Tested:** fast tier — 19 passed / 0 failed / 0 skipped (one smoke test per
  crate; bin test in `aurora_shell`); `cargo fmt --check` clean;
  `cargo clippy --workspace --all-targets -- -D warnings` clean;
  `scripts/check-deps.sh` OK; `scripts/wbs-progress.sh` reports 2,680 open / 0 done.
- **Demo (§7.2, verified at HEAD):** `cargo run -p aurora_shell -- --version`
  → `aurora-shell 0.1.0 (commit 2ccc011)`
- **Bench:** n/a (no benchmarks exist yet; §10.1 table lands with first benches)
- **Decisions:** see decision log above (ADR-0001, ADR-0002).
- **Debts opened/closed:** none. Environment note: rustfmt/clippy installed via
  distro packages (no rustup on this machine); `rust-toolchain.toml` advisory
  here, binding in CI — recorded in ADR-0001.
- **Next task:** §6.9 item 1 via §5.1 — begin M1 with `aurora_url`: the WHATWG
  URL parser and its unit/golden tests (§12.3 rule c, dependency-first: URL is
  the prerequisite of every §6.9 network item).
