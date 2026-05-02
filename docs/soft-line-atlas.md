# Soft Line Atlas

This atlas captures the current linework reference for project-wide guide strokes and future mask/annotation tooling.

Scope:
- linework only
- no fills
- no sprites
- no raster masks

The purpose is to keep direction and angle families unambiguous before the guide system starts steering vines, masks, rulers, and other world-space annotations in world space.

The target is universal line coverage across the full YAM world size:

- any line
- any direction
- any valid world-space start/end pair
- one deterministic geometry layer plus one glyph grammar layer

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

- light entry / exit: `.`, `,`, `` ` ``; punctuation should lean with the stroke direction rather than acting as a neutral filler
- soft transition: `'`, `-`, `_`
- hard directional anchors: `/`, `\`
- vertical emphasis for rulers / axes: `|`, `:`

## Sample-Inferred Grammar

The handwritten study files suggest the visible line grammar is more structured than a pure slope formula:

- long shallow runs often start with punctuation, transition through `_` or `` ` ``, and resolve into `-`
- short shallow runs can compress into `.` + `` ` `` + `-` / `_`
- steep or medium diagonals often prefer slash/backslash as the stable core glyphs
- mirrored directions stay within the same bucket family, but the cadence inside the bucket can differ
- axis-aligned runs stay simple: horizontal is `-`-dominant and vertical is `|`-dominant

The key implication is that the renderer should treat glyph choice as a small typed grammar, not as a single global ramp.

Cross-reference:

- `lines_straight_soft_primitives.txt`
- `filled_soft_line_slope_form_v0_1.md`

## Grammar Key V1

The current working key for the renderer is:

`LineFamily -> LengthBucket -> Direction -> PhaseRole -> CellBand -> LocalStep`

Where:

- `LineFamily` captures the broad slope class, such as axis, shallow, medium, or steep
- `LengthBucket` captures whether the line is short, medium, or long relative to the sampled world span
- `Direction` captures the mirrored traversal family
- `PhaseRole` captures entry, transition, core, and exit behavior
- `CellBand` captures the coarse sub-cell placement of the stroke inside the glyph cell, such as top, middle, or bottom bias
- `LocalStep` refines the glyph choice using neighboring step context only after the higher-level cadence is chosen

This ordering is intentionally more specific than a pure slope lookup and more general than the current sample buckets. It is meant to support any valid line anywhere in the YAM world.

The current Rust implementation mirrors this key in `core::guide_line` as a literal glyph atlas table, so the atlas can be compared against code instead of remaining prose-only.

For the canonical long-shallow family, the current implementation also uses a cadence-driven helper so the line can lean into a visible entry / ramp / core / exit rhythm instead of collapsing into a flat underscore-only body, and the glyph choice is now also biased by the coarse cell-band classification so punctuation can better match where the stroke sits inside the cell.

The current debug surface also reports a soft-band readout for the canonical probe so the `CellBand` classification can be checked against the visible stroke without changing the rendered line itself.

The `CellBand` classifier is now driven by the line’s sub-cell position relative to the ideal segment rather than by step progress alone, so the band reflects where the stroke actually sits inside the glyph cell.

## Inferred Bucket Table

The current sample set suggests the following first-pass bucket interpretation:

| Family | Representative cadence | Notes |
| --- | --- | --- |
| `10x1` / horizontal | `-` only, or minimal edge softness | Axis-aligned and stable |
| `1x5` / vertical | `|` only | Axis-aligned and stable |
| `10x5` / long shallow | `--''`-style lead-in, then punctuation/body, then lighter exit glyphs | Strong entry/core/exit segmentation |
| `9x5` / shallow-mid | `` ` `` + `.` lead-in, `\` or `/` core, punctuation exit | Direction-mirrored but bucket-stable |
| `8x3` / shallow | punctuation lead-in, short structural core, punctuation exit | Less ramp, more core glyph |
| `7x2` / very shallow | compressed punctuation + underscore / dash ramp | Shorter cadence, still soft |
| `6x5` / steeper medium | slash/backslash becomes the stable core earlier | Structural glyphs dominate sooner |
| `5x4` / medium | mixed punctuation and slash/backslash core | Transitional bucket |
| `4x2`, `3x2` / short diagonal | short punctuation lead-in, then structural core | Very compact cadence |

The long-span study files also suggest these useful concrete cadence groups:

- `10x2` and `10x3`: short punctuation entry, short transition, structural core, punctuation exit
- `10x4` and `10x5`: extended punctuation/underscore ramp, then core, then exit punctuation
- `64x10`: the canonical mirrored long-shallow calibration pair, with a lead-in that reads like `--''` and a lighter exit that reads like `__. -`
- `7x2`: short ramp that still prefers punctuation before structural resolution
- `8x3` and `9x4`: medium ramps where slash/backslash starts to dominate earlier
- `6x5`: steeper medium strokes where slash/backslash becomes the structural center quickly

This is not yet a final spec. It is the first sample-backed bucket hypothesis the renderer should be compared against.

## Known Gap

The canonical `64x10` line still does not render with the full smooth cadence seen in `test_line_64x10.txt`.

Current symptoms:

- the line geometry is close, but the visual output still leans too hard toward repeated `_` / `-` cells
- the lead-in / body / exit cadence is not yet as rich as the reference sample
- the grammar is still too coarse for long shallow lines, even after the atlas was converted into a table
- the long-shallow family now has a dedicated cadence helper, but it remains a calibration target rather than a final proven grammar
- punctuation glyphs in the long-shallow family should match the visible lean of the stroke cell, so a rising-right segment can read with comma-like marks instead of only generic dot/apostrophe placeholders

Observed failure modes as of the latest probe pass:

- the rendered stroke still does not resemble the hand-drawn `test_line_64x10.txt` sample in overall silhouette
- the visible result still reads as a segmented ASCII trace rather than a smooth long-shallow line
- the new `CellBand` readout does not yet correlate strongly enough with the perceived glyph placement in the screenshot
- punctuation and banding are still too weak to produce the reference-style `__.-` / `--''` cadence as a readable whole
- mirrored long-shallow variants still share too much of the same body rhythm
- the current output can look directionally plausible while still being visually unlike the reference because the glyph anchor inside the cell is not yet convincing
- the current long-shallow tuning is still more of a calibration scaffold than a finished grammar
- the renderer still lacks a reliable notion of how the glyph’s internal shape should correspond to the local virtual line path inside the cell

This means the problem is not finished geometry; it is still a glyph-grammar fidelity gap in the long-shallow family.

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

The atlas is therefore a calibration surface for a world-scale engine, not a narrow catalog of allowed cases.

For project-wide guide drawing and future mask edges, the generator should be able to cover the full YAM world size with the same geometry/appearance split, so future guide lines and guide boundaries can share one deterministic line engine instead of inventing separate placement logic.

## Construction Rules

- Prefer connected strokes over isolated dots.
- Keep endpoints readable and consistent across mirrored directions.
- Use the sparse soft ramp for shallow transitions.
- Reserve `|` and `:`-style glyphs for axes and rulers.
- Treat the atlas as a linework reference, not as a mask or a fill system.
- Extend the same grammar across the full YAM world size rather than switching to a separate raster logic for longer spans.
- Prefer bucketed cadences for long shallow spans instead of a one-size-fits-all phase ramp.
- Keep the sample-derived long-line cadence readable as punctuation -> transition -> structural core -> exit punctuation.
- Treat buckets as learning data for a world-size line engine, not as an upper bound on supported geometry.
- Keep the sampled long-span families visible as calibration targets for `10x2`, `10x3`, `10x4`, `10x5`, `7x2`, `8x3`, `9x4`, and `6x5`.
