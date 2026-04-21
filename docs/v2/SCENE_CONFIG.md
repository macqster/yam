# YAM v2 Scene Config

This document records the explicit file-backed scene-level configuration surface.

## Fields

- `clock_font_name`
- `day_format`
- `gif_path`
- `hero_anchor`
- `hero_width`
- `hero_height`
- `hero_offset_x`
- `hero_offset_y`
- `clock_format`
- `theme_name`

## File

- `v2/scene_config.json`

## Commands

- `yam --scene-config show`
- `yam --scene-config edit`
- `yam --scene-config reset`
- `yam --scene-set key=value`

## Supported Updates

- `clock_font_name`
- `day_format`
- `gif_path`
- `hero_anchor`
- `hero_width`
- `hero_height`
- `hero_offset_x`
- `hero_offset_y`
- `clock_format`
- `theme_name`

## Defaults

- `clock_font_name`: `Fender`
- `day_format`: `%A, %d %B`
- `clock_format`: `%H:%M`
- `gif_path`: `v2/hero/assets/hero_go.gif`
- `hero_anchor`: `left`
- `hero_width`: `10`
- `hero_height`: `6`
- `hero_offset_x`: `0`
- `hero_offset_y`: `0`
- `theme_name`: `btas_dark_deco`

## Notes

- this is intentionally small and now serves the clock observer layer first
- the scene config still sits above engine and below launcher
- keep config explicit rather than implicit in runtime code
- the launcher loads the repo-tracked JSON file by default
- the live v2 scene reloads the config file when it changes
- the Bubble Tea runtime is canonical; the helper path is verification-only
- `clock_font_name` selects the live FIGlet clock font used by the Go runtime
- `clock_font_path` is only a legacy compatibility alias
- `gif_path` and `theme_name` remain for fallback compatibility and future scene expansion
- `hero_anchor` and the hero size/offset fields control the explicit hero placement contract for the live scene
- the live day label is rendered in Polish date form by default: `wtorek, 21 kwietnia`
