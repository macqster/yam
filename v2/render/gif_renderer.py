"""Basic GIF-to-shape renderer for the v2 scaffold."""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

from PIL import Image

from v2.shape.model import ShapeInstance
from v2.shape.policy import policy_glyph
from v2.theme.model import Theme

SHADE_GLYPHS = " .:-=+*#%@"


@dataclass(slots=True)
class GifRenderer:
    """Load a GIF and convert a frame into renderable shapes."""

    path: Path
    width: int
    height: int

    def _frame_index(self, tick: int, frame_count: int) -> int:
        return tick % max(1, frame_count)

    def render(self, tick: int) -> list[ShapeInstance]:
        if not self.path.exists():
            return []

        shapes: list[ShapeInstance] = []
        theme = Theme()
        with Image.open(self.path) as img:
            total = getattr(img, "n_frames", 1)
            img.seek(self._frame_index(tick, total))
            frame = img.convert("RGBA").resize((self.width, self.height), Image.Resampling.BILINEAR)
            pixels = frame.load()
            for y in range(self.height):
                for x in range(self.width):
                    r, g, b, a = pixels[x, y]
                    if a < 24:
                        continue
                    lum = int(round(0.2126 * r + 0.7152 * g + 0.0722 * b))
                    glyph = SHADE_GLYPHS[min(len(SHADE_GLYPHS) - 1, max(0, lum * (len(SHADE_GLYPHS) - 1) // 255))]
                    glyph = policy_glyph(glyph, theme)
                    if glyph == " ":
                        continue
                    shapes.append(ShapeInstance(x=x, y=y, glyph=glyph, z=5))
        return shapes
