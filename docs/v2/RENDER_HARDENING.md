# YAM v2 Render Hardening

This document records the current render composition rules.

## Layer Order

- `world` renders first
- `overlay` renders last

## Z Rules

- GIF content stays in the world layer
- clock text stays in the overlay layer
- overlay items must use a higher z-band than world items

## Constraints

- do not sort by glyph type
- do not mix overlay and world content implicitly
- keep the composition deterministic
