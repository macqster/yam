# YAM-RUST Render Contract

Ratatui is an immediate-mode renderer.
Every frame is rebuilt from state and emitted as a complete terminal grid.

The active renderer treats ratatui as the final output adapter. Scene layers write into engine-owned `Grid` values first, and only the final composed grid is converted back into ratatui text.

## Layer Order

- L0 - field/background
- L10 - hero/entity
- L100 - anchored clock
- L300 - debug overlay
- L1000 - status/footer

## Rules

- layers must emit `LayerOutput`
- `LayerOutput.grid` is the layer proposal for the full frame
- `LayerOutput.mask` is optional compositor data
- `Scene` sorts layers by `z_index`
- `merge_grid` is the only active cell merge path
- final output is rendered once through `Paragraph::new(grid_to_lines(&final_grid))`
- no layer should rely on ratatui layout wrapping for hero/image content
- viewport selection is now a full-frame pass; the old centered tiered viewport box no longer drives layer placement

## Pipeline

- `runtime` receives input and ticks state
- `render_scene` builds a temporary `Scene`
- `Scene::render` uses the full terminal area for viewport and viewport rect values
- `Scene::render` also computes a single read-only `RenderState` for hero/clock/debug values
- each layer writes to a full-frame `Grid`
- scene captures the hero mask, currently applying it only to field output
- scene merges all grids into `final_grid`
- scene clears the frame and draws final lines

## Current Camera Contract

The active implementation treats camera as a top-left world offset:

- `screen = world - camera`
- `Viewport::from_camera` copies camera coordinates directly
- debug border sampling uses the same top-left mapping

This is the contract the current code follows. It is intentionally narrower than the older projection notes in the research bundle, which discuss center-based camera framing.

## Masks

Mask values use compositor semantics:

- `true` means a top-layer write is allowed
- `false` means a top-layer write is blocked

Current mask behavior is intentionally limited. The hero layer can emit a silhouette mask, and the scene can apply that mask to the field layer as a verification probe. This is not yet a general occlusion system.

## Text And Geometry Caveats

- Hero frames must remain fixed width and fixed height before render.
- Hero rendering must not use ratatui wrapping.
- `write_string` currently iterates `char`s, not display-width-aware graphemes.
- `grid_to_lines` groups adjacent cells by style.
- Clock attachment must preserve `clock_screen = hero_visual_anchor + clock_offset`; visibility should clip, not clamp, the relationship.

## Current Risks

- Camera math is not yet a single-source contract across hero, field, viewport, and debug border rendering.
- The active grid path coexists with legacy frame-render methods.
- Some debug/attachment values are produced by render-time side effects into `UiState`.
