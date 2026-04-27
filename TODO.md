# YAM-RUST TODO

This file compiles the open work captured in the recent `new/` concept notes and folds it into a single repo-wide backlog.

## 1. Scene Model and Presentation Contract

- Keep `docs/SCENE_MODEL.md` and `docs/ARCHITECTURE.md` as the source of truth for world, HUD, and overlay behavior.
- Keep projection defined in one place only.
- Preserve the deterministic frame pipeline:
  - update state
  - generate primitives
  - apply masks
  - project world to screen
  - sort into layers
  - compose frame buffer
  - render through ratatui
- Keep world-space, screen-space, and anchor-space distinct in code.
- Keep HUD attached to the terminal frame and overlays modal/top-z-index.
- Treat the terminal as a framebuffer, not a panel dashboard.

## 2. Greenhouse / World Modes

- Model greenhouse as a separate world mode, not as panel chrome.
- Keep room selection internal to the world model.
- Avoid side-by-side tab UI as the primary greenhouse architecture.
- Define how `WorldMode::Main` and `WorldMode::Greenhouse` interact with camera and input.

## 3. Footer, HUD, and Debug

- Keep footer layout deterministic with left / center / right zones.
- Preserve truncation rules:
  - center collapses first
  - right indicators survive longest
  - no wrapping
- Keep debug split into passive HUD info and active overlay inspection.
- Keep scrollbars as viewport indicators, not widget chrome.
- Make sure hotkey visibility and hotkey behavior remain separate concerns.
- Keep passive debug in HUD and modal inspection in overlay.

## 4. Hero GIF Rendering

- Move hero rendering toward stable, renderer-owned frame ownership.
- Prefer precomputed or cached frame grids over streamed ANSI output.
- Reduce color instability caused by per-frame quantization.
- Reduce geometry drift by avoiding per-frame glyph remapping where possible.
- Keep a distinct “hero frame representation” concept in the renderer.
- Investigate whether a cached frame bake or internal grid renderer should replace the current chafa-heavy path.

### 4.1 Hero GIF Checklist

- Keep hero frames fixed-size before render.
- Keep hero frame ownership inside the renderer, not in ad hoc terminal output.
- Preserve the full-canvas / flattened-frame geometry contract for partial GIF frames.
- Avoid changing color mapping per frame unless a stable cache is used.
- Keep hero rendering deterministic across resize and scene stabilisation checkpoints.
- Decide whether the hero path should stay chafa-backed or move toward cached frame ownership before starting vines.

## 5. Implementation Order

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

### 5.1 UI Phase Checklist

- Audit the current UI split so `ui/` remains runtime UI state and screen-space widgets only.
- Keep world-state mutation out of UI widgets.
- Keep camera/viewport ownership in the projection path, not in UI widgets.
- Keep footer, HUD, and overlay responsibilities explicit before changing scene code.
- Make sure the UI phase does not reintroduce panel-style layout as the primary model.

### 5.2 Main Scene Stabilisation Checklist

- Confirm projection stays singular and lives in one path only.
- Verify world-ui remains world-pinned and hud-ui remains screen-attached.
- Add or tighten invariance coverage for resize, camera, anchor, and rounding behavior.
- Keep `RenderState` as the shared per-frame contract for world and HUD facts.
- Check that the footer row, debug overlay, and border probe still obey the presentation contract.
- Do not advance to hero gif work until the scene is stable and deterministic under resize.

### 5.3 Main Scene Stabilisation Exit Criteria

- Resize does not change world attachment semantics.
- Camera changes do not produce multiple projection meanings.
- Footer stays on the bottom HUD row with deterministic truncation.
- Debug telemetry matches visible placement.
- Border probe remains a world-border indicator, not a second UI system.
- `RenderState` remains read-only and shared by the layers that need it.

### 5.4 Main Scene Vines Checklist

- Add vines only after the scene is stable under resize.
- Keep vines as world-attached scene content, not HUD or overlay chrome.
- Reuse the single projection path and the shared `RenderState` contract.
- Keep vine layering deterministic relative to scaffold, hero, and background.
- Preserve collision / masking rules so vines do not reintroduce geometry drift.
- Stop vine work if it starts to destabilize the hero or footer contracts.

## 6. Ratatui Research Follow-Ups

- Keep evaluating layout vs `SceneLayout` mapping.
- Keep reviewing event/focus patterns from ratatui ecosystem crates only when they solve a concrete problem.
- Keep `Canvas`, popup, tabs, scrollbar, and widget research tied to actual scene needs.
- Prefer research that preserves the world/HUD/overlay split and single projection contract.

## 7. Contract Debt To Avoid Reintroducing

- Do not reintroduce mixed camera semantics.
- Do not split projection across multiple code paths.
- Do not let HUD content inherit world motion.
- Do not let world content become screen-fixed by accident.
- Do not treat masking as “empty pixels”.
- Do not render inside logic systems.

## 8. Notes Ingested

- `new/01_yam-rust_ui_concepting.md`
- `new/02_scene_and_layer_model.md`
- `new/yam-rust_hero_gif_render_concepts.md`
- `new/yam-rust_to-do.md`
- `new/yam-rust_ui_architecture_concepting.md`

## 9. Reference Archive

- Use `docs/REFERENCE_ARCHIVE.md` for the dump folder and other historical notes that are reference-only.
- Do not move archived notes back into the active backlog unless the active docs explicitly promote them.

## 10. Archive-Derived Ideas Worth Preserving

- Keep the single projection gateway as a first-class implementation rule.
- Add invariance tests for resize, camera, projection, anchor, and rounding behavior.
- Prefer `ViewOrigin`-style translation semantics over ambiguous camera behavior if a simplification is needed.
- Keep greenhouse as a world mode, not tabbed UI chrome.
- Treat footer layout as deterministic truncation logic, not generic widget layout.
- Split debug into passive HUD telemetry and active overlay inspection.
- Prefer renderer-owned or cached hero frame ownership if chafa-stream instability remains a problem.
- Keep the bug taxonomy around as a basis for concrete tests and regression checks.

## 11. Concrete Next Steps

- Add or tighten tests for:
  - resize invariance
  - camera projection consistency
  - anchor integrity
  - rounding / jitter stability
- Audit the current projection path for any remaining split responsibilities.
- Decide whether the hero path should keep the current chafa-backed flow or migrate toward cached frame ownership.
- Use the bug taxonomy as a checklist for missing regression coverage.
- Keep `TODO.md` aligned with `docs/SCENE_MODEL.md` and `docs/ARCHITECTURE.md` whenever the work-order sequence changes.

## 12. Projection Contract

- Define projection as a single pure function from world position, camera, and viewport to screen position.
- Keep projection free of side effects, masking, layering, and state mutation.
- Route every world-to-screen conversion through the same implementation path.

## 13. Core Invariants

- Keep world-space resolution independent.
- Keep HUD camera-independent and screen-attached.
- Keep overlay modal and top-z above world and HUD.
- Keep masks applied before composition is finalized.
- Keep layer ordering fixed, even if it is encoded numerically.

## 14. RenderState and Validation

- Define the required `RenderState` fields and their ownership lifecycle.
- Keep `RenderState` constructed once per frame and treated as read-only by render layers.
- Add measurable stabilization checks for identical buffer output, resize invariance, and zero-jitter projection behavior.
- Expose instrumentation for projection output, masks, and layer boundaries when stabilizing scene work.
- Define the hero rendering decision gate before vines work begins so the chafa-backed and cached-frame options are compared explicitly.
