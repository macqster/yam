# YAM-RUST Render Contract

## Assertions

- Numeric layer ordering implements the conceptual layer model defined in [`scene-model.md`](scene-model.md).
- Debug border geometry is world-space data projected through the active camera.
- Hero frames must be fixed size before render.

## Change Impact

- If you change this, also review `docs/scene-model.md`, `docs/architecture.md`, and the frame-rendering tests.

Ratatui is an immediate-mode renderer.
Every frame is rebuilt from state and emitted as a complete terminal grid.

The active renderer treats ratatui as the final output adapter. Scene layers write into engine-owned `Grid` values first, and only the final composed grid is converted back into ratatui text.

## Layer Order

- L0 - field/background
- L10 - hero/entity
- L100 - anchored clock
- L300 - debug overlay
- L400 - settings popup
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
- shared projection helpers on `RenderState` are the source of truth for telemetry values that must match visible layer placement
- the clock is a world entity: debug/info panels report its projected screen position, but they do not define it

## Pipeline

- `runtime` receives input and ticks state
- all scene rendering passes through `render_scene`
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
- default follow-hero camera mode keeps the visible crop centered on the world datum across resizes; manual pan mode is still clamped in runtime state and in `build_render_state(...)` so the visible crop can overscan the world border/frame by at most one cell on any edge
- the default centered windowed start uses camera `(-62, -16)` for `124x32`
- debug border sampling is a datum-centered world-space probe that is projected through the active camera; it is not HUD chrome
- world-ui layers attach to world entities and resolve before screen-space overlay work
- hud-ui layers attach to the viewport/camera/terminal frame and do not inherit world motion directly
- the clock is a world entity: it follows the hero in world space and keeps its own relative offset
- the footer/status bar is hud-ui: it is screen-attached and does not inherit world motion
- world-ui features move only with world attachment/projection, while hud-ui features stay terminal-fixed
- fullscreen is a special case of the camera contract: when the viewport matches or exceeds the world extent, the visible crop should be static and centered on the world datum `(0, 0)`, even if debug controls still mutate the stored camera position
- fullscreen lock is now exercised in `build_render_state(...)`: the stored camera can still move, but the frame uses a datum-centered crop whenever the terminal fully covers the world extent
- `RenderState::clock_screen()` is the shared projected clock position used by both the clock layer and the debug overlay
- `resolve_world_ui(...)` is the helper for world-attached elements that stay pinned in world space
- `resolve_hud_ui(...)` is the helper for screen-attached overlays
- footer placement is intentionally the bottom row of the HUD frame via `footer_row(height)`
- the footer bar is a full-width highlighted strip in soft dark green, and its text is rendered as dark inverse content on top of that bar
- the default footer help is a compact `[q]uit • [d]ev mode` hint with the version stamp right-aligned, and the dev-mode footer keeps the same compact punctuation style for the runtime controls
- the debug overlay can include passive camera/world scrollbar indicators anchored one cell inward from the terminal edge; they are read-only, derived from `RenderState`, rendered as a minimal dark-blue gauge using `┄`/`═` horizontally and `┊`/`║` vertically, and sized/positioned from camera origins normalized across the world range so they report camera/world placement rather than acting like a scrollable panel
- the debug info panel stays compact and reports only the live control facts needed for resize and entity-edit checks: FPS, frame, play state, camera mode, move mode/target, camera position, hero world/screen position, hero visibility, clock world/screen position, and clock visibility
- the dev-mode footer stays compact and uses `[h]otkeys` to open the modal hotkeys popup, where camera centering and other developer controls are described
- dev mode and settings-style presentation flags are metamechanics inputs; they are consumed by the scene layers, not rendered outside the pipeline
- the settings popup is a modal overlay rendered in the overlay layer; it uses tabbed sections for positions, widgets, gif, and theme values
- the hotkeys popup is a modal overlay rendered between debug and move/settings; it lists the current developer controls without adding footer clutter
- the move popup is a modal overlay rendered between hotkeys and settings; it makes entity movement explicit with `1/2/3` selection and `hjkl` movement
- the dev-mode footer also uses `[m]ove` to open the modal move popup; while it is open, `1/2/3` select the active entity target and `hjkl` move that target
- the move popup shows the active target and keeps entity movement explicit instead of spreading more hotkeys into the footer

This is the contract the current code follows. It is intentionally narrower than the older projection notes in the research bundle, which discuss center-based camera framing.

## Masks

Mask values use compositor semantics:

- `true` means a top-layer write is allowed
- `false` means a top-layer write is blocked

Current mask behavior is intentionally limited. The hero layer can emit a silhouette mask, and the scene applies that mask only to the field layer as a verification probe. HUD and debug layers do not consume the hero mask.

## Text And Geometry Caveats

- Hero source GIF is `820x820` pixels and is rendered into a fixed `96x48` cell footprint.
- That `96x48` target is the current layout result used to preserve the GIF's proportions in terminal cell space.
- GIF subimage frames are expanded and flattened onto an opaque full `820x820` logical canvas before chafa rendering so partial frames, including frames 15 and 30, cannot stretch vertically.
- Hero frames must remain fixed width and fixed height before render.
- Hero rendering must not use ratatui wrapping.
- Hero rendering uses the chafa-backed frame conversion path; cached-frame ownership remains a future migration option if measurable instability returns.
- `write_string` currently iterates `char`s, not display-width-aware graphemes.
- `grid_to_lines` groups adjacent cells by style.
- Clock attachment on the active path is world-pinned: the clock follows the hero in world space and keeps its own hero-relative offset. It does not inherit camera, viewport, or terminal motion directly.
- Debug info that prints clock position is reporting the projected world-entity clock position, not a screen-attached UI placement.
- Debug world borders are rendered as a stable ASCII 2x2 datum-centered indicator in world space, so they move with camera panning and remain a debug view of the real world bounds. It keeps one top padding row and one side padding cell for symmetry, and those margins are intentional and reserved for future UI placement. The bottom one-row padding is currently occupied by the footer.
- The world itself keeps a 1-cell inset boundary, and the HUD/viewport overlay layer also keeps a 1-cell inset boundary where needed for future UI elements.

## Current Risks

- Legacy helper functions remain in `src/render/hero.rs`, but the active scene path uses layer grids and `LayerOutput`.
- Fullscreen lock should remain a structural invariant: the code should treat fullscreen as an immovable, datum-centered crop, not just a larger windowed viewport.

## Known Gaps

- Masking is limited to the probe behavior described above.
- There is no generalized scene-wide mask pipeline yet.
