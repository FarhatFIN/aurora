# MASTER PROMPT — Build a Web Browser Engine and Browser From Scratch

**Codename:** AURORA (you may rename it; if you do, rename it consistently everywhere in the first session and never again).
**Audience:** an autonomous AI software engineer (or a human team) with full authority over a fresh, empty repository.
**Document class:** project constitution + full technical specification + work breakdown structure + session protocol.
**Length:** ~10,000 lines. This is deliberate. Read the consumption rules in Part 0 before starting.

---

## PART 0 — How to Use This Prompt

### §0.1 What this document is

This is not a sketch. It is the complete operating contract for building a web browser
engine and a desktop browser application from first principles. It contains:

1. **Mission and scope** (Part 1) — what you are building and, just as important, what you are NOT building.
2. **Working method** (Part 2) — how to make decisions, in what order, and how to verify yourself.
3. **Technology policy** (Part 3) — the language, dependencies, and tooling decisions, already made so you do not waste sessions relitigating them.
4. **System architecture** (Part 4) — the crate map, data flow, threading model, and the contracts between subsystems.
5. **Subsystem specifications** (Part 5) — one section per subsystem: goals, public API sketches, algorithms, invariants, pitfalls, and definitions of done.
6. **Work breakdown structure** (Part 6) — the exhaustive, itemized task checklist: every HTML element, every CSS property, every DOM interface, every JavaScript builtin, each with its own acceptance criteria. This is the part you tick off, item by item, over many sessions.
7. **Milestones** (Part 7) — fourteen milestones, each ending in a demonstrable, tested artifact.
8. **Testing strategy** (Part 8) — the test pyramid, golden files, pixel tests, and Web Platform Tests adoption.
9. **Code quality standard** (Part 9) — naming, error handling, forbidden patterns, review checklist.
10. **Performance budgets** (Part 10) — numeric limits you must respect and measure.
11. **Deliverables and reporting** (Part 11) — what exists in the repository when you are done, and what every session report must contain.
12. **Session protocol** (Part 12) — how to survive context loss, how to resume, and how to keep a durable progress ledger.
13. **Appendices** (A–F) — reference tables you will consult constantly: named colors, character references, HTTP headers, MIME types, CSS units, keyboard maps.

### §0.2 Who it is for

It is written for an AI agent that can: create and edit files, run shell commands,
compile and test code, read standards documents online or offline, and persist state
between sessions. It assumes no human is watching in real time. It assumes the agent
may be killed at any moment and must therefore leave the repository in a state where
any successor session can resume exactly where it stopped.

### §0.3 How to consume 10,000 lines without drowning

You will not hold this document in working memory. Follow this ritual:

- **First session, once:** read Parts 0–4 and Part 7 fully (roughly 2,500 lines). Skim Part 5. You now know the shape of the machine.
- **Every session after:** read Part 12 (session protocol, short), the current milestone section in Part 7, the subsystem spec for whatever you are working on, and the relevant slice of Part 6. That is 300–800 lines per session, not 10,000.
- **On demand:** treat Part 6 and the Appendices as lookup tables, like documentation. Open the exact entries you need; ignore the rest.
- **Never** let "I haven't read all of it" become an excuse to improvise architecture. If you are unsure whether a decision is covered, search this file first (`grep -n "§" | grep -i <keyword>`), then decide, then record the decision in an ADR (§4.9) so the next session inherits it.

### §0.4 Precedence and conflict resolution

When instructions conflict, resolve in this order:

1. **Standards win.** WHATWG HTML, WHATWG URL, WHATWG Fetch, CSSWG specs, and ECMA-262 are the ground truth for *behavior*. This document is the ground truth for *architecture, process, scope, and quality*. If this document contradicts a standard on behavior, follow the standard, then open an ADR noting the discrepancy and propose a fix to this document.
2. **This document wins** over your prior habits, over brevity, and over convenience.
3. **Newer explicit user instruction** wins over everything; when it arrives, update `PROGRESS.md` and, if durable, this document.
4. **The WBS (Part 6) is a work-aid, not a spec.** Its per-item notes summarize behavior; where a summary and the standard disagree, the standard is right. Flag systematic errors you find — do not silently copy them into code.

---
