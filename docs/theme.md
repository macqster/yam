# Theme

YAM's visual identity is based on a reusable BTAS/TNBA-inspired theme layer.

## Core idea

- The BTAS/TNBA reference palette is the shared source of truth for color selection.
- The shared curated palette now also has a primitive alias layer so dotfiles and YAM can talk about the same colors without mixing display labels and runtime roles.
- Semantic styles are derived from the palette, not hand-written at each call site.
- Runtime UI, debug overlays, modal shells, footer text, and future scene layers should all consume the same theme contract.
- The sandbox/dev tooling can open the current curated palette together with the extracted-source swatch group as a modal inspection sheet, so BTAS/TNBA color decisions can be judged in the same terminal/runtime surface as the rest of YAM instead of only in external docs or PDFs.

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
  - `WEATHER_RAIN`
  - `WEATHER_RAIN_HEAVY`
  - `WEATHER_SUN_CORE`
  - `WEATHER_SUN_RAY`
  - `WEATHER_LIGHTNING`
  - `WEATHER_ALERT`
  - `HERO_BG`
  - `MODAL_BG`
  - `CAMERA_TRACK`
  - `CAMERA_THUMB`
  - `POINTER_PROBE`
  - `VINE_HEALTHY`
  - `VINE_AGED`

The broader palette docs outside YAM now also define primitive aliases such as `graphite`, `gray_light`, `green_balanced`, `blue_bright`, and `amber_dusty`.
Those aliases are shared palette handles, not YAM runtime token names.

Active BTAS/TNBA anchor tones in YAM now follow the calmer reference direction:

- neutrals: `#1E2124` panel, `#262A2E` lifted, `#373C41` divider
- text: `#E6E8EB` primary, `#A0A6AD` secondary
- extracted warm foreground candidate: `#DCD8CB` is now used selectively for the clock/date/weather main foreground path so those companion widgets read as one visually paired stack a little closer to the BTAS source sheet without changing the broader primary UI text contract everywhere else
- extracted neutral slate `#727C76` is now used for the global footer text, keeping the bottom status line quieter and closer to the BTAS source-neutral family than the brighter secondary text path
- blue emphasis: `#2C4C8A` / `#355FA8`
- green emphasis: `#3E7357` / `#4F8E6C`
- warm marker/accent: `#C48A2C` with `#A33A32` reserved as the stronger red warning/accent family

Current popup-shell grammar is intentionally more specific:

- modal border chrome: subtle green `#2F5A45`
- popup title and footer control glyphs: balanced blue `#2C4C8A`
- settings tabs: inactive rust `#B24E2E`, active brick red `#A33A32`
- selected settings row background: navy `#243B73`

Weather-widget work should stay inside this palette vocabulary as well; the canonical weather-specific role mapping and sprite-color guidance live in [`weather-widget.md`](weather-widget.md).

The current weather token pass intentionally keeps the atlas monochrome asset files frozen while giving runtime theme application more semantic range, especially for heavy-rain and thunder/lightning families that were previously sharing broader generic accents.

For a human-friendly summary of the current palette chain as YAM should understand it, see [`palette-reference.md`](palette-reference.md). For the PDF/layout-ready palette-sheet source, see [`palette-sheet-reference.md`](palette-sheet-reference.md). For the currently adopted shareable packet export, see [`btas_tnba_color_system_canonical_packet.pdf`](btas_tnba_color_system_canonical_packet.pdf). The extracted source SSOT and curated workstation master palette currently live in `~/_git/dotfiles/themes/`.

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
