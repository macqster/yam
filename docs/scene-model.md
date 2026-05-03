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
- Clock
- Guides
- Vines
- Scaffold
- Particles
- UI
- Main Scene
- Greenhouse

Each system owns its own state and emits renderable primitives.

Rules:

- hero and clock are world-attached systems, not HUD systems
- the clock follows the hero in world space and carries its own relative offset
- guides are world-attached semantic primitives: for now they are linework-only points, lines, polylines, and outline shapes that future vines and other world systems can query and that the debug overlay can visualize, but they are not raster masks or filled solids; each guide has its own label and may belong to an optional named group, and guide sets are modeled explicitly so collections can be addressed as named groupings as well as individual primitives; guide sets should be created through the core guide API, not by render layers; sprites and solid masks are future work
- the guide / line generator is project-wide, not vines-only: it supports guide drawing now and should remain suitable for future mask edges, rulers, and other world-space annotations, and it must remain capable of generating any line in any direction across the full YAM world size
- linework guides are rendered through a Bresenham-style geometry layer plus a glyph-appearance layer, following [`docs/soft-line-atlas.md`](soft-line-atlas.md) for shallow/stroke transitions and longer world-spanning lines, not with filled blocks or raster masks; the engine target is universal line coverage across the full YAM world size, using the grammar key `LineFamily -> LengthBucket -> Direction -> PhaseRole -> CellBand -> LocalStep`
- the soft-line drawing engine is expected to become exhaustive for the world: every possible world-space line direction and span should be representable without falling back to block fills or ad hoc exceptions
- the pointer probe is the practical authoring aid for linework: it should be usable to record precise coordinates for guides, points, and masks while the guide system remains world-attached and queryable; the term `nodes` is currently reserved for plant morphology/anatomy systems and should be treated as provisional until the spatial terminology is researched further
- the intended end-state is a single spatial relation graph that can express absolute datum guides, relative anchors, masks, and organism guidance paths for growth, movement, and lifecycle state without duplicating attachment math across systems
- the smallest canonical spatial layer should start with datum/world transforms, attachment resolution, guide-set lookup, and screen projection helpers; masks and organism guidance can remain layered concerns until the base relation layer is proven
- the first canonical spatial API surface stays narrow: `SpatialPoint`, `SpatialAnchor`, `SpatialAttachment`, `SpatialProjection`, `SpatialGuideIndex`, and `SpatialResolver` are enough to express the shared resolver without collapsing guide data or render helpers into one blob
- the likely module mapping is:
  - `scene/coords.rs` for point and projection primitives
  - `scene/entity.rs` for attachment composition
  - `core/guide.rs` for the guide index and guide-set storage
  - `render/guide.rs` for render-only guide visualization
  - `core/spatial` for the shared resolver layer that already exists as the first cut
- the safest migration order is:
  1. add the new shared spatial types without changing visible output
  2. move projection and attachment math behind the new resolver
  3. update guide lookup consumers to the guide index abstraction
  4. reroute guide rendering through the new projection helpers
  5. remove old paths only after the projection, guide, and render tests still pass
- the spatial model should lean on Cartesian and Euclidean logic where possible, because signed axes, centered datum math, and direct distance reasoning make world authoring and placement easier to reason about
- the plant-side morphology model should stay graph-based and segment-based: plants are growth programs that emit geometry, built from repeating metamers under meristem-driven rules rather than from one universal vine-shaped template
- the project's plant-structure thinking has historical inspiration from `cbonsai`, and that lineage should remain visible as a useful constraint even as YAM grows beyond a single bonsai-style plant into a broader greenhouse and multi-species model
- inspiration lineage for the current plant/UI thinking can stay short and explicit:
  - `cbonsai` for compact plant growth, branching, and readable terminal botany
  - `Dwarf Fortress` for ASCII-era muscle memory, dense inspectable state, and strong world-model thinking
  - `Cataclysm: DDA` for keyboard-first survival UI, modal discipline, and practical terminal ergonomics
- the plant-side terminology should stay biologically informed but implementation-friendly: `metamer` for the repeating structural unit, `internode` for the segment, `meristem` for the growth point, `axis` for a branch system, and `insertion` or `attachment` for where organs connect; `node` remains provisional until the plant-side model is more fully researched
- a minimal Rust-shaped plant model can stay small and explicit: `Plant` owns species and lifecycle state, `Axis` owns a growth branch and ordered metamers, `Metamer` carries one internode and organ attachments, `Meristem` represents an active growth point, and `Organ` covers leaf/flower/fruit/branch outputs; geometry should be derived from this state rather than owning it
- a minimal lifecycle update loop can stay equally small: `Seed -> Growth -> Mature -> Senescent -> Decay`, with active meristems driving growth steps, species rules choosing new metamers or organs, and geometry being regenerated from state each tick
- organ state should remain explicit and inspectable: buds, leaves, flowers, and fruit can each progress through `bud -> growing -> mature -> aging -> dead` without requiring the whole plant to collapse into a single global state
- the greenhouse/lab space should be the place where lifecycle tuning becomes visible, while the main scene can keep the current prototypes comparatively static and readable
- each plant organism should be treated as an independent life-form with its own life state, stats, and variables; a species registry or database layer should hold species definitions, morphology traits, growth rules, and other reusable data that drive in-YAM generation and emulation
- the species registry should be read-heavy and simulation-friendly: it can store canonical species metadata, but per-plant runtime state stays with the living organism instance in `WorldState` or its flora subsystem
- each life-form should also have a dedicated log-journal so lifecycle events, growth changes, and debugging notes can be tracked per organism without flattening everything into a single global log
- a life-form journal should stay compact and event-oriented: lifecycle transitions, growth steps, organ births/removals, environment influences, damage/pruning, and debug annotations are the highest-value entries
- the journal should be human-readable first and machine-friendly second, so greenhouse inspection can scan it quickly without losing deterministic simulation detail
- the species registry payload should stay compact and reusable: species id/name, morphology defaults, branching pattern, internode length, leaf distribution, growth rate, tropism rules, lifecycle tuning, allowed organs, and debug labels are the highest-value fields
- the registry should not store per-instance life history; that belongs in the individual plant journal and runtime state
- the state/stat/journal/registry layer may deserve its own dedicated render mode in the future, distinct from the main scene and greenhouse, if the inspection burden grows enough to justify a specialized view
- an alternative UI strategy is a per-life-form popup window that shows the organism’s relevant data and allows limited tweaks such as growth rate or lifecycle length, while keeping the underlying state registry-backed and the journal per-instance
- comparison rule of thumb: a dedicated mode fits large-scale registry browsing and greenhouse administration; per-life-form popups fit quick inspection or light tuning; lightweight debug overlays fit routine day-to-day checks when the organism count is still manageable
- Lua scripting could be used as a bounded optional layer for species authoring or debug/dev plugins, but it should stay Rust-hosted and should not own the canonical world state, lifecycle authority, or render path
- the registry taxonomy can be kept explicit by grouping fields into anatomy defaults, growth rules, visual phenotype ranges, lifecycle tuning, and debug labels; this makes the same registry useful for generation, emulation, and inspection without overloading one field bucket
- botanical basics should remain a reference point for species design: if a morphology term, growth rule, or anatomy field is unclear, the registry should be informed by real plant structure before the term is committed to YAM
- terminology authority should be explicit:
  - `strict` terms are botanically grounded and should match standard plant meaning as closely as practical
  - `inferred` terms are YAM-specific design interpretations built from real plant structure and terminal constraints
  - `provisional` terms are placeholders that may change once the plant model or spatial terminology is researched further
- plant-language references live in [`docs/glossary.md`](glossary.md); this contract only needs the YAM-specific rule that `node` stays reserved for plant morphology/anatomy and the spatial side should continue to use points, anchors, guides, lines, and polylines
- a concrete species-entry checklist can help keep the registry consistent: `species_id`, `display_name`, `habit/form`, `anatomy defaults`, `morphology notes`, `branching pattern`, `internode length`, `leaf distribution`, `growth rate`, `tropism rules`, `lifecycle tuning`, `allowed organs`, `debug labels`, and `journal hints`
- the current prototype targets should be treated as concrete morphological briefs: the tree-stump scaffold is a 3-5 cell-thick bark-textured support with a fork just below the hero, the vine family is a border-aware sprawling growth form with a thick main stem and smaller branches, and the monstera-like plant is a multi-stem growth area with big fenestrated leaves and species-specific lifecycle behavior
- example species entry: the Y-shaped tree-stump scaffold can be modeled as a woody support habit with a short trunk, one early fork below the hero, thick bark-textured segments, dead or minimal meristems, and a structural role rather than active canopy growth; this is an inference inspired by woody stem/meristem basics, not a claim that the scaffold is a real species
- the scaffold should be treated as a pre-rendered hero-support remnant in the main scene: a dead or senescent trunk-like form with one stable fork under the hero GIF, minimal lifecycle behavior, and only very light moss growth over the bark while vines can later attach and partially overgrow it
- the scaffold should not behave like a normal active plant: its lifecycle can be fixed or nearly fixed, with the primary requirement being structural stability, bark texture, and readable Y-shaped support geometry
- example species entry: the vine family can be modeled as a climbing or sprawling habit with an active apical meristem, long internodes, optional lateral meristems, border-awareness, and the ability to produce leaves, flowers, and possibly fruit along the main stem; this is an inference inspired by climbing plant and node/internode basics
- example species entry: the monstera-like plant can be modeled as an aroid-style climbing or self-supporting form with multiple stems from one growth area, large fenestrated leaves, aerial/climbing tendencies, and a lifecycle that can produce flowers and fruit later in development; this is an inference inspired by Monstera morphology rather than a direct species copy
- a species-entry template can stay consistent by filling these fields for each prototype: `species_id`, `display_name`, `habit/form`, `support strategy`, `stem/axis plan`, `growth-tip behavior`, `branching pattern`, `internode range`, `leaf shape/distribution`, `organ outputs`, `life-state defaults`, `registry tags`, `journal hints`, and `inspection notes`
- a botanical species-template should keep a few fields separate from general registry data: `taxonomic inspiration`, `support habit`, `growth mode`, `leaf architecture`, `reproductive strategy`, `life cycle notes`, and `ecology cues`; this helps keep the registry grounded while still remaining a YAM-specific abstraction
- compact species template examples:
  - tree-stump scaffold: `species_id = yam.scaffold.stump_v1`, `display_name = bifurcated stump scaffold`, `habit/form = woody support`, `support strategy = fixed hero anchor`, `stem/axis plan = short trunk with one fork`, `growth-tip behavior = minimal/dead`, `branching pattern = one early Y-fork`, `leaf shape/distribution = none or negligible`, `organ outputs = structural only`, `life-state defaults = dead-or-senescent`, `registry tags = scaffold, support, hero-anchor, bark`, `inspection notes = pre-rendered, stable fork, moss-only lifecycle, vine attachment zones`
  - vine family: `species_id = yam.vine.border_v1`, `display_name = border-aware vine family`, `habit/form = climbing/sprawling`, `support strategy = border-aware guide-following`, `stem/axis plan = long main stem plus optional laterals`, `growth-tip behavior = active apical meristem`, `branching pattern = opportunistic side branches`, `leaf shape/distribution = repeat along stem`, `organ outputs = leaf, flower, fruit optional`, `life-state defaults = dynamic`, `registry tags = vine, climbing, border-aware, guided`, `inspection notes = gravity response, wall avoidance, mask interaction, thick main stem`
  - monstera-like plant: `species_id = yam.floral.monstera_like_v1`, `display_name = fenestrated multi-stem flora`, `habit/form = aroid-like climber or self-supporting multi-stem`, `support strategy = multiple stems from one growth area`, `stem/axis plan = several axes from a base`, `growth-tip behavior = active and species-specific`, `branching pattern = sparse to moderate`, `leaf shape/distribution = large fenestrated leaves`, `organ outputs = leaf, flower, fruit possible`, `life-state defaults = active growth`, `registry tags = monstera-like, fenestrated, multi-stem, aroid-inspired`, `inspection notes = leaf size, perforation, stem count, lifecycle pacing`
- the main scene is the live visualiser space: hero GIF, clock plus weather widget, and procedurally generated flora under the same world-datum contract
- the greenhouse is a separate simulation space for rooms, labs, and multiple flora species, with each organism allowed its own anatomy, morphology, and growth behavior
- vines are one flora family, not a singular architectural exception, and the system must make room for other procedurally growing plant organisms with different shapes and lifecycle rules
- the main scene should be treated as the live visualiser/screensaver composition, with hero GIF, tree-stump-like hero scaffolding split into a Y-shaped fork under the hero, clock widget, weather widget, and procedurally generated vines acting as the organic frame around the composition
- the greenhouse remains an early conceptual simulation space for rooms, labs, pots, bowls, and controlled-environment biome themes where different plant organisms can be developed and their lifecycle simulated
- current flora concepts include pre-generated tree-stump scaffolding at boot, tropical vines framing the composition, and a monstera-like plant with large aesthetic growing leaves
- the initial flora implementation should favor a small, inspectable set of prototypes: the Y-shaped tree-stump scaffold, a guide-following vine family, and a monstera-like plant with large leaf organs
- UI is screen-attached presentation, not world-attached simulation state
- HUD and UI may still derive spacing, insets, and alignment rules from the shared world-spacing model, but their final placement remains screen-attached presentation
- everyday debug overlays may stay available in the normal experience, while deeper dev controls remain explicitly gated and visually distinct
- keyboard interaction should preserve muscle-memory conventions, with stable mode boundaries and a discoverability path for rare actions
- visual hygiene is a scene-model requirement: text, glyphs, and numerals must remain readable as density increases
- a command palette or similar action hub should exist as the fallback route for infrequent actions and entity jumps
- the mode map should remain stable and explicit:
  - `normal`: everyday scene use with lightweight HUD and visible-but-safe debug
  - `inspect`: focused reading, selection, and drill-down detail
  - `debug`: diagnostic overlays for spatial, camera, and runtime inspection
  - `dev`: gated editing, mutation, and simulation tooling
  - `command palette`: search-based fallback for rare actions and navigation
- the keybinding surface should match the mode map instead of blurring it:
  - `normal` favors navigation, toggles, and spatial reading
  - `inspect` favors focus movement, drill-down, and entity inspection
  - `debug` favors passive visibility and diagnostic toggles
  - `dev` favors the explicit modal controls already present in code, such as hotkeys, move, settings, pointer probe, and camera-home actions
  - `command palette` is the rare-action entrypoint for search, jump, and non-muscle-memory commands
- precise guide authoring should treat the pointer probe and soft-line renderer as a pair: the pointer records coordinates, and the line engine visualizes the resulting geometry in world space

## Guide Capture Workflow

The practical authoring flow for world relations is:

1. move the pointer probe to an exact world coordinate
2. record the coordinate as a guide point, node, or anchor
3. connect recorded coordinates into a line or polyline when a path is needed
4. wrap recorded points into an outline when a mask or bounded region is needed
5. visualize the result with the soft-line renderer before it is treated as a stable spatial primitive

Rules:

- points are the atomic capture unit for spatial guide capture; `nodes` should remain reserved for plant morphology/anatomy systems unless the terminology is formally expanded later
- lines and polylines are ordered relations between captured coordinates
- masks should be derived from explicit captured outlines, not from visual guesses
- the pointer probe is the precision capture instrument, and the soft-line renderer is the visual validation instrument
- guide capture should remain world-space first and dev-overlay second
- the UI should support at least a read-only preview list of guide sets and subsets, so grouped geometry like a polyline outline for a mask shape can be inspected without editing it

## Capture UI Contract

The current debug/dev surface for guide capture is:

- pointer probe: capture exact coordinates in world space
- debug info panel: inspect the live pointer position, camera position, and projected entity facts
- hotkeys popup: remind the user that pointer, camera-home, and move/settings are the current dev controls
- move popup: step selected world-attached entities when authoring placement relationships

Rules:

- capture should happen in world space first, not in HUD space
- the debug info panel is the factual readout, not the authoring editor
- the hotkeys popup should remain discoverability, not a second editor surface
- move mode is for explicit world-attached positioning, not for hidden geometry mutation
- guide-set previews should be read-only by default; they are for inspection, not for mutating the spatial graph
- the footer should remain compact and instructional, not a full command reference; it is the stable reminder surface, while the palette and modal shells handle the denser action vocabulary

## Presentation Layers

Conceptually, the terminal presentation is organized as:

- World: the rendered scene and world-attached systems
- HUD: screen-attached footer, indicators, and passive debug
- Overlay: modal or top-z-index panels such as settings and active debug UI

## Screen Zones

The active scene model should also be readable as stable screen zones:

- `main scene` - the world playfield for hero, vines, scaffold, flowers, guides, and other world-attached simulation content
- `hud/footer` - the reserved bottom row for compact status, hints, and runtime reminders
- `debug/inspect` - readable diagnostic and inspection surfaces that may remain visible during normal use
- `modal overlay` - centered, top-most panels that temporarily sit above both world and HUD

Rules:

- world content may be affected by projection
- HUD content stays screen-space only
- HUD content stays screen-space only, even when its spacing is derived from world-model rules
- overlay content sits above both and may block input
- screen zones are stable and should not be reinterpreted ad hoc by individual layers
- the main scene owns world-attached composition; the HUD owns screen-attached reminders; debug/inspect owns readable diagnostics; overlays own modal interactions

## Visible Content Map

- main scene: hero GIF, tree-stump scaffolding, vines, flora, guides, weather/clock composition, and world-tied diagnostics
- hud/footer: compact mode hint, version stamp, and one-line runtime reminders only
- debug/inspect: coordinate readouts, camera/world position, probe state, entity detail, and other readable diagnostics; it may show numbers and labels, but not the main command vocabulary
- modal overlay: hotkeys, move, settings, command help, and temporary control surfaces that are opened intentionally

## Modal Vocabulary

The currently implemented modal vocabulary is intentionally small and grouped:

- `hotkeys` - discoverability for the current dev controls
- `move` - target selection and movement for world-attached entities
- `settings` - tabbed presentation/state inspection for positions, widgets, gif, and theme values
- `pointer` - dev-only probe state, shown through the debug surface rather than as a standalone modal
- `camera home` - stored and recalled through runtime keys, not through a separate overlay

Rules:

- hotkeys should describe the current dev controls that are actually implemented
- move should stay focused on target choice and explicit motion
- settings should stay tabbed and presentation-oriented
- the pointer probe and camera-home actions are dev-only helpers, not always-on HUD content

Rules:

- the footer must stay compact and instruction-oriented
- the main scene should carry composition and density, not control help
- debug/inspect may be visible in daily use, but they must remain legible and secondary to the footer
- overlays are temporary action surfaces and should not replace the permanent HUD contract
- the footer may show the current mode and the minimal runtime hint only
- debug/inspect may show state facts and labels, but not the full action menu
- modal overlays may show the denser key vocabulary, but only while active

## Coordinate Spaces

The engine must keep these spaces distinct:

- World Space: logical positions of entities
- Screen Space: terminal cell coordinates
- Anchor Space: offsets relative to another rendered object

Rules:

- world space is resolution independent
- world space is Euclidean and centered on the datum
- screen space is terminal specific
- anchor space is relational, not absolute
- world and screen spaces must never be mixed implicitly
- the world playfield is a static `212x56` full-screen area, while the full terminal frame is `212x57` with the bottom row reserved for the footer

## Glossary

- world-space: logical positions of entities in the simulation
- world-space: Euclidean positions in the simulation with `(0, 0)` at the centered datum
- screen-space: terminal cell coordinates
- anchor-space: offsets relative to another rendered object
- vines: the world-attached growth systems that occupy the growth-system layer
- guides: world-space annotations and constraints that vines may query or follow

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
- the ordering ladder is a precedence contract, not just a rendering convenience: lower layers are eligible to be overwritten by higher layers
- world layers compose before HUD layers, and HUD layers compose before overlay layers
- world-tied debug/dev assets belong to the world stack and should be ordered with other world content, not with HUD chrome
- modal overlays are always top-most among visible layers
- masks may filter write paths, but they do not override the ordering ladder
- a layer may only overwrite content in the region it proposes; it must not assume later layers will repair accidental writes

The higher-level presentation stack maps this as:

- world below HUD below overlay
- overlays are modal when active
- footer and passive indicators belong to the HUD, not the overlay
- the current visible runtime controls are intentionally narrow: `[d]ev mode` in the footer, with `[h]otkeys`, `[m]ove`, `[s]ettings`, `[p]ointer`, `[C]` store camera home, `[c]` recall camera home, and `F5` available once dev mode is active

## Masking and Occlusion

Masking is a first-class scene-model concept. The current renderer implements only the limited probe behavior described in [`rendering.md`](rendering.md).

Types:

- hero mask
- trunk mask
- no-go zones

Rules:

- masks are applied before final render
- the hero mask currently applies only to the field layer; HUD and debug layers ignore it
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
