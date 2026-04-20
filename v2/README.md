# YAM v2 Source Tree

This directory is the native source layout for the YAM v2 rebuild.

## Status

This tree is a scaffold only.

It exists to define package boundaries, ownership, and the intended flow of data before implementation starts.

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

Use `python3 -m v2.check_golden` to compare the live scaffold output against the documented golden frame.
