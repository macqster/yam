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
- the initial slice predates the Bubble Tea default runtime and remains a historical baseline
- no TUI behavior beyond the minimal scaffold
- no filesystem side effects

## Intent

The vertical slice exists to prove the v2 tree can render a deterministic frame before any larger engine work starts.
