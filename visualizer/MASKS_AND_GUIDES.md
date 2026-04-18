# Masks and Guides

Date: 2026-04-18
Scope: `visualizer/`
Purpose: record the spatial design model for layout, collision, growth guidance, and placement.

This document captures the stable ideas from the masks-and-guides brainstorm and translates them into a reusable visualizer design reference.

## Core Model

The visualizer should treat scene composition as a spatial pipeline:

`GRID -> MASKS -> FIELDS -> GUIDES -> RENDER`

That means:

- space is the primary source of truth
- geometry should be expressed as masks and fields, not hardcoded offsets
- behavior should be driven by fields and guides, not implicit one-off collision hacks
- rendering should be the last stage

## Coordinate Space

All runtime placement should be computed in terminal grid space.

Normalized coordinates are still useful for config and higher-level reasoning, but they must be converted before rendering.

Suggested convention:

- normalized space: `0.0 -> 1.0`
- grid space: terminal cells

This keeps layout logic resolution-independent and reduces offset drift when the terminal size changes.

## Masks

A mask defines where something exists, is allowed, or is blocked.

### Mask Types

- blocking masks: hero silhouette, UI panels, any other no-go region
- growth masks: areas where vines may grow
- influence masks: regions that attract or repel motion, density, or attention

### Representation

The simplest representation is binary:

- `0` = empty or disallowed
- `1` = occupied or allowed

For many layout problems, a soft mask is better:

- `0.0` -> no influence
- `1.0` -> strong influence

Soft masks are especially useful when a hard boundary is too brittle but the system still needs a clear spatial preference.

## Distance Fields

Distance fields are the critical bridge between masks and behavior.

Examples:

- distance to hero boundary
- distance to trunk zone
- distance to scaffold
- distance to scene edges
- distance to other blocked regions

Why they matter:

- they enable smooth avoidance
- they support organic growth
- they let the renderer and growth system prefer gradients instead of binary thresholds
- they make mask influence visible and tunable

Current runtime already uses this idea in places such as the support field and the shared render field.

## Derived Fields

Useful derived surfaces include:

- `distance_to_hero`
- `distance_to_edges`
- `distance_to_scaffold`
- `density_field`
- `edge_field`

These are not decorative intermediates. They are the place where the visualizer can express:

- how crowded an area feels
- how close growth is to a boundary
- how strongly a region should attract or repel structure

## Guides

Guides interpret masks and fields into behavior.

Examples:

- grow along gradients
- prefer mid-distance zones
- avoid uniform regions
- cluster near curvature
- bias movement toward readable corridors

Guides should remain distinct from the renderer.

If a rule changes where things are allowed or how they move, it belongs in a guide or field stage, not in final glyph selection.

## Element Placement Contract

Spatial elements should expose a consistent baseline control interface.

Required per-element parameters:

- `enabled`
- `anchor`
- `offset`

This gives each spatial element its own explicit on/off switch and a predictable placement model.

### Anchor Model

Terminal anchor options:

- `top_left`
- `top_center`
- `top_right`
- `center_left`
- `center`
- `center_right`
- `bottom_left`
- `bottom_center`
- `bottom_right`

Element self-anchor options:

- `top_left`
- `top_center`
- `top_right`
- `center_left`
- `center`
- `center_right`
- `bottom_left`
- `bottom_center`
- `bottom_right`

Placement rule:

1. choose a terminal anchor
2. choose an element self-anchor
3. align those points
4. apply the explicit offset

That makes placement predictable across hero, scaffold, panel, overlays, and future spatial elements.

### Example

```yaml
enabled: true
anchor:
  terminal: bottom_left
  self: bottom_left
offset:
  x: 2
  y: -1
```

## Design Rules

- use masks for presence and collision
- use distance fields for smooth influence
- use guides for behavior and composition
- keep rendering separate from spatial decision-making
- prefer normalized input where it helps, but resolve to grid space before draw time
- do not use debug mode as the master visibility toggle for ordinary spatial elements

## Current Runtime Notes

The current visualizer already reflects parts of this model:

- `hero_mask` is the authoritative hero collision geometry
- `support_mask` guides scaffold placement and routing; the live config keys still use `trunk_mask_*`
- `support_field` provides a soft distance surface for routing and selection; the live runtime name is `trunk_field`
- `render_field` acts as the intermediate density and priority layer before glyph output

Those pieces should be treated as the current implementation of the spatial pipeline, not as unrelated one-off systems.
