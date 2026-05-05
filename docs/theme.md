# Theme

YAM's visual identity is based on a reusable BTAS theme layer.

## Core idea

- The BTAS palette is the shared source of truth for color selection.
- Semantic styles are derived from the palette, not hand-written at each call site.
- Runtime UI, debug overlays, modal shells, footer text, and future scene layers should all consume the same theme contract.

## Palette model

- `BTAS` is the canonical palette bundle in `src/theme/btas.rs`.
- `src/theme/palette.rs` exposes semantic aliases for the architecture:
  - `PANEL_BG`
  - `PRIMARY_FG`
  - `ACCENT`
  - `MARKER`
  - `HERO_BG`
  - `FOOTER_FG`
  - `MODAL_BG`
  - `CAMERA_TRACK`
  - `CAMERA_THUMB`
  - `POINTER_PROBE`

## Style model

- `src/theme/style.rs` maps the semantic tokens into reusable style helpers.
- Modal shells, panels, hero overlays, camera indicators, pointer probes, footer text, debug text, and guide traces should use these helpers rather than inventing one-off colors.

## Reuse rule

- If a UI surface needs color, it should first ask: "which BTAS token is this?"
- If a surface needs a style, it should first ask: "which semantic helper already exists?"
- The goal is one theme vocabulary across the whole YAM architecture.

## Intentional exceptions

- Low-level render primitives, glyph/debug calibration, and hero-art experiments may still use local color literals when the point is to test rendering behavior rather than define a reusable UI surface.
- Test code may also use explicit color literals when it is checking a precise renderer outcome.
- These exceptions should stay narrow and should not become the default pattern for scene/UI surfaces.
