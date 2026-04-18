from __future__ import annotations

from dataclasses import dataclass

from terminal import RESET


BRAILLE_LEVELS = [
    "⠀",
    "⠁",
    "⠃",
    "⠇",
    "⡇",
    "⡟",
    "⡿",
    "⣿",
]

@dataclass(frozen=True)
class RenderCell:
    density: float
    color: tuple[int, int, int] | None = None
    priority: int = 0
    glyph: str = ""
    style: str = ""


class RenderField:
    def __init__(self, width: int, height: int) -> None:
        self.width = width
        self.height = height
        self._cells: dict[tuple[int, int], RenderCell] = {}

    def add(
        self,
        x: int,
        y: int,
        *,
        density: float,
        priority: int,
        glyph: str = "",
        color: tuple[int, int, int] | None = None,
        style: str = "",
    ) -> None:
        if not (0 <= x < self.width and 0 <= y < self.height):
            return

        existing = self._cells.get((x, y))
        if existing is not None:
            if priority < existing.priority:
                return
            if priority == existing.priority and density <= existing.density:
                return

        self._cells[(x, y)] = RenderCell(
            density=max(0.0, min(1.0, density)),
            color=color if color is not None else (existing.color if existing else None),
            priority=priority,
            glyph=glyph if glyph else (existing.glyph if existing else ""),
            style=style if style else (existing.style if existing else ""),
        )

    def to_rows(self) -> list[list[str]]:
        rows = [[" " for _ in range(self.width)] for _ in range(self.height)]
        for (x, y), cell in self._cells.items():
            glyph = glyph_for_density(cell.density, cell.glyph)
            if glyph == " ":
                continue

            style = cell.style
            if not style and cell.color is not None:
                r, g, b = cell.color
                style = f"\x1b[38;2;{r};{g};{b}m"

            if style:
                rows[y][x] = f"{style}{glyph}{RESET}"
            else:
                rows[y][x] = glyph
        return rows


def density_to_braille(density: float) -> str:
    normalized = max(0.0, min(1.0, density))
    index = min(len(BRAILLE_LEVELS) - 1, int(round(normalized * (len(BRAILLE_LEVELS) - 1))))
    return BRAILLE_LEVELS[index]


def glyph_for_density(density: float, glyph_hint: str = "") -> str:
    if glyph_hint and not glyph_hint.isspace() and glyph_hint != "⠀":
        return glyph_hint
    return density_to_braille(density)
