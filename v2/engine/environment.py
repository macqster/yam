"""Environment state for the v2 scaffold."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(slots=True)
class Environment:
    """Global modifier state for time, weather, and conditions."""

    tick: int = 0
    light: float = 0.5
    humidity: float = 0.5
    temperature: float = 0.5

    def step(self) -> "Environment":
        """Advance the environment deterministically."""
        next_tick = self.tick + 1
        return Environment(
            tick=next_tick,
            light=0.5 + ((next_tick % 5) - 2) * 0.05,
            humidity=0.5 + ((next_tick % 3) - 1) * 0.04,
            temperature=0.5 + ((next_tick % 7) - 3) * 0.03,
        )
