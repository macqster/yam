# YAM v2 Scene Config

This document records the explicit file-backed scene-level configuration surface.

## Fields

- `clock_font_name`
- `day_format`
- `gif_path`
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
- `clock_format`
- `theme_name`

## Defaults

- `clock_font_name`: `Fender`
- `day_format`: `%A`
- `clock_format`: `%H:%M`
- `gif_path`: `visualizer/assets/source.gif`
- `theme_name`: `btas_dark_deco`

## Notes

- this is intentionally small and now serves the clock observer layer first
- the scene config still sits above engine and below launcher
- keep config explicit rather than implicit in runtime code
- the launcher loads the repo-tracked JSON file by default
- the live v2 scene reloads the config file when it changes
- the Bubble Tea default is canonical; Python is a thin verification helper
- `clock_font_name` selects the live FIGlet clock font used by the Go runtime
- `clock_font_path` is only a legacy compatibility alias
- `gif_path` and `theme_name` remain for fallback compatibility and future scene expansion
