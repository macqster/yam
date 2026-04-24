# YAM-RUST Architecture Contract

## Core Rules

- `core/` - data only, no UI, no terminal, no rendering
- `systems/` - mutate `WorldState` only, no rendering
- `render/` - terminal render primitives, chafa/hero conversion, grid composition, masks, and final text conversion
- `scene/` - layer ordering, camera/viewport types, coordinate helpers, and scene-level grid composition
- `ui/` - runtime UI state, persisted offsets/settings, screen-space widgets, and temporary scene adapter
- `runtime.rs` - event loop, input, tick, and render orchestration only

## Forbidden Coupling

- `core -> ui`
- `core -> render`
- `systems -> ui`
- `systems -> render`
- `render -> world mutation`
- `scene layers -> persistent state mutation`
- `ui widgets -> world mutation`

## Rendering Pipeline

- runtime draw closure calls `render_scene`
- `Scene` builds ordered layers through `ui::scene::build_ui_layers`
- `Scene` computes a read-only `RenderState` once per frame and passes it to every layer
- each layer emits a full-frame `LayerOutput`
- `Scene` merges layer grids with `render::compositor::merge_grid`
- `Scene` converts the final grid into ratatui `Line`s
- ratatui receives one final `Paragraph` for the frame
- scene rendering now uses the full terminal area for viewport and viewport-rect values; the earlier centered tiered viewport box is no longer used to place layers

## Active Layers

- field/background: `z_index = 0`
- hero/entity: `z_index = 10`
- clock/anchored UI: `z_index = 100`
- debug overlay: `z_index = 300`
- status/footer: `z_index = 1000`

## Coordinate Contract

The intended model is:

- world space: simulation/object positions
- camera space: projection offset from world to viewport
- anchor space: offsets relative to another rendered object
- screen space: fixed terminal overlay positions

The current implementation does not fully enforce that model yet. Camera semantics are split between top-left-offset math and center-point viewport math. New features should not build on those mixed semantics until the camera contract is resolved.

## Known Architectural Debt

- `Layer::render(...)` remains as a legacy API while `Layer::render_to_grid(...)` is the active path.
- Hero and clock layers read from the per-frame `RenderState`; they no longer publish render-derived values into `UiState` on the active path.
- `coords::Space` exists but is not yet the authoritative position resolver.
- Masks are present but are still a probe, not a complete scene-wide occlusion policy.
