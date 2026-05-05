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
