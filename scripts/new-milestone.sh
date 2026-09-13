#!/usr/bin/env bash
# Milestone switch (§7.1): points PROGRESS.md's current-state line at the new
# milestone. Exit criteria live in BROWSER_ENGINE_PROMPT.md Part 7; the first
# session of the new milestone quotes them in its session report.
set -euo pipefail
cd "$(dirname "$0")/.."

MILESTONE="${1:?usage: new-milestone.sh M# \"name\"}"
NAME="${2:?usage: new-milestone.sh M# \"name\"}"

[ -f PROGRESS.md ] || { echo "PROGRESS.md not found" >&2; exit 1; }
sed -i "s|^- Milestone: .*|- Milestone: ${MILESTONE} — ${NAME}|" PROGRESS.md
echo "PROGRESS.md now points at ${MILESTONE} — ${NAME}."
echo "Do not start it with the previous milestone's exit criteria unverified (§7.1)."
