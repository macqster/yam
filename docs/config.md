# Scene Config

## Assertions

- `scene_config.json` is active for the visualizer / experimental tooling path.
- Its current hero asset path, dimensions, offsets, and theme are intentional defaults for that tooling path, not placeholder values.
- Changes here should be reflected in the tooling and the docs together.

## Mental Model

- `scene_config.json` is a tooling preset, not a Rust runtime source of truth.

## Authority

- `scene_config.json` is authoritative for the visualizer / experimental tooling.
- `scene_config.json` is not authoritative for the Rust renderer under `src/render/*`.
- Hero rendering fields in `scene_config.json` are ignored by the Rust renderer and only matter to the tooling path.

This note owns the active, repo-tracked default scene configuration for the visualizer / experimental tooling path.

## Terminal Baseline

- Ghostty default config currently uses `JetBrainsMono Nerd Font` with ligatures enabled, `font-size = 12`, and `font-thicken = true`
- `adjust-cell-height` is currently commented out, with `8%` noted as the candidate value if cell metrics need tuning later
- the default Ghostty window size is `124x32` cells, and this is the boot/start frame size YAM uses when it opens
- these settings are environment notes for layout and glyph-appearance tuning, not Rust runtime source of truth

## What It Controls

- clock font and formats
- hero GIF path and placement
- theme selection

## Rule

- Keep this file small and explicit.
- Update it when the default visualizer/experimental scene surface changes.
- Treat it as active configuration, not example material.
- If the tooling ever stops using it, move it to the archive or to a legacy tool area and document the replacement.
