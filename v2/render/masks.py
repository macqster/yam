"""Mask model for the current YAM runtime."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(slots=True)
class Mask:
    """Spatial render constraint."""

    width: int
    height: int
    blocked: set[tuple[int, int]]

    def allows(self, x: int, y: int) -> bool:
        return 0 <= x < self.width and 0 <= y < self.height and (x, y) not in self.blocked


def empty_mask(width: int, height: int) -> Mask:
    return Mask(width=width, height=height, blocked=set())
