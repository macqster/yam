# Moebius Matrix V1

This file documents the plain-text full-atlas sheet intended for manual recoloring and tweaking in Moebius.

Primary sheet:

- [`moebius-matrix-v1.txt`](moebius-matrix-v1.txt)

Preferred editor/export artifacts:

- `XBIN` / `.xb` should be treated as the canonical Moebius round-trip artifact for this sheet
- ANSI / `.ans` may still be exported for inspection, but should be treated as secondary

## Purpose

- give Moebius one stable, full-atlas composition instead of many separate small files
- preserve one fixed tile order so edited exports stay easy to compare against YAM runtime states
- keep the first recolor/tweak pass focused on the full matrix rather than on isolated sprite files

## Sheet Contract

- format: plain text
- columns: `6`
- rows of sprite tiles: `4`
- tile height: `5` sprite rows plus `1` label row
- gutter: `2` spaces between tiles
- labels: short ASCII-heavy slugs with numeric IDs

## Tile Order

1. `sunny`
2. `clear_night`
3. `partly_cloudy`
4. `cloudy`
5. `very_cloudy`
6. `overcast`
7. `mist`
8. `fog`
9. `light_showers`
10. `light_rain`
11. `heavy_showers`
12. `heavy_rain`
13. `light_snow`
14. `heavy_snow`
15. `light_snow_showers`
16. `heavy_snow_showers`
17. `light_sleet`
18. `light_sleet_showers`
19. `sleet`
20. `thundery_showers`
21. `thundery_heavy_rain`
22. `thundery_snow_showers`
23. `storm`
24. `unknown`

## Round-Trip Rule

- treat `moebius-matrix-v1.txt` as the import seed
- after manual recolor/tweak work in Moebius, exported artifacts should preserve this tile order
- if a future export is re-ingested into YAM-facing notes, keep the same numeric IDs so variant discussion stays stable even if labels or colors evolve

## Format Preference

Observed behavior from the first manual export pass:

- `.xb` preserves the full matrix composition, spacing, and glyph edits cleanly
- `.ans` is useful as an optional terminal export, but can reflow or spatially degrade the full sheet

Practical rule:

- prefer `.xb` when preserving the full atlas sheet for future editing or re-ingest
- treat `.ans` as an auxiliary export only

## Re-Ingest Checklist

When reviewing a future edited Moebius export:

1. inspect the `.xb` render first
2. verify tile order still matches IDs `01-24`
3. verify the `6 x 4` atlas grid still reads cleanly
4. note both recolor changes and glyph/shape edits
5. promote approved edits back into per-sprite runtime files deliberately, not by bulk replacement
6. keep the source-art sheet and runtime files separate even when a whole batch is accepted

## Scope

- this sheet is source-art only
- it does not replace the live runtime files in `../../runtime/`
- promotion into runtime should still happen deliberately per sprite or per batch after review
