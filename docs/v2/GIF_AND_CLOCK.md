# YAM v2 GIF and Clock

This document records the minimal working scene target for v2.

## GIF Render

- a basic GIF frame is rendered into shapes
- the current source asset is `visualizer/assets/source.gif`
- GIF output is composited into the v2 render pipeline
- the source path, clock format, and theme name are now explicit file-backed scene config inputs

## Clock

- a simple clock is overlaid in the top-right corner
- the clock is runtime-driven and not part of engine state
- verification uses a fixed clock string so the snapshot remains stable
- the runtime clock format is configurable through the scene config file

## Live Behavior

- the default `yam` launch stays alive and redraws continuously through the Bubble Tea runtime
- use `--steps N` for finite one-shot output during debugging
- the runtime enters the alternate screen and restores the terminal on exit
- changes to `v2/scene_config.json` are reloaded live

## Constraints

- keep the GIF renderer minimal
- keep the clock outside the engine
- do not couple this scene target to the legacy visualizer runtime
