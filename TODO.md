# Active Backlog

This file is the repo-wide work order and stabilization checklist.

## 1. Scene Model and Presentation Contract

- [verify] `docs/scene-model.md` and `docs/architecture.md` remain the source of truth for world, HUD, and overlay behavior.
- [verify] projection remains defined in one place only.
- [verify] the deterministic frame pipeline remains:
  - update state
  - generate primitives
  - apply masks
  - project world to screen
  - sort into layers
  - compose frame buffer
  - render through ratatui
- [verify] world-space, screen-space, and anchor-space remain distinct in code.
- [verify] HUD stays attached to the terminal frame and overlays stay modal/top-z-index.
- [verify] the terminal is treated as a framebuffer, not a panel dashboard.

## 2. Greenhouse / World Modes

- [verify] greenhouse remains a separate world mode, not panel chrome.
- [verify] room selection stays internal to the world model.
- [verify] side-by-side tab UI is not the primary greenhouse architecture.
- [verify] `WorldMode::Main` and `WorldMode::Greenhouse` interaction with camera and input stays defined.

## 3. Footer, HUD, and Debug

- [verify] footer layout remains deterministic with left / center / right zones.
- [verify] truncation rules remain:
  - center collapses first
  - right indicators survive longest
  - no wrapping
- [verify] debug stays split into passive HUD info and active overlay inspection.
- [verify] scrollbars stay viewport indicators, not widget chrome.
- [verify] hotkey visibility and hotkey behavior stay separate concerns.
- [verify] passive debug stays in HUD and modal inspection stays in overlay.

## 4. Hero GIF Rendering

- [verify] hero rendering stays stable and renderer-owned.
- [verify] precomputed or cached frame grids stay preferred over streamed ANSI output.
- [verify] color instability caused by per-frame quantization stays minimized.
- [verify] geometry drift from per-frame glyph remapping stays minimized.
- [verify] the renderer keeps a distinct “hero frame representation” concept.
- [verify] cached-frame or internal grid migration is revisited only if the chafa-backed baseline fails stabilization checkpoints again.

### 4.1 Hero GIF Checklist

- [verify] hero frames stay fixed-size before render.
- [verify] hero frame ownership stays inside the renderer, not in ad hoc terminal output.
- [verify] the full-canvas / flattened-frame geometry contract holds for partial GIF frames.
- [verify] color mapping does not change per frame unless a stable cache is used.
- [verify] hero rendering stays deterministic across resize and scene stabilisation checkpoints.
- [verify] `scene_config.json` stays aligned with the active hero asset and footprint defaults that the runtime currently expects.
- [verify] hero rendering uses the chafa-backed baseline unless a measured regression justifies cached-frame migration.

## 5. Implementation Order

- [verify] the implementation order remains explicit:
  1. ui
  2. main scene stabilisation
  3. hero gif
  4. main scene stabilisation
  5. main scene vines
  6. main scene stabilisation
- [verify] each stabilization step remains a hard checkpoint before moving on.
- [verify] passes stop when the scene is not clean, flicker-free, and layout-stable.
- [verify] each stabilization checkpoint ends with the scene matching the current presentation contract before the next phase starts.

### 5.1 UI Phase Checklist

- [verify] `ui/` remains runtime UI state and screen-space widgets only.
- [verify] world-state mutation stays out of UI widgets.
- [verify] camera/viewport ownership stays in the projection path, not in UI widgets.
- [verify] footer, HUD, and overlay responsibilities stay explicit before changing scene code.
- [verify] the UI phase does not reintroduce panel-style layout as the primary model.

### 5.2 Main Scene Stabilisation Checklist

- [verify] projection stays singular and lives in one path only.
- [verify] world-ui remains world-pinned and hud-ui remains screen-attached.
- [verify] resize, camera, anchor, and rounding invariance coverage stays tight.
- [verify] `RenderState` remains the shared per-frame contract for world and HUD facts.
- [verify] the footer row, debug overlay, and border probe obey the presentation contract.
- [verify] hero gif work does not advance until the scene is stable and deterministic under resize.

### 5.3 Main Scene Stabilisation Exit Criteria

- [verify] resize does not change world attachment semantics.
- [verify] camera changes do not produce multiple projection meanings.
- [verify] footer stays on the bottom HUD row with deterministic truncation.
- [verify] debug telemetry matches visible placement.
- [verify] border probe remains a world-border indicator, not a second UI system.
- [verify] `RenderState` remains read-only and shared by the layers that need it.

### 5.4 Main Scene Vines Checklist

- [verify] vines are added only after the scene is stable under resize.
- [verify] vines remain world-attached scene content, not HUD or overlay chrome.
- [verify] vines reuse the single projection path and the shared `RenderState` contract.
- [verify] vine layering remains deterministic relative to scaffold, hero, and background.
- [verify] collision / masking rules prevent vine geometry drift.
- [verify] vine work stops if it destabilizes the hero or footer contracts.

## 6. Ratatui Research Follow-Ups

- [verify] layout vs `SceneLayout` mapping remains evaluated.
- [verify] event/focus patterns from ratatui ecosystem crates are reviewed only when they solve a concrete problem.
- [verify] `Canvas`, popup, tabs, scrollbar, and widget research stays tied to actual scene needs.
- [verify] research preserves the world/HUD/overlay split and single projection contract.

## 7. Contract Debt To Avoid Reintroducing

- [verify] mixed camera semantics are not reintroduced.
- [verify] projection is not split across multiple code paths.
- [verify] HUD content does not inherit world motion.
- [verify] world content does not become screen-fixed by accident.
- [verify] masking is not treated as “empty pixels”.
- [verify] rendering does not happen inside logic systems.

## 8. Concrete Next Steps

- [verify] add or tighten tests for:
  - resize invariance
  - camera projection consistency
  - anchor integrity
  - rounding / jitter stability
- [inspect] audit the current projection path for any remaining split responsibilities.
- [verify] use the bug taxonomy as a checklist for missing regression coverage.
- [verify] the active backlog stays aligned with `docs/scene-model.md` and `docs/architecture.md` whenever the work-order sequence changes.

## 9. Contract Pointers

- [verify] projection details stay in `docs/scene-model.md` and `docs/rendering.md`.
- [verify] layering and `RenderState` ownership details stay in `docs/architecture.md` and `docs/rendering.md`.
- [verify] invariants and determinism checks stay referenced from `docs/scene-model.md`.
- [verify] greenhouse integration rules stay in `docs/scene-model.md`.
- [verify] render-time validation goals stay in the active backlog here without duplicating contract text.

## 10. Further Development Guidelines

- [verify] add new backlog items as execution steps, inspections, or regression checks; keep contract wording in the owning docs.
- [verify] prefer one narrowly scoped item per line so TODO stays easy to prune.
- [verify] if a TODO item survives multiple passes without changing shape, either promote it to the owning contract doc or remove it.
- [verify] any behavior change keeps its test, log entry, and owning doc update in the same change.
- [verify] new work that introduces a concept already named in an active doc must reference the canonical doc instead of restating the rule.
