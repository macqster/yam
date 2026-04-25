# Repo Audit

Date: 2026-04-24

## Summary

The repository tracks the Rust terminal scene engine as the active implementation.
The older `/Users/maciejkuster/_git/yam` checkout has been removed to avoid split-tree confusion.

Legacy Python systems are isolated under `tools/legacy-python/` and are not part of the Rust runtime.

The current Rust runtime has moved from direct ratatui widget rendering toward a scene-owned grid pipeline:

- layers emit `LayerOutput`
- `Scene` allocates a full-frame `Grid`
- layer outputs are merged through `render::compositor`
- masks exist as compositor data and are currently wired as a hero/field probe
- final output is converted back to ratatui `Line`s and drawn once

## Keep

- `Cargo.toml`
- `Cargo.lock`
- `src/`
- `assets/`
- `docs/`
- `scripts/`
- `tools/legacy-python/`
- `tools/experiments/`
- `README.md`

## Current Known Issues

- Research ingest from `yam-rust_debugging_260424-2009` confirms the same top-tier failure cluster: viewport recenter drift, camera semantic drift, and projection pipeline fragmentation.
- The later ChatGPT audit report (`yam-rust_chatgpt_audit_report_260424-2104.md`) independently reinforces the same risk set: resize invariance, render-time mutation coupling, duplicate projection paths, and stale docs around camera/viewport semantics.
- The research also confirms that the active code still mixes camera-as-offset and camera-as-center behavior, which makes resize and fullscreen transitions non-invariant.
- Anchor handling is now order-independent on the active path because render-derived values are captured in a per-frame `RenderState` snapshot and then read by later layers.
- Render-time `UiState` anchor writes have been removed from the active hero/clock paths; debug reconstructs those values from the shared snapshot and state helpers.
- The active fix direction is now explicit: `Camera` is the world-space origin of the visible crop, while `Viewport` is the terminal-sized crop rectangle and not a centering transform.
- Fullscreen still needs a stronger lock rule: when the terminal crop equals the world extent, arrow-key camera motion should not produce visible movement.
- The active contract now explicitly treats `(0, 0)` as the world datum, with signed quadrants around that origin.
- World coordinates are Cartesian (`y` upward); terminal/screen coordinates remain terminal-style (`y` downward).
- World-ui elements are attached to world entities; HUD-ui elements are attached to the viewport/camera/terminal frame.
- The clock is now explicitly classified as world-ui because it follows the hero in world space and carries its own relative offset.
- The footer/status bar is explicitly classified as hud-ui because it is screen-attached and carries hotkeys plus the version/build label.
- The repo now has explicit helper-level vocabulary for this split in `src/scene/coords.rs`:
  - `resolve_world_ui(...)` for world-attached elements that stay pinned in world space
  - `resolve_hud_ui(...)` for screen-attached overlays
- `RenderState` is now split into `world` and `hud` sections, and the resize invariance test checks that the world facts stay stable while the HUD crop changes with terminal size.
- footer row placement is now encoded in `footer_row(height)` so the HUD bottom row contract is testable.
- Camera semantics are inconsistent across modules. Hero/clock code uses `screen = world - camera`, while `Viewport` and the debug world border still treat camera as a center point.
- The remaining drift is not a missing viewport tie; it is an incomplete fullscreen contract. Windowed mode may pan the crop, fullscreen must freeze it.
- `follow_hero` is still present in state/camera controls, but no longer has a complete active render behavior.
- `hero_visual_anchor` and `clock_final` now live in the shared per-frame `RenderState`, reducing layer-order dependency on the active path.
- `(0, 0)` is used as a sentinel for hero world defaults in layer code, which prevents `(0, 0)` from being a clean valid world origin.
- World constants are `212x57`, while `UiState` still constructs `Hero::new(300, 120)`. The bounds model is not yet unified.
- Field rendering now receives the full frame as `viewport_rect`; the previous centered tiered viewport box was removed from the active scene path.
- `Viewport` still exists as a top-left camera wrapper, but the active scene no longer uses centered viewport tiers to place layers.
- The legacy `Layer::render(...)` API remains alongside the active `render_to_grid(...)` API.
- Clock visibility currently depends on anchor-space offsets and clipping behavior; hero and clock are now world-pinned on the active path and should not be reprojected by camera movement.
- Debug visibility checks and border exclusion are screen-space workarounds rather than a formal UI pass.
- The active debug border is now a stable ASCII datum-centered indicator rendered in world space, so it moves with camera panning and remains a debug view of the real world bounds. It intentionally leaves a top padding row and one side padding cell for symmetry; that space is reserved for future UI placement. The bottom one-row padding is currently occupied by the footer.
- The latest screenshot set reinforces that the remaining confusion is semantic: hud-ui is terminal-fixed, world-ui is world-pinned, and the app still needs stronger wording/tests so those roles are not accidentally swapped in later changes.
- `scripts/update.sh` and `yam --check-updates` assume optional external cargo tools may be installed.
- `render::compositor::write_string` writes by `char`, not terminal display width or grapheme cluster width.
- `render::compositor::write_string` and `lines_to_grid` now account for grapheme clusters and display width, but the engine still stores only one `char` per cell.
- Mask semantics are provisional: the hero mask is captured and applied only to the field layer as a visual verification path.
- `coords::Space` and `resolve_position` are placeholders; `Space::Anchor` does not yet resolve through an entity registry.
- `UiOffsets` now stores offsets, camera, font, and animation settings; it should eventually be renamed or split.
- `Viewport::from_camera` and the debug border code still reflect different presentation layers; the broader camera/anchor model still needs consolidation.
- `UiState` no longer carries render-cache fields for hero or clock anchors; that dependency was removed from the active render path.
- The legacy `Layer::render(...)` API has been removed from the active layer contract; only `render_to_grid(...)` remains.
- Hero world position is no longer inferred through a `(0,0)` sentinel path.
- `Scene::render` computes a per-frame read-only `RenderState` so hero, clock, and debug all read the same frame facts without render-time state mutation.
- `src/scene/coords.rs` now has basic invariance tests for anchor/world/screen composition and screen-space camera independence.
- The hero source asset is square `820x820` pixels; the active terminal render footprint is fixed at `96x48` cells to preserve visual proportions in cell space.
- `docs/CURRENT_ISSUES_2026-04-25.md` captures the current contract confusion points: camera visibility, world-ui vs hud-ui boundaries, and the static-vs-dynamic classification problem.

## Research Rule Summary

The research docs converge on these rules:

- projection must be singular
- viewport must be a crop, not a transform
- camera must have one meaning across the entire pipeline
- anchor-space elements should resolve before final screen projection
- visibility should clip, not mutate position
- rendering must be deterministic from state alone

## Gaps

- formal automated frame tests
- a stable camera/viewport/anchor contract
- scene-level tests for layer order and mask behavior
- golden-frame tests for fixed hero geometry
- CLI tests for `--update` and `--check-updates`
- release notes for the Rust binary and install command

## Next Actions

1. keep the docs current with implementation changes
2. formalize camera semantics before adding more world-space features
3. replace render-time `UiState` side effects with an explicit frame/render context
4. remove the legacy `Layer::render(...)` path once grid rendering is fully stable
5. keep the tree buildable after each edit
6. prune temporary output before committing
