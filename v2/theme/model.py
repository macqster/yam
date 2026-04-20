"""Theme model for the v2 scaffold."""

from __future__ import annotations

from dataclasses import dataclass, field


@dataclass(slots=True)
class Theme:
    """Visual identity constraints."""

    name: str = "btas_dark_deco"
    structural_glyphs: str = "│─╱╲"
    fill_glyphs: str = " ░▒▓█"
    detail_glyphs: str = ".,'`"
    accent_glyphs: str = "*+x"
    clock_position: tuple[int, int] = (0, 0)
    palette: dict[str, str] = field(default_factory=lambda: {
        "background": "#0b0c10",
        "foreground": "#d9d9d9",
        "accent": "#7aa2f7",
    })


def glyph_for_kind(kind: str, theme: Theme) -> str:
    """Return a stable glyph for a semantic kind."""
    mapping = {
        "hero": "█",
        "seed": "·",
        "detail": "·",
        "accent": "*",
        "fill": "▓",
    }
    return mapping.get(kind, "·")


def theme_by_name(name: str) -> Theme:
    """Return a theme by name, falling back to the default."""
    themes = {
        "btas_dark_deco": Theme(),
        "monochrome": Theme(name="monochrome", accent_glyphs="+x", fill_glyphs=" ░▒▓█"),
    }
    return themes.get(name, Theme(name=name))
