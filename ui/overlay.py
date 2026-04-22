"""UI overlay generation for the current YAM runtime."""

from __future__ import annotations

from shape.model import ShapeInstance
from ui.model import UIModel


def ui_overlay_shapes(ui: UIModel) -> list[ShapeInstance]:
    """Convert UI state into overlay shapes."""
    shapes: list[ShapeInstance] = []
    if ui.status:
        for offset, char in enumerate(ui.status):
            shapes.append(ShapeInstance(x=1 + offset, y=1, glyph=char, z=1000))
    return shapes
