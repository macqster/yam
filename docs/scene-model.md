# YAM-RUST Scene Model

This document defines the deterministic scene model that sits above ratatui.

## Assertions

- World-space entities must not depend on viewport dimensions.
- Given identical world state, camera, and viewport inputs, projection must produce identical screen coordinates.
- HUD elements must not use world-space projection.
- The conceptual layer order in this file is the source of truth for scene categories.

## Change Impact

- If you change this, also review `docs/rendering.md`, `docs/architecture.md`, and the projection-related tests.

## Purpose

The scene is the single source of truth for everything visible on screen.

It owns:

- spatial coherence
- layer ordering
- world to screen projection
- masking and occlusion
- separation of logic from rendering

Ratatui remains the final renderer only.

## Core Rule

Scene state must flow through this pipeline:

`Scene State -> Projection -> Layer Composition -> Render`

No system should render itself outside that path.

## Scene Systems

The scene is composed of these systems:

- Hero
- Vines
- Scaffold
- Particles
- UI

Each system owns its own state and emits renderable primitives.

## Presentation Layers

Conceptually, the terminal presentation is organized as:

- World: the rendered scene and world-attached systems
- HUD: screen-attached footer, indicators, and passive debug
- Overlay: modal or top-z-index panels such as settings and active debug UI

Rules:

- world content may be affected by projection
- HUD content stays screen-space only
- overlay content sits above both and may block input

## Coordinate Spaces

The engine must keep these spaces distinct:

- World Space: logical positions of entities
- Screen Space: terminal cell coordinates
- Anchor Space: offsets relative to another rendered object

Rules:

- world space is resolution independent
- screen space is terminal specific
- anchor space is relational, not absolute
- world and screen spaces must never be mixed implicitly

## Glossary

- world-space: logical positions of entities in the simulation
- screen-space: terminal cell coordinates
- anchor-space: offsets relative to another rendered object
- vines: the world-attached growth systems that occupy the growth-system layer

## Camera Model

Camera is a world-to-screen projection helper.

Responsibilities:

- map world space to screen space
- frame the viewport
- control offset

Rules:

- camera must not mutate world state
- viewport is not the camera
- camera must stay deterministic

## Out of Scope

- This document does not define the numeric layer implementation; see [`rendering.md`](rendering.md).
- This document does not define module ownership or coupling rules; see [`architecture.md`](architecture.md).

## Layering Model

The scene must render in a fixed order:

1. Background
2. Scaffold
3. Vines
4. Hero
5. Particles
6. UI
7. Debug

Rules:

- layer ordering is explicit
- no implicit z-index behavior
- no dynamic reordering at render time

The higher-level presentation stack maps this as:

- world below HUD below overlay
- overlays are modal when active
- footer and passive indicators belong to the HUD, not the overlay

## Masking and Occlusion

Masking is a first-class scene-model concept. The current renderer implements only the limited probe behavior described in [`rendering.md`](rendering.md).

Types:

- hero mask
- trunk mask
- no-go zones

Rules:

- masks are applied before final render
- masks are derived from scene state, not from visual output
- masking should not be simulated with empty cells

## Render Primitives

Scene systems must emit primitives, not draw directly.

Examples:

- glyph
- line segment
- filled region

The renderer later projects, sorts, and rasterizes those primitives.

## Frame Pipeline

1. Update scene state
2. Generate primitives per system
3. Apply masks
4. Project world to screen
5. Sort into layers
6. Compose the final frame buffer
7. Hand the buffer to ratatui

## Determinism

Given the same input, the scene must evolve the same way and render the same output.

No randomness without explicit seeding.

## Debug Layer

Debug is a dedicated layer.

It may show:

- bounding boxes
- masks
- anchors
- coordinate grids

It must not alter core scene state.

## Anti-Patterns

Avoid:

- rendering inside logic systems
- implicit coordinate conversion
- mixing world and screen space
- dynamic layer ordering
- masking by omission
