"""Layer model for the current YAM runtime."""

from __future__ import annotations

from dataclasses import dataclass, field

from shape.model import ShapeInstance


@dataclass(slots=True)
class Layer:
    """Logical render grouping."""

    name: str
    z: int
    shapes: list[ShapeInstance] = field(default_factory=list)

    def add(self, shape: ShapeInstance) -> None:
        self.shapes.append(shape)


def default_layers() -> list[Layer]:
    """Return the canonical v2 layer order."""
    return [
        Layer(name="world", z=100),
        Layer(name="overlay", z=1000),
    ]
