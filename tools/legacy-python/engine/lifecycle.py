"""Lifecycle state machine for the current YAM runtime."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(slots=True)
class Lifecycle:
    """Deterministic organism lifecycle state machine."""

    name: str = "seed"
    age: int = 0
    state: str = "seed"

    def step(self) -> "Lifecycle":
        next_age = self.age + 1
        if next_age < 3:
            state = "seed"
        elif next_age < 6:
            state = "growth"
        elif next_age < 9:
            state = "maturity"
        elif next_age < 12:
            state = "aging"
        else:
            state = "decay"
        return Lifecycle(name=self.name, age=next_age, state=state)
