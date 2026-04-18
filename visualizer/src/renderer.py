from __future__ import annotations

from dataclasses import dataclass

from layout import SceneLayout
from render_field import RenderField
from terminal import RESET, TerminalSize
from tree_scaffold import build_tree_scaffold


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
    config: dict | None = None,
    falling_leaf_segments: dict[tuple[int, int], str] | None = None,
    debug_enabled: bool = False,
) -> str:
    field = RenderField(size.columns, size.rows)

    # Static scaffold layer (wood structure) – rendered before vines so vines can overgrow it
    scaffold_config = config or {}
    scaffold = build_tree_scaffold(layout, scaffold_config)
    hero_mask = layout.hero_raw_mask_cells
    for cell in scaffold.cells:
        if (cell.x, cell.y) in hero_mask:
            continue
        field.add(
            cell.x,
            cell.y,
            density=cell.density,
            priority=10,
            glyph=cell.glyph,
            style=cell.style,
        )

    # Base growth layer
    smoothed_segments = _smooth_vine_segments(vine_segments)
    for (x, y), glyph in smoothed_segments.items():
        if 0 <= x < size.columns and 0 <= y < size.rows:
            field.add(
                x,
                y,
                density=_segment_density(glyph),
                priority=20,
                glyph=glyph,
            )

    rows = field.to_rows()

    blocks = [
        PositionedBlock(layout.hero.x, layout.hero.y, hero_lines),
        PositionedBlock(layout.info.x, layout.info.y, panel_lines),
    ]
    for block in blocks:
        _stamp_block(rows, size, block)

    if debug_enabled:
        _stamp_mask_dots(rows, size, layout.hero_raw_mask_cells, DEBUG_MASK_BOX)
        # Debug: trunk mask visualization (bright cyan dots)
        trunk_cells = getattr(layout, "trunk_mask_cells", None)
        if trunk_cells:
            _stamp_mask_dots(rows, size, trunk_cells, "\x1b[38;5;51m")
        _stamp_rect_frame(rows, size, layout.no_go_zones[1], DEBUG_MASK_BOX)
        _stamp_rect_frame(rows, size, layout.hero, DEBUG_VISIBLE_BOX)
        _stamp_rect_frame(rows, size, layout.info, DEBUG_VISIBLE_BOX)

    # Topmost particle overlay, but keep UI and hero pixels readable.
    if falling_leaf_segments:
        panel = layout.info
        hero_mask = layout.hero_raw_mask_cells
        for (x, y), glyph in falling_leaf_segments.items():
            if not (0 <= x < size.columns and 0 <= y < size.rows):
                continue

            # Do not draw particles over the info panel.
            if (
                panel.x <= x < panel.x + panel.width
                and panel.y <= y < panel.y + panel.height
            ):
                continue

            # Keep particles off the visible hero pixels so the focal point
            # stays clean while allowing overlap around the silhouette.
            if (x, y) in hero_mask:
                continue

            rows[y][x] = glyph

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


def _segment_density(glyph: str) -> float:
    text, _ = _split_cell_style(glyph)
    if not text or text.isspace() or text == "⠀":
        return 0.0
    if text in HEAVY_GLYPHS:
        return 0.88
    if text in MID_GLYPHS:
        return 0.64
    if text in THIN_GLYPHS:
        return 0.42
    if text in {"|", "/", "\\", "┆", "╌", "─", "│", "╱", "╲", "═", "║"}:
        return 0.58
    if text in {"*", "+", "·", "•", "◦", "o", "O"}:
        return 0.48
    return 0.5


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

            if _is_transparent_cell(glyph):
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


def _is_transparent_cell(cell: str) -> bool:
    """Return True when a terminal cell carries no visible foreground glyph.

    Chafa can emit styled ASCII spaces as well as the Unicode braille blank
    character (U+2800). Both should behave like transparency inside the hero
    block so vines remain visible anywhere the hero art is visually empty.
    """
    text, _ = _split_cell_style(cell)
    return text == "" or text.isspace() or text == "⠀"


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
