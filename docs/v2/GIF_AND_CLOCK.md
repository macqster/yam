# YAM v2 Clock Scene

This document records the minimal working default scene target for v2.

## Clock Render

- the default scene now renders only a FIGlet clock and a full day-name label
- the live clock is rendered by the Go FIGlet engine using `clock_font_name`
- the default clock font is `Fender`
- the live FIGlet renderer uses full-width mode so digits retain visible separation
- the clock block is anchored near x = 3/4 of terminal width and y = 1/4 of terminal height
- the day name is rendered beneath the clock in plain text
- a digit reference row is rendered below the day label using the same FIGlet engine
- the clock is runtime-driven and not part of engine state
- verification uses a fixed clock string and day string so the snapshot remains stable
- the runtime clock and day formats are configurable through the scene config file
- the live Go Bubble Tea renderer owns the clock output; the Python helper shells out to the Go one-shot renderer for verification only
- the FIGlet engine owns spacing and smushing; do not reintroduce grid-style per-glyph composition for the live clock

## Live Behavior

- the default `yam` launch stays alive and redraws continuously through the Bubble Tea runtime
- use `--once` with `--clock` and `--day` for finite one-shot output during debugging
- the runtime enters the alternate screen and restores the terminal on exit
- changes to `v2/scene_config.json` are reloaded live

## Constraints

- keep the FIGlet clock readable and compact
- keep the clock outside the engine
- do not couple this scene target to the legacy visualizer runtime
