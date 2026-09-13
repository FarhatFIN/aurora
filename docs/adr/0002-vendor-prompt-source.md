# ADR-0002: The prompt source is vendored in the repository

- **Status:** accepted (M0, first session)
- **Context:** §12.1 keeps `BROWSER_ENGINE_PROMPT.md` at the repo root as a
  living file — its Part 6 WBS checkboxes are ticked in the session that
  completes each item. §6.1 and §6.12 rule 3 require those ticks to be made in
  the WBS *data files* (`tools/wbs_data_*.py`) followed by regeneration
  (`python3 tools/generate_wbs.py`), never by hand-editing generated text, and
  CI must assert byte-identical regeneration. §11.1's repository layout omits
  `parts/`, `generated/`, and `tools/`, which would make the required
  discipline impossible inside this repository.
- **Decision:** the prompt assembly sources are vendored at the repo root:
  `parts/` (hand-written sections), `generated/` (WBS + appendices), `tools/`
  (`generate_wbs.py`, `build_prompt.py`, `wbs_data_a.py`, `wbs_data_b.py`),
  alongside the assembled `BROWSER_ENGINE_PROMPT.md`. `scripts/test-all.sh`
  regenerates and fails on any drift (the §6.12 CI assert).
- **Consequences:** WBS ticks follow: edit `tools/wbs_data_*.py` → run
  `python3 tools/build_prompt.py` → review the diff → commit (message prefix
  `bless:` is reserved for golden *test* files; WBS ticks carry the WBS
  reference in the session report). The §11.1 layout is amended by this ADR;
  no other files may appear outside it without one.
