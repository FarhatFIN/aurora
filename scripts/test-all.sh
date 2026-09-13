#!/usr/bin/env bash
# Full test tier (§8.8): everything the fast tier runs, plus the dependency
# policy check, the WBS progress report, and the byte-identical regeneration
# assert for the WBS and the assembled prompt (§6.12 rule 3). Vendor corpora
# and the fuzz smoke join here as they are adopted (M2+, M8+).
set -euo pipefail
cd "$(dirname "$0")/.."

scripts/test-fast.sh
scripts/check-deps.sh

echo "== WBS progress =="
scripts/wbs-progress.sh

echo "== WBS/prompt regeneration is byte-identical =="
python3 tools/generate_wbs.py > /dev/null
python3 tools/build_prompt.py > /dev/null
git diff --exit-code -- generated/ BROWSER_ENGINE_PROMPT.md || {
    echo "FAIL: regeneration changed generated/ or BROWSER_ENGINE_PROMPT.md." >&2
    echo "Edit tools/wbs_data_*.py (never the generated text), review the diff, commit." >&2
    exit 1
}

echo "== full tier green =="
