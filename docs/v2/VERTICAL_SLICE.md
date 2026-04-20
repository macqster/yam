# YAM v2 Vertical Slice

This document records the first runnable v2 scaffold.

## Scope

- runtime model
- minimal ecosystem model
- framebuffer
- ANSI emitter
- deterministic demo frame

## Demo Contract

The demo frame is intentionally small and fixed:

- width: 40
- height: 12
- one hero glyph
- one seed glyph

## Constraints

- no dependence on the existing `visualizer/` runtime
- the initial slice is historical context now that Bubble Tea is the default runtime
- no TUI behavior beyond the minimal scaffold
- no filesystem side effects

## Intent

The vertical slice exists to preserve the smallest deterministic frame proof that preceded the Bubble Tea runtime shell.
