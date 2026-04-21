"""Deterministic grid clock glyphs for the YAM observer layer."""

from __future__ import annotations

Glyph = tuple[str, ...]

BREAM_DECO_FONT: dict[str, Glyph] = {
    "0": (
        " --- ",
        "|   |",
        "|   |",
        "|   |",
        "|   |",
        "|   |",
        " --- ",
    ),
    "1": (
        "  |  ",
        "  |  ",
        "  |  ",
        "  |  ",
        "  |  ",
        "  |  ",
        "  |  ",
    ),
    "2": (
        " --- ",
        "    |",
        "    |",
        " --- ",
        "|    ",
        "|    ",
        " --- ",
    ),
    "3": (
        " --- ",
        "    |",
        "    |",
        " --- ",
        "    |",
        "    |",
        " --- ",
    ),
    "4": (
        "|   |",
        "|   |",
        "|   |",
        " --- ",
        "    |",
        "    |",
        "    |",
    ),
    "5": (
        " --- ",
        "|    ",
        "|    ",
        " --- ",
        "    |",
        "    |",
        " --- ",
    ),
    "6": (
        " --- ",
        "|    ",
        "|    ",
        " --- ",
        "|   |",
        "|   |",
        " --- ",
    ),
    "7": (
        " --- ",
        "    |",
        "    |",
        "    |",
        "    |",
        "    |",
        "    |",
    ),
    "8": (
        " --- ",
        "|   |",
        "|   |",
        " --- ",
        "|   |",
        "|   |",
        " --- ",
    ),
    "9": (
        " --- ",
        "|   |",
        "|   |",
        " --- ",
        "    |",
        "    |",
        " --- ",
    ),
    ":": (
        "     ",
        "  |  ",
        "     ",
        "     ",
        "  |  ",
        "     ",
        "     ",
    ),
}


def render_clock(text: str) -> str:
    rows = [""] * 7
    for ch in text:
        glyph = BREAM_DECO_FONT.get(ch)
        if glyph is None:
            continue
        for i in range(7):
            rows[i] += glyph[i] + " "
    return "\n".join(rows)
