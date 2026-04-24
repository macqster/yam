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

- Camera semantics are inconsistent across modules. Hero/clock code uses `screen = world - camera`, while `Viewport` and the debug world border still treat camera as a center point.
- `follow_hero` is still present in state/camera controls, but no longer has a complete active render behavior.
- `hero_visual_anchor` and `clock_final` are written through `UiState` side effects during rendering, so clock/debug correctness depends on layer order.
- `(0, 0)` is used as a sentinel for hero world defaults in layer code, which prevents `(0, 0)` from being a clean valid world origin.
- World constants are `212x57`, while `UiState` still constructs `Hero::new(300, 120)`. The bounds model is not yet unified.
- Field rendering still uses `viewport_rect` and `Viewport`; hero, clock, debug, and status operate closer to full-grid screen coordinates.
- Fixed viewport tiers can create discontinuities when resizing.
- The legacy `Layer::render(...)` API remains alongside the active `render_to_grid(...)` API.
- Clock visibility currently depends on anchor-space offsets and clipping behavior; it should preserve `clock = hero_visual_anchor + offset` and never clamp the final relationship.
- Debug visibility checks and border exclusion are screen-space workarounds rather than a formal UI pass.
- `scripts/update.sh` and `yam --check-updates` assume optional external cargo tools may be installed.
- `render::compositor::write_string` writes by `char`, not terminal display width or grapheme cluster width.
- Mask semantics are provisional: the hero mask is captured and applied only to the field layer as a visual verification path.
- `coords::Space` and `resolve_position` are placeholders; `Space::Anchor` does not yet resolve through an entity registry.
- `UiOffsets` now stores offsets, camera, font, and animation settings; it should eventually be renamed or split.

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
