"""Ecosystem state for the current YAM runtime."""

from __future__ import annotations

from dataclasses import dataclass, field

from v2.engine.balance import Balance
from v2.engine.environment import Environment
from v2.engine.lifecycle import Lifecycle
from v2.engine.species import Species


@dataclass(slots=True)
class Organism:
    """Minimal organism representation for the vertical slice."""

    name: str
    x: int
    y: int
    glyph: str = "◉"
    species: Species | None = None
    lifecycle: Lifecycle = field(default_factory=Lifecycle)


@dataclass(slots=True)
class Ecosystem:
    """Structured collection of organisms and environment state."""

    organisms: list[Organism] = field(default_factory=list)
    environment: Environment = field(default_factory=Environment)
    balance: Balance = field(default_factory=Balance)

    def step(self, width: int, height: int) -> "Ecosystem":
        """Advance the ecosystem deterministically."""
        next_environment = self.environment.step()
        updated: list[Organism] = []
        for index, organism in enumerate(self.organisms):
            lifecycle = organism.lifecycle.step()
            if lifecycle.state == "decay":
                continue
            if organism.name == "hero":
                updated.append(
                    Organism(
                        name=organism.name,
                        x=organism.x,
                        y=organism.y,
                        glyph=organism.glyph,
                        species=organism.species,
                        lifecycle=lifecycle,
                    )
                )
                continue
            dx = ((next_environment.tick + index) % 3) - 1
            dy = ((next_environment.tick + index) % 2)
            x = min(width - 1, max(0, organism.x + dx))
            y = min(height - 1, max(0, organism.y + dy))
            updated.append(
                Organism(
                    name=organism.name,
                    x=x,
                    y=y,
                    glyph=organism.glyph,
                    species=organism.species,
                    lifecycle=lifecycle,
                )
            )
        limited = updated[: self.balance.clamp_count(len(updated))]
        return Ecosystem(
            organisms=limited,
            environment=next_environment,
            balance=self.balance,
        )
