# YAM-RUST Architecture Contract

## Assertions

- Ownership and data flow belong here, not in scene-model, rendering, TODO, or archive docs.
- `RenderState` is built once per frame and then treated as read-only.

## Change Impact

- If you change this, also review `docs/scene-model.md`, `docs/rendering.md`, and the `RenderState` tests.

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
- `metamechanics -> world mutation outside the explicit UI state seam`

## Rendering Pipeline

- runtime draw closure calls `render_scene`
- `Scene` builds ordered layers through `ui::scene::build_ui_layers`
- `Scene` computes a read-only `RenderState` once per frame and passes it to every layer
- `RenderState` is split into:
  - `world`: hero and clock attachment facts that stay world-pinned
  - `hud`: viewport and camera facts that stay screen/terminal-attached
- `UiState` owns the runtime attachment offsets that feed the hero/clock attachment object
- hero and clock attachment facts are computed through explicit `scene::entity::HeroClockAttachment` produced by `scene::entity::hero_and_clock_poses(...)`
- each layer emits a full-frame `LayerOutput`
- `Scene` merges layer grids with `render::compositor::merge_grid`
- `Scene` converts the final grid into ratatui `Line`s
- ratatui receives one final `Paragraph` for the frame
- scene rendering uses the full terminal area for viewport and viewport-rect values
- the scene model contract lives in [`scene-model.md`](scene-model.md) and defines the deterministic layer/space/masking rules above ratatui
- the presentation stack is conceptualized as world -> HUD -> overlay, with overlays reserved for modal or top-z-index panels
- metamechanics is a subordinate control/observation seam inside `ui/`; it may toggle overlays or presentation flags, but it does not own world state, projection, or render order
- `dev_mode` is the umbrella metamechanics toggle: it enables the layout/editing surface and the debug overlay, while `debug` remains the actual diagnostic presentation
- `settings` is the modal metamechanics popup: it shows tabbed, dev-mode controls for positions, widgets, gif, and theme values without owning world state or projection
- the clock is not a UI entity: it is a world-attached hero companion, and the debug/info panels only observe its projected screen position

## Out of Scope

- This document does not define conceptual scene categories; see [`scene-model.md`](scene-model.md).
- This document does not define numeric layer meaning in detail; see [`rendering.md`](rendering.md).

## Active Layers

- field/background: `z_index = 0`
- hero/entity: `z_index = 10`
- clock/world entity: `z_index = 100`
- debug overlay: `z_index = 300`
- settings popup: `z_index = 400`
- status/footer: `z_index = 1000`

## Presentation Contract

- world systems are the scene and world-attached content
- HUD systems are screen-attached footers, indicators, and passive debug
- overlay systems are modal or top-z-index panels such as settings or active debug UI
- terminal layout should be treated as a framebuffer, not a panel dashboard

## Coordinate Contract

The intended model is:

- datum/origin: `(0, 0)`
- world is centered around the datum
- the world quadrants are sign-defined around that datum:
  - top-left: `(-x, -y)`
  - top-right: `(x, -y)`
  - bottom-left: `(-x, y)`
  - bottom-right: `(x, y)`
- world coordinates use Cartesian orientation: `x` increases to the right, `y` increases upward
- terminal/screen coordinates use the usual terminal orientation: `x` increases to the right, `y` increases downward
- world space: simulation/object positions
- camera: world-space origin of the visible crop, not the viewport itself
- viewport: terminal-sized crop rectangle, not the camera itself
- windowed camera crops are clamped to one cell of overscan beyond the world border/frame
- the default `124x32` windowed crop starts at camera `(-69, -17)`
- anchor space: offsets relative to another rendered object
- screen space: fixed terminal overlay positions
- world border and HUD border each keep a 1-cell inset where needed for symmetry and future UI placement
- world-ui elements stay tied to world entities and follow the world contract
- hud-ui elements stay tied to viewport/camera/terminal position and follow the screen contract
- clock is treated as a world entity: it stays tied to the hero in world space and carries its own hero-relative offset
- footer/status is treated as hud-ui: it lives in screen space alongside hotkeys and version/build labels
- the repo now exposes explicit helpers for both sides of that split:
- `resolve_world_ui(...)` resolves anchor + offset in world space and stays world-pinned
- `resolve_hud_ui(...)` keeps hud values screen-attached and camera-independent
- the footer row is intentionally the bottom terminal row; `footer_row(height)` encodes that contract
- projection is defined in `docs/scene-model.md` and applied by the renderer

## Hero Geometry Contract

- source hero GIF: square `820x820` pixels
- terminal render target: fixed `96x48` cells
- decoded GIF subimage frames are expanded and flattened to an opaque full `820x820` logical canvas before terminal conversion
- the target is a layout/scaling result, not a raw pixel-to-cell division
- hero world anchor: `(0, 0)` when centered in world space
- hero visual center should cross the datum, while the rendered cell footprint remains `96x48`
- the world retains a 1-cell inset boundary for world-ui border work, and the active HUD/border layout also preserves a 1-cell inset for overlay/UI work
- world-ui should not be repositioned by camera semantics after it is anchored in world space
- hud-ui should not inherit world coordinates directly; it should use viewport/screen positioning
- the debug border probe is a datum-centered world-border indicator that is rendered in world space and therefore moves with camera panning

The remaining architectural gap is that `coords::Space` is not yet the authoritative resolver for all placement paths. Camera semantics are intentionally treated as a viewport crop helper on the active path; new features should not invent a second meaning for camera or viewport.

## Known Architectural Debt

- Historical `Layer::render(...)` references are archive-only; the active layer API is `Layer::render_to_grid(...)`.
- Hero and clock layers read from the per-frame `RenderState`; on the active path they are world-pinned and do not move with camera projection.
- `coords::Space` exists but is not yet the authoritative position resolver.
- Masks are present but are still a probe, not a complete scene-wide occlusion policy.
