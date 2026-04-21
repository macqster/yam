# YAM Runtime Tree

This directory contains the live runtime modules and support code for the current YAM terminal scene.

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

Use `python3 -m v2.check_golden` to compare the live output against the documented golden frame.
