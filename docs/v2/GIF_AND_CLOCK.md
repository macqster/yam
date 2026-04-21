# YAM v2 Clock Scene

This document records the minimal working default scene target for v2.

## Clock Render

- the default scene now renders only a FIGlet clock and a full day-name label
- the live clock is rendered by the Go FIGlet engine using `clock_font_name`
- the hero GIF is rendered separately through the Chafa renderer and placed as a world-layer image block
- the canonical hero asset is repo-tracked at `hero/assets/hero_go.gif`
- the default clock font is `Fender`
- the live FIGlet renderer uses full-width mode so digits retain visible separation
- the clock block is anchored near x = 3/4 of terminal width and y = 1/4 of terminal height
- the day label is rendered beneath the clock in Polish date form, e.g. `wtorek, 21 kwietnia`
- a Bubble Tea help footer is rendered below the day label using the `bubbles/help` module
- the clock colon blinks once per second on the live path by swapping `:` for a blank on odd seconds
- the clock is runtime-driven and not part of engine state
- verification uses a fixed clock string and day string so the snapshot remains stable
- the runtime clock and day formats are configurable through the scene config file
- the live Go Bubble Tea renderer owns the clock output; the helper path shells out to the Go one-shot renderer for verification only
- the hero renderer uses the explicit hero anchor and size fields from the scene config, with the current stable default at `10x6` and zero offsets
- the FIGlet engine owns spacing and smushing; do not reintroduce grid-style per-glyph composition for the live clock

## Live Behavior

- the default `yam` launch stays alive and redraws continuously through the Bubble Tea runtime
- use `--once` with `--clock` and `--day` for finite one-shot output during debugging
- the runtime enters the alternate screen and restores the terminal on exit
- changes to `scene_config.json` are reloaded live

## Constraints

- keep the FIGlet clock readable and compact
- keep the clock outside the engine
- do not couple this scene target to the legacy visualizer runtime
