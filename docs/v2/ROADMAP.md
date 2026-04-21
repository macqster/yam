# YAM v2 Roadmap

This file tracks the rebuild at a high level.

## Goals

- preserve the v2 spec as the source of truth
- rebuild the codebase in explicit layers
- keep changes small, reviewable, and reversible
- maintain clear documentation for every subsystem

## Milestones

### M0. Documentation import

- import preliminary spec notes
- add a spec index
- establish tracking and logs

### M1. Repository audit

- map current code to v2 layers
- identify keep / adapt / replace candidates
- list hard blockers and coupling risks
- define a native v2 source tree
- keep the old visualizer implementation separate

### M2. Vertical slice

- build a minimal runnable simulation loop
- render one organism to the framebuffer
- emit one deterministic terminal frame
- wire `app.py` to the runtime model and message flow
- document the slice in `VERTICAL_SLICE.md`
- add runtime tick and input message handling
- document the loop in `RUNTIME_LOOP.md`

### M3. Core engine

- add ecosystem, environment, lifecycle, and species
- define deterministic state transitions
- add tests for time and seed behavior
- record the engine step contract in `ENGINE_MODEL.md`

### M4. Render stack

- add morphology, shape language, layers, masks, and emitters
- add renderer selection and z-resolution
- validate frame output against snapshots
- record morphology and shape translation in `MORPHOLOGY_AND_SHAPES.md`
- record the render composition split in `RENDER_PIPELINE.md`
- add the first golden frame snapshot in `GOLDEN_FRAME.md`
- add a minimal golden-frame verification command in `VERIFICATION.md`

### M4b. Render hardening

- make overlay/world ordering explicit
- document the z-band contract in `RENDER_HARDENING.md`

### M5. TUI and controls

- add panels, focus, keybindings, and commands
- keep runtime state isolated from engine state
- document all shortcuts and command paths
- prefer Bubble Tea core types first; use `bubbles` only when a component needs a ready-made widget
- consider `bubbleboxer` only if nested panel layout becomes cumbersome in the core runtime

### M6. Visual system

- add themes and renderer rules
- encode the BTAS / Dark Deco constraints
- verify readability and noise suppression
- prefer `lipgloss` for layout and styling primitives
- keep `additional-bubbles` and `mritd/bubbles` as optional references unless a missing widget blocks progress

### M7. Minimal working scene

- render a basic GIF frame
- overlay a runtime clock
- document the scene target in `GIF_AND_CLOCK.md`
- keep VHS available for recording and shareable terminal demos
- plan the hero GIF pipeline in `HERO_GIF_PLAN.md`

### M8. Launcher migration

- switch the default launcher to v2
- keep v1 behind an explicit compatibility flag
- document the launcher contract in `LAUNCHER.md`

### M9. UI separation

- move UI state out of the engine path
- add a minimal UI router and overlay contract
- document the boundary in `UI_INPUT.md`

### M10. Theme and glyph policy

- add a minimal theme contract
- constrain glyph families during shape and GIF conversion
- document the theme policy in `THEME_AND_GLYPHS.md`

### M11. Scene config surface

- add explicit scene config fields for GIF, clock, and theme
- document the scene config in `SCENE_CONFIG.md`

### M12. File-backed scene config

- add `scene_config.json`
- load scene config from the repo-tracked JSON file by default

### M13. Scene config live reload

- reload `scene_config.json` during the live loop
- document the reload contract in `SCENE_CONFIG.md`

### M14. Scene config command

- add `yam --scene-config` helper commands
- document the command surface in `LAUNCHER.md`

### M15. Scene config set helper

- add a constrained `yam --scene-set key=value` path
- support only the explicit scene config keys

### M16. Bubble Tea migration

- map the current scaffold to a Bubble Tea runtime
- keep the render and config layers intact during the swap
- document the migration in `BUBBLETEA_MIGRATION.md`
- add a Go runtime shell under `v2/cmd/yamv2`
- expose a `--runtime bubbletea` launcher path
- make Bubble Tea the default v2 runtime

### M17. Dependency policy

- document which upstream terminal UI libraries are core, optional, or reference-only
- keep the adoption policy explicit in `DEPENDENCY_MATRIX.md`

### M18. Hero GIF plan

- document the hero GIF pipeline choice and implementation order
- keep Chafa as the first candidate for terminal image rendering
- record the best moment to switch into Codex planning mode before implementation

### M19. Hero renderer scaffold

- add a dedicated Go hero renderer boundary
- start with a Chafa CLI adapter
- document the first hero implementation slice in `HERO_RENDERER.md`

## Working Rules

- no hidden state changes
- no mixing simulation with rendering
- no undocumented file moves
- no broad refactors without a logged reason
- every meaningful change gets a note in `docs/v2/LOG.md`
- the branch and release policy live in `docs/RELEASE_MODEL.md`
- the source-tree flattening path lives in `docs/FLATTENING_PLAN.md`
