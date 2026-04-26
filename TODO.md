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

## 4. Hero GIF Rendering

- Move hero rendering toward stable, renderer-owned frame ownership.
- Prefer precomputed or cached frame grids over streamed ANSI output.
- Reduce color instability caused by per-frame quantization.
- Reduce geometry drift by avoiding per-frame glyph remapping where possible.
- Keep a distinct “hero frame representation” concept in the renderer.
- Investigate whether a cached frame bake or internal grid renderer should replace the current chafa-heavy path.

## 5. Implementation Order

- Keep the implementation order explicit:
  1. core + hero + renderer
  2. scene + layout
  3. asset loader
  4. ivy integration
  5. temporal stabilization
  6. compiler / tooling support
- Stop the first pass when hero animation is clean, flicker-free, and layout-stable.

## 6. Ratatui Research Follow-Ups

- Keep evaluating layout vs `SceneLayout` mapping.
- Keep reviewing event/focus patterns from ratatui ecosystem crates only when they solve a concrete problem.
- Keep `Canvas`, popup, tabs, scrollbar, and widget research tied to actual scene needs.

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

