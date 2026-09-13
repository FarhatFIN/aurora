#!/usr/bin/env python3
"""Assemble BROWSER_ENGINE_PROMPT.md from parts/ and generated/.

Order matters: Part 6 (generated WBS) sits between Part 5 and Part 7; the
hand-written Appendix G is inserted between the generated appendices F and H
so the appendix lettering stays sequential (A…O).
"""
from __future__ import annotations

import os
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

ORDER = [
    "parts/00_frontmatter_toc.md",
    "parts/01_toc.md",
    "parts/02_mission.md",
    "parts/03_principles.md",
    "parts/04_technology.md",
    "parts/05_architecture.md",
    "parts/06a_subsystems_infra.md",
    "parts/06b_subsystems_documents.md",
    "parts/06c_subsystems_scripting.md",
    "parts/06d_subsystems_output_shell.md",
    "generated/06_wbs.md",
    "parts/07_milestones.md",
    "parts/08_testing.md",
    "parts/09_quality.md",
    "parts/10_11_12_ops.md",
    "parts/11_runbooks.md",
    "generated:appendices_a_f",        # appendices A–F (generated)
    "parts/12_appendix_g_ua.md",       # appendix G (hand-written UA stylesheet)
    "generated:appendices_h_onwards",  # appendix H (generated)
    "parts/13_appendix_ijk.md",
    "parts/14_appendix_lmn.md",
    "parts/15_appendix_o_glossary.md",
]


def split_appendices() -> tuple[list[str], list[str]]:
    path = os.path.join(ROOT, "generated", "13_appendices.md")
    text = open(path, encoding="utf-8").read().splitlines()
    marker = "## APPENDIX H"
    idx = next(i for i, line in enumerate(text) if line.startswith(marker))
    return text[:idx], text[idx:]


def main() -> None:
    subprocess.run([sys.executable, os.path.join(ROOT, "tools", "generate_wbs.py")],
                   check=True, stdout=subprocess.DEVNULL)
    head, tail = split_appendices()

    out: list[str] = []
    for entry in ORDER:
        if entry == "generated:appendices_a_f":
            out.extend(head)
        elif entry == "generated:appendices_h_onwards":
            out.extend(tail)
        else:
            part = open(os.path.join(ROOT, entry), encoding="utf-8").read().rstrip("\n")
            out.append(part)
        out.append("")  # one blank line between top-level files

    target = os.path.join(ROOT, "BROWSER_ENGINE_PROMPT.md")
    with open(target, "w", encoding="utf-8") as f:
        f.write("\n".join(line.rstrip() for line in out).rstrip("\n") + "\n")

    lines = sum(1 for _ in open(target, encoding="utf-8"))
    print(f"{target}: {lines} lines")


if __name__ == "__main__":
    main()
