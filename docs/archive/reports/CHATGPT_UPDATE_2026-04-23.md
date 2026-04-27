# ChatGPT Update: Last 5 Edits

Date: 2026-04-23
Branch: `yam-rust` (now the default branch for `yam`)

This note summarizes the last five commits on `yam-rust`, in order from oldest to newest, with the intent, files touched, and the resulting behavior.

## 1. `6342f63` - `render: stream chafa hero frames`

### Goal
Move the hero from a one-shot cached image to a streaming animation pipeline.

### What changed
- Added a persistent chafa stream path in `src/render/chafa.rs`.
- Added hero stream ownership to `src/render/hero.rs`.
- Added state plumbing in `src/ui/state.rs` so the hero lives with UI state.
- Updated `src/runtime.rs` to tick the hero every loop.
- Updated `src/ui/scene.rs` to render the hero from the live stream path.

### Result
- The hero asset was no longer limited to a static snapshot.
- The code now had the correct place to ingest live frame output from chafa.

### Notes
- This commit introduced the streaming architecture, but the frame parsing was still rough and later needed correction.

## 2. `262bfeb` - `render: restore static chafa snapshot helper`

### Goal
Undo an accidental regression in the snapshot helper.

### What changed
- Updated `hero/renderer.py` so the Python helper matched the static snapshot mode again.

### Result
- The snapshot helper stopped pretending to animate.
- This did not affect the Rust binary directly, but it kept the helper tree aligned.

### Notes
- This commit mattered because the Rust app and the Python helper had drifted apart.

## 3. `5e9740f` - `render: fix hero stream parsing and input flow`

### Goal
Fix the first broken streaming iteration and keep the UI input loop sane.

### What changed
- Tightened `src/render/chafa.rs` parsing.
- Updated `src/render/hero.rs` to consume streamed frames through a tick/update path.
- Updated `src/runtime.rs` to call the hero tick before rendering.
- Adjusted `src/ui/panels/status_bar.rs` to show the footer more cleanly.
- Adjusted `src/ui/scene.rs` so the render order matched the new flow.

### Result
- The hero pipeline was wired through a non-blocking update step.
- The runtime loop now had a clean place to advance visual state before drawing.

### Notes
- This was the first attempt at real animation flow, but the stream parser still emitted noisy artifacts.

## 4. `1edc564` - `render: fix hero stream parsing and clipping`

### Goal
Remove the noisy control-artifact output and make the hero clip more predictably.

### What changed
- Simplified `src/render/chafa.rs` so valid frames were not dropped by an overly strict filter.
- Changed `src/render/hero.rs` to crop against the viewport instead of hard-clamping the sprite origin.

### Result
- The hero could move more freely.
- The stream parser stopped throwing away legitimate frames because of the old heuristic.

### Notes
- This fixed the obvious `DDDD...`-style artifact path, but it exposed a later layout seam issue.

## 5. `413a935` - `render: remove viewport seam clipping for hero`

### Goal
Remove the invisible seam that was clipping the hero around the viewport split.

### What changed
- Updated `src/render/hero.rs` so the hero clip used the full terminal frame rather than the centered viewport rect.
- Adjusted `src/ui/scene.rs` so the debug/hero projection no longer treated the viewport seam as a hard boundary.

### Result
- The hero stopped getting chopped at roughly the quarter-width seam.
- The render path now uses screen-space clipping for the sprite instead of panel-seam clipping.

### Notes
- This was the last structural fix needed to remove the invisible blocker in the visible UI.

## Summary

The last five edits moved the hero pipeline through three distinct phases:

1. Static hero rendering.
2. Streaming hero ingestion.
3. Cleanup of the rendering boundary so the hero can move freely in screen space.

The important architectural outcome is that the Rust app now owns the actual runtime behavior. The Python helper file is only a side path and does not affect `yam-install` or the running `yam-rust` binary.

## Current takeaways

- `src/render/chafa.rs` is the real source of hero rendering behavior.
- `src/render/hero.rs` owns frame state and the per-tick update path.
- `src/runtime.rs` is responsible for advancing hero state before drawing.
- `src/ui/scene.rs` controls layering and composition.
- Footer rendering is kept as the final screen-space overlay.

