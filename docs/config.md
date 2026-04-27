# Scene Config

`scene_config.json` is the active, repo-tracked default scene configuration for the visualizer / experimental runtime path.

## What It Controls

- clock font and formats
- hero GIF path and placement
- theme selection

## Rule

- Keep this file small and explicit.
- Update it when the default visualizer/runtime scene surface changes.
- Treat it as active configuration, not example material.
- If the runtime ever stops using it, move it to the archive or to a legacy tool area and document the replacement.
