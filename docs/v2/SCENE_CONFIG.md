# YAM v2 Scene Config

This document records the explicit file-backed scene-level configuration surface.

## Fields

- `clock_font_path`
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

- `clock_font_path`
- `day_format`
- `gif_path`
- `clock_format`
- `theme_name`

## Defaults

- `clock_font_path`: `v2/assets/fonts/Gothic.flf`
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
- both the Bubble Tea default and the Python fallback read the same file-backed JSON
- `clock_font_path` is retained for compatibility and future typography work, but the current default clock scene uses the native grid renderer in both Go and Python paths
- `gif_path` and `theme_name` remain for fallback compatibility and future scene expansion, but they do not affect the current default clock scene
- the start screen also prints a plain `0 1 2 3 4 5 6 7 8 9` control row under the day label
