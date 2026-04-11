from __future__ import annotations

from dataclasses import dataclass

from layout import SceneLayout
from terminal import RESET, TerminalSize


AMBIENT = "\x1b[38;5;238m"
DEBUG_VISIBLE_BOX = "\x1b[38;5;117m"
DEBUG_MASK_BOX = "\x1b[38;5;250m"

THIN_GLYPHS = {"·", ":"}
MID_GLYPHS = {"┆", "╌"}
HEAVY_GLYPHS = {"=", "║"}


@dataclass
class PositionedBlock:
    x: int
    y: int
    lines: list[str]


def compose_scene(
    size: TerminalSize,
    layout: SceneLayout,
    hero_lines: list[str],
    vine_segments: dict[tuple[int, int], str],
    panel_lines: list[str],
    debug_enabled: bool = False,
) -> str:
    rows: list[list[str]] = [[" " for _ in range(size.columns)] for _ in range(size.rows)]

    # Base growth layer first; protected scene objects are stamped later so
    # hero and panel always remain readable.
    smoothed_segments = _smooth_vine_segments(vine_segments)
    for (x, y), glyph in smoothed_segments.items():
        if 0 <= x < size.columns and 0 <= y < size.rows:
            rows[y][x] = glyph

    # Future leaf/flower/particle overlays should be composed after the base
    # vine layer and before the final protection/debug pass.
    blocks = [
        PositionedBlock(layout.hero.x, layout.hero.y, hero_lines),
        PositionedBlock(layout.info.x, layout.info.y, panel_lines),
    ]
    for block in blocks:
        _stamp_block(rows, size, block)

    if debug_enabled:
        _stamp_mask_dots(rows, size, layout.hero_raw_mask_cells, DEBUG_MASK_BOX)
        _stamp_rect_frame(rows, size, layout.no_go_zones[1], DEBUG_MASK_BOX)
        _stamp_rect_frame(rows, size, layout.hero, DEBUG_VISIBLE_BOX)
        _stamp_rect_frame(rows, size, layout.info, DEBUG_VISIBLE_BOX)

    base_rows = [_compose_row(row) for row in rows]
    if layout.warning and size.rows > 1:
        base_rows[1] = f"{AMBIENT}{layout.warning.center(size.columns)}{RESET}"
    return "\n".join(base_rows)


def _smooth_vine_segments(vine_segments: dict[tuple[int, int], str]) -> dict[tuple[int, int], str]:
    def neighbor_count(x: int, y: int) -> int:
        return sum(
            (nx, ny) in vine_segments
            for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1))
        )

    smoothed: dict[tuple[int, int], str] = {}
    for (x, y), glyph in vine_segments.items():
        neighbors = neighbor_count(x, y)

        if glyph in HEAVY_GLYPHS and neighbors <= 1:
            smoothed[(x, y)] = "┆"
        elif glyph in THIN_GLYPHS and neighbors >= 3:
            smoothed[(x, y)] = "┆"
        else:
            smoothed[(x, y)] = glyph

    return smoothed


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
            if glyph == " ":
                continue
            rows[row_index][column] = glyph


def _stamp_rect_frame(rows: list[list[str]], size: TerminalSize, rect, style: str) -> None:
    width = rect.width
    height = rect.height
    if width <= 0 or height <= 0:
        return

    left = rect.x
    right = rect.x + width - 1
    top = rect.y
    bottom = rect.y + height - 1

    _set_debug_cell(rows, size, left, top, "┌", style)
    _set_debug_cell(rows, size, right, top, "┐", style)
    _set_debug_cell(rows, size, left, bottom, "└", style)
    _set_debug_cell(rows, size, right, bottom, "┘", style)

    for x in range(left + 1, right):
        _set_debug_cell(rows, size, x, top, "─", style)
        _set_debug_cell(rows, size, x, bottom, "─", style)
    for y in range(top + 1, bottom):
        _set_debug_cell(rows, size, left, y, "│", style)
        _set_debug_cell(rows, size, right, y, "│", style)


def _set_debug_cell(rows: list[list[str]], size: TerminalSize, x: int, y: int, glyph: str, style: str) -> None:
    if 0 <= x < size.columns and 0 <= y < size.rows:
        rows[y][x] = f"{style}{glyph}{RESET}"


def _stamp_mask_dots(
    rows: list[list[str]],
    size: TerminalSize,
    mask_cells: frozenset[tuple[int, int]],
    style: str,
) -> None:
    if not mask_cells:
        return

    for x, y in mask_cells:
        if not (0 <= x < size.columns and 0 <= y < size.rows):
            continue
        rows[y][x] = f"{style}·{RESET}"


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
