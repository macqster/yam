# Theme

YAM's visual identity is based on a reusable BTAS/TNBA-inspired theme layer.

## Core idea

- The BTAS/TNBA reference palette is the shared source of truth for color selection.
- Semantic styles are derived from the palette, not hand-written at each call site.
- Runtime UI, debug overlays, modal shells, footer text, and future scene layers should all consume the same theme contract.

## Palette model

- `BTAS` in `src/theme/btas.rs` is the runtime palette bundle, but it should stay aligned with the repo-wide BTAS/TNBA reference palette maintained outside YAM.
- `src/theme/palette.rs` exposes semantic aliases for the architecture:
  - `PANEL_BG`
  - `SURFACE_LIFTED`
  - `DIVIDER`
  - `PRIMARY_FG`
  - `SECONDARY_FG`
  - `DEBUG_FG`
  - `MARKER`
  - `HERO_BG`
  - `MODAL_BG`
  - `CAMERA_TRACK`
  - `CAMERA_THUMB`
  - `POINTER_PROBE`
  - `VINE_HEALTHY`
  - `VINE_AGED`

Active BTAS/TNBA anchor tones in YAM now follow the calmer reference direction:

- neutrals: `#1E2124` panel, `#262A2E` lifted, `#373C41` divider
- text: `#E6E8EB` primary, `#A0A6AD` secondary
- blue emphasis: `#2C4C8A` / `#355FA8`
- green emphasis: `#3E7357` / `#4F8E6C`
- warm marker/accent: `#C48A2C` with `#A33A32` reserved as the stronger red warning/accent family

Current popup-shell grammar is intentionally more specific:

- modal border chrome: subtle green `#2F5A45`
- popup title and footer control glyphs: balanced blue `#2C4C8A`
- settings tabs: inactive rust `#B24E2E`, active brick red `#A33A32`
- selected settings row background: navy `#243B73`

Weather-widget work should stay inside this palette vocabulary as well; the canonical weather-specific role mapping and sprite-color guidance live in [`weather-widget.md`](weather-widget.md).

## Style model

- `src/theme/style.rs` maps the semantic tokens into reusable style helpers.
- Modal shells, panels, hero overlays, camera indicators, pointer probes, footer text, debug text, and guide traces should use these helpers rather than inventing one-off colors.
- The current aim is a calmer BTAS/TNBA look: darker surfaces, softer greens, steadier blues, and a more restrained warm-accent path than the earlier brighter BTAS bundle.

## Reuse rule

- If a UI surface needs color, it should first ask: "which BTAS token is this?"
- If a surface needs a style, it should first ask: "which semantic helper already exists?"
- The goal is one theme vocabulary across the whole YAM architecture.

## Intentional exceptions

- Low-level render primitives, glyph/debug calibration, and hero-art experiments may still use local color literals when the point is to test rendering behavior rather than define a reusable UI surface.
- Test code may also use explicit color literals when it is checking a precise renderer outcome.
- These exceptions should stay narrow and should not become the default pattern for scene/UI surfaces.
