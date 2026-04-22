# YAM Scene Config

This document records the explicit file-backed scene-level configuration surface.

## Fields

- `clock_font_name`
- `day_format`
- `clock_format`
- `theme_name`

## File

- `scene_config.json`

## Commands

- `yam --scene-config show`
- `yam --scene-config edit`
- `yam --scene-config reset`
- `yam --scene-set key=value`

## Supported Updates

- `clock_font_name`
- `day_format`
- `clock_format`
- `theme_name`

## Defaults

- `clock_font_name`: `Fender`
- `day_format`: `%A, %d %B`
- `clock_format`: `%H:%M`
- `theme_name`: `btas_dark_deco`

## Notes

- this is intentionally small and now serves the clock observer layer first
- the scene config still sits above engine and below launcher
- keep config explicit rather than implicit in runtime code
- the launcher loads the repo-tracked JSON file by default
- the live scene reloads the config file when it changes
- the Bubble Tea runtime is canonical; the helper path is verification-only
- `clock_font_name` selects the live FIGlet clock font used by the Go runtime
- the live day label is rendered in Polish date form by default: `wtorek, 21 kwietnia`
