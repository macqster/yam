# YAM Runtime Tree

This directory contains the supporting runtime packages while the repo is flattened toward the root.

## Layer Map

- `runtime/` - application loop and message routing
- `engine/` - ecosystem, environment, lifecycle, species, and balance
- `morphology/` - structural translation from growth to space
- `shape/` - glyph-level visual grammar
- `render/` - layers, masks, framebuffer, and emitters
- `ui/` - panels, focus, keybindings, and commands
- `theme/` - palette and visual identity

## Operating Rule

Keep simulation, rendering, and UI separated.

Do not add behavior here unless it belongs to one of the declared layers.

## Verification

Use `python3 -m check_golden` to compare the live output against the documented golden frame.
