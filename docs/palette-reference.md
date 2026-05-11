# Palette Reference

This is YAM's human-friendly palette reference guide.

It exists so YAM-side theme, widget, weather, flora, and future scene work can refer to one readable summary without confusing:

- extracted BTAS source swatches
- curated workstation palette choices
- YAM runtime semantic tokens
- shared primitive palette aliases

## Source Chain

YAM should understand the current palette chain like this:

1. extracted source SSOT in `~/_git/dotfiles/themes/yam_btas_extracted_swatches.md`
2. curated workstation master palette in `~/_git/dotfiles/themes/btas-tnba_color_palette.md`
3. YAM runtime semantic theme layer in:
   - `src/theme/btas.rs`
   - `src/theme/palette.rs`
   - `src/theme/style.rs`
4. adopted visual packet asset in:
   - `docs/btas_tnba_color_system_canonical_packet.pdf`

This file is the YAM-side reader guide for that chain.

## Naming layers

YAM should read the shared palette through three layers:

1. display label:
   human-facing names used in the PDF packet and palette docs
2. primitive alias:
   stable machine-friendly names such as `graphite`, `gray_light`, `green_root`, `blue_bright`, and `amber_dusty`
3. runtime semantic token:
   YAM-facing role names such as `PANEL_BG`, `PRIMARY_FG`, `WEATHER_RAIN_HEAVY`, or future domain-specific helpers

Important boundary:

- primitive aliases describe the curated color values themselves
- YAM semantic tokens describe how YAM uses those values

That means YAM can share the same palette vocabulary as dotfiles without collapsing into raw-hex-at-call-site usage.

## Current Shared Palette Anchors

### Surfaces

| Display label | Primitive alias | Hex | Current YAM usage |
| --- | --- | --- | --- |
| Background | `blackish` | `#16181A` | Sandbox palette sheet and future full-scene dark backdrop candidate. |
| Sidebar | `blackish_alt` | `#181A1D` | Reserved darker side-surface variant for future workstation-aligned shells. |
| Panel | `graphite` | `#1E2124` | Main dark surface anchor. |
| Lifted | `gray_dark` | `#262A2E` | Raised surface / active dark block. |
| Divider | `gray` | `#373C41` | Separators, muted structure, cloud shadow/fog support. |
| Secondary text | `gray_light` | `#A0A6AD` | Semantic `SECONDARY_FG`. |
| Primary text | `whitish` | `#E6E8EB` | Semantic `PRIMARY_FG`. |

### Cool accents

| Display label | Primitive alias | Hex | Current YAM usage |
| --- | --- | --- | --- |
| Deep Blue | `blue_deep` | `#1E2F5C` | Held as a deeper cool anchor; not heavily surfaced yet. |
| Subtle Navy | `blue_navy` | `#243B73` | Selected settings row background. |
| Balanced Blue | `blue_balanced` | `#2C4C8A` | Structural accent, popup title/footer chrome family. |
| Bright Blue | `blue_bright` | `#355FA8` | Selection/weather/rain/info emphasis. |
| Highlight Blue | `blue_highlight` | `#4B7BD6` | Heavy-rain/highlight family. |

### Warm accents

| Display label | Primitive alias | Hex | Current YAM usage |
| --- | --- | --- | --- |
| Auburn | `auburn` | `#8A3B2E` | Deep warm anchor; not heavily surfaced yet. |
| Brick Red | `brick` | `#A33A32` | Strong warning/accent family. |
| Rust | `rust` | `#B24E2E` | Inactive warm tab/accent family. |
| Ochre | `ochre` | `#C48A2C` | Marker / warm highlight / sun-core family. |
| Dusty Amber | `amber_dusty` | `#A67C34` | Softer warm support / sun-ray family. |

### Greens

| Display label | Primitive alias | Hex | Current YAM usage |
| --- | --- | --- | --- |
| Deep Green | `green_deep` | `#1F3B2C` | Deeper green anchor; not heavily surfaced yet. |
| Soft Green | `green_soft` | `#2F5A45` | Modal border chrome family. |
| Balanced Green | `green_balanced` | `#3E7357` | Calm green emphasis. |
| Root Green | `green_root` | `#4F8E6C` | Fresher flora/cursor/highlight family. |
| Bright Green | `green_bright` | `#69C090` | High-energy green highlight candidate for future accents. |

## Current YAM Interpretation Rule

YAM should not treat the palette as a raw table to paste into render code.

Instead:

- `src/theme/btas.rs` carries the color bundle
- the shared palette docs expose primitive aliases
- `src/theme/palette.rs` exposes YAM semantic aliases
- `src/theme/style.rs` exposes reusable style helpers

If a YAM surface needs color, the question should be:

- which semantic role is this?

not:

- which raw hex should I type here?

## Weather Note

The weather widget currently uses the shared palette through semantic weather roles instead of embedding palette choices in the sprite assets.

That means:

- monochrome sprite files stay palette-free
- weather token changes can happen in the theme layer
- future color tuning can happen without reopening the glyph assets themselves

See also:

- `docs/weather-widget.md`
- `docs/theme.md`

## Flora Note

The current palette also leaves room for future flora refinement:

- balanced/root greens as living-system anchors
- ochre/rust/brown tones as scaffold/bark support
- muted neutrals for non-dominant linework and debug overlays

The source SSOT contains stronger source-aligned flora candidates, but not all of them belong in the current curated workstation palette.

## Accuracy Rule

This guide is intentionally safer than the earlier one-page palette sheet experiment.

It does not claim:

- that all extracted BTAS scan colors are already canonical runtime colors
- that every listed role is final forever
- that one human-readable sheet can replace the source SSOT plus semantic theme layer

It does claim:

- the current curated palette family
- the primitive alias layer YAM can mirror from dotfiles
- the current YAM-facing anchor colors
- the correct relationship between extracted source, curated palette, and runtime semantic roles

For the current adopted shareable visual packet, see [`btas_tnba_color_system_canonical_packet.pdf`](btas_tnba_color_system_canonical_packet.pdf).

YAM now mirrors that relationship directly through a dev-mode palette popup that shows both the curated workstation palette and the extracted-source swatch group in one compact terminal inspection sheet.
