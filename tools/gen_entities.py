#!/usr/bin/env python3
"""Regenerates crates/aurora_html/src/tables.rs from the WHATWG entities
JSON (§9.2 tables convention: source + refresh procedure live here)."""
import json
import pathlib
import urllib.request

SOURCE = "https://html.spec.whatwg.org/entities.json"
TARGET = pathlib.Path(__file__).resolve().parent.parent / "crates" / "aurora_html" / "src" / "tables.rs"

def rust_char(c: str) -> str:
    if c == "'":
        return "'\\''"
    if c == "\\":
        return "'\\\\'"
    code = ord(c)
    if code < 0x20 or code > 0x7E:
        return f"'\\u{{{code:X}}}'"
    return f"'{c}'"

def main() -> None:
    with urllib.request.urlopen(SOURCE) as response:
        data = json.load(response)
    entities = sorted(
        (name, info["codepoints"])
        for name, info in data.items()
        if name != "legacy" and isinstance(info, dict) and "codepoints" in info
    )
    lines = [
        "//! Named character references (WHATWG HTML 13.2.5). GENERATED FILE —",
        "//! do not edit by hand (9.2 tables convention).",
        "//!",
        "//! Source: <https://html.spec.whatwg.org/entities.json> (fetched 2026-09-14).",
        "//! Regenerate: `python3 tools/gen_entities.py` (writes this file).",
        "",
        "/// `(name, codepoints)` for every named reference, sorted by name. Names",
        "/// include both the `;`-terminated and the legacy no-semicolon forms;",
        "/// the tokenizer matches the longest prefix.",
        "pub static NAMED_REFERENCES: &[(&str, &[char])] = &[",
    ]
    for name, points in entities:
        chars = ", ".join(rust_char(chr(c)) for c in points)
        lines.append(f'    ("{name}", &[{chars}]),')
    lines.append("];")
    TARGET.write_text("\n".join(lines) + "\n")
    print(f"wrote {TARGET} ({len(entities)} entries)")

if __name__ == "__main__":
    main()
