"""Text overlay helpers for the current YAM runtime."""

from __future__ import annotations

from dataclasses import dataclass

from v2.shape.model import ShapeInstance


@dataclass(slots=True)
class TextOverlay:
    x: int
    y: int
    text: str
    z: int = 1000

    def shapes(self) -> list[ShapeInstance]:
        return [
            ShapeInstance(x=self.x + offset, y=self.y, glyph=char, z=self.z)
            for offset, char in enumerate(self.text)
        ]
