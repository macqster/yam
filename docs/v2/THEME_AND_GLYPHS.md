# YAM v2 Theme and Glyphs

This document records the current minimal visual policy.

## Theme

- theme name: `btas_dark_deco`
- the theme constrains structural, fill, detail, and accent glyph families
- palette exists as a minimal contract, not yet a full color pipeline

## Glyph Policy

- structural glyphs are preferred for anchors and form
- fill glyphs are used for mass and render density
- detail glyphs are used sparingly
- unknown glyphs fall back to a neutral dot

## Notes

- this is intentionally minimal
- the policy is enforced at shape and GIF conversion time
- render composition remains unchanged
- the current golden frame reflects the sparse glyph baseline produced by this policy
