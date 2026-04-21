# YAM v2 Clock Scene

This document records the minimal working default scene target for v2.

## Clock Render

- the default scene now renders only an ASCII-art clock and a full day-name label
- the clock uses the repo-tracked Bream-Deco box-drawing grid font
- the hour is rendered as deterministic grid output
- the day name is rendered beneath the clock in plain text
- a digit reference row `0 1 2 3 4 5 6 7 8 9` is rendered below the day label
- the clock is runtime-driven and not part of engine state
- verification uses a fixed clock string so the snapshot remains stable
- the runtime clock and day formats are configurable through the scene config file
- the live Go Bubble Tea renderer owns the clock output; the Python helper mirrors the shared font file for verification only
- the current glyph set is the restored narrow 5x7 working contract, kept for deterministic readability testing

## Live Behavior

- the default `yam` launch stays alive and redraws continuously through the Bubble Tea runtime
- use `--steps N` for finite one-shot output during debugging
- the runtime enters the alternate screen and restores the terminal on exit
- changes to `v2/scene_config.json` are reloaded live

## Constraints

- keep the grid clock minimal
- keep the clock outside the engine
- do not couple this scene target to the legacy visualizer runtime
