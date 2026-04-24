# Debug Research Ingest - 2026-04-24

Source directory:

- `/Users/maciejkuster/Desktop/yam-rust_debugging_260424-2009`

## Ingested Findings

The research set is consistent across the issue log, live traces, and enforcement notes:

- `BUG-001`: viewport recenter drift
- `BUG-002`: camera semantic drift
- `BUG-003`: projection pipeline fragmentation
- anchor drift and order-dependent attachment behavior
- clipping/visibility should be separated from position math
- resize/fullscreen must not mutate world state

## Shared Root Cause

The same structural problem appears in multiple documents:

- multiple projection semantics are active at once
- camera is treated as both translation and centering mechanism
- viewport is leaking into world-space projection
- some render-time values are still derived through `UiState` side effects

## Practical Handling Rules

The research recommends these invariants:

- keep a single projection pipeline
- choose one camera contract and apply it everywhere
- treat viewport as crop-only
- resolve anchor-space before screen projection
- clip visibility instead of clamping position
- avoid entity-specific projection logic
- log world, camera, viewport, view, and screen values when debugging

## Current Repo Relevance

These findings match the current Rust repo debt:

- camera semantics are still mixed across modules
- field/hero/clock/debug do not yet share one formal transform contract
- render-derived state is still written into `UiState`
- the active grid renderer exists, but the projection model is not yet fully normalized

## Recommended Next Step

Before adding new visual systems, settle the projection contract first:

1. choose camera semantics
2. encode them once in code
3. update all layers to use that single contract
4. add tests that lock resize and anchor behavior
