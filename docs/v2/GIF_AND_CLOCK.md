# YAM v2 Clock Scene

This document records the minimal working default scene target for v2.

## Clock Render

- the default scene now renders only a FIGlet clock and a full day-name label
- the live clock is rendered by the Go FIGlet engine using `clock_font_name`
- the default clock font is `Fender`
- the live FIGlet renderer uses full-width mode so digits retain visible separation
- the clock block is anchored at the exact center of the terminal
- the day label is rendered beneath the clock in Polish date form, e.g. `wtorek, 21 kwietnia`
- the clock colon blinks once per second on the live path by swapping `:` for a blank on odd seconds
- the clock is runtime-driven and not part of engine state
- verification uses a fixed clock string and day string so the snapshot remains stable
- the runtime clock and day formats are configurable through the scene config file
- the FIGlet engine owns spacing and smushing; do not reintroduce grid-style per-glyph composition for the live clock

## Live Behavior

- the default `yam` launch stays alive and redraws continuously through the Bubble Tea runtime
- use `--once` with `--clock` and `--day` for finite one-shot output during debugging
- the runtime enters the alternate screen and restores the terminal on exit
- changes to `scene_config.json` are reloaded live

## Constraints

- keep the FIGlet clock readable and compact
- keep the clock outside the engine
- keep this scene target isolated from retired runtime experiments
