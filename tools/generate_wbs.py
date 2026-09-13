#!/usr/bin/env python3
"""Generate the Work Breakdown Structure (Part 6) and Appendices for the
AURORA browser-engine prompt, expanding the compact datasets in wbs_data_a.py
and wbs_data_b.py into per-item task checklists.

Outputs (relative to the prompt project root):
    generated/06_wbs.md        — Part 6 sections 6.1–6.12
    generated/13_appendices.md — Appendices A–F

The generator is idempotent: identical data produces a byte-identical file
(asserted in CI per §6.12). Output lines carry no trailing whitespace; blank
lines are explicit empty strings.
"""
from __future__ import annotations

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import wbs_data_a as A
import wbs_data_b as B


def rows(text: str) -> list[list[str]]:
    """Split a pipe-delimited dataset block into field lists, dropping blanks/comments."""
    out = []
    for line in text.strip().splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        out.append([f.strip() for f in line.split("|")])
    return out


def wrap_list(items: list[str], width: int = 96, indent: str = "  ") -> list[str]:
    """Render semicolon-separated member lists as continuation-wrapped text lines."""
    lines: list[str] = []
    cur = ""
    for item in items:
        piece = f"{item}; " if item is not items[-1] else f"{item}."
        if cur and len(cur.rstrip()) + len(piece) > width:
            lines.append(cur.rstrip())
            cur = indent + piece
        else:
            cur += piece
    if cur:
        lines.append(cur.rstrip())
    return lines


# --------------------------------------------------------------------------
# Part 6 header matter
# --------------------------------------------------------------------------

WBS_HEADER = """## PART 6 — Work Breakdown Structure (WBS)

### §6.1 How to read and use the WBS

- The WBS is the project's **task ledger**. Every `- [ ]` item is a task with an
  acceptance bar: done means implemented, tested (§8), and reported (§11.4).
- Items are grouped by surface, not by crate: one item may touch several crates;
  the crate map (§4.3) tells you where the code lives.
- Milestones (Part 7) reference these sections; each milestone names the subset
  of items it claims. An item checked in a later milestone than its surface's
  "home" milestone is normal — the ledger records *when*, the WBS records *what*.
- Where a checklist row and the standard disagree, the standard wins (§0.4).
  Rows marked *verify* contain values you must confirm against the spec when you
  implement them — treat them as leads, not answers.
- Tick items **in the same session** that completes them (§12.1). A checked item
  without a passing-test reference in the session log is treated as unchecked by
  the final audit (§12.7).
- This section is generated from `tools/wbs_data_*.py`; regenerate with
  `python3 tools/generate_wbs.py` after editing the data files, and never
  hand-edit the generated text — edit the data, regenerate, review the diff.
"""

# --------------------------------------------------------------------------
# §6.2 HTML elements
# --------------------------------------------------------------------------

HTML_BLOCK = """#### `<{name}>` — {category}
- Content model: {content}.
- Parser behavior: {parser}.
- UA defaults: {ua}.
- Layout: {layout}.
**Tasks:**
- [ ] Tree construction: the element's insertion-mode rules per HTML §13.2.6 (§5.5).
- [ ] Attributes: parsed, reflected to IDL (§6.6); presentational hints mapped to style where applicable.
- [ ] UA stylesheet: the defaults above encoded in the UA origin (§7.5) and `docs/UA-STYLESHEET.md`.
- [ ] Layout/paint: the box behavior above wired through §5.9/§5.10 into §5.16.
- [ ] Tests: one tree-construction case, one style case, one layout golden case exercising the element."""


def gen_html(out: list[str]) -> None:
    out.append("### §6.2 HTML element checklist")
    out.append("")
    out.append("One block per element of the supported surface (§1.3). Legacy elements are")
    out.append("parse-compatible only; `non-goal` surfaces stay stubs by design.")
    out.append("")
    for name, cat, content, parser, ua, layout in rows(B.HTML_ELEMENTS):
        out.append(HTML_BLOCK.format(name=name, category=cat, content=content,
                                     parser=parser, ua=ua, layout=layout))
        out.append("")


# --------------------------------------------------------------------------
# §6.3 CSS properties
# --------------------------------------------------------------------------

SHORTHANDS = {
    "margin", "padding", "inset", "overflow", "gap", "flex", "flex-flow",
    "place-items", "place-content", "place-self", "grid", "grid-template",
    "grid-row", "grid-column", "grid-area", "background", "border", "border-width",
    "border-style", "border-color", "border-top", "border-right", "border-bottom",
    "border-left", "border-radius", "border-image", "outline", "columns",
    "column-rule", "transition", "animation", "list-style", "text-decoration",
    "text-emphasis", "scroll-margin", "scroll-padding", "overscroll-behavior",
    "mask", "font", "all",
}

CSS_BLOCK = """#### `{name}`
- Inherited: {flags} · Initial: `{initial}` · Owner: {owner} · {shorthand}
**Tasks:**
- [ ] Parsing: the accepted value grammar in the property registry (§5.7); invalid values drop the declaration with a parse-error record.
- [ ] Computed value: keywords, relative units, percentages and `calc()` resolved per the owner section; `inherit`/`initial`/`unset`/`revert` per §5.8.3.
- [ ] Integration: the owner subsystem consumes the property (layout geometry, paint value, runtime behavior, or interaction state).
- [ ] UA stylesheet: default reviewed (§7.5); where the initial value is not the UA default, record the difference.
- [ ] Tests: grammar round-trip (valid + invalid), cascade case (§6.5), layout/paint golden where the property affects output."""


def gen_css(out: list[str]) -> None:
    out.append("### §6.3 CSS property checklist")
    out.append("")
    out.append("The engine's supported property surface. `Initial` values marked *verify*")
    out.append("must be confirmed against the property's CSSWG definition during")
    out.append("implementation; everything else follows §0.4 precedence. Shorthands are")
    out.append("expansion sugar — their longhands carry the real behavior.")
    out.append("")
    groups: dict[str, list[tuple[str, str, str]]] = {}
    for name, group, initial, inherited in rows(A.CSS_PROPERTIES):
        groups.setdefault(group, []).append((name, initial, inherited))
    for group, props in groups.items():
        owner = ""
        for line in A.CSS_GROUPS.strip().splitlines():
            if line.startswith(group + "|"):
                owner = line.split("|", 1)[1].strip()
        out.append(f"#### Group `{group}` — {owner}")
        out.append(f"- {len(props)} properties; work each block's five tasks in order; the group owner section defines what integration means here.")
        out.append("")
        for name, initial, inherited in props:
            flags = "Yes" if inherited.upper().startswith("Y") else "No"
            shorthand = ("Shorthand: expands into its longhands (order and reset semantics "
                         "per CSS Cascading §7).")
            if name not in SHORTHANDS:
                shorthand = ("Longhand: covered by its shorthand's expansion, if any.")
            out.append(CSS_BLOCK.format(name=name, flags=flags, initial=initial,
                                        owner=group, shorthand=shorthand))
            out.append("")


# --------------------------------------------------------------------------
# §6.4 at-rules, §6.5 selectors
# --------------------------------------------------------------------------

def gen_at_rules(out: list[str]) -> None:
    out.append("### §6.4 CSS at-rules checklist")
    out.append("")
    for rule, behavior in rows(A.CSS_AT_RULES):
        out.append(f"#### `{rule}`")
        out.append(f"- Behavior: {behavior}")
        out.append("**Tasks:**")
        out.append("- [ ] Parsing: prelude grammar + block handling + error recovery (§5.7).")
        out.append("- [ ] Evaluation: the condition/registration above wired to the consuming subsystem.")
        out.append("- [ ] Serialization: round-trips through the DevTools display (§5.19.5).")
        out.append("- [ ] Tests: parse case, evaluate case, nested-rule case where applicable.")
        out.append("")


def gen_selectors(out: list[str]) -> None:
    out.append("### §6.5 Selector engine checklist")
    out.append("")
    out.append("Each selector: parsed into the `Selector` AST, compiled into the matching")
    out.append("plan (§5.8.1), invalidation-registered (§5.8.6), and covered by a matching")
    out.append("test with positive and negative cases.")
    out.append("")
    for fields in rows(A.CSS_SELECTORS):
        # Selector notation itself contains pipes (svg|rect, [a|=v]) — split on the
        # first two separators only.
        sel, kind = fields[0], fields[1]
        note = " | ".join(fields[2:])
        out.append(f"- [ ] `{sel}` — *{kind}*: {note}")
    out.append("")


# --------------------------------------------------------------------------
# §6.6 DOM interfaces, §6.7 JS builtins
# --------------------------------------------------------------------------

def gen_dom(out: list[str]) -> None:
    out.append("### §6.6 DOM interface checklist")
    out.append("")
    out.append("One block per script-visible interface (§5.15.2 binding rules apply to")
    out.append("every one): correct prototype chain, attribute getters/setters with the")
    out.append("right exceptions, method overloads and optional arguments, stringifier /")
    out.append("iterable / legacy platform object behaviors where marked.")
    out.append("")
    for name, inherits, members, tasks in rows(B.DOM_INTERFACES):
        header = f"#### `{name}`"
        if inherits:
            header += f" — inherits `{inherits}`"
        out.append(header)
        out.append("- Surface:")
        out.extend(wrap_list([m.strip() for m in members.split(";") if m.strip() and m.strip() != "-"], 92, "  "))
        out.append(f"- Tasks: {tasks}.")
        out.append("- [ ] Binding: prototype chain, brand checks, exceptions per §5.15.2; IDL string/enum/union conversions; overloads and optional/default arguments.")
        out.append("- [ ] Tests: construction + one representative operation + wrong-argument-type exception; property/attribute reflection cases.")
        out.append("")


def gen_js(out: list[str]) -> None:
    out.append("### §6.7 JavaScript builtin checklist")
    out.append("")
    out.append("Every builtin is implemented with the interpreter's primitives (§5.14)")
    out.append("and verified against the adopted test262 subset (§8.6). `M9` marks the")
    out.append("baseline inventory; anything beyond lands in M13 polish unless a milestone")
    out.append("section pulls it earlier.")
    out.append("")
    for obj, members, notes in rows(B.JS_BUILTINS):
        out.append(f"#### `{obj}`")
        out.append(f"- {notes}")
        out.append("- Surface:")
        out.extend(wrap_list([m.strip() for m in members.split(";") if m.strip()], 92, "  "))
        out.append("- [ ] Implement: spec algorithms with spec names (§5.14.8); exceptions and coercion order exact; property enumeration order observable and tested.")
        out.append("- [ ] Tests: unit cases + the test262 slice for the object; subclass/species behavior where specified.")
        out.append("")


# --------------------------------------------------------------------------
# §6.8–§6.11 catalogs
# --------------------------------------------------------------------------

def gen_events(out: list[str]) -> None:
    out.append("### §6.8 DOM event catalog")
    out.append("")
    out.append("For each event: dispatch path (§5.6), the interface binding (§6.6), the")
    out.append("default action (if any) and its `preventDefault` behavior, and an")
    out.append("integration test with the scripted input driver (§8.4).")
    out.append("")
    for name, bubbles, cancelable, iface, when in rows(B.DOM_EVENTS):
        out.append(f"- [ ] `{name}` — bubbles: {bubbles}; cancelable: {cancelable}; `{iface}` — fired when: {when}.")
    out.append("")


def gen_network(out: list[str]) -> None:
    out.append("### §6.9 Network and protocol checklist")
    out.append("")
    out.append("Loader-level behaviors (§5.1–§5.3); each item carries a mock-server test.")
    out.append("")
    items = [
        "Scheme handling: `http`, `https`, `file`, `data`, `about:blank`, `about:srcdoc`; unknown scheme → error page.",
        "URL normalization before fetch; fragment stripped on the wire; base resolution for every subresource.",
        "GET pipeline end-to-end: DNS → connect → TLS → request → response, all phases cancelable (§5.2.5).",
        "Redirect chain semantics (301/302/303/307/308) including body dropping and method preservation.",
        "Keep-alive pooling with idle expiry; connection error → one clean retry on a fresh connection.",
        "Chunked body decoding; content-length framing; until-close fallback with a console note.",
        "gzip/deflate content decoding (owned inflate, or the approved crate until it lands).",
        "Conditional revalidation flow (ETag + Last-Modified) against the disk cache.",
        "Vary-keyed cache entries; no-store honored; stale-while-revalidate treated as stale (documented).",
        "Cookie jar read/write on send/response with SameSite + Secure rules (§5.17.1).",
        "CORS simple request, preflight round-trip, credentialed request, wildcard rules (§5.18.3).",
        "Referrer generation per policy, downgrade stripping.",
        "HSTS upgrade-before-connect and policy expiry (§5.3).",
        "Content sniffing table for images and top-level text/html/text/plain ambiguity; nosniff honored.",
        "Timeout matrix per phase; abort mid-body surfaces a clean network error to the pipeline.",
        "HTTP/1.1 protocol violations mapped to NetError::Protocol with the offending bytes logged (debug).",
        "Mock-server harness: per-test server with scripted responses, delays, and truncations (§8.2).",
        "Byte-exactness tests: emitted request lines/headers match the recorded golden bytes.",
        "WebSocket handshake + frame codecs (client side) with the event surface of §6.6.",
        "Non-GET methods (POST/PUT/DELETE/HEAD/OPTIONS) for fetch with body framing rules.",
        "DNS resolution with TTL caching and the hosts-file override for tests.",
        "Connection coalescing guard: two concurrent fetches to one origin use the pool, not two sockets (unless over the 6-connection cap).",
        "Request body streaming from fetch (chunked upload) and its backpressure story.",
        "Response body error injection tests: truncation, invalid chunk size, premature close.",
        "Proxy configuration (env vars) honored at the loader layer with tests via a local proxy harness.",
        "Integrity metadata (SRI) verification for scripts/styles with failure = network error.",
        "DevTools network event emission for every request lifecycle transition (§5.19.5).",
    ]
    # Ticked items (§6.12 rule 1: verified code + green tests + session-log
    # entry). Kept here as indices into `items` so the data stays single-
    # sourced; PROGRESS.md's session log records the evidence per item.
    done = {
        1,  # URL normalization; fragment stripped; base resolution (M1)
        2,  # GET pipeline end-to-end incl. TLS; phases cancelable (M1)
        3,  # Redirect chain semantics for the M1 method set (M1)
        4,  # Keep-alive pooling + one clean retry (M1)
        5,  # Chunked / content-length / until-close framing (M1)
        6,  # gzip/deflate via the approved crate (M1)
        16, # Mock-server harness with delays and truncations (M1)
        17, # Byte-exactness tests vs the recorded golden head (M1)
    }
    for index, it in enumerate(items):
        mark = "x" if index in done else " "
        out.append(f"- [{mark}] {it}")
    out.append("")


def gen_storage(out: list[str]) -> None:
    out.append("### §6.10 Storage and persistence checklist")
    out.append("")
    out.append("Persistence surface (§5.17); every item includes a crash-recovery or")
    out.append("corruption-recovery test.")
    out.append("")
    items = [
        "Cookie jar: RFC 6265bis §5.1–5.6 algorithms, host-only vs domain cookies, path-match, sort order.",
        "Cookie prefixes `__Secure-`/`__Host-` enforced; Secure-only delivery; SameSite default Lax.",
        "Cookie partitioning by top-level site; public-suffix list file loaded and versioned.",
        "document.cookie serialization (one string, semicolon-joined) honoring HttpOnly invisibility.",
        "localStorage per-origin file: write-temp-rename, version byte, CRC per record, load-recovery test.",
        "sessionStorage per-tab lifecycle: cleared on tab close, not shared across tabs, no storage events.",
        "Storage events delivered to other same-origin tabs with old/new values.",
        "Quota enforcement (5 MB UTF-16 units) with QuotaExceededError and a console message.",
        "HTTP disk cache: entry format (headers + body + metadata + version), sharded directories.",
        "Cache freshness math: Age, heuristic freshness, must-revalidate, no-cache revalidation.",
        "LRU eviction on byte budget with pinning for in-flight resources; eviction test with tiny budget.",
        "HSTS store persistence and expiry; security-state versioning (§4.8).",
        "Profile layout `~/.aurora/<profile>/` per §5.17; `--profile` flag; temp profiles in tests.",
        "Clear-browsing-data (cookies, storage, cache) with in-flight navigation safety.",
        "Cookie jar eviction: expired-cookie sweep on load and on a periodic timer.",
        "Storage keying includes the origin's port and scheme (tuple origin, §5.18.1).",
        "Disk-space accounting: stores report their footprint to DevTools (§5.19.5).",
        "Cache checksum-per-record verified on read; corrupt entry evicted, store survives.",
        "sessionStorage survives tab reloads but not tab close (test both).",
        "document.cookie set/delete round-trip through the jar with path-scoped deletion.",
        "Preferences file (shell) versioned, atomically written, hot-reloaded on the settings page.",
    ]
    for it in items:
        out.append(f"- [ ] {it}")
    out.append("")


def gen_keyboard(out: list[str]) -> None:
    out.append("### §6.11 Keyboard and input map")
    out.append("")
    out.append("Input plumbing checks (§5.19.3): platform events → engine input messages →")
    out.append("DOM events with correct `key`/`code`/modifiers. The full key table is")
    out.append("Appendix F; here, the wiring items:")
    out.append("")
    items = [
        "Platform key events mapped through the Appendix F table to KeyboardEvent key/code values.",
        "Modifier liveness: ctrl/shift/alt/meta state correct across focus changes and getModifierState.",
        "Text input funnel: keydown (default-check) → composition (if IME) → beforeinput → input.",
        "Focus navigation via Tab/Shift+Tab over the sequential focus navigation order.",
        "Scroll keys (Space, arrows, PageUp/Down, Home/End) hit the focused scroller or the document.",
        "Shortcut dispatch order: shell shortcuts first, then page keydown handlers (§5.19.1).",
        "Mouse: hit-test → enter/leave pairing → down/up → click synthesis with button/bitmask rules.",
        "Wheel: delta normalization, scroll chaining from innermost scroller outward, listener default action.",
        "Pointer: pointerId assignment, implicit capture for touch, mouse-event synthesis from pointers.",
        "IME: composition event sequence with correct data/isComposing through the editing funnel.",
        "Drag-and-drop of files onto the window routed to the drop event or navigation (§5.19.3).",
        "Cursor and tooltip updates from the hit-test result rendered by the shell.",
        "Repeat-key rate: keydown repeat timing surfaces per platform conventions.",
        "Alt-key menu acceleration does not leak into page key handlers when the shell consumes it.",
        "Zoom (Ctrl+wheel) changes the page zoom factor and re-runs layout, not a bitmap scale.",
        "Text selection drag: mousemove selection updates with shift-extension and double/triple-click word/line selection.",
        "Focus follows click on editable areas with caret placement at the click point.",
    ]
    for it in items:
        out.append(f"- [ ] {it}")
    out.append("")


def gen_milestone_map(out: list[str]) -> None:
    out.append("### §6.13 Milestone → WBS mapping")
    out.append("")
    out.append("Which WBS sections each milestone claims. An item may be *introduced*")
    out.append("in one milestone and *completed* in another — the ledger (§6.12) records")
    out.append("completion; this table records planning intent. `→` marks completion of")
    out.append("work introduced earlier.")
    out.append("")
    out.append("| Milestone | Claims (introduce → complete) |")
    out.append("|---|---|")
    mapping = [
        ("M0 Bootstrap", "— (infrastructure only; no WBS items)"),
        ("M1 Fetch", "§6.9 network items 1–7; Appendices C/D tables land"),
        ("M2 HTML→DOM", "§6.2 all parse/tree tasks; §6.6 DOM-core interfaces; §6.5 selector parsing only"),
        ("M3 Style", "§6.3 properties flagged [M3]; §6.4 at-rules parse+evaluate; §6.5 matching for basic/structural selectors"),
        ("M4 Block layout", "§6.3 box/flex/grid group *parsing* complete; layout integration for box+table groups begins"),
        ("M5 Paint", "§6.3 color/bg/border integration; pixel corpus opens"),
        ("M6 Window", "§6.3 ui-group scroll subset; §6.11 wiring items 1–4 (keys) and 7 (mouse)"),
        ("M7 Text", "§6.3 font+text integration; §6.5 linguistic selectors; Appendix E unit resolution complete"),
        ("M8 Images", "§6.3 image-bearing properties (object-fit, background-image); decoder fuzz targets open"),
        ("M9 JS engine", "§6.7 all [M9] builtin blocks; test262 slices adopted"),
        ("M10 Scriptable DOM", "§6.6 remaining [M10] interfaces; §6.8 all events; §6.11 wiring items 5–6, 8–10"),
        ("M11 Shell", "— (§5.19 surface; §6.11 shell shortcuts; DevTools v1)"),
        ("M12 Storage", "§6.10 all; §6.9 items 8–13 →; §6.5 resource-state selectors"),
        ("M13 Release", "every remaining unchecked item → complete or struck with waiver (§6.12)"),
    ]
    for m, claims in mapping:
        out.append(f"| {m} | {claims} |")
    out.append("")
    out.append("Reading rule for §12.3 task selection: the live claim set of the current")
    out.append("milestone defines which sections' unchecked items are eligible; the")
    out.append("dependency-first override (§12.3) may pull from a later claim set only")
    out.append("when the current section names it as a prerequisite.")


def gen_ledger(out: list[str]) -> None:
    out.append("### §6.12 The completion ledger rules")
    out.append("")
    out.append("1. **Check means verified.** A box is ticked only with: code merged, the")
    out.append("   item's named tests green, and the session report (§11.4) referencing it.")
    out.append("2. **Waivers are explicit.** Anything consciously not done gets `~~struck~~`")
    out.append("   text plus a one-line waiver reason and, if architectural, an ADR. Silent")
    out.append("   gaps are the one dishonesty this project cannot survive.")
    out.append("3. **Regeneration discipline.** This section is generated")
    out.append("   (`tools/generate_wbs.py`); edits go to the data files, never the generated")
    out.append("   markdown. The generator is idempotent: regenerating with unchanged data")
    out.append("   must produce a byte-identical file (CI asserts this).")
    out.append("4. **Progress accounting.** `scripts/wbs-progress.sh` counts checked/total per")
    out.append("   §6.x and writes the percentage into `PROGRESS.md` (§12.1). Milestone exit")
    out.append("   reports quote it.")
    out.append("5. **Ordering is advisory.** Within a section, work top-to-bottom; across")
    out.append("   sections, the milestone's exit criteria (Part 7) choose which sections")
    out.append("   are live. Dependency-first overrides (§12.3) beat document order.")
    out.append("")
    out.append("### §6.14 The progress script specification")
    out.append("")
    out.append("`scripts/wbs-progress.sh` (implemented in the shell of your choice, keep it")
    out.append("dependency-free) produces the numbers quoted in `PROGRESS.md`:")
    out.append("")
    out.append("1. Input: this document (Part 6 only). A WBS item is a line matching")
    out.append("   `^- \\[ \\] ` (open) or `^- \\[x\\] ` (done, case-insensitive on the x).")
    out.append("2. Output: a table of `section | open | done | total | percent` for §6.2")
    out.append("   through §6.13, plus a grand total row, sorted by section number.")
    out.append("3. Struck-through items (`~~...~~`) count in `total` as waived: they are")
    out.append("   excluded from both open and done, and reported in their own column")
    out.append("   so waivers stay visible (§6.12 rule 2).")
    out.append("4. Exit code 0 always; it is a reporting tool, not a gate — the gates")
    out.append("   are the test tiers (§8.8).")
    out.append("5. A `--strict` flag exits non-zero if any §6.x shows done + waived < total")
    out.append("   (used at milestone exits and at M13).")


# --------------------------------------------------------------------------
# Appendices
# --------------------------------------------------------------------------

def gen_appendices(out: list[str]) -> None:
    out.append("## APPENDIX A — CSS Named Colors")
    out.append("")
    out.append("The 148 CSS Color 4 named colors (case-insensitive keywords; the legacy")
    out.append("synonym pairs `gray`/`grey`, `aqua`/`cyan`, `magenta`/`fuchsia`, and the")
    out.append("`darkslategray`-style variants are distinct keywords mapping to equal values).")
    out.append("Table: keyword → RGB hex; implemented as a perfect-hash table in the")
    out.append("color parser (§5.7) with a serialization test per row.")
    out.append("")
    out.append("| Keyword | Hex | Keyword | Hex |")
    out.append("|---|---|---|---|")
    colors = rows(A.CSS_COLORS)
    half = (len(colors) + 1) // 2
    for i in range(half):
        a = colors[i]
        b = colors[i + half] if i + half < len(colors) else ("", "")
        out.append(f"| {a[0]} | #{a[1]} | {b[0]} | #{b[1]} |")
    out.append("")
    out.append("Plus: `transparent` = rgba(0,0,0,0); `currentcolor` resolves from the")
    out.append("`color` property at computed-value time (§5.8.3); system colors")
    out.append("(`canvastext`, `canvas`, `linktext`, `visitedtext`, `buttontext`,")
    out.append("`buttonface`, `buttonborder`, `field`, `fieldtext`, `highlight`,")
    out.append("`highlighttext`, `graytext`, `mark`, `marktext`) map to the default theme.")
    out.append("")
    out.append("## APPENDIX B — HTML Named Character References (common subset)")
    out.append("")
    out.append("Named references the tokenizer's character-reference state must resolve")
    out.append("(§5.4); the full standard table (~2,231 entries) is generated from the")
    out.append("entities JSON at build time — this subset must be hand-verified.")
    out.append("")
    out.append("| Reference | Codepoint | Glyph note |")
    out.append("|---|---|---|")
    for name, cp, note in rows(A.HTML_ENTITIES):
        out.append(f"| &{name}; | U+{int(cp):04X} | {note} |")
    out.append("")
    out.append("Numeric references: decimal `&#NNN;` and hex `&#xHHH;`, with the")
    out.append("windows-1252 remapping for 0x80–0x9F and U+FFFD for surrogates and")
    out.append("out-of-range codepoints. Unmatched named references serialize literally.")
    out.append("")
    out.append("## APPENDIX C — HTTP Header Field Reference")
    out.append("")
    out.append("Engine behavior per header (§5.2); direction is the side the engine")
    out.append("sends or consumes it on. Unknown headers pass through untouched and")
    out.append("appear in the DevTools network view.")
    out.append("")
    out.append("| Header | Direction | Engine behavior |")
    out.append("|---|---|---|")
    for name, direction, behavior in rows(B.HTTP_HEADERS):
        out.append(f"| `{name}` | {direction} | {behavior} |")
    out.append("")
    out.append("## APPENDIX D — MIME Type Table")
    out.append("")
    out.append("Document-type decisions (§5.2.7, §5.5.5); parameters such as `charset`")
    out.append("are honored where the row says so.")
    out.append("")
    out.append("| MIME type | Engine behavior |")
    out.append("|---|---|")
    for mime, behavior in rows(B.MIME_TYPES):
        out.append(f"| `{mime}` | {behavior} |")
    out.append("")
    out.append("## APPENDIX E — CSS Units")
    out.append("")
    out.append("Resolution rules live in §5.8.3 (style time) and §5.9 (layout time).")
    out.append("")
    out.append("| Unit | Kind | Notes |")
    out.append("|---|---|---|")
    for unit, kind, notes in rows(A.CSS_UNITS):
        out.append(f"| `{unit}` | {kind} | {notes} |")
    out.append("")
    out.append("## APPENDIX F — Keyboard Event Map")
    out.append("")
    out.append("The `KeyboardEvent.code`/`key` wiring and shell shortcut table (§5.19.1,")
    out.append("§6.11). Platform layouts may remap `key`; `code` is positional and stable.")
    out.append("")
    out.append("| Key / shortcut | Category | Default action note |")
    out.append("|---|---|---|")
    for key, category, note in rows(B.KEYBOARD_MAP):
        out.append(f"| `{key}` | {category} | {note} |")
    out.append("")
    out.append("## APPENDIX H — Console Message Catalog")
    out.append("")
    out.append("The engine's console-facing messages (§5.15.4). Every message has a")
    out.append("stable id, a level, and a documented trigger; message text may change,")
    out.append("ids may not (tests and DevTools filters key on them). Add new messages")
    out.append("to `tools/wbs_data_b.py` and regenerate.")
    out.append("")
    out.append("| Id | Level | Trigger |")
    out.append("|---|---|---|")
    for msg_id, level, trigger in rows(B.CONSOLE_MESSAGES):
        out.append(f"| `{msg_id}` | {level} | {trigger} |")
    out.append("")


# --------------------------------------------------------------------------

def main() -> None:
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    gen_dir = os.path.join(root, "generated")
    os.makedirs(gen_dir, exist_ok=True)

    wbs: list[str] = [WBS_HEADER, ""]
    gen_html(wbs)
    gen_css(wbs)
    gen_at_rules(wbs)
    gen_selectors(wbs)
    gen_dom(wbs)
    gen_js(wbs)
    gen_events(wbs)
    gen_network(wbs)
    gen_storage(wbs)
    gen_keyboard(wbs)
    gen_milestone_map(wbs)
    gen_ledger(wbs)

    app: list[str] = []
    gen_appendices(app)

    wbs_path = os.path.join(gen_dir, "06_wbs.md")
    app_path = os.path.join(gen_dir, "13_appendices.md")
    with open(wbs_path, "w", encoding="utf-8") as f:
        f.write("\n".join(wbs) + "\n")
    with open(app_path, "w", encoding="utf-8") as f:
        f.write("\n".join(app) + "\n")

    wbs_lines = sum(1 for _ in open(wbs_path, encoding="utf-8"))
    app_lines = sum(1 for _ in open(app_path, encoding="utf-8"))
    print(f"{wbs_path}: {wbs_lines} lines")
    print(f"{app_path}: {app_lines} lines")
    print(f"generated total: {wbs_lines + app_lines} lines")


if __name__ == "__main__":
    main()
