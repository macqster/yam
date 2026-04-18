# Visualizer Vocabulary

This is the canonical dictionary for the visualizer.
Use these names in code, config, docs, and patch notes.

## System
- `vines`: the procedurally generated jungle vine sprawl as a whole.
- `scaffold`: the static woody support structure beneath the hero.
- `hero`: the Chafa-rendered figure on the left.
- `panel`: the boxed info panel on the right.
- `mask`: a collision or silhouette boundary used for layout and growth.
- `field`: an intermediate density/priority surface used before final glyph output.

## Growth Structure
- `trunk`: the primary woody spine of the vine system.
- `branch`: a side split that grows from a trunk or another branch.
- `stem`: a committed structural cell in the vine body.
- `spur`: a short offshoot or stub-like branch fragment.
- `tip`: an active growth front that can still move or branch.

## Ornament
- `leaf`: a foliage ornament stamped from mature growth.
- `flower`: a bloom ornament with its own lifecycle.
- `canopy`: the broader foliage mass produced by leaf and flower placement.

## Layout
- `hero_mask`: the hero silhouette mask used for collision and boundary guidance.
- `support_mask`: the guide mask used to shape scaffold placement and routing. The current config keys still use `trunk_mask_*` for compatibility.
- `support_field`: the soft distance field derived from `support_mask`. The current runtime name is `trunk_field`.
- `region`: a named area of the scene such as above-hero, below-hero, left field, or right field.
- `allowed cell`: a terminal cell where growth or ornament placement is permitted.
- `no-go zone`: a blocked area that vines must not enter.

## Rendering
- `render field`: the intermediate accumulation layer that stores density, color, and priority before glyph selection.
- `glyph selection`: the final step that turns field data into terminal characters.
- `density`: how visually occupied a cell is, from empty to fully solid.

## Naming Rules
- Prefer `vines` over older project-specific names.
- Prefer `leaf` and `leaves`; do not use `leafs`.
- Prefer `spur` for short offshoots or stub-like fragments.
- Prefer `support` for the mask/field that guides scaffold placement.
- Prefer `trunk` for the main woody spine, `branch` for side splits, and `stem` for committed cells.
