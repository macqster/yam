# YAM v2 Repo Audit

Date: 2026-04-20

This document maps the current repo against the v2 spec and records the initial reconstruction stance.

## Summary

The current repository is still organized around the existing `visualizer/` stack and shell startup integration.

There is not yet a native v2 code layout in the repo.

The practical path is:

- preserve the current repo as the v1 baseline
- add v2 code alongside it in a clearly separated layout
- replace the visualizer stack only where the v2 architecture demands it

## Keep

These parts are useful and can remain as-is until v2 code replaces them:

- `v1/`
- `ghostty/`
- `fastfetch/`
- `chafa/`
- `bin/fastfetch-chafa`
- `bin/yam`
- `docs/` maps for the current setup
- `visualizer/README.md`, `STATUS.md`, and process docs as historical context

## Adapt

These parts may be reused conceptually or partially, but they need review before any direct carryover:

- `visualizer/src/main.py`
  - runtime loop and config reload flow
- `visualizer/src/renderer.py`
  - scene composition patterns
- `visualizer/src/layout.py`
  - geometry and collision model
- `visualizer/src/vines_engine.py`
  - phase-structured engine ideas
- `visualizer/src/render_field.py`
  - intermediate field composition
- `visualizer/src/tree_scaffold.py`
  - scaffold layering behavior

## Replace

These areas are incompatible with the intended v2 spec and should be treated as provisional implementation details:

- the current Python visualizer as a compatibility baseline, not the v2 default
- ad hoc vines heuristics as the long-term engine model
- renderer logic that mixes composition, fallback behavior, and visual policy in one place
- any implicit coupling between layout, growth, and draw order

## Gaps

The repo currently lacks:

- a native `v2` source tree
- a dedicated v1 landing page
- a tracked package/module layout matching the v2 layers
- engine/simulation primitives named in the v2 docs
- explicit framebuffer, morphology, and emitter packages for v2
- a test harness for deterministic frame output
- a v2 migration log tied to implementation steps

## Risk Notes

- Reusing too much of `visualizer/` would collapse the separation between the old stack and the v2 spec.
- Keeping the old visualizer as the main implementation path would make it harder to enforce the forward-only architecture.
- The new work should be added in a separate v2 tree rather than buried in the current visualizer modules.

## Next Actions

1. create a dedicated v2 source tree
2. define module/package boundaries from the v2 layer model
3. add a minimal vertical slice
4. log every structural change in `docs/v2/LOG.md`
