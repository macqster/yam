# YAM-RUST Architecture Contract

## Core Rules

- `core/` - data only, no UI, no terminal, no rendering
- `systems/` - mutate `WorldState` only, no rendering
- `render/` - pure drawing, no state mutation
- `ui/` - composition, camera, viewport
- `runtime.rs` - event loop, input, tick, and render orchestration only

## Forbidden Coupling

- `core -> ui`
- `core -> render`
- `systems -> ui`
- `render -> world mutation`
- `ui -> world mutation`

## Rendering Pipeline

- `world -> viewport -> terminal`
- no direct `world -> terminal` access
