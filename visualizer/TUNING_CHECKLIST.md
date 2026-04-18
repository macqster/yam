# Visualizer Tuning Checklist

Use this when the scene looks off and you want to converge quickly.

## 1. Identify the problem class

- `layout`
- `mask`
- `growth`
- `render`
- `palette`
- `timing`

## 2. Reproduce the scene

- pick a recipe or config snapshot
- keep the seed fixed if growth behavior matters
- note terminal size
- make sure the same source asset is being used

## 3. Check the high-level order

Use this order before reaching for lower-level tuning:

1. source quality
2. preprocessing and thresholding
3. palette and color space
4. glyph density and symbol set
5. layout and masking
6. growth behavior
7. final render details

## 4. Inspect the right layer first

- layout issue -> masks, anchors, offsets
- mask issue -> support field, hero boundary, collision geometry
- growth issue -> route/state/field logic
- render issue -> density, priority, glyph selection
- palette issue -> color mode, palette, color space
- timing issue -> cadence, frame count, tick rate

## 5. Make one change

- avoid mixing unrelated fixes
- keep the change small
- prefer a recipe overlay for repeatability

## 6. Verify against a baseline

- compare to a known-good recipe
- check whether the change affected only the intended layer
- confirm the output still matches the current visual direction

## 7. Record the result

- recipe or config used
- terminal size
- seed, if applicable
- short before/after note
- snapshot if the change is visual

## Fast Rules

- if the output is unstable, reduce frame count or slow the cadence before touching layout
- if the hero looks weak, revisit source quality and preprocessing before tuning growth
- if the scene is noisy, check thresholding and dithering before changing structure
- if a change is not reproducible, it is not ready
