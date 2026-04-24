# Repo Audit

Date: 2026-04-24

## Summary

The repository tracks the Rust terminal scene engine as the active implementation.

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
- The research also confirms that the active code still mixes camera-as-offset and camera-as-center behavior, which makes resize and fullscreen transitions non-invariant.
- Anchor handling is still order-dependent in practice because render-derived values are written into `UiState` and then read by later layers.
- Render-time `UiState` anchor writes have been removed from the active hero/clock paths; debug now reconstructs those values from state helpers.
- The active fix direction is now explicit: `Camera` is treated as a top-left world offset for projection, while `Viewport` is a crop helper and not a centering transform.
- Camera semantics are inconsistent across modules. Hero/clock code uses `screen = world - camera`, while `Viewport` and the debug world border still treat camera as a center point.
- `follow_hero` is still present in state/camera controls, but no longer has a complete active render behavior.
- `hero_visual_anchor` and `clock_final` are written through `UiState` side effects during rendering, so clock/debug correctness depends on layer order.
- `(0, 0)` is used as a sentinel for hero world defaults in layer code, which prevents `(0, 0)` from being a clean valid world origin.
- World constants are `212x57`, while `UiState` still constructs `Hero::new(300, 120)`. The bounds model is not yet unified.
- Field rendering now receives the full frame as `viewport_rect`; the previous centered tiered viewport box was removed from the active scene path.
- `Viewport` still exists as a top-left camera wrapper, but the active scene no longer uses centered viewport tiers to place layers.
- The legacy `Layer::render(...)` API remains alongside the active `render_to_grid(...)` API.
- Clock visibility currently depends on anchor-space offsets and clipping behavior; it should preserve `clock = hero_visual_anchor + offset` and never clamp the final relationship.
- Debug visibility checks and border exclusion are screen-space workarounds rather than a formal UI pass.
- `scripts/update.sh` and `yam --check-updates` assume optional external cargo tools may be installed.
- `render::compositor::write_string` writes by `char`, not terminal display width or grapheme cluster width.
- `render::compositor::write_string` and `lines_to_grid` now account for grapheme clusters and display width, but the engine still stores only one `char` per cell.
- Mask semantics are provisional: the hero mask is captured and applied only to the field layer as a visual verification path.
- `coords::Space` and `resolve_position` are placeholders; `Space::Anchor` does not yet resolve through an entity registry.
- `UiOffsets` now stores offsets, camera, font, and animation settings; it should eventually be renamed or split.
- `Viewport::from_camera` and the debug border code were updated to match the top-left camera contract, but the broader camera/anchor model still needs consolidation.
- `UiState` no longer carries render-cache fields for hero or clock anchors; that dependency was removed from the active render path.
- The legacy `Layer::render(...)` API has been removed from the active layer contract; only `render_to_grid(...)` remains.
- Hero world position is no longer inferred through a `(0,0)` sentinel path.
- `Scene::render` now computes a per-frame read-only `FrameContext` so hero, clock, and debug all read the same projection facts without render-time state mutation.

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
