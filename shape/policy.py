"""Glyph policy for the current YAM runtime."""

from __future__ import annotations

from theme.model import Theme


def policy_glyph(glyph: str, theme: Theme) -> str:
    """Clamp glyphs to the theme's allowed visual language."""
    if glyph in theme.structural_glyphs:
        return glyph
    if glyph in theme.accent_glyphs:
        return glyph
    if glyph in theme.fill_glyphs:
        return glyph
    if glyph.isdigit():
        return "·"
    return "·"
