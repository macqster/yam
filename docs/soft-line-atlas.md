# Soft Line Atlas

This atlas captures the current linework reference for project-wide guide strokes and future mask/annotation tooling.

Scope:
- linework only
- no fills
- no sprites
- no raster masks

The purpose is to keep direction and angle families unambiguous before the guide system starts steering vines, masks, rulers, and other world-space annotations in world space.

## Geometry And Appearance

The line generator should be thought of as two layers:

- geometry: Bresenham-style stepping that chooses the cells
- appearance: glyph selection that gives the stroke its soft or hard visual profile

The atlas documents the appearance side, while the geometry layer guarantees that the line reaches the correct cells in the first place.

## Direction Families

The current soft-line grammar is organized around four traversal directions:

- `left-right`
- `right-left`
- `up-down`
- `down-up`

These are mirror families. The glyph vocabulary stays the same; only traversal direction and endpoint emphasis change.

## Size Buckets

The straight-primitive family is bucketed by height / span:

- `10x1`
- `10x2`
- `10x3`
- `10x4`
- `10x5`

The smaller the bucket, the more the line must fall back to a hard axis stroke. The larger the bucket, the more room it has to express the soft ramp before it resolves into a harder anchor glyph.

For longer world spans, the same grammar scales into extended slope families:

- `7x1`
- `8x1`
- `9x1`
- `10x1`
- `7x2`
- `8x3`
- `9x4`
- `10x5`

These longer forms are intended to let the guide engine cover full-world lines without switching away from the soft stroke vocabulary.

## Glyph Vocabulary

Current guide-line glyph families:

- light entry / exit: `.`, `,`, `` ` ``
- soft transition: `'`, `-`, `_`
- hard directional anchors: `/`, `\`
- vertical emphasis for rulers / axes: `|`, `:`

## Primitive Matrix

This is the current reference matrix for straight guide strokes:

| Bucket | left-right | right-left | up-down | down-up |
| --- | --- | --- | --- | --- |
| `10x1` | hard axial stroke with minimal softness | mirrored hard axial stroke | vertical emphasis / ruler stroke | mirrored vertical emphasis / ruler stroke |
| `10x2` | soft entry, short core, soft exit | mirrored soft entry / exit | vertical lead-in and lead-out | mirrored vertical lead-in and lead-out |
| `10x3` | soft entry, a visible midstroke transition, soft exit | mirrored transition | vertical transition with a center anchor | mirrored vertical transition |
| `10x4` | sparse soft ramp before the core stroke resolves | mirrored sparse soft ramp | stronger vertical backbone with soft edge cells | mirrored stronger vertical backbone |
| `10x5` | fullest soft ramp before the stroke becomes hard | mirrored fullest soft ramp | fullest vertical guide with sparse edge softness | mirrored fullest vertical guide |

## Extended Slope Examples

These shapes show how the same linework grammar should scale for longer spans:

- `7x1` to `10x1`: horizontal runs should stay `-`-dominant, with only minimal punctuation at the ends if needed.
- `7x2` to `10x5`: shallow diagonals should keep the soft lead-in / transition cadence, then resolve into a stable stroke pattern instead of flattening to a single generic ramp.
- `1x2` to `1x5`: vertical runs should remain ruler-like with `|` emphasis, not drift into horizontal punctuation.

The important rule is that longer lines should preserve the directional family, not just repeat a short primitive and stretch it.

For project-wide guide drawing and future mask edges, the generator should be able to cover the full YAM world size with the same geometry/appearance split, so future guide lines and guide boundaries can share one deterministic line engine instead of inventing separate placement logic.

## Construction Rules

- Prefer connected strokes over isolated dots.
- Keep endpoints readable and consistent across mirrored directions.
- Use the sparse soft ramp for shallow transitions.
- Reserve `|` and `:`-style glyphs for axes and rulers.
- Treat the atlas as a linework reference, not as a mask or a fill system.
- Extend the same grammar across the full YAM world size rather than switching to a separate raster logic for longer spans.
