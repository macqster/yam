# YAM v2 Clock Contract

This note records the current contract for the default clock scene.

## Contract

- The live clock is rendered by the Go FIGlet engine in `v2/cmd/yamv2`.
- The canonical live font is selected by `clock_font_name` in `v2/scene_config.json`.
- The default font is `Fender`.
- The live renderer uses FIGlet full-width mode so adjacent digits do not smush together.
- The live runtime renders FIGlet output as a block and anchors that block around x = 3/4 width and y = 1/4 height from the top-left corner.
- The day label uses a Polish date format: `wtorek, 21 kwietnia`.
- The live clock colon blinks on odd seconds by replacing `:` with a blank in the rendered clock text.
- A Bubble Tea help footer is rendered at the bottom using `bubbles/help` with simple keys for quit and pause.
- The Python layer is snapshot/check only and should not define a second live clock renderer.

## What This Means

- if the clock looks cramped or airy, adjust the FIGlet font selection or FIGlet options
- do not reintroduce per-glyph grid rendering for the live clock
- do not duplicate clock geometry in a second live implementation path
- keep Python as a thin verification harness, not an alternate renderer

## Baseline

- use this contract when changing the clock font or its baseline snapshot
- keep the live renderer and the verifier aligned on the same FIGlet engine output
