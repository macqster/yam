# YAM v2 Repo Audit

Date: 2026-04-20

This document maps the historical repo state against the v2 spec and records the initial reconstruction stance.

## Summary

The repository now has a native Rust baseline in `/src` with a persistent world, UI stack, render stack, and system scaffold.

The older Go-era root artifacts were pruned from the tracked baseline.

The practical path is now:

- keep the Rust rewrite as the canonical implementation tree
- record every material change in `docs/v2/LOG.md`
- keep the old architecture only in historical notes and migration docs

## Keep

These parts are useful and should remain tracked:

- `Cargo.toml`
- `Cargo.lock`
- `src/`
- `assets/`
- `README.md`
- `docs/v2/`

## Adapt

These areas still require review before major extension:

- `src/ui/`
  - composition and interaction layers
- `src/render/`
  - debug overlays and figure rendering
- `src/systems/`
  - future simulation behavior

## Replace

These areas remain provisional and should be revised when they start to accumulate special cases:

- hard-coded offsets or coordinate hacks in rendering
- ad hoc debug overlays that do not have a documented contract
- any coupling between world state and view-only calibration

## Gaps

Current follow-up gaps:

- a formal test harness for deterministic frame output
- a documented camera and viewport contract for all scene layers
- a stable hero/chafa ingestion boundary
- release notes for the Rust binary and installer commands

## Risk Notes

- Reusing too much of `visualizer/` would have collapsed the separation between the old stack and the v2 spec.
- Keeping the old visualizer as the main implementation path would have made it harder to enforce the forward-only architecture.
- The new work was added in a separate root tree rather than buried in the current visualizer modules.

## Next Actions

1. keep logging structural changes in `docs/v2/LOG.md`
2. keep pruning build output before commits
3. extend the render-layer contract only when the current baseline is stable
