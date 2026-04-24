# Coordinate Audit - 2026-04-24

This audit records the current state after the grid renderer, persistent UI offsets, debug controls, and mask probe work.

## Stable Invariants

- Rust is the active runtime.
- Legacy Python code is isolated under `tools/legacy-python/`.
- Scene rendering now goes through grid composition.
- Hero frames are intended to be normalized to fixed dimensions before rendering.
- Clock font, hero FPS, camera position, and offsets are persisted in `~/.config/yam/state.json`.
- Debug-only controls are routed through runtime input handling.

## Highest Priority Issues

### 1. Camera Semantics Are Mixed

Observed contracts:

- Hero/clock projection uses camera as a world offset: `screen = world - camera`.
- `Viewport::from_camera` treats camera as a center point by subtracting half the viewport.
- Debug border rendering also treats camera as a center point.

This must be unified before adding more world-space features. Pick one camera contract and rewrite all projections to match it.

### 2. Render-Time State Side Effects

Hero and clock layers publish values such as `hero_visual_anchor` and `clock_final` into `UiState` while rendering. This makes clock/debug behavior depend on layer order.

Recommended correction: introduce a per-frame render context or frame facts struct that is produced before layers render and passed read-only to dependent layers.

### 3. Anchor Space Is Not Formal Yet

`coords::Space` and `resolve_position` exist, but `Space::Anchor` does not resolve through an entity registry. Clock attachment is still hand-coded.

Recommended correction: add a small explicit resolver for:

- world position
- screen projection
- anchor-to-world or visual-anchor-to-screen relationships

### 4. Hero World Origin Is Ambiguous

Layer code currently uses default/sentinel logic around `(0, 0)`. That prevents `(0, 0)` from being treated as a valid world position.

Recommended correction: store explicit defaults in `UiOffsets` and remove sentinel behavior.

### 5. Viewport Tiering Can Cause Resize Jumps

`select_viewport_tier` and `viewport_rect` introduce discrete size tiers. This is useful for predictable layouts, but it can create discontinuities when resizing.

Recommended correction: document the viewport policy and ensure field, hero, clock, border, and debug layers all consume the same viewport contract.

## Medium Priority Issues

- The legacy `Layer::render(...)` method is still present.
- `UiOffsets` contains camera, clock font, hero FPS, and offsets; the name is no longer accurate.
- `write_string` is not display-width aware.
- `--check-updates` depends on `cargo outdated` being installed.
- `scripts/update.sh` updates the lockfile and rebuilds, but does not install optional helper tools.
- Mask behavior is a probe, not a complete policy.

## Do Not Build On Yet

Avoid adding these until the camera/anchor contract is settled:

- vines
- particles
- full mask routing
- screen/world hybrid weather effects
- mouse picking
- follow camera mode

## Recommended Next Fix Order

1. Define camera as either top-left viewport origin or viewport center.
2. Update hero, field, debug border, viewport, and clock projection to use that single camera contract.
3. Replace render-time `UiState` side effects with a frame facts/context object.
4. Remove the legacy `Layer::render(...)` API.
5. Add golden tests for fixed hero frame geometry and layer order.
