# YAM v2 UI and Input

This document records the current UI separation contract.

## Boundary

- engine state stays in `v2/engine`
- UI state stays in `v2/ui`
- UI routing must not mutate engine state directly

## Current UI Signals

- `tab` moves focus
- `d` toggles debug/live mode
- `h` sets a help/status message

## Overlay

- UI status text is rendered as overlay shapes
- overlays belong above world content

## Notes

- this is the first explicit UI boundary in the v2 tree
- input handling is intentionally minimal right now
- prefer `bubbletea` primitives for events and focus handling
- add `bubbles` widgets only when the UI needs a concrete component that is not worth reimplementing
- keep `bubbleboxer` and other optional layout helpers as later-stage additions, not default dependencies
