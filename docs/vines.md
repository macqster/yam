# Vines Contract

This document is the pre-runtime ownership contract for future YAM vine work.
It is intentionally not an implementation plan for a full flora runtime yet.

## Current Status

- Vines are deferred until the stability, spatial, guide, and documentation contracts remain green together.
- Hero GIF aesthetics are frozen for now; vine work must not require changing the active hero appearance.
- The scaffold, guides, and pointer probe can prepare vine placement, but they do not yet imply active vine growth.
- Vines are one flora family, not a special-case renderer.

## Ownership Contract

| Concern | Owner | Rule |
| --- | --- | --- |
| Vine instance state | future world/flora state | stores life state, axes, growth progress, and per-organism journal identity |
| Species defaults | future species registry | stores reusable vine traits, not per-instance history |
| Guide geometry | `core::guide::GuideState` | stores labeled points, lines, polylines, outlines, and guide sets |
| Spatial resolution | `core::spatial` plus the active compatibility helpers | resolves world points, anchors, guide lookup, and screen projection |
| Rendering | scene/render layers | visualizes already-resolved vine geometry; does not own vine state |
| UI/debug | HUD, debug, or future inspect surfaces | reads state and diagnostics; does not mutate growth unless gated by dev tooling |

## Minimal Future Data Shape

The first implementation should stay small enough to inspect:

- `VineInstance` - organism identity, species id, life state, current stats, journal id, and root/world attachment.
- `VineAxis` - ordered run of vine segments, including a main axis and optional lateral axes.
- `VineSegment` - start/end world points, thickness class, age, health, and optional guide association.
- `VineOrgan` - optional leaf, flower, fruit, or particle source attached to a vine segment.
- `VineGrowthTip` - active or dormant endpoint that decides the next segment under species and guide rules.

This shape is a contract sketch, not a demand to introduce these Rust types immediately.

## Guide Rules

- Vines may query `GuideState` by label, group, or guide set.
- Guide primitives stay world-space first and linework-only until the mask contract is promoted.
- Captured guide points and polylines may act as growth paths, attachment hints, or exclusion outlines.
- Spatial authoring uses points, anchors, guides, lines, and polylines; `node` remains reserved for plant morphology/anatomy.
- A vine should never reinterpret a rendered guide as pixels; it should consume the underlying guide data.

## Mask And Boundary Rules

- Do not make vines depend on raster masks, filled sprites, or empty-cell masking.
- Border awareness should begin as world-bound and guide-bound logic, not compositor side effects.
- If a future mask blocks or permits vine growth, that mask must be an explicit spatial primitive with tests.
- Vines must not change HUD, footer, modal, or debug overlay placement semantics.

## Render Rules

- A future vine render layer may draw stems, branchlets, leaves, flowers, fruit, and particles.
- That layer must receive resolved geometry from vine/world/spatial state.
- The render layer must not be the source of truth for growth, attachment, guide following, or lifecycle state.
- The same projection path used by hero, clock, and guides must be reused for vine world placement.

## Readiness Tests

Before vine behavior is added, keep these checks green:

- signed world-to-screen projection and off-screen preservation
- anchor identity resolution through the active compatibility layer
- screen-attached HUD/footer invariance under camera movement
- guide rendering from `SpatialGuideIndex`
- resize and camera round-trip scene tests
- negative tests for world/HUD boundary blur
- footer and hero baseline behavior

## First Implementation Slice

When implementation starts, the safest order is:

1. Add storage-only vine instance state with no visible output.
2. Add deterministic guide lookup for one named guide set.
3. Derive a simple world-space main-axis polyline from that guide set.
4. Render the derived axis through the existing projection path.
5. Add one negative test proving the vine layer cannot affect HUD/footer placement.

Stop if the implementation needs raster masks, hero aesthetic changes, or render-owned growth state.

## Detailed Roadmap

This chapter turns the vine ownership contract into a paced implementation path.
The north star is that vines become the first real flora subsystem, not a
decorative one-off layer. A vine is a world-attached organism whose geometry is
derived from state, guides, species rules, and spatial relations. Rendering is
only the last-mile visualization step.

The key discipline is to make each slice inspectable before moving to the next
one. Vines should become visible only after their ownership, spatial behavior,
and regression tests are stable.

### Phase 0: Readiness Gate

Before feature work starts, keep the current invariants green together:

- signed world-to-screen projection and off-screen preservation
- guide-set lookup through `GuideState` and `SpatialGuideIndex`
- anchor identity resolution through the active compatibility layer
- screen-attached HUD and footer invariance under camera movement
- hero and clock placement stability
- resize and camera round-trip scene behavior

Use `scripts/check.sh` as the regular small gate. The full `cargo test` suite is
useful at milestone boundaries, but targeted tests are better during tight
inner-loop work when the change touches only core data, spatial helpers, or a
single layer.

Phase 0 status as of 2026-05-05:

- targeted readiness checks are green for `core::spatial`, `scene::coords`,
  `render::render_state`, `scene::layers::status_layer`, and the key
  resize-round-trip scene tests
- the readiness gate should be treated as validated for the current branch state
  before Phase 1 storage-only flora work begins
- the remaining caution is structural, not red-test failure: the broader spatial
  layer is still only partly consolidated, so Phase 1 should stay narrow and
  avoid inventing new projection or guide ownership paths

### Phase 1: Storage-Only Flora

Add inert flora state with no visible output. The first code shape should be
small enough that it can be printed, debugged, and asserted directly.

Likely first module shape:

- `src/core/flora.rs` for the flora subsystem entry point
- `FloraState` as the world-owned flora container
- vine-specific structs inside `flora` or a nested `flora::vine` module
- `WorldState { flora: FloraState, ... }`

Likely first vine data:

- `VineInstance` for organism identity, species id, lifecycle state, stats,
  journal id, root/world attachment, axes, organs, and growth tips
- `VineAxis` for the main axis and optional lateral axes
- `VineSegment` for start and end world points, thickness class, age, health,
  and optional guide association
- `VineGrowthTip` for active or dormant endpoints
- `VineOrgan` for later leaves, flowers, fruit, or particle sources

Acceptance criteria:

- `WorldState::new()` initializes empty flora state
- no render output changes
- no UI changes
- tests prove vine state can be stored and inspected
- `scripts/check.sh` stays green

Phase 1 status as of 2026-05-05:

- `core::flora` now exists as a storage-only subsystem entry point
- `WorldState` now owns `flora: FloraState`
- the first inert vine data shape is present and testable without any render,
  UI, or system behavior changes
- targeted `core::flora` and `core::world` tests are green, and
  `scripts/check.sh` remains green
- Phase 2 should begin from this world-owned state surface rather than creating
  a second vine store elsewhere

### Phase 2: Deterministic Vine Seed

Add one deterministic, fixture-like vine instance, still invisible. This can
start in `WorldState::new()` or a narrow initializer, as long as it is explicit
and easy to remove or replace later.

Example seed:

- species id: `yam.vine.border_v1`
- display name: border-aware vine family
- journal id: stable per-organism id
- root attachment: world point or guide reference
- one dormant or active growth tip
- no segments until the guide or growth slice creates them

Acceptance criteria:

- the seed is deterministic across runs
- tests verify identity, species id, root attachment, and tip state
- the seed does not affect rendering or UI state

Phase 2 status as of 2026-05-05:

- `WorldState::new()` now seeds one deterministic border-vine instance through
  the world-owned flora store
- the seed remains invisible: zero segments, zero organs, and one dormant growth
  tip at the root attachment
- the seed shape is centralized in `core::flora` rather than being assembled ad
  hoc in world or render code
- targeted `core::flora` and `core::world` tests are green, and
  `scripts/check.sh` remains green
- Phase 3 should consume guide data through the existing guide/spatial path
  rather than changing where the seed lives

### Phase 3: Guide Lookup Slice

Teach vines to consume a named guide set without growth and without rendering.
This must use the guide data, not the rendered guide pixels.

The useful first helper is a pure query/derivation function that can resolve a
label such as `main-scene-vine-frame` into world-space guide primitives:

```rust
derive_vine_axis_from_guide_set(&world.guides, "main-scene-vine-frame")
```

Acceptance criteria:

- missing guide sets return an empty or no-op result
- guide lookup is deterministic by label
- line and polyline guides can produce ordered world-space points
- disabled guides are ignored where the guide contract says they should be
- no screen coordinates are stored in vine state
- no render module dependency enters `core` or `systems`

Phase 3 status as of 2026-05-05:

- `core::flora` now exposes `derive_vine_axis_from_guide_set(...)` as a pure
  guide-consumption helper
- guide lookup reads through `SpatialGuideIndex` and stays in world space
- the current helper accepts enabled `Line` and `Polyline` guides and emits
  ordered world-space points per guide path
- missing sets, disabled guides, and unsupported guide shapes resolve to an
  empty or skipped result without touching render code
- targeted `core::flora` and `core::spatial` tests are green, and
  `scripts/check.sh` remains green

### Phase 4: Static Axis Derivation

Turn guide data into a simple vine main axis. At this stage, vine geometry can
exist as data, but growth is still not a simulation yet.

The derived result should be an ordered set of `VineSegment`s:

- start and end in world space
- guide association retained where available
- stable thickness class
- stable age and health defaults
- no leaves or organs yet

Acceptance criteria:

- derivation is deterministic for the same guide set
- segments preserve signed world coordinates
- no raster masks, filled sprites, or empty-cell masking are involved
- tests can assert the exact segment list from a small guide fixture

Phase 4 status as of 2026-05-05:

- `core::flora` now exposes `derive_static_main_axis(...)` to turn guide paths
  into deterministic `VineSegment` geometry
- static axis derivation starts from the vine's explicit `root.world`, so the
  organism retains full control over its origin point in world space while guide
  paths shape the downstream segment run
- the derived axis stores the guide-set label plus per-segment guide association
- the current derivation emits healthy stem segments only and remains free of
  render, mask, and growth-side behavior
- targeted `core::flora` tests are green, and `scripts/check.sh` remains green

### Phase 5: Vine Render Layer

Add a `VineLayer` only after static world-space vine geometry exists. The layer
should read resolved vine state and project it through the same frame path used
by guides, hero, and clock.

Likely layer placement:

- after field/background
- before HUD, modal, and status layers
- before debug if debug should be allowed to overwrite diagnostic marks

The layer may draw stems first. Branchlets, leaves, flowers, fruit, and
particles can wait until the stem contract is pleasant and stable.

Acceptance criteria:

- vines appear as world content and move with camera/world projection
- the render layer does not mutate `WorldState`
- rendering owns glyph/style choices only, not growth or attachment decisions
- footer, modal, debug, and HUD placement remain unchanged
- add at least one negative test proving vine rendering cannot blur the
  world/HUD boundary or disturb the footer row

Phase 5 status as of 2026-05-05:

- the runtime now includes a read-only `VineLayer` that projects world-owned
  vine segments through the existing scene path
- world boot now realizes one static border-vine axis from the named guide set
  before rendering, so the layer visualizes existing geometry rather than
  deriving it on the fly
- layer ordering now treats vines as world content between the field and later
  world/HUD surfaces
- coverage now includes one render test proving vine visibility in frame output
  and one negative test proving the layer does not write into the footer row
- visual direction should stay flexible: `cbonsai` remains useful inspiration
  for layout, colors, and glyph economy, but the vine layer should keep evolving
  toward aesthetics that satisfy both the eye and the code-bounded terminal
  reality rather than freezing into a strict imitation

### Phase 6: Debug Inspection

Expose just enough read-only dev-mode information to inspect the new organism
state. This should not become an editor yet.

Useful first facts:

- vine count
- first or selected vine id
- species id
- segment count
- active and dormant tip counts
- guide-set label or guide association when present

Acceptance criteria:

- debug reads flora state without mutating it
- footer grammar stays compact
- no new modal is added unless there is a concrete inspection need
- everyday HUD and modal placement remain unchanged

Phase 6 status as of 2026-05-05:

- the existing debug panel now surfaces read-only vine inspection facts instead
  of introducing a separate editor or modal
- current coverage includes vine count, first vine id/species, axis count,
  segment count, active/dormant tip counts, and guide-set provenance
- the inspection surface remains subordinate to `dev_mode` and does not mutate
  vine growth, placement, or guide state
- the footer and modal grammar remain unchanged
- targeted debug-layer tests are green, and `scripts/check.sh` remains green

### Phase 7: First Growth System

Only after static vines render correctly, let `systems::growth` advance vine
tips. Start with a deliberately small deterministic rule.

First growth rule:

- every fixed number of world ticks, an active tip may extend by one segment
- guide-following chooses the next segment when guide data is available
- a simple fallback direction can be used when no guide is available
- max segment count prevents runaway growth
- dormant tips do nothing

Acceptance criteria:

- growth is deterministic for a given initial world state and tick count
- tests can advance the world and assert exact segment output
- render remains read-only and continues to visualize state only
- randomness is absent or explicitly seeded

Phase 7 status as of 2026-05-05:

- `systems::growth` now advances the boot vine on a fixed deterministic cadence
- the current rule is intentionally small: one active tip extends by one segment
  using the current terminal segment direction
- growth remains bounded by an explicit max-segment cap; once the cap is hit the
  vine stops with a spent tip and a mature life state
- render remains read-only and only visualizes the updated world-owned segment
  state
- focused growth tests, the updated debug-panel fact test, and
  `scripts/check.sh` are green
- the first old-branch-inspired lifecycle carryover is now explicit local tip
  lifespan: each `VineGrowthTip` tracks its own age and remaining growth budget
  so growth can stop because the tip is spent, not only because a global segment
  cap was reached
- the next anatomical carryover is now explicit mature-segment organ hosting:
  `run_aging` increments segment age, mature segments become eligible leaf
  hosts, and leaf organs are rebuilt from vine state rather than improvised in
  the render layer
- stem rendering now treats the main axis as plant structure rather than guide
  trace: `VineThicknessClass::Stem` uses a continuity-first glyph policy
  derived from local path connectivity, while threads still use the more
  expressive guide-line grammar

### Projection Contract Note

Vines depend on the shared world-to-screen projection, so the vine feature must
stay aligned with the repo-wide Cartesian world contract rather than carrying
its own rendering convention.

Projection note as of 2026-05-05:

- shared projection now treats world `y` as upward-positive and screen `y` as
  downward-positive
- the `y` flip happens at the projection boundary using the active viewport
  height; vines do not apply any local inversion of their own
- vine origin, guide paths, growth, and segment storage all remain in signed
  world space; only render-time projection converts them into terminal rows
- focused spatial, coordinate, render-state, clock-layer, and debug-layer
  checks are green again after the projection correction
- the hero sprite path and dev movement controls must stay aligned with that
  same contract: hero raster rows descend by decreasing world `y`, and `up`
  controls now mean `+1 y` in world space for hero offsets, clock offsets,
  pointer probes, and camera motion
- vines should remain clients of the shared drawing engine: stem/path raster
  policy now lives in `render/drawing.rs`, so future anatomy work can build on
  shared stroke, stamp, and occupancy primitives instead of accumulating
  vines-only cell-writing logic

### Phase 8: Branching And Organs

After the main axis is reliable, add lateral axes and organs in small slices.
Do not mix branching, leaves, flowers, fruit, and particle behavior into one
large pass.

Suggested order:

1. lateral axes with deterministic branch points
2. small leaves attached to specific segment positions
3. larger decorative leaves only after clipping and z-order are stable
4. flowers, fruit, or particles once lifecycle state needs them

Acceptance criteria:

- each organ is attached to a vine segment or axis insertion point
- organ state is inspectable
- z-order and clipping are tested before dense visuals land
- no organ behavior changes the hero or footer contracts

### Phase 9: Border Awareness

Border awareness should begin with explicit world and guide bounds, not masks.
Use world limits, boundary guides, or exclusion outlines before introducing any
mask-like behavior.

Acceptance criteria:

- border behavior is expressed in world or guide space
- vines do not depend on raster masks or compositor side effects
- growth near boundaries is deterministic
- any future mask promotion introduces an explicit spatial primitive with tests

### Pace And Direction

Keep the feature moving in reviewable increments:

1. data only
2. seeded instance
3. guide lookup
4. static derived geometry
5. render layer
6. negative HUD/footer test
7. deterministic growth
8. organs and branching
9. border behavior

Each slice should answer one question and preserve the existing scene contract.
If a slice needs hero aesthetic changes, render-owned growth state, raster masks,
or HUD/footer layout changes, stop and repair the roadmap before implementing it.

Update the relevant docs with the same change:

- update this file when the vine ownership or implementation direction changes
- update `TODO.md` when execution order or regression checkpoints change
- update `docs/LOG.md` when a decision or milestone is completed
