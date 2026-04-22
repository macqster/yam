"""Framebuffer implementation for the current YAM runtime."""

from __future__ import annotations

from dataclasses import dataclass

from render.masks import Mask


@dataclass(slots=True)
class Cell:
    """Single terminal cell."""

    glyph: str = " "
    z: int = -10**9


class Framebuffer:
    """2D visual truth buffer."""

    def __init__(self, width: int, height: int) -> None:
        self.width = width
        self.height = height
        self.cells = [[Cell() for _ in range(width)] for _ in range(height)]

    def clear(self) -> None:
        for row in self.cells:
            for cell in row:
                cell.glyph = " "
                cell.z = -10**9

    def write(self, x: int, y: int, glyph: str, z: int = 0, mask: Mask | None = None) -> None:
        if not (0 <= x < self.width and 0 <= y < self.height):
            return
        if mask is not None and not mask.allows(x, y):
            return
        cell = self.cells[y][x]
        if z >= cell.z:
            cell.glyph = glyph
            cell.z = z

    def rows(self) -> list[str]:
        return ["".join(cell.glyph for cell in row) for row in self.cells]
