# Visualizer Workflow

Date: 2026-04-18
Scope: `visualizer/`
Purpose: provide a short operating guide for day-to-day development and tuning.

This is the practical loop to use when working on the visualizer.

## Working Loop

1. Pick one concern
   - `layout`
   - `mask`
   - `growth`
   - `render`
   - `docs`
   - `tooling`

2. Reproduce it
   - choose a recipe or config preset
   - run config lint on the base config or selected recipe if the change is configuration-related
   - record terminal size
   - record seed if growth behavior matters

3. Inspect the right layer first
   - layout issue -> masks and anchors
   - growth issue -> route/state/field logic
   - render issue -> density/priority composition

4. Make one change
   - avoid mixing unrelated fixes
   - keep the change as small as possible

5. Verify against a baseline
   - compare to a known-good scene or recipe
   - check whether the change helped the intended layer only

## Triage Order

When something looks wrong, inspect in this order:

1. config
2. layout and masks
3. fields and guides
4. growth
5. renderer

This avoids tuning the wrong layer first.

## Definition of Done

A change is only done when:

- the target issue is fixed
- the change does not break the other visible layers
- the relevant docs are updated if a stable assumption changed
- the result is reproducible with a recipe or config snapshot

## Default Artifacts To Record

- recipe name or config file used
- terminal size
- seed, if applicable
- before/after notes
- screenshot or snapshot if the change is visual

## Reproducibility Rules

Use these when a scene is hard to reason about:

- prefer a named recipe over ad hoc JSON edits
- keep one known-good baseline scene for comparison
- if growth behavior matters, keep the seed fixed
- if output looks wrong, verify layout and masks before tuning growth or glyph settings
- if a change is not reproducible, it is too expensive to debug

## Rendering Priorities

The research work reinforces this tuning order:

1. source image quality
2. preprocessing and thresholding
3. palette and color space
4. symbol density
5. layout and masking
6. growth behavior
7. final ANSI emission

That order is useful when deciding whether a problem belongs in `CONFIG.md`, `WORKFLOW.md`, or the renderer itself.

## Relationship To Other Docs

- [PROJECT_PROCESS.md](PROJECT_PROCESS.md) covers the longer process conventions
- [DEV_TOOLS.md](DEV_TOOLS.md) covers the most useful tooling to build
- [MASKS_AND_GUIDES.md](MASKS_AND_GUIDES.md) covers the spatial model
- [CONFIG.md](CONFIG.md) covers the live config surface
- [TUNING_CHECKLIST.md](TUNING_CHECKLIST.md) gives a short step-by-step tuning order
