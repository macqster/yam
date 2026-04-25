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
- `RenderState` is split into `world` and `hud` sections to keep world-pinned attachments separate from screen-attached overlays

## Pipeline

- `runtime` receives input and ticks state
- `render_scene` builds a temporary `Scene`
- `Scene::render` uses the full terminal area for viewport and viewport rect values
- `Scene::render` also computes a single read-only `RenderState` for hero/clock/debug values
- `Scene::render` builds `RenderState` through `build_render_state(...)`, which is covered by a resize-invariance test
- each layer writes to a full-frame `Grid`
- scene captures the hero mask, currently applying it only to field output
- scene merges all grids into `final_grid`
- scene clears the frame and draws final lines

## Current Camera Contract

The active implementation treats camera as a viewport crop helper:

- world positions are defined around the `(0, 0)` datum
- the world quadrants are sign-defined around that datum
- world coordinates use Cartesian orientation (`y` increases upward)
- terminal/screen coordinates use terminal orientation (`y` increases downward)
- camera is the world-space origin of the visible crop
- viewport is the terminal-sized crop rectangle that follows camera
- `Viewport::from_camera` copies camera coordinates directly as the visible crop origin
- debug border sampling is a datum-centered screen-stable probe, not a camera-driven crop
- world-ui layers attach to world entities and resolve before screen-space overlay work
- hud-ui layers attach to the viewport/camera/terminal frame and do not inherit world motion directly
- the clock is world-ui: it follows the hero in world space and keeps its own relative offset
- the footer/status bar is hud-ui: it is screen-attached and does not inherit world motion
- `resolve_world_ui(...)` is the helper for world-attached elements that stay pinned in world space
- `resolve_hud_ui(...)` is the helper for screen-attached overlays
- footer placement is intentionally the bottom row of the HUD frame via `footer_row(height)`

This is the contract the current code follows. It is intentionally narrower than the older projection notes in the research bundle, which discuss center-based camera framing.

## Masks

Mask values use compositor semantics:

- `true` means a top-layer write is allowed
- `false` means a top-layer write is blocked

Current mask behavior is intentionally limited. The hero layer can emit a silhouette mask, and the scene can apply that mask to the field layer as a verification probe. This is not yet a general occlusion system.

## Text And Geometry Caveats

- Hero source GIF is `820x820` pixels and is rendered into a fixed `96x48` cell footprint.
- That `96x48` target is the current layout result used to preserve the GIF's proportions in terminal cell space.
- Hero frames must remain fixed width and fixed height before render.
- Hero rendering must not use ratatui wrapping.
- `write_string` currently iterates `char`s, not display-width-aware graphemes.
- `grid_to_lines` groups adjacent cells by style.
- Clock attachment on the active path is world-pinned: the clock follows the hero in world space and keeps its own hero-relative offset. It does not inherit camera, viewport, or terminal motion directly.
- Debug world borders are rendered as a stable ASCII 2x2 datum-centered indicator in world space, so they move with camera panning and remain a debug view of the real world bounds. It keeps one top padding row and one side padding cell for symmetry, and those margins are intentional and reserved for future UI placement. The bottom one-row padding is currently occupied by the footer.
- The world itself keeps a 1-cell inset boundary, and the HUD/viewport overlay layer also keeps a 1-cell inset boundary where needed for future UI elements.

## Current Risks

- Camera math is not yet a single-source contract across hero, field, viewport, and debug border rendering.
- The active grid path coexists with legacy frame-render methods.
- Some debug/attachment values are reconstructed from the per-frame `RenderState` snapshot rather than written through render-time `UiState` side effects.
