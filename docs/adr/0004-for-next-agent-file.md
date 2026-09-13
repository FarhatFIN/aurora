# ADR-0004: FOR-NEXT-AGENT.txt is part of the repository layout

- **Status:** accepted (M2, first session)
- **Context:** §11.1 fixes the repository layout and forbids files outside
  it without an ADR. The project's owner (a human directing multiple AI
  sessions, possibly with different models) requires a durable,
  model-agnostic handoff hint at the repo root that any successor session
  reads first. §12.6 already defines the handoff package (master document +
  `PROGRESS.md` + ADRs); the new file is a thin pointer and protocol
  reminder on top of it, not a second ledger.
- **Decision:** `FOR-NEXT-AGENT.txt` lives at the repository root. It
  points at the three sources of truth, restates the session protocol and
  task-selection rules, lists the environment notes, and carries a
  "CURRENT STATE" block that is refreshed at the end of every working
  session. `PROGRESS.md` remains the single source of truth for state; in
  a conflict, `PROGRESS.md` wins. The file is referenced from
  `PROGRESS.md`'s decision log.
- **Consequences:** one more root file to keep fresh; the session report
  template gains the standing instruction "update FOR-NEXT-AGENT.txt's
  CURRENT STATE block". No other layout changes.
