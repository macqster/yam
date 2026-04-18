# Visualizer Dev Tools Roadmap

Date: 2026-04-18
Scope: `visualizer/`
Purpose: define a small set of development tools that reduce tuning time and make regressions easier to isolate.

This is a planning document, not a runtime spec.

The goal is to avoid broad tooling churn and instead build a few high-leverage tools that support the current `vines`-based visualizer architecture.

## Selection Criteria

The most useful dev tools here should:

- expose layout and field state directly
- make growth behavior reproducible
- catch invalid spatial states early
- reduce dependence on visual guesswork

## Recommended Priority Order

1. config lint
2. field inspector
3. seeded replay mode
4. runtime invariant checks

These tools give the best balance of validation, visibility, repeatability, and correctness.

## 1. Config Lint

### Purpose

Validate the live config surface and recipe overlays before they reach runtime.

### Minimum Behavior

- parse the base config
- parse each recipe overlay
- reject unknown keys in overlays
- check critical numeric ranges
- verify referenced source assets exist

### What It Should Answer

- Is a config edit structurally valid?
- Did a recipe accidentally introduce a typo or incompatible key?
- Are critical source files still present?
- Did a tuning pass move something outside a safe range?

### Useful Output Modes

- single-file lint
- base-config lint
- all-recipes lint

### Why It Matters

This is the fastest way to catch bad config states before they turn into runtime confusion.

### Current Tool

The current implementation is [lint_config.py](lint_config.py).

## 2. Field Inspector

### Purpose

Render or dump the spatial layers that actually drive the scene.

### Minimum Surfaces to Inspect

- `hero_mask`
- `support_mask`
- `support_field`
- `render_field`
- allowed-cell maps
- no-go regions

### What It Should Answer

- Is the mask rasterized where we expect?
- Is the support field actually changing when config changes?
- Is a growth issue coming from layout, scoring, or final glyph selection?
- Are debug overlays reflecting the real runtime state?

### Useful Output Modes

- terminal overlay
- compact text dump
- per-layer heatmap-style view

### Why It Matters

This is the fastest way to stop guessing about whether a bug is in geometry, guide logic, or rendering.

## 3. Seeded Replay Mode

### Purpose

Make growth runs reproducible.

### Minimum Behavior

- accept an explicit RNG seed
- record the seed used for a run
- replay the same growth sequence with the same config and terminal size

### What It Should Answer

- Did a change actually improve the scene or just change the random sample?
- Can a suspicious growth run be reproduced exactly?
- Are layout changes causing deterministic differences or just variance?

### Why It Matters

Without replayability, the current system is hard to debug because growth artifacts can look like noise.

### Suggested Scope

- seed the vines engine
- seed any ornament randomness
- surface the active seed in debug output

## 4. Runtime Invariant Checks

### Purpose

Detect broken spatial states immediately instead of discovering them visually later.

### Suggested Checks

- hero collision is not pierced
- panel collision is not pierced
- scaffold remains inside its allowed region
- anchors resolve inside the terminal grid
- offsets do not produce invalid placement
- fields remain finite and well-formed

### Severity Model

- warn for soft layout drift
- fail or highlight hard geometry violations

### Why It Matters

The system already has enough spatial structure that some errors should be treated as violations, not just cosmetic issues.

## Suggested Later Tool

### Spatial Summary Dump

This is useful, but lower priority than the three tools above.

It should print:

- terminal size
- anchor positions
- mask extents
- field ranges
- major config values

That would be a quick “what state am I actually in?” command for debugging.

## Recommended Implementation Order

If these tools are built incrementally, the order should be:

1. field inspector
2. runtime invariant checks
3. seeded replay mode

That order gets visibility first, then correctness, then reproducibility.

## Non-Goals

- do not build a large new UI just to inspect state
- do not make debug overlays the primary runtime behavior
- do not expand the engine architecture just to support tooling
- do not replace config tuning with tooling that hides the underlying state

## Relationship To Current Docs

This roadmap should be read alongside:

- [MASKS_AND_GUIDES.md](MASKS_AND_GUIDES.md)
- [CONFIG.md](CONFIG.md)
- [STATUS.md](STATUS.md)

Those documents describe the spatial model, live config surface, and current implementation state.
