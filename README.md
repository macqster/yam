# yam-rust

Terminal scene engine in Rust.

## What This Repo Is

This repository contains the active Rust runtime tree for YAM-Rust.

It is the master entry point for:

- the runtime application
- the current docs map
- the active work order
- the working notes pointer

The scene and architecture contracts live in `docs/scene-model.md` and `docs/architecture.md`.

## Quick Links

- [`docs/README.md`](docs/README.md) - docs map
- [`TODO.md`](TODO.md) - active backlog
- [`NOTES.md`](NOTES.md) - working notes pointer
- [`docs/LOG.md`](docs/LOG.md) - append-only repo log
- [`docs/scene-model.md`](docs/scene-model.md) - deterministic scene model
- [`docs/architecture.md`](docs/architecture.md) - architecture contract
- [`docs/rendering.md`](docs/rendering.md) - render order contract
- [`docs/audit.md`](docs/audit.md) - current repo audit
- [`docs/archive/README.md`](docs/archive/README.md) - historical reports and reviews

## Runtime

- `yam-rust` is the installed command
- `yam-install` rebuilds and reinstalls the binary
- `q` exits

## Active Work

The current work order is:

1. ui
2. main scene stabilisation
3. hero gif
4. main scene stabilisation
5. main scene vines
6. main scene stabilisation

## Scene Model and Presentation Contract

The core contracts that define the work are:

- projection is singular
- world, HUD, and overlay stay separated
- the terminal is treated as a framebuffer, not a panel dashboard
- stabilization checkpoints must be measurable before moving forward

Keep these source documents authoritative:

- `docs/scene-model.md`
- `docs/architecture.md`

## Greenhouse / World Modes

- Model greenhouse as a separate world mode, not as panel chrome.
- Keep room selection internal to the world model.
- Avoid side-by-side tab UI as the primary greenhouse architecture.
- Define how `WorldMode::Main` and `WorldMode::Greenhouse` interact with camera and input.

## Footer, HUD, and Debug

- Keep footer layout deterministic with left / center / right zones.
- Preserve truncation rules:
  - center collapses first
  - right indicators survive longest
  - no wrapping
- Keep debug split into passive HUD info and active overlay inspection.
- Keep scrollbars as viewport indicators, not widget chrome.
- Make sure hotkey visibility and hotkey behavior remain separate concerns.
- Keep passive debug in HUD and modal inspection in overlay.

## Hero GIF Rendering

- Move hero rendering toward stable, renderer-owned frame ownership.
- Prefer precomputed or cached frame grids over streamed ANSI output.
- Reduce color instability caused by per-frame quantization.
- Reduce geometry drift by avoiding per-frame glyph remapping where possible.
- Keep a distinct “hero frame representation” concept in the renderer.
- Investigate whether a cached frame bake or internal grid renderer should replace the current chafa-heavy path.

### Hero GIF Checklist

- Keep hero frames fixed-size before render.
- Keep hero frame ownership inside the renderer, not in ad hoc terminal output.
- Preserve the full-canvas / flattened-frame geometry contract for partial GIF frames.
- Avoid changing color mapping per frame unless a stable cache is used.
- Keep hero rendering deterministic across resize and scene stabilisation checkpoints.
- Decide whether the hero path should stay chafa-backed or move toward cached frame ownership before starting vines.

## Implementation Order

- Keep the implementation order explicit:
  1. ui
  2. main scene stabilisation
  3. hero gif
  4. main scene stabilisation
  5. main scene vines
  6. main scene stabilisation
- Treat each stabilization step as a hard checkpoint before moving on.
- Stop any pass when the scene is not clean, flicker-free, and layout-stable.
- Each stabilization checkpoint must end with the scene matching the current presentation contract before the next phase starts.

### UI Phase Checklist

- Audit the current UI split so `ui/` remains runtime UI state and screen-space widgets only.
- Keep world-state mutation out of UI widgets.
- Keep camera/viewport ownership in the projection path, not in UI widgets.
- Keep footer, HUD, and overlay responsibilities explicit before changing scene code.
- Make sure the UI phase does not reintroduce panel-style layout as the primary model.

### Main Scene Stabilisation Checklist

- Confirm projection stays singular and lives in one path only.
- Verify world-ui remains world-pinned and hud-ui remains screen-attached.
- Add or tighten invariance coverage for resize, camera, anchor, and rounding behavior.
- Keep `RenderState` as the shared per-frame contract for world and HUD facts.
- Check that the footer row, debug overlay, and border probe still obey the presentation contract.
- Do not advance to hero gif work until the scene is stable and deterministic under resize.

### Main Scene Stabilisation Exit Criteria

- Resize does not change world attachment semantics.
- Camera changes do not produce multiple projection meanings.
- Footer stays on the bottom HUD row with deterministic truncation.
- Debug telemetry matches visible placement.
- Border probe remains a world-border indicator, not a second UI system.
- `RenderState` remains read-only and shared by the layers that need it.

### Main Scene Vines Checklist

- Add vines only after the scene is stable under resize.
- Keep vines as world-attached scene content, not HUD or overlay chrome.
- Reuse the single projection path and the shared `RenderState` contract.
- Keep vine layering deterministic relative to scaffold, hero, and background.
- Preserve collision / masking rules so vines do not reintroduce geometry drift.
- Stop vine work if it starts to destabilize the hero or footer contracts.

## Ratatui Research Follow-Ups

- Keep evaluating layout vs `SceneLayout` mapping.
- Keep reviewing event/focus patterns from ratatui ecosystem crates only when they solve a concrete problem.
- Keep `Canvas`, popup, tabs, scrollbar, and widget research tied to actual scene needs.
- Prefer research that preserves the world/HUD/overlay split and single projection contract.

## Contract Debt To Avoid Reintroducing

- Do not reintroduce mixed camera semantics.
- Do not split projection across multiple code paths.
- Do not let HUD content inherit world motion.
- Do not let world content become screen-fixed by accident.
- Do not treat masking as “empty pixels”.
- Do not render inside logic systems.

## Notes Ingested

- `new/01_yam-rust_ui_concepting.md`
- `new/02_scene_and_layer_model.md`
- `new/yam-rust_hero_gif_render_concepts.md`
- `new/yam-rust_to-do.md`
- `new/yam-rust_ui_architecture_concepting.md`

## Reference Archive

- Use `docs/REFERENCE_ARCHIVE.md` for the dump folder and other historical notes that are reference-only.
- Do not move archived notes back into the active backlog unless the active docs explicitly promote them.

## Archive-Derived Ideas Worth Preserving

- Keep the single projection gateway as a first-class implementation rule.
- Add invariance tests for resize, camera, projection, anchor, and rounding behavior.
- Prefer `ViewOrigin`-style translation semantics over ambiguous camera behavior if a simplification is needed.
- Keep greenhouse as a world mode, not tabbed UI chrome.
- Treat footer layout as deterministic truncation logic, not generic widget layout.
- Split debug into passive HUD telemetry and active overlay inspection.
- Prefer renderer-owned or cached hero frame ownership if chafa-stream instability remains a problem.
- Keep the bug taxonomy around as a basis for concrete tests and regression checks.

## Concrete Next Steps

- Add or tighten tests for:
  - resize invariance
  - camera projection consistency
  - anchor integrity
  - rounding / jitter stability
- Audit the current projection path for any remaining split responsibilities.
- Decide whether the hero path should keep the current chafa-backed flow or migrate toward cached frame ownership.
- Use the bug taxonomy as a checklist for missing regression coverage.
- Keep the active backlog aligned with `docs/scene-model.md` and `docs/architecture.md` whenever the work-order sequence changes.

## Repo Structure

- `src/core/` - world, grid, cell, entity, fields
- `src/systems/` - tick pipeline and system scaffolding
- `src/render/` - clock, hero, fonts, and render helpers
- `src/ui/` - camera, viewport, panels, layout, scene composition, and debug overlays
- `docs/` - documentation index, active contracts, and archive pointers

## Working Rules

- keep changes logged in `docs/LOG.md`
- keep build output out of the repository
- avoid reintroducing old runtime artifacts unless explicitly needed
- reserve uppercase markdown filenames for the repo front door and other high-visibility entry points
- use `docs/README.md` when you need the docs map
- keep active contract details in `docs/scene-model.md`, `docs/architecture.md`, and `docs/rendering.md`

## Maintenance

- run `scripts/check.sh` before committing
- `cargo clippy -- -D warnings` must pass without warnings
- keep the render order contract in `docs/rendering.md`

## Notes

The working-notes file exists only as a lightweight pointer. The authoritative docs are the active contracts and the backlog.
