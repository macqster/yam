"""Shape instance for the current YAM runtime."""

from __future__ import annotations

from dataclasses import dataclass

from v2.engine.ecosystem import Organism
from v2.morphology.model import Morphology, Node, build_morphology
from v2.shape.policy import policy_glyph
from v2.theme.model import Theme, glyph_for_kind


@dataclass(slots=True)
class ShapeInstance:
    """Renderable shape derived from morphology."""

    x: int
    y: int
    glyph: str
    z: int = 0


def shapes_from_morphology(morphology: Morphology, organisms: list[Organism]) -> list[ShapeInstance]:
    """Convert morphology into renderable shapes."""
    theme = Theme()
    shapes: list[ShapeInstance] = []
    for organism, node in zip(organisms, morphology.nodes):
        base_glyph = policy_glyph(organism.glyph or glyph_for_kind(organism.name, theme), theme)
        shapes.append(ShapeInstance(x=node.x, y=node.y, glyph=base_glyph, z=10))
        if organism.name == "hero":
            shapes.append(ShapeInstance(x=node.x, y=node.y - 1, glyph=policy_glyph("█", theme), z=11))
        elif organism.name.startswith("seed"):
            shapes.append(ShapeInstance(x=node.x, y=node.y + 1, glyph=policy_glyph("·", theme), z=9))
    return shapes


def build_shapes(organisms: list[Organism]) -> list[ShapeInstance]:
    """Convenience wrapper from organisms to renderable shapes."""
    morphology = build_morphology(organisms)
    return shapes_from_morphology(morphology, organisms)
