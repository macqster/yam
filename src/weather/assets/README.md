# Weather Assets

This directory holds the weather-sprite asset tree used by YAM.

## Purpose

- keep live sprite shapes close to the weather atlas code
- keep runtime assets plain-text and monospace-safe
- keep color out of the asset files; YAM applies semantic color roles at render time
- keep manual design exploration separate from the currently selected runtime shapes

## File Roles

Runtime asset files live in `runtime/`:

- `runtime/clear.txt`
- `runtime/clear_night.txt`
- `runtime/cloudy.txt`
- `runtime/very_cloudy.txt`
- `runtime/overcast.txt`
- `runtime/mist.txt`
- `runtime/fog.txt`
- `runtime/partly_cloudy.txt`
- `runtime/light_showers.txt`
- `runtime/light_rain.txt`
- `runtime/heavy_showers.txt`
- `runtime/heavy_rain.txt`
- `runtime/sleet.txt`
- `runtime/light_sleet.txt`
- `runtime/light_sleet_showers.txt`
- `runtime/light_snow.txt`
- `runtime/heavy_snow.txt`
- `runtime/light_snow_showers.txt`
- `runtime/heavy_snow_showers.txt`
- `runtime/thundery_showers.txt`
- `runtime/thundery_heavy_rain.txt`
- `runtime/thundery_snow_showers.txt`
- `runtime/storm.txt`
- `runtime/unknown.txt`

Source-art and review notes live in `source-art/`:

- `source-art/plain-text/prototypes-v1.md`
  - imported plain-text monochrome design candidates
  - preserves alternatives for future manual selection
  - does not change runtime atlas behavior by itself
- `source-art/plain-text/moebius-matrix-v1.txt`
  - full plain-text atlas sheet prepared for one-shot manual recolor/tweak work in Moebius
- `source-art/plain-text/moebius-matrix-v1-legend.md`
  - tile-order, geometry, and round-trip contract for the Moebius atlas sheet
- `source-art/colored/`
  - reserved for future manually colored design notes or import references
  - not part of the live runtime atlas

## Contract

- one file should contain one plain-text sprite shape
- runtime assets should stay ANSI-free
- runtime assets should not embed palette decisions
- approved manual edits should be normalized back into plain-text before entering the live atlas
- the dedicated `[W]eather` dev popup is the preferred comparative inspection surface before promoting a candidate into the live widget
- for full-sheet Moebius round-trips, prefer `.xb` as the canonical preserved artifact; treat `.ans` as secondary/inspection-oriented export

## Current Organization Rule

- `runtime/` is the weather-atlas source of truth for live plain-text shapes
- `source-art/plain-text/` holds monochrome exploration and prototype notes
- `source-art/plain-text/` also owns the Moebius full-sheet contract and re-ingest notes
- `source-art/colored/` is reserved for future colored/manual reference work
- broader design reasoning lives in:
  - `docs/weather-widget.md`
  - `docs/LOG.md`
- manual candidate sets can live here as markdown notes as long as they are clearly marked as non-runtime source-art
