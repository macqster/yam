from __future__ import annotations

from dataclasses import dataclass


Direction = tuple[int, int]
Point = tuple[int, int]

NEIGHBORS_4: tuple[Direction, ...] = ((1, 0), (-1, 0), (0, 1), (0, -1))

BROWN = "\x1b[38;5;94m"
DARK_BROWN = "\x1b[38;5;58m"
GREEN = "\x1b[38;5;34m"
LIGHT_GREEN = "\x1b[38;5;82m"
OLIVE = "\x1b[38;5;106m"


@dataclass
class GrowthTip:
    x: int
    y: int
    dx: int
    dy: int
    life: float
    max_life: float
    is_trunk: bool
