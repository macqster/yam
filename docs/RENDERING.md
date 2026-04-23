# YAM-RUST Render Contract

Ratatui is an immediate-mode renderer.
Every frame is rebuilt from state.
Draw order defines visual layering.

## Layer Order

- L0 - world, field, background
- L1 - hero
- L2 - UI overlays, clock, panels
- L3 - debug, layout, text
- L4 - particles, future
- L5 - scaffolding, future top-most

## Rules

- later layers overwrite earlier ones
- no element may render outside its assigned layer
- no implicit z-order exists
- order is the visual truth

## Pipeline

- world -> viewport -> terminal buffer
- every world-space render must pass through `Viewport::world_to_view()`
