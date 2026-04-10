from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass

from layout import SceneLayout
from terminal import RESET, TerminalSize


AMBIENT = "\x1b[38;5;238m"


@dataclass
class PositionedBlock:
    x: int
    y: int
    lines: list[str]


def _blank_row(width: int) -> str:
    return " " * width


def compose_scene(
    size: TerminalSize,
    layout: SceneLayout,
    hero_lines: list[str],
    ivy_segments: dict[tuple[int, int], str],
    panel_lines: list[str],
) -> str:
    row_segments: dict[int, list[tuple[int, str]]] = defaultdict(list)
    blocks = [
        PositionedBlock(layout.hero.x, layout.hero.y, hero_lines),
        PositionedBlock(layout.info.x, layout.info.y, panel_lines),
    ]

    for (x, y), glyph in ivy_segments.items():
        if 0 <= y < size.rows and 0 <= x < size.columns:
            row_segments[y].append((x, glyph))

    for block in sorted(blocks, key=lambda item: (item.y, item.x, len(item.lines))):
        _stamp_block(row_segments, size, block)

    base_rows = [_compose_row(size.columns, row_segments.get(row, [])) for row in range(size.rows)]
    if layout.warning:
        warning = f"{AMBIENT}{layout.warning.center(size.columns)}{RESET}"
        if size.rows > 1:
            base_rows[1] = warning

    return "\n".join(base_rows)


def _stamp_block(
    row_segments: dict[int, list[tuple[int, str]]],
    size: TerminalSize,
    block: PositionedBlock,
) -> None:
    for line_index, content in enumerate(block.lines):
        row_index = block.y + line_index
        if row_index < 0 or row_index >= size.rows:
            continue
        if block.x >= size.columns:
            continue
        row_segments[row_index].append((block.x, content))


def _compose_row(width: int, segments: list[tuple[int, str]]) -> str:
    if not segments:
        return _blank_row(width)

    cursor = 0
    parts: list[str] = []
    for x, content in sorted(segments, key=lambda item: item[0]):
        if x > cursor:
            parts.append(" " * (x - cursor))
            cursor = x
        parts.append(content)
        cursor = x + len(_strip_ansi(content))
    return "".join(parts)


def _strip_ansi(text: str) -> str:
    result: list[str] = []
    in_escape = False
    for char in text:
        if in_escape:
            if char.isalpha():
                in_escape = False
            continue
        if char == "\x1b":
            in_escape = True
            continue
        result.append(char)
    return "".join(result)
