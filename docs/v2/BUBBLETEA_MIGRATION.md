# YAM v2 Bubble Tea Migration

This document records the move to a Bubble Tea-first runtime.

## Why Bubble Tea

- it fits the existing message-driven runtime model
- it gives a real terminal app loop instead of a custom redraw shell
- it aligns with the docs already imported from the v2 notes

## Current State

- the helper path remains available only as a verification harness
- the live loop, frame composition, and config handling now live in the Go Bubble Tea path
- the default runtime uses Bubble Tea rather than the custom terminal loop
- a separate Bubble Tea runtime shell now exists in `v2/cmd/yamv2`
- Bubble Tea is the default v2 runtime shell
- Python is retained only as a fallback verifier path

## Migration Order

1. keep Bubble Tea as the default `Init`, `Update`, and `View` path
2. keep `TickMsg`, resize handling, and key routing in Bubble Tea messages
3. keep engine, morphology, and render composition as downstream packages
4. preserve the file-backed scene config and clock observer surface
5. keep the Go launcher path as the only live launch path
6. keep the helper path isolated as verification-only while Bubble Tea remains default

## Package Mapping

### Runtime

- `v2/runtime/system.py` is verifier scaffolding, not the live bridge
- `v2/runtime/messages.py` becomes the message vocabulary
- `v2/runtime/model.py` becomes the Bubble Tea model state
- `v2/cmd/yamv2/main.go` is the first Go runtime shell

### UI

- `v2/ui/router.py` becomes the key routing and mode layer
- `v2/ui/model.py` becomes UI-only state
- `v2/ui/overlay.py` continues to generate overlay shapes

### Render

- the current GIF, clock, and theme pipeline stays in place
- Bubble Tea should eventually call into the existing render composer

## Non-Goals

- do not rewrite engine logic during the runtime swap
- do not redesign the render stack while swapping runtimes
- do not drop the file-backed scene config

## Constraints

- keep the migration incremental
- keep the Go launcher path as the only live launch path
- keep the repo-tracked logs and docs current
