"""Species templates for the v2 scaffold."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(slots=True)
class Species:
    """Reusable behavior and appearance definition."""

    name: str
    glyph: str
    growth_rate: float = 1.0
    max_age: int = 12
