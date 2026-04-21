"""Deterministic grid clock glyphs for YAM snapshot checks.

The live runtime is Go-canonical; this module only mirrors the shared
renderer font file for verification and golden-frame checks.
"""

from __future__ import annotations

import ast
import re
from pathlib import Path

Glyph = tuple[str, ...]

def _parse_font_file(path: Path) -> dict[str, Glyph]:
    raw = path.read_text(encoding="utf-8")
    font: dict[str, Glyph] = {}
    for block in raw.split("\n\n"):
        block = block.strip()
        if not block:
            continue
        header, *rows = block.splitlines()
        key = header.split("'", 2)[1]
        glyph_rows = []
        for row in rows:
            match = re.search(r'"(.*)"', row)
            if match is None:
                continue
            glyph_rows.append(ast.literal_eval(match.group(0)))
        if glyph_rows:
            font[key] = tuple(glyph_rows)
    return font


_FONT_PATH = Path(__file__).resolve().parent / "fonts" / "go_deco.txt"
BREAM_DECO_FONT: dict[str, Glyph] = _parse_font_file(_FONT_PATH)


def render_clock(text: str) -> str:
    rows = [""] * 7
    for ch in text:
        glyph = BREAM_DECO_FONT.get(ch)
        if glyph is None:
            continue
        for i in range(7):
            rows[i] += glyph[i]
    return "\n".join(rows)
