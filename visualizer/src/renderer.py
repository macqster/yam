from __future__ import annotations

from dataclasses import dataclass

from layout import SceneLayout
from terminal import RESET, TerminalSize


AMBIENT = "\x1b[38;5;238m"


@dataclass
class PositionedBlock:
    x: int
    y: int
    lines: list[str]


def compose_scene(
    size: TerminalSize,
    layout: SceneLayout,
    hero_lines: list[str],
    ivy_segments: dict[tuple[int, int], str],
    panel_lines: list[str],
) -> str:
    rows: list[list[str]] = [[" " for _ in range(size.columns)] for _ in range(size.rows)]

    for (x, y), glyph in ivy_segments.items():
        if 0 <= x < size.columns and 0 <= y < size.rows:
            rows[y][x] = glyph

    blocks = [
        PositionedBlock(layout.hero.x, layout.hero.y, hero_lines),
        PositionedBlock(layout.info.x, layout.info.y, panel_lines),
    ]
    for block in blocks:
        _stamp_block(rows, size, block)

    base_rows = [_compose_row(row) for row in rows]
    if layout.warning and size.rows > 1:
        base_rows[1] = f"{AMBIENT}{layout.warning.center(size.columns)}{RESET}"
    return "\n".join(base_rows)


def _stamp_block(rows: list[list[str]], size: TerminalSize, block: PositionedBlock) -> None:
    for line_index, content in enumerate(block.lines):
        row_index = block.y + line_index
        if row_index < 0 or row_index >= size.rows:
            continue
        if block.x >= size.columns:
            continue

        for offset, glyph in enumerate(_tokenize_ansi_cells(content)):
            column = block.x + offset
            if column < 0 or column >= size.columns:
                continue
            rows[row_index][column] = glyph


def _compose_row(cells: list[str]) -> str:
    parts: list[str] = []
    active_style = ""
    for cell in cells:
        text, style = _split_cell_style(cell)
        if style != active_style:
            if active_style and not style:
                parts.append(RESET)
            elif style:
                parts.append(style)
            active_style = style
        parts.append(text)
    if active_style:
        parts.append(RESET)
    return "".join(parts)


def _tokenize_ansi_cells(text: str) -> list[str]:
    tokens: list[str] = []
    style = ""
    index = 0
    length = len(text)
    while index < length:
        char = text[index]
        if char == "\x1b":
            end = index + 1
            while end < length and not text[end].isalpha():
                end += 1
            if end < length:
                end += 1
            sequence = text[index:end]
            style = "" if sequence == RESET else sequence
            index = end
            continue
        tokens.append(f"{style}{char}{RESET}" if style else char)
        index += 1
    return tokens


def _split_cell_style(cell: str) -> tuple[str, str]:
    if not cell.startswith("\x1b"):
        return cell, ""

    index = 0
    style_parts: list[str] = []
    while index < len(cell) and cell[index] == "\x1b":
        end = index + 1
        while end < len(cell) and not cell[end].isalpha():
            end += 1
        if end < len(cell):
            end += 1
        sequence = cell[index:end]
        if sequence != RESET:
            style_parts.append(sequence)
        index = end
        if index < len(cell) and cell[index] != "\x1b":
            break

    text_end = cell.find("\x1b", index)
    if text_end == -1:
        text_end = len(cell)
    return cell[index:text_end], "".join(style_parts)
