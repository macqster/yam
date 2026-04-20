# YAM v2 Scene Config

This document records the explicit file-backed scene-level configuration surface.

## Fields

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

- `gif_path`
- `clock_format`
- `theme_name`

## Defaults

- `gif_path`: `visualizer/assets/source.gif`
- `clock_format`: `%H:%M`
- `theme_name`: `btas_dark_deco`

## Notes

- this is intentionally small
- the scene config sits above engine and below launcher
- keep config explicit rather than implicit in runtime code
- the launcher loads the repo-tracked JSON file by default
- the live v2 scene reloads the config file when it changes
- both the Bubble Tea default and the Python fallback read the same file-backed JSON
