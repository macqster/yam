# Scene Config

## Assertions

- `scene_config.json` is active for the visualizer / experimental tooling path.
- Its current hero asset path, dimensions, offsets, and theme are intentional defaults for that tooling path, not placeholder values.
- Changes here should be reflected in the tooling and the docs together.

## Authority

- `scene_config.json` is authoritative for the visualizer / experimental tooling.
- `scene_config.json` is not authoritative for the Rust renderer under `src/render/*`.

This note owns the active, repo-tracked default scene configuration for the visualizer / experimental tooling path.

## What It Controls

- clock font and formats
- hero GIF path and placement
- theme selection

## Rule

- Keep this file small and explicit.
- Update it when the default visualizer/experimental scene surface changes.
- Treat it as active configuration, not example material.
- If the tooling ever stops using it, move it to the archive or to a legacy tool area and document the replacement.
