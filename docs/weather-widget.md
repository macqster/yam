# Weather Widget

This document is the canonical design brief for the YAM weather widget.

## Current Status

- The runtime now includes a first compact `WeatherLayer` scaffold in the main scene.
- The current layer reads a cached weather snapshot from `UiState` and renders it through a small native Ratatui render path.
- The current runtime refresh path now tries a live `wttr.in` provider in a background refresh seam and falls back to a static prototype snapshot if live fetch fails.
- The normalization layer now resolves `wttr.in` weather codes first, so upstream condition families such as showers, sleet, snow, and thunder variants can survive into YAM before the atlas chooses how much art distinction to give them.
- The runtime still keeps provider work out of the render path and out of synchronous per-frame drawing.
- The presentation path is now explicitly split into sprite atlas, layout composition, and localized text seams so the current compact widget is only one presentation over a more modular weather core.
- The default compact layout now follows a more `wttr.in`-like vertical fact stack, with a left sprite column and right-side condition / temperature / wind / visibility / precipitation rows.
- The default compact layout should behave like a strict two-column composition: a fixed-width sprite column on the left and a fixed-start facts column on the right, so weather facts never drift into the sprite silhouette when specific rows are shorter.
- Weather sprite shapes are now stored as compile-time plain-text assets, while color remains a separate semantic render concern owned by YAM.
- A dedicated dev-mode `[W]eather` popup now serves as the comparative sprite-inspection surface, so atlas reviews can happen away from the main-scene composition without consuming sandbox world-space.
- The sandbox review should prefer grouped comparison sheets over flat state dumps, so hard visual boundaries like `cloudy / very cloudy / overcast` or `light showers / light rain / heavy showers / heavy rain` can be judged as a grammar rather than as isolated icons.
- External ANSI/ASCII finds may still be useful as one-off provenance sources for individual shapes; when that happens, the imported shape should be converted into plain-text assets and semantic YAM color roles rather than embedded as raw ANSI escape art.
- Treat XBIN/Moebius-style editors as a future offline sprite-authoring workflow only; do not make YAM depend on raw XBIN blobs at runtime unless a later dedicated import/export pass proves worth the complexity.
- For full-sheet Moebius atlas work, prefer `.xb` as the preserved round-trip artifact; ANSI `.ans` exports may still help with terminal inspection, but should be treated as secondary because they can spatially degrade a dense atlas sheet.
- The current implementation goal is to keep ownership, sizing, layering, and styling stable now that the first live weather refresh seam is already in place.

## Core Direction

- Weather is a first-class YAM companion widget, parallel to the clock rather than nested under it.
- The widget should consume a provider-neutral internal weather model.
- YAM should own weather rendering natively in Ratatui.
- External weather services provide data, not terminal UI.
- Visual style should follow the shared BTAS/TNBA palette contract.
- Main-scene visual aesthetics are a first-class concern, so the weather widget should optimize for compositional beauty as much as for factual readability.

## Ownership

- `weather` should live behind its own module boundary rather than being folded into the clock layer.
- The provider/network/cache path should stay separate from widget rendering.
- The render layer should consume normalized weather state only.
- The widget should use the existing hero-scene companion seam, where hero, clock, and weather are sibling world-attached companions.
- The companion seam should stay extensible enough to host the already-live `date` surface together with a future `calendar` surface as sibling widgets or clock-cluster features without forcing a second attachment-model rewrite.

## Provider Rule

- `wttr.in` is the preferred first implementation source because it is terminal-friendly, zero-key, and easy to prototype against.
- `Open-Meteo` is the preferred serious backend for later structured forecasting work.
- The runtime should not depend on one provider's decorative output format.
- Location should be explicit and configured, not silently inferred every frame.
- The current default runtime location is `Krakow, Poland`.
- Upstream `wttr.in` translation files are a valid source for condition wording, especially for Polish localization, as long as YAM still owns final layout and rendering.

## Data Boundary

The render side should consume a normalized model, not raw provider payloads.

Suggested module split:

```text
weather/
  mod.rs
  provider.rs
  wttr.rs
  open_meteo.rs
  model.rs
  cache.rs
  render.rs
```

Suggested trait seam:

```rust
trait WeatherProvider {
    fn snapshot(&self, location: &WeatherLocation) -> Result<WeatherSnapshot>;
}
```

Suggested normalized state:

```rust
struct WeatherSnapshot {
    location_label: String,
    observed_at: DateTime<Utc>,
    temperature_c: Option<f32>,
    feels_like_c: Option<f32>,
    humidity_pct: Option<u8>,
    wind_kph: Option<f32>,
    wind_dir: Option<String>,
    precip_mm: Option<f32>,
    condition_text: Option<String>,
    condition_code: Option<i32>,
    forecast: Vec<ForecastPoint>,
    source: WeatherSource,
    stale: bool,
}
```

Rules:

- The weather widget should never block frame rendering.
- Provider refresh should happen behind a cached runtime seam, such as a background worker or other non-render refresh path.
- Tests may inject deterministic provider results, but they must exercise the same background worker and channel path used by production refreshes.
- Cached stale data is better than a stalled frame.
- Provider fetch failures should degrade gracefully into stale or missing weather state.
- Normalized visual/weather facts should be the source of truth; any human-facing weather wording should be derived at the presentation layer rather than treated as provider-owned final UI text.

## Rendering Rule

- Do not paste raw `wttr.in` full ANSI output into YAM.
- Use `wttr.in` and `wego` as visual references only.
- YAM should own a sprite atlas and compose it through Ratatui spans/cells.
- Weather rendering should be deterministic and fixed-size at the sprite level.
- The widget should balance minimalist aesthetic sprites with clear textual and numerical facts rather than becoming either a pure art block or a pure telemetry dump.

Target pipeline:

```text
provider data
  -> WeatherSnapshot
  -> WeatherVisual
  -> WeatherSprite atlas
  -> Ratatui spans/cells
```

Not:

```text
curl wttr.in full output
  -> paste block directly into scene
```

## Visual Model

The widget should stay close to the clock's visual footprint.

Target width guidance:

- default compact weather width should be roughly the same as the clock width
- assume a design target of about `24` cells wide unless a specific mode intentionally opts out
- the widget should read as a sibling companion to the clock rather than as a larger secondary panel that dominates the main scene

Recommended fixed sprite sizes:

- `1x1` for footer-scale status
- runtime compact sprites should currently stay within a `<=15x5` envelope
- the dedicated `[W]eather` popup currently assumes one label row plus `5` sprite rows per inspected state
- any future alternate compact/panel mode can use a different envelope, but it should be documented explicitly instead of silently reusing the older `5x4` / `7x4` guidance

Recommended compact composition:

- minimalist sprite on the left
- right-side stacked facts in a compact `wttr.in`-like column with a stable start edge across all rows
- condition, temperature band, wind, visibility, and precipitation as the default five informational rows
- no wide forecast tables in the default main-scene companion mode
- current compact stacked layout is only one layout preset; widget infrastructure should remain modular enough to support alternate compact arrangements later without re-owning provider parsing or sprite logic

The first useful visual set should cover:

- sunny
- clear night
- partly cloudy
- cloudy
- very cloudy / overcast
- light showers / heavy showers
- light rain
- heavy rain
- light snow / heavy snow
- light sleet
- thunder variants
- fog
- unknown

Suggested visual normalization layer:

```rust
enum WeatherVisual {
    Sunny,
    ClearNight,
    PartlyCloudy,
    Cloudy,
    VeryCloudy,
    Overcast,
    Mist,
    Fog,
    LightShowers,
    LightRain,
    HeavyShowers,
    HeavyRain,
    LightSnow,
    HeavySnow,
    LightSnowShowers,
    HeavySnowShowers,
    LightSleet,
    LightSleetShowers,
    Sleet,
    ThunderyShowers,
    ThunderyHeavyRain,
    ThunderySnowShowers,
    Unknown,
}
```

Rules:

- The internal visual family set may be broader than the first finished atlas pass; nearby families may temporarily share sprite art while the palette/layout contract stays stable.
- `wttr.in` condition-code distinctions such as light vs heavy snow, showers vs steady precipitation, and thunder variants should not be collapsed away at parse time unless YAM deliberately chooses to merge them.

## Sprite Contract

Weather sprites should be YAM-owned and data-driven.

Recommended sprite representation:

```rust
pub enum WeatherColorRole {
    PanelBg,
    PanelLifted,
    Divider,
    Text,
    TextDim,
    CloudEdge,
    CloudShadow,
    SunCore,
    SunRay,
    Rain,
    RainHeavy,
    Snow,
    FogBack,
    FogFront,
    Lightning,
    Alert,
    CalmGreen,
    FreshGreen,
}

pub struct WeatherSpriteSpan {
    pub text: &'static str,
    pub role: WeatherColorRole,
}

pub struct WeatherSpriteLine {
    pub spans: &'static [WeatherSpriteSpan],
}

pub struct WeatherSprite {
    pub width: u16,
    pub height: u16,
    pub lines: &'static [WeatherSpriteLine],
}
```

Rules:

- Sprite parts should use semantic color roles, not inline raw colors.
- Sprite shapes should stay plain-text and ANSI-free; color belongs in the semantic role layer, not inside imported terminal escape sequences.
- Ratatui `Span`-level styling is the intended primitive.
- Compile-time text assets are preferred over hand-escaped inline Rust literals for sprite shapes, so glyph-heavy art can be edited and reviewed safely without string-escape churn.
- Weather art should feel like painted terminal silhouettes rather than bright emoji weather.
- Compact weather art should remain visually restrained enough to coexist with the hero, clock, vines, and other main-scene elements without stealing the whole composition.
- All supported weather states should resolve through one unified sprite atlas contract, even if some early states temporarily share placeholder art.
- `Mist`, `Fog`, `Snow`, and `Sleet` should not silently collapse into `Unknown`; the atlas should keep explicit entries for them even when the art is still evolving.
- Sandbox sprite review should be organized around the hardest differentiation families first: base sky states, cloud mass, atmosphere veil, rain grammar, snow grammar, sleet grammar, and thunder grammar.

## Manual Sprite Batch V1

The current sprite set is a good seed grammar and should be extended derivationally rather than replaced with unrelated one-off icons.

Current parent shapes:

- `clear` is the bright sky parent for `sunny`
- `partly_cloudy` is a dedicated mixed sky/cloud parent and should stay visually special
- `cloudy` is the canonical rounded cloud body for precipitation-driven families
- the current dense `mist` asset reads closer to a first `fog` candidate than to true light mist
- `unknown` is the dim fallback parent

The first manual-tweak batch should add four new parent shapes:

- `clear_night`
- `very_cloudy`
- `overcast`
- a lighter true `mist`

Derivation rules for this batch:

- use `cloudy` as the precipitation parent for rain, snow, sleet, shower, and thunder families
- use rows `4-5` as the primary intensity-control surface
- differentiate `showers` from steady precipitation by localization, not just density
- reserve cloud-body changes for sky-coverage semantics such as `cloudy / very_cloudy / overcast / storm`

Recommended first manual batch targets:

- keep `clear`, `partly_cloudy`, `cloudy`, and `unknown` as current anchors
- normalize `clear` punctuation later if portability testing shows curly quotes are risky
- treat the current dense `mist` asset as a likely `fog` candidate
- add:
  - `clear_night`
  - `very_cloudy`
  - `overcast`
  - lighter `mist`
  - `light_showers`
  - `heavy_rain`
  - `heavy_snow`
  - `light_sleet`

First promoted runtime selection:

- the first live monochrome promotion pass now gives dedicated runtime shapes to:
  - `clear_night`
  - `very_cloudy`
  - `overcast`
  - `mist`
  - `fog`
  - `light_showers`
  - `light_rain`
  - `heavy_showers`
  - `heavy_rain`
  - `light_snow`
  - `heavy_snow`
  - `light_snow_showers`
  - `heavy_snow_showers`
  - `light_sleet`
  - `light_sleet_showers`
  - `sleet`
  - `thundery_showers`
  - `thundery_heavy_rain`
  - `thundery_snow_showers`
  - `storm`
- `sunny`, `partly_cloudy`, `cloudy`, and `unknown` remain anchored to the selected current-shape family
- this is still a first-pass monochrome atlas, so color-role tuning and any second-pass silhouette refinement remain separate follow-up work

Recommended first comparative popup rows:

- `Sunny | Clear Night | Partly Cloudy | Cloudy | Very Cloudy | Overcast | Mist | Fog | Unknown`
- `Light Showers | Light Rain | Heavy Showers | Heavy Rain | Light Snow | Heavy Snow | Light Sleet | Sleet`
- `Thundery Showers | Thundery Heavy Rain | Thundery Snow Showers | Thunderstorm`

This batch remains an offline/manual-art exercise. Approved sprite work should still be normalized back into plain-text assets plus semantic YAM color-role mapping before it becomes part of the live atlas.

## Palette Roles

The widget should stay inside the shared BTAS/TNBA palette contract documented in [`theme.md`](theme.md).

Recommended weather-role anchors:

- panel background: `#1E2124`
- panel lifted surface: `#262A2E`
- divider / low-contrast cloud shadow: `#373C41`
- primary text / snow: `#E6E8EB`
- secondary text / cloud edge: `#A0A6AD`
- rain: `#355FA8`
- heavy rain: `#4B7BD6`
- sun core: `#C48A2C`
- sun ray / lightning warmth: `#A67C34`
- severe alert: `#A33A32`
- calm/fresh green accents: `#3E7357` / `#4F8E6C`

Current runtime note:

- the theme layer now exposes distinct semantic handles for `rain`, `heavy rain`, `sun core`, `sun ray`, `lightning`, and `alert`, so atlas role maps can differentiate thunder-family glyphs without baking literal colors into sprite assets

Configurability rules:

- the weather widget should not hard-freeze one palette forever
- palette application should be semantic and swappable at runtime or through settings/config later
- sprite parts, text emphasis, dividers, and panel/background treatment should all be themeable through explicit roles rather than literal colors embedded in widget code

## Display Modes

The first design pass should assume explicit modes:

- `Off`
- `Compact`
- `Panel`
- `Debug`

Intent:

- `Off` means no fetch and no render.
- `Compact` is the likely first production mode.
- `Panel` is for richer text plus sprite presentation.
- `Debug` is for raw provider facts, cache state, and normalized field inspection.

## Layout Configurability

The internal layout should be intentionally configurable.

At minimum, the weather widget should be able to vary:

- sprite-first vs text-first balance
- compact single-column vs lightly stacked internal arrangement
- metric selection and ordering
- label verbosity
- palette/theme role mapping

Rules:

- the main-scene default should stay compact, beautiful, and clock-adjacent
- richer layouts belong to explicit alternate modes such as `Panel` or `Debug`
- layout customization should operate on semantic slots rather than ad hoc per-provider formatting strings

## Localization

If the widget uses human-readable text, localization should be a first-class seam rather than a late textual patch.

Rules:

- provider wording is not the final UI wording
- localized weather labels should resolve from normalized weather state, preferring condition-code-specific labels when available and falling back to generalized `WeatherVisual` labels otherwise
- English and Polish should both be supportable by the presentation layer
- Polish does not need to be the default, but the infrastructure should make it available
- abbreviated metric rows may remain language-light, but any named weather condition should have a localized path
- the compact widget now prefers a two-line condition header for longer localized states, a dedicated day/night temperature row rendered as two compact icon/value units (` 8C |  20C`), a wind row, and a precipitation row that disappears when there is no measurable precipitation
- the compact wind row should use a direction-aware arrow glyph rather than a fixed up-arrow, so `N / NE / E / SE / S / SW / W / NW` families remain legible at a glance even in the narrow panel

Recommended direction:

- keep locale as an explicit weather-widget presentation setting
- map `WeatherVisual` to localized human-facing fallback labels
- avoid hard-coding provider-origin strings as the canonical visible condition text
- when `wttr.in` condition codes are available, prefer their upstream translation-table wording for locale-specific condition labels; for Polish, `share/translations/pl.txt` is now the reference wording source for numeric conditions, while YAM keeps a separate generalized visual-label fallback for non-code cases

## Recommended Rollout

Stage 1:

- add `weather/` module boundaries
- define `WeatherSnapshot` and `WeatherVisual`
- implement one provider, preferably `wttr.in`
- add a compact sprite atlas v0
- render one compact `WeatherLayer`

Stage 2:

- add panel/debug modes
- add cache TTL and stale-state behavior
- support provider swap or dual-provider experiments

Stage 3:

- optionally let normalized weather facts drive scene ambience such as rain, snow, fog, or day/night tinting
- keep ambience optional and separate from widget ownership

## Non-Goals

- not a pasted third-party terminal weather block
- not a clock-owned sub-widget
- not a blocking network path in the render loop
- not a provider-specific UI contract
- not a bright generic weather-app palette that ignores YAM's BTAS/TNBA direction
