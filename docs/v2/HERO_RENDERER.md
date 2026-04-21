# YAM v2 Hero Renderer

This note tracks the first implementation slice for the hero GIF pipeline.

## Scope

- Convert the hero GIF asset into terminal art.
- Keep the hero renderer separate from the clock renderer.
- Keep the hero renderer separate from engine state.

## Current Choice

- Use the `chafa` CLI as the first implementation backend.
- Keep the Go runtime as the integration point.

## Boundary

- The hero renderer only knows about GIF input and terminal-sized output.
- The scene composer decides where the hero output lands.
- The renderer does not own animation timing.

## Next Step

- Wire the hero renderer into the v2 scene composer once the first frame path is stable.
