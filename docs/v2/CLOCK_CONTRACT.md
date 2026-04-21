# YAM v2 Clock Contract

This note records the current contract for the default clock scene.

## Contract

- `v2/render/fonts/go_deco.txt` is the authoritative clock font file.
- The runtime renders that file as-is.
- The renderer does not add extra inter-glyph padding.
- The font file itself is responsible for internal spacing and shape.
- The live Go Bubble Tea runtime is canonical.
- The Python helper is snapshot-only and mirrors the same file for verification.

## What This Means

- if the clock looks cramped or airy, the font file is the thing to adjust
- do not reintroduce renderer-side spacing fixes unless the contract changes
- do not duplicate clock geometry in a second implementation path

## Baseline

- use this contract when changing the clock font or its baseline snapshot
- keep the live renderer and the verifier aligned on the same source file
