#!/usr/bin/env bash
# WBS progress report (§6.14). Reads Part 6 of BROWSER_ENGINE_PROMPT.md at the
# repo root and prints `section | open | done | waived | total | percent`,
# plus a grand total row. Reporting tool only; the gates are the test tiers.
#
#   - open:   `- [ ]` items not struck through
#   - done:   `- [x]` items (case-insensitive x)
#   - waived: checkbox items struck through with ~~...~~ (§6.12 rule 2)
#   - percent: done / (total - waived), so 100% means "checked or waived"
#
# --strict: exit non-zero if any §6.x has done + waived < total (milestone
# exits and M13 use this).
set -euo pipefail
cd "$(dirname "$0")/.."

DOC=BROWSER_ENGINE_PROMPT.md
[ -f "$DOC" ] || { echo "missing $DOC" >&2; exit 1; }

awk -v strict="${1:-}" '
    /^## PART 7/ { in_wbs = 0 }
    /^### §6\.[0-9]+/ {
        in_wbs = 1
        sec = $2
        sub(/^§/, "", sec)
        if (!(sec in seen)) { seen[sec] = 1; order[++n] = sec }
    }
    in_wbs && /^- \[[xX]\] / {
        done[sec]++; total[sec]++
        if ($0 ~ /~~/) waived[sec]++
    }
    in_wbs && /^- \[ \] / {
        total[sec]++
        if ($0 ~ /~~/) waived[sec]++; else open[sec]++
    }
    END {
        printf "%-8s %6s %6s %8s %7s %9s\n", "section", "open", "done", "waived", "total", "percent"
        to = do_ = wa = op = 0; violations = 0
        for (i = 1; i <= n; i++) {
            s = order[i]
            t = total[s] + 0; d = done[s] + 0; w = waived[s] + 0; o = open[s] + 0
            pct = (t - w > 0) ? int(d * 100 / (t - w) + 0.5) : 100
            printf "%-8s %6d %6d %8d %7d %8d%%\n", s, o, d, w, t, pct
            to += t; do_ += d; wa += w; op += o
            if (d + w < t) violations++
        }
        tpct = (to - wa > 0) ? int(do_ * 100 / (to - wa) + 0.5) : 100
        printf "%-8s %6d %6d %8d %7d %8d%%\n", "TOTAL", op, do_, wa, to, tpct
        if (strict == "--strict" && violations > 0) {
            print strict ": " violations " section(s) with done + waived < total" > "/dev/stderr"
            exit 1
        }
    }
' "$DOC"
