"""Render composition for the current YAM runtime."""

from __future__ import annotations

from v2.render.emitter import Emitter
from v2.render.framebuffer import Framebuffer
from v2.render.layers import default_layers
from v2.render.masks import Mask, empty_mask
from v2.shape.model import ShapeInstance


def compose_frame(
    width: int,
    height: int,
    shapes: list[ShapeInstance],
    mask: Mask | None = None,
) -> str:
    """Compose shapes into a final text frame."""
    framebuffer = Framebuffer(width, height)
    framebuffer.clear()
    active_mask = mask or empty_mask(width, height)
    layers = default_layers()

    for shape in shapes:
        target = layers[1] if shape.z >= 1000 else layers[0]
        target.add(shape)

    for layer in sorted(layers, key=lambda item: item.z):
        for shape in layer.shapes:
            framebuffer.write(shape.x, shape.y, shape.glyph, z=layer.z + shape.z, mask=active_mask)

    return Emitter().emit(framebuffer)
