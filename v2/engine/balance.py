"""Balance rules for the current YAM runtime."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(slots=True)
class Balance:
    """Controls density, competition, and growth limits."""

    max_organisms: int = 8
    growth_bias: float = 1.0

    def clamp_count(self, count: int) -> int:
        return min(count, self.max_organisms)
