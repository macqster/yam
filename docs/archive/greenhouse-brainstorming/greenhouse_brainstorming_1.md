# Greenhouse Brainstorming 1

Status: brainstorming
Scope: future expansion planning
Repo baseline: YAM `0.3.9`
Snapshot note: version number is contextual, not a long-term contract
Implementation status: do not implement from this file directly

## Purpose

This document collects candidate directions for the future YAM greenhouse/lab
expansion.

The goal is not to define a final feature spec yet. The goal is to capture
architecturally plausible ideas in a form that Codex can later transform into
small, reviewable planning docs, TODO entries, tests, or implementation patches.

## Promotion Rules

A section from this file may become implementation work only after it is
promoted into one of the following:

- `TODO.md` for execution tracking
- `docs/greenhouse.md` for architecture contract
- `docs/audit.md` for risk/monitoring notes
- an explicit Codex task with named files and bounded scope

Until promoted, all sections are non-binding candidate material.

## Reading Rules For Codex

- Treat this file as a brainstorming source, not an implementation contract.
- Do not implement features directly from this document unless a later task
  explicitly promotes a section into `TODO.md` or an owning docs contract.
- Preserve YAM's current architecture guardrails:
  - `core/` owns pure data and simulation vocabulary.
  - `systems/` may mutate world state but must not render.
  - `scene/` and `render/` visualize existing state.
  - `ui/` owns presentation state, modals, settings, and screen-attached flows.
- Keep greenhouse work world-attached, not dashboard-like.
- Prefer small seams, probes, and debug surfaces before visible mechanics.

## Current Repo Context

YAM currently has:

- main scene world
- sparse sandbox world
- Chafa-backed animated hero layer
- scaffold and procedural vine framing
- clock/date/weather companions
- weather sprite atlas and review surfaces
- modal/dev surfaces
- early `src/systems/` seams
- flora vocabulary in progress
- existing docs/TODO references to future greenhouse direction

The greenhouse should build from existing world, scene, flora, organism, spatial,
and systems contracts rather than inventing a parallel UI framework.

## Core Greenhouse Premise

The greenhouse is a separate YAM space for observing, growing, testing, and
curating terminal-native organisms.

It should feel like:

- a contained botanical lab
- a nursery for future scene organisms
- a testbed for plant behavior
- a diegetic development surface
- a world-space environment, not a settings panel

## Candidate Feature Lanes

### Lane A: Greenhouse World

Label: greenhouse-world

Type: major future world mode
Priority: high concept, not immediate implementation

Description:

A separate world mode entered from YAM runtime or launched through a dedicated
command/mode flag.

Possible names:

- greenhouse
- lab
- nursery
- conservatory
- vivarium

Candidate responsibilities:

- host experimental organisms
- display growth stages
- inspect species profiles
- test density, aging, and spatial constraints
- provide safe non-main-scene expansion space

Architecture notes:

- should be modeled as a world or room model
- should not be a HUD overlay
- should reuse viewport/camera/world projection where possible
- should expose dev-friendly inspection surfaces

Open questions:

- Is greenhouse a separate scene mode, a separate world type, or a room inside a
  larger world model?
- Should it have its own launcher command?
- Should it share the main scene hero anchor or avoid hero rendering entirely?

### Lane B: Species Registry

Label: species-registry

Type: core vocabulary / data model
Priority: high

Description:

A registry of plant/organism definitions used by greenhouse and future main
scene flora.

Candidate fields:

- stable species id
- display name
- visual family
- growth habit
- preferred anchor type
- density behavior
- lifecycle profile
- sprite/glyph profile
- palette profile
- journal template
- debug tags

Example ids:

- `stump_scaffold`
- `border_vine`
- `hanging_vine`
- `moss_cluster`
- `root_fan`
- `terminal_fern`
- `weather_lichen`

Architecture notes:

- likely belongs under `core::organism` or adjacent flora vocabulary
- should remain render-agnostic
- render glyphs/palettes should be referenced semantically, not hard-owned by
  core data

Open questions:

- Should species definitions be Rust constants first?
- Should later species data become external plain-text assets?
- How much botanical terminology should be encoded versus documented?

### Lane C: Organism Journal

Label: organism-journal

Type: inspection / diegetic metadata
Priority: medium-high

Description:

Each organism can expose a compact journal entry for inspection.

Candidate journal data:

- species
- age
- growth stage
- current anchor
- health/status
- recent events
- environmental preferences
- debug notes

Possible use cases:

- greenhouse inspector
- dev modal
- future save/debug dump
- species documentation generation

Architecture notes:

- journal data should be derived from simulation state where possible
- journal rendering belongs in UI/scene/dev surface, not core
- persistent journal entries should be carefully scoped to avoid premature save
  system complexity

Open questions:

- Is the journal user-facing, dev-facing, or both?
- Should journal entries be generated text or structured fields?
- Should the greenhouse expose one selected organism at a time?

### Lane D: Growth System Probes

Label: growth-system-probes

Type: systems/runtime mechanics
Priority: medium

Description:

Small systems that mutate organism state over ticks.

Candidate systems:

- aging
- growth stage transition
- density pressure
- anchor seeking
- local crowding
- seasonal/weather influence
- pruning/recovery

Architecture notes:

- belongs in `src/systems/`
- mutates `WorldState` or flora state
- must not render
- should be testable without terminal output

Implementation posture:

- start with deterministic probes
- avoid real-time complexity early
- prefer fixed seed / deterministic fixtures

Open questions:

- Should growth run continuously or only in greenhouse mode?
- Should main scene flora stay mostly decorative for now?
- What is the minimum interesting lifecycle?

### Lane E: Greenhouse Inspector Surface

Label: greenhouse-inspector-surface

Type: dev/ui surface
Priority: medium

Description:

A modal or side surface for inspecting greenhouse state.

Candidate views:

- selected organism
- species registry
- growth tick stats
- density map summary
- palette/glyph profile
- recent simulation events

Architecture notes:

- screen-attached modal is acceptable for inspection
- inspected content should refer to world-attached organisms
- avoid turning greenhouse into a dashboard

Open questions:

- Should inspector reuse existing modal architecture?
- Should navigation be keyboard-only with minimal bindings?
- Should this be dev-only at first?

### Lane F: Terminal Plant Art Atlas

Label: terminal-plant-art-atlas

Type: visual asset / rendering vocabulary
Priority: medium-high

Description:

A curated atlas of terminal-native plant glyph forms.

Candidate atlas categories:

- stems
- forks
- roots
- leaves
- moss
- flowers
- tendrils
- buds
- seed pods
- bark texture
- soil texture
- glass/greenhouse frame elements

Architecture notes:

- should separate shape vocabulary from semantic color mapping
- should remain plain text where practical
- should support review/debug surfaces
- should avoid opaque ANSI blobs as canonical sources

Open questions:

- Should assets live under `assets/`, `docs/`, or a new flora atlas path?
- Should atlas entries be Rust constants, text files, or both?
- Should Moebius/RexPaint-style editing be part of the long-term workflow?

### Lane G: Greenhouse Environment Model

Label: greenhouse-environment-model

Type: simulation / world metadata
Priority: medium

Description:

A lightweight environmental model that affects greenhouse organisms.

Candidate environment variables:

- light
- humidity
- warmth
- soil / substrate
- airflow
- enclosure type
- weather influence
- artificial lamp state

Architecture notes:

- keep this small and symbolic
- avoid numerical simulation bloat
- expose state as semantic categories first

Example categories:

- `light = low | filtered | bright | artificial`
- `humidity = dry | balanced | damp`
- `temperature = cool | mild | warm`
- `substrate = bark | soil | stone | scaffold`

Open questions:

- Should outside weather influence greenhouse conditions?
- Should the weather widget provide data to greenhouse systems?
- Should greenhouse have time-of-day visual shifts?

### Lane H: Main Scene Transfer Rules

Label: main-scene-transfer-rules

Type: integration policy
Priority: high before main-scene expansion

Description:

Define how organisms developed in greenhouse can later appear in the main scene.

Candidate transfer rules:

- only stable species can leave greenhouse
- main scene organisms must respect hero/scaffold composition
- no dense panel-like flora
- no obstruction of core companion readability
- visible organisms must remain world-attached
- debug-only species remain greenhouse-only

Architecture notes:

- prevents greenhouse experiments from destabilizing the curated main scene
- supports staged expansion without visual chaos

Open questions:

- What qualifies a species as stable?
- Should species have a `greenhouse_only` flag?
- Should the main scene use hand-curated placement instead of full simulation?

## Candidate Milestones

### Milestone 0: Planning Only

Goal:

Record greenhouse vocabulary, constraints, and candidate lanes.

Exit criteria:

- brainstorming file exists
- no implementation changes required
- future work lanes are labeled clearly

### Milestone 1: Contract Draft

Goal:

Create an owning docs contract for greenhouse architecture.

Likely file:

- `docs/greenhouse.md`
- or expand `docs/greenhouse-roadmap.md`

Exit criteria:

- defines greenhouse role
- defines what greenhouse is not
- identifies owning modules
- records architecture guardrails

### Milestone 2: Data Vocabulary Probe

Goal:

Add minimal species/organism vocabulary without visible runtime behavior.

Likely areas:

- `src/core/organism`
- flora state definitions
- tests for stable ids / profiles

Exit criteria:

- no UI behavior change
- deterministic tests pass
- docs updated

### Milestone 3: Sandbox Greenhouse Prototype

Goal:

Render a minimal greenhouse-like space in an isolated mode or existing sparse
sandbox path.

Possible constraints:

- no persistence
- no complex growth
- one or two organisms
- dev-only or hidden launcher path

Exit criteria:

- clearly separate from main scene
- no regression in main scene
- render order documented

### Milestone 4: Inspection Surface

Goal:

Inspect selected greenhouse organisms.

Exit criteria:

- selected organism has structured info
- inspector remains screen-attached
- organism remains world-attached
- no dashboard drift

### Milestone 5: Transfer Candidate

Goal:

Promote one greenhouse organism into a stable main-scene candidate.

Exit criteria:

- species profile stable
- visual rules documented
- placement rules documented
- main scene readability preserved

## Hard Constraints

- Do not turn greenhouse into a generic dashboard.
- Do not bypass world/viewport semantics.
- Do not put simulation truth in render layers.
- Do not make core depend on UI, render, terminal, or Ratatui.
- Do not add large mechanics before vocabulary and contracts stabilize.
- Do not let main scene enrichment precede greenhouse containment rules.
- Do not commit opaque runtime art blobs as canonical editable sources.
- Do not promote greenhouse brainstorm items into runtime behavior without a
  contract doc or TODO promotion step.

## Useful Labels

Use these labels in later notes and TODO extraction:

- `candidate`
- `constraint`
- `open-question`
- `contract-needed`
- `test-needed`
- `visual-atlas`
- `core-vocabulary`
- `systems-probe`
- `greenhouse-only`
- `main-scene-transfer`

## Immediate Brainstorming Questions

1. Should the greenhouse be visually closer to a laboratory, conservatory, or
   botanical diorama?
2. Should the first greenhouse prototype include the hero GIF, or should it be a
   quieter non-hero space?
3. What is the first organism worth modeling beyond the current vine/scaffold?
4. Should weather affect greenhouse behavior, or should greenhouse initially be
   isolated from external conditions?
5. Should plant growth be visible animation, slow state drift, or mostly
   inspectable metadata?
6. What should be allowed to graduate from greenhouse into the main scene?

## Brainstorming Pass 2: Greenhouse Identity And First Expansion Shape

Status: brainstorming
Scope: identity, first prototypes, organism vocabulary, and world feel
Implementation status: do not implement directly from this pass

### Conceptual Reference Notes

These notes summarize external conceptual patterns. They are not source
citations and should not be treated as authoritative project contracts or
implementation requirements.

- Botanic gardens often treat living plant collections as central collections,
  not decorative background. YAM can borrow this idea by treating greenhouse
  organisms as curated living entries rather than random scenery.
- Real living collections use labels/accession-like identifiers to make plants
  inspectable, traceable, and useful to staff or researchers. YAM can translate
  this into stable organism ids, species profiles, and journal/inspector fields.
- Greenhouse and nursery language separates propagation, staging, care, display,
  and transfer to permanent planting. YAM can use the same split for incubating
  organisms before any main-scene promotion.
- Ratatui uses immediate-mode rendering, so greenhouse state should remain in
  YAM-owned state/systems and be visualized by render/UI surfaces rather than
  owned by widgets.
- Unicode and box-drawing glyphs are useful for terminal-native structures, but
  font support and cell alignment must be treated as visual constraints.

### Greenhouse Identity Options

#### Identity Option A: Botanical Lab

Label: identity-botanical-lab

Description:

The greenhouse is a controlled research space for observing synthetic terminal
organisms.

Strong traits:

- structured benches
- accession tags
- controlled lamps
- specimen trays
- glass or frame geometry
- dev-friendly inspection surfaces

Good for:

- organism registry
- deterministic growth probes
- debug overlays
- state inspection

Risks:

- may become too dashboard-like
- may visually over-emphasize panels and labels
- may feel less organic than main YAM scene

Use if:

- greenhouse starts as a dev-first organism testbed
- inspection and traceability matter more than atmosphere

#### Identity Option B: Conservatory Diorama

Label: identity-conservatory-diorama

Description:

The greenhouse is an atmospheric indoor garden space with curated organisms,
glass framing, and quiet environmental variation.

Strong traits:

- world-space glass architecture
- plant clusters arranged as visual compositions
- subtle time/weather influence
- fewer visible labels by default
- strong scene identity

Good for:

- user-facing greenhouse mode
- visual continuity with main scene
- slow ambient simulation

Risks:

- may hide useful dev information
- may become visually dense too early
- may blur boundary between greenhouse and main scene

Use if:

- greenhouse is intended as a second polished YAM world
- atmosphere matters as much as organism testing

#### Identity Option C: Nursery / Propagation Room

Label: identity-nursery-propagation-room

Description:

The greenhouse is a staging room where small organisms are grown from simple
forms into candidates for later world transfer.

Strong traits:

- trays
- cuttings
- seedlings
- small pots
- propagation shelves
- staged growth lanes

Good for:

- minimal first prototype
- lifecycle staging
- transfer rules
- species profile tests

Risks:

- may look too small-scale if not framed well
- may underuse YAM's broader world/camera model

Use if:

- first greenhouse implementation should be small, safe, and testable
- growth stages matter more than complex environment simulation

#### Preferred Starting Blend

Label: identity-starting-blend

Recommendation:

Start with a nursery / propagation room wrapped in botanical lab discipline.

Rationale:

- small enough for a first isolated prototype
- compatible with stable organism ids and species profiles
- naturally supports greenhouse-only staging
- avoids immediate pressure to build a full conservatory scene
- can later expand into a more atmospheric conservatory

### First Prototype Shape

Label: first-prototype-shape

Candidate name:

- `greenhouse_nursery`

Description:

A quiet greenhouse nursery space with one or two organism trays, minimal glass
framing, and optional inspector access.

Non-goals:

- no persistence
- no save/load system
- no complex simulation economy
- no large plant collection
- no main-scene transfer yet
- no new dashboard framework

Candidate visible elements:

- greenhouse frame or glass roof hint
- one propagation bench
- one tray of seedlings/cuttings
- one larger scaffold or vine specimen
- optional selected-organism marker
- optional compact accession label

Candidate invisible/state elements:

- species id
- organism id
- growth stage
- age ticks
- anchor kind
- environment profile
- greenhouse-only flag

### First Organism Candidates

#### Organism Candidate A: Seedling Tray

Label: organism-seedling-tray

Type: clustered small organism
Priority: high for first prototype

Description:

A compact tray of tiny sprouts used to test repeated organism instances,
growth stage variation, and simple inspection.

Candidate states:

- empty tray
- germinating dots
- two-leaf sprouts
- uneven small stems
- overgrown tray

Why useful:

- visually compact
- easy to keep world-attached
- good for density testing
- low risk of damaging main composition

Testable properties:

- stable deterministic layout
- glyph profile does not overflow tray bounds
- growth stage changes only through system state
- render output remains stable under fixed seed

#### Organism Candidate B: Cutting Jar

Label: organism-cutting-jar

Type: single contained organism
Priority: medium-high

Description:

A small cutting in water or substrate, useful for testing roots, stems, and
transparent/container-like terminal art.

Candidate states:

- bare cutting
- root nubs
- small root fan
- leaf pair
- ready for transfer

Why useful:

- strong nursery/lab identity
- visually readable in a small footprint
- creates a natural transfer metaphor

Testable properties:

- root glyphs remain below water/substrate line
- stem glyphs remain connected
- container frame remains readable across fonts

#### Organism Candidate C: Weather Lichen

Label: organism-weather-lichen

Type: environmental indicator organism
Priority: medium

Description:

A small lichen patch that changes subtle visual state based on symbolic
humidity/light/weather input.

Candidate states:

- dry
- balanced
- damp
- glowing/debug-active

Why useful:

- creates a bridge to weather data
- can stay small and decorative
- supports environment model without full plant simulation

Risks:

- may over-couple greenhouse to weather widget too early
- should remain optional until environment contracts stabilize

#### Organism Candidate D: Training Vine

Label: organism-training-vine

Type: controlled vine specimen
Priority: medium

Description:

A greenhouse-contained vine trained along a small support frame, distinct from
the main-scene border vine.

Candidate states:

- young tendril
- attached tendril
- forked vine
- leafing vine
- overreaching vine

Why useful:

- reuses existing vine/scaffold conceptual work
- good for anchor seeking and pruning probes
- visually tied to current YAM identity

Risks:

- may duplicate current main-scene vine too closely
- may invite premature main-scene transfer

### Accession / Label Model

Label: accession-label-model

Description:

Borrow the idea of living-collection labels without importing full botanical
database complexity.

Candidate fields:

- `organism_id`
- `species_id`
- `display_name`
- `origin_space`
- `greenhouse_status`
- `growth_stage`
- `stability_class`
- `main_scene_eligible`

Example:

```text
organism_id: gh-0001
species_id: seedling_tray
display_name: Seedling Tray A
origin_space: greenhouse_nursery
greenhouse_status: observed
growth_stage: two_leaf_sprouts
stability_class: experimental
main_scene_eligible: false
```

Architecture notes:

- accession-like fields should remain plain structured data
- label rendering belongs to UI/render surfaces
- labels should be optional in visual mode
- dev mode may show more fields than user-facing mode

Open questions:

- Should ids be generated or hand-authored constants at first?
- Should accession labels be visible in-world or only in inspector?
- Should main-scene eligibility be derived or manually curated?

### Greenhouse Spatial Zones

Label: greenhouse-spatial-zones

Description:

Treat greenhouse as a small world with named zones instead of a generic panel.

Candidate zones:

- `entry_path`
- `propagation_bench`
- `specimen_shelf`
- `training_frame`
- `substrate_bed`
- `inspection_marker`
- `glass_frame`
- `lamp_zone`

First prototype minimum:

- `propagation_bench`
- `glass_frame`
- `inspection_marker`

Architecture notes:

- zones should be world-space concepts
- zones may later host organism anchors
- zones should not imply UI panels
- first pass can be static geometry only

### Visual Grammar Notes

Label: greenhouse-visual-grammar

Preferred glyph families:

- box drawing for greenhouse/glass frame
- light diagonals or soft punctuation for stems
- braille or dot glyphs for seeds/moss/lichen
- block shades only where density is intentional
- minimal heavy borders unless dev/debug mode is active

Visual constraints:

- keep plants readable in monospaced terminal fonts
- avoid glyphs with poor fallback behavior unless explicitly tested
- separate greenhouse architecture glyphs from organism glyphs
- avoid turning plant clusters into noisy texture fields
- prefer small curated glyph palettes per organism

BTAS/YAM palette direction:

- greenhouse frame: muted blue/green slate
- plant primary: restrained green family
- growth highlight: soft warm green or amber
- warning/overgrowth: muted red/brown, not bright alarm red
- labels: low-contrast neutral foreground unless selected

### Environment Model V0

Label: environment-model-v0

Description:

Keep the first environment model symbolic and tiny.

Candidate v0 fields:

```text
light: filtered | artificial
humidity: balanced | damp
temperature: mild
substrate: tray | water | bark | scaffold
outside_weather_link: disabled
```

Rules:

- no numeric simulation at first
- no external weather coupling at first
- environment may influence display text or debug status before visible growth
- visual behavior must remain deterministic under fixed state

Future extension:

- enable optional weather influence only after greenhouse state is stable
- expose weather influence through environment profile, not direct widget calls

### First Stable Work Package Candidates

#### Work Package A: Greenhouse Contract Doc

Label: wp-greenhouse-contract-doc

Type: documentation promotion
Implementation risk: low

Goal:

Promote stable constraints from this brainstorming file into an owning docs
contract.

Candidate target:

- `docs/greenhouse.md`

Must include:

- what greenhouse is
- what greenhouse is not
- owning modules
- first prototype boundaries
- promotion rules
- main-scene transfer policy

#### Work Package B: Species Profile Vocabulary Only

Label: wp-species-profile-vocabulary

Type: core vocabulary probe
Implementation risk: low-medium

Goal:

Add minimal data vocabulary for species profiles without rendering or runtime
behavior changes.

Must not include:

- UI changes
- growth systems
- persistence
- visible scene behavior

#### Work Package C: Static Greenhouse Nursery Mock

Label: wp-static-greenhouse-nursery-mock

Type: isolated visual prototype
Implementation risk: medium

Goal:

Render a static greenhouse nursery composition through an isolated path or
sparse sandbox path.

Must not include:

- main scene changes
- organism simulation
- persistence
- weather coupling

#### Work Package D: Inspector Shape Sketch

Label: wp-inspector-shape-sketch

Type: UI/dev planning
Implementation risk: low-medium

Goal:

Define the fields and layout style for inspecting one selected greenhouse
organism.

Must decide:

- in-world label vs modal inspector
- user-facing vs dev-only fields
- minimum selected organism model

### Recommendation After Pass 2

Decision status: recommendation, not accepted contract

Recommended next promoted direction:

1. Draft `docs/greenhouse.md` as an architecture contract.
2. Keep first implementation target limited to species/organism vocabulary.
3. Delay visible greenhouse rendering until the contract names the first mode
   boundary.
4. Use `greenhouse_nursery` as the preferred first prototype identity.
5. Use `seedling_tray` as the safest first organism candidate.

Rationale:

This sequence preserves YAM's documentation-first discipline while giving the
greenhouse a concrete identity. It also avoids prematurely coupling simulation,
UI, weather, and main-scene transfer.

## Brainstorming Pass 3: Interaction Model, Lifecycle Discipline, And Room Grammar

Status: brainstorming
Scope: interaction grammar, organism lifecycle, room semantics, and first-mode boundaries
Implementation status: do not implement directly from this pass

### Conceptual Reference Notes

These notes summarize external conceptual patterns. They are not source
citations and should not be treated as authoritative project contracts or
implementation requirements.

- Living-collection practice distinguishes accessioning, record retention,
  propagation material, and de-accessioning. YAM can borrow the lifecycle
  discipline without importing a full botanical database.
- Some living-collection policies accession material immediately when it enters
  a collection, including seeds, cuttings, scions, and full plants. YAM can use
  the same principle for greenhouse organisms: every inspectable organism gets
  a stable id as soon as it enters the greenhouse model.
- Collection records are useful because they connect identity, origin, spatial
  placement, status, and future use. YAM can translate this into organism id,
  origin space, zone anchor, growth stage, stability class, and transfer status.
- Ratatui event handling is commonly centralized before messages or subcalls are
  dispatched. YAM should preserve a similar discipline: input should select,
  inspect, or request mode changes, while greenhouse state remains owned by YAM
  state/systems.
- Box-drawing and semigraphic characters work best when treated as a constrained
  terminal vocabulary, not as arbitrary decorative glyph soup. Greenhouse glass,
  benches, trays, and labels should therefore use small tested glyph sets.

### Greenhouse Mode Boundary

Label: greenhouse-mode-boundary

Description:

The greenhouse should have an explicit runtime boundary before it gains visible
complexity.

Candidate boundary models:

- separate scene mode
- sparse sandbox variant
- hidden dev-only mode
- future named room inside a larger world model

Preferred first boundary:

- hidden or dev-only `greenhouse_nursery` mode, backed by existing sparse
  sandbox or equivalent isolated render path

Rationale:

- prevents accidental main-scene coupling
- allows static visual experiments without visible product promises
- keeps early organism vocabulary testable
- provides a safe place for inspector and growth probes

Non-goals:

- no public mode switch until contract exists
- no default launch path change
- no main scene replacement
- no shared hero requirement

Open questions:

- Should the greenhouse be entered through a command flag, debug key, or config
  toggle?
- Should the first mode be compiled in normal builds or only reachable in dev
  mode?
- Should sparse sandbox become the generic experimental world host?

### Interaction Grammar V0

Label: interaction-grammar-v0

Description:

Define a minimal interaction grammar for selecting and inspecting greenhouse
organisms without introducing a dashboard framework.

Candidate interactions:

- move selection marker between inspectable organisms
- open selected organism inspector
- close inspector
- toggle labels
- toggle dev overlay
- pause/resume growth ticks if growth exists

Explicitly excluded interactions:

- dragging plants
- free-form editing
- mouse-first workflows
- live species creation
- persistence-affecting commands
- direct mutation from render layer

Preferred first input model:

- keyboard-only
- small fixed binding set
- dev-only if uncertain
- all actions routed through existing app/event handling patterns

Candidate binding sketch:

```text
tab       next inspectable organism
shift-tab previous inspectable organism
enter     open inspector for selected organism
esc       close inspector / return to greenhouse
l         toggle compact labels
d         toggle dev overlay
space     pause/resume growth ticks, if growth exists
```

Architecture notes:

- input changes selection or requests actions
- systems mutate organism/growth state
- render reads state and selection
- inspector displays derived fields
- no widget should own organism truth

Open questions:

- Should labels be always visible in dev mode?
- Should selection marker be world-attached or screen-attached?
- Should inspector open as modal or sidecar panel?

### Inspectable Entity Model

Label: inspectable-entity-model

Description:

Not every visual cell should be an organism. The greenhouse needs a small
category for entities that can be selected and inspected.

Candidate inspectable kinds:

- organism instance
- organism cluster
- zone marker
- environment controller
- greenhouse fixture

Preferred v0 inspectable kinds:

- organism instance
- organism cluster
- zone marker

Candidate fields:

- `inspectable_id`
- `kind`
- `world_anchor`
- `display_name`
- `summary_line`
- `source_ref`

Example:

```text
inspectable_id: gh-inspect-0001
kind: organism_cluster
world_anchor: propagation_bench:left_tray
display_name: Seedling Tray A
summary_line: two-leaf sprouts, experimental, greenhouse-only
source_ref: organism:gh-0001
```

Architecture notes:

- inspectable entity data may be derived from world/organism state
- selection state belongs to app/UI state
- inspectable world anchors must remain stable across viewport changes
- render may draw a marker but must not invent inspectable truth

Open questions:

- Should inspectable ids be separate from organism ids?
- Should fixtures like lamps be inspectable in v0?
- Should inspectable markers be visible when labels are off?

### Organism Lifecycle V0

Label: organism-lifecycle-v0

Description:

Define a symbolic lifecycle that is small enough to test but expressive enough
to support nursery identity.

Candidate lifecycle states:

- `registered`
- `propagating`
- `established`
- `observed`
- `stable_candidate`
- `greenhouse_only`
- `retired`

Suggested meaning:

- `registered`: organism exists in greenhouse records/state
- `propagating`: organism is early-stage material such as seedling/cutting
- `established`: organism has a stable visible form
- `observed`: organism is being tracked for behavior or visual quality
- `stable_candidate`: organism may be considered for later transfer
- `greenhouse_only`: organism is not eligible for main-scene transfer
- `retired`: organism is removed from active display but retained in records

Architecture notes:

- lifecycle state should not imply persistence in v0
- retired/de-accessioned semantics should be conceptual only at first
- state transitions should be explicit and testable if implemented
- main-scene transfer requires separate promotion policy

Open questions:

- Is `greenhouse_only` a lifecycle state or a flag?
- Should `stable_candidate` be manually assigned only?
- Should `retired` exist before persistence exists?

### Room Grammar V0

Label: room-grammar-v0

Description:

Define greenhouse as a small composed room with visual zones and fixtures, not
as a free-floating panel.

Candidate room elements:

- back glass frame
- roof/glass hint
- propagation bench
- tray row
- cutting jar slot
- training frame
- lamp line
- floor/substrate strip
- selected marker
- optional label tag

Preferred v0 room composition:

```text
[glass / roof hint]
[lamp line, optional]
[training frame or specimen shelf]
[propagation bench with tray + cutting jar]
[floor/substrate strip]
```

Visual rules:

- room geometry is world-attached
- labels may be screen-attached or world-adjacent, but must be clearly scoped
- glass/frame glyphs should be lighter than debug borders
- benches/trays should not visually compete with plants
- selection marker should be visible without becoming a bright cursor blob

Open questions:

- Should greenhouse use perspective-like composition or flat orthographic room
  layout?
- Should the room scroll with camera movement or fit inside one fixed viewport?
- Should greenhouse preserve the centered world datum convention?

### Minimal State Sketch

Label: minimal-state-sketch

Description:

A non-binding shape for the smallest useful greenhouse state.

Candidate structures:

```text
GreenhouseState
  mode_id
  environment_profile
  zones[]
  organisms[]
  inspectables[]
  selected_inspectable_id
  labels_visible
  growth_paused
```

```text
GreenhouseOrganism
  organism_id
  species_id
  display_name
  origin_space
  zone_anchor
  growth_stage
  lifecycle_status
  stability_class
  main_scene_eligible
```

```text
GreenhouseZone
  zone_id
  display_name
  world_rect_or_anchor
  zone_kind
```

Scope notes:

- this is vocabulary sketching, not Rust API design
- actual structs should follow existing repo conventions
- do not add persistence assumptions from this sketch
- avoid storing render glyphs directly in organism state

### Test Surface Ideas

Label: test-surface-ideas

Description:

If promoted, greenhouse work should start with cheap deterministic tests.

Candidate tests:

- species ids are stable and unique
- organism ids are stable within fixture state
- inspectables resolve to existing organism or zone refs
- selected inspectable survives viewport resize
- labels toggle without changing organism state
- growth pause does not alter render-only state
- greenhouse mode does not affect main scene state
- greenhouse render fixture is deterministic under fixed seed

Test posture:

- prefer pure data tests first
- add render snapshot tests only after visual contract exists
- do not test speculative UI bindings before event boundary is chosen

### Risks Introduced By Pass 3

Label: pass-3-risks

Risk: premature interaction system

Mitigation:

- keep interaction grammar as planning only
- require mode boundary before bindings
- make inspector dev-only at first if implemented

Risk: accidental database bloat

Mitigation:

- borrow accession discipline only as lightweight identity/status vocabulary
- avoid persistence until explicitly promoted
- keep records as runtime state first

Risk: visual over-framing

Mitigation:

- use minimal greenhouse room glyphs
- avoid heavy boxes around every organism
- reserve strong borders for debug overlays

Risk: state/render leakage

Mitigation:

- no organism truth in render layer
- no widget-owned greenhouse data
- keep systems/render/ui roles explicit

### Recommendation After Pass 3

Decision status: recommendation, not accepted contract

Recommended next promoted direction:

1. Treat `greenhouse_nursery` as a hidden/dev-first room mode.
2. Treat `seedling_tray` and `cutting_jar` as first inspectable organism
   candidates.
3. Keep interaction grammar limited to selection, inspection, label toggle, and
   dev overlay toggle.
4. Use accession-like ids immediately for inspectable organisms, but do not add
   persistence.
5. Promote only the mode boundary, lifecycle vocabulary, and state/render
   separation into `docs/greenhouse.md`.

Rationale:

## Brainstorming Pass 4: Visual Staging, Label Semantics, And First Art Direction

Status: brainstorming
Scope: visual grammar, label behavior, fixture roles, and first static composition
Implementation status: do not implement directly from this pass

### Conceptual Reference Notes

These notes summarize external conceptual patterns. They are not source
citations and should not be treated as authoritative project contracts or
implementation requirements.

- Botanic-garden collection practice treats accession numbers, labels, records,
  and maps as linked systems. YAM can borrow the linked-system idea while
  keeping its first greenhouse model lightweight and runtime-only.
- Accession-like labels should identify living material, but labels do not need
  to carry all record data visually. YAM can use compact in-world tags plus a
  richer inspector surface.
- Immediate-mode TUI rendering means visual staging should be recreated from
  current state every frame. Therefore, greenhouse art direction should describe
  stable room/organism state and render rules, not persistent widget objects.
- Unicode glyph availability is broad but font-dependent. Greenhouse art should
  prefer tested glyph families and treat exotic characters as optional atlas
  candidates, not core rendering dependencies.

### Static Composition V0

Label: static-composition-v0

Description:

Define the first greenhouse as a readable terminal room composition before any
growth, interaction, or persistence exists.

Preferred composition name:

- `greenhouse_nursery_static_v0`

Candidate layout:

```text
┌──────────────────────── glass / roof hint ────────────────────────┐
│                                                                    │
│        lamp line / filtered light                                  │
│                                                                    │
│                         training frame                            │
│                         vine/cutting test                          │
│                                                                    │
│        propagation bench                                           │
│        [ seedling tray ]   [ cutting jar ]   [ empty tray ]         │
│                                                                    │
└──────────────────────── substrate / floor ─────────────────────────┘
```

Visual intent:

- quiet dark greenhouse corner
- more world-space room than interface panel
- enough structure to imply place
- enough emptiness to avoid texture noise
- plants remain the visual focus

Non-goals:

- no ornate glasshouse architecture
- no full-screen dashboard
- no inventory grid
- no mouse-driven editor feel
- no photorealistic botanical density

Open questions:

- Should the static greenhouse fit inside the current large viewport tier?
- Should it use the same status/footer area as main YAM?
- Should the first visual mock render in sparse sandbox or a dedicated mode?

### Fixture Vocabulary V0

Label: fixture-vocabulary-v0

Description:

Greenhouse fixtures are world-attached supporting structures. They are not
organisms, but they may host organisms or labels.

Candidate fixture kinds:

- `glass_frame`
- `roof_hint`
- `propagation_bench`
- `tray`
- `cutting_jar`
- `training_frame`
- `lamp_line`
- `substrate_strip`
- `accession_tag`

Fixture rules:

- fixtures may define anchors
- fixtures may be inspectable only if explicitly promoted
- fixtures should not own organism lifecycle state
- fixtures should not encode species behavior
- fixtures may own stable world rectangles or anchor points

Architecture notes:

- fixture vocabulary may belong near scene/world composition, not core organism
  data
- organism state may reference fixture anchors by stable ids
- render code may draw fixtures, but fixture definitions should remain simple

Open questions:

- Should trays and jars be fixtures, organism containers, or both?
- Should accession tags be fixtures or render-only label overlays?
- Should fixtures be represented in the same inspectable model as organisms?

### Label Semantics V0

Label: label-semantics-v0

Description:

Labels should make organisms traceable without overwhelming the scene.

Label classes:

- `hidden`: no visible label
- `compact`: small in-world tag near organism
- `selected`: expanded label for selected organism
- `inspector`: structured detail view outside direct room art
- `debug`: verbose dev-only fields

Preferred v0 behavior:

- default visual mode uses `hidden` or `compact`
- selected organism may show `selected`
- inspector may show structured fields
- dev mode may show `debug`

Compact label candidate:

```text
[gh-0001] Seedling Tray A
```

Selected label candidate:

```text
gh-0001 · seedling_tray · two_leaf_sprouts · experimental
```

Inspector fields candidate:

```text
Organism: gh-0001
Species: seedling_tray
Display: Seedling Tray A
Origin: greenhouse_nursery
Zone: propagation_bench:left_tray
Stage: two_leaf_sprouts
Lifecycle: propagating
Stability: experimental
Main scene eligible: false
```

Label rules:

- labels identify records; they do not create records
- labels must be derived from state
- labels should be optional in visual mode
- labels must not obscure primary plant silhouettes
- debug labels may be ugly but must be clearly dev-only

Open questions:

- Should compact labels use organism id only, or id plus display name?
- Should label placement be world-adjacent or screen-stabilized?
- Should selected label share the footer/status area or stay near the organism?

### Glyph Palette V0

Label: glyph-palette-v0

Description:

Define a conservative first glyph palette for greenhouse art direction.

Frame candidates:

```text
┌ ┐ └ ┘ ─ │ ╱ ╲
```

Bench/tray candidates:

```text
─ │ ┌ ┐ └ ┘ ═ ║ ░
```

Plant candidates:

```text
· . , ' ` : ; i l / \\ ╱ ╲ ⠂ ⠆ ⠒ ⠢
```

Root/moss/lichen candidates:

```text
· : ⁙ ⠂ ⠆ ⠒ ⣀ ⣄ ⣆
```

Selection marker candidates:

```text
◦ ◌ ◇ ▸ ▹
```

Rules:

- prefer ASCII, box drawing, and already-tested braille-like marks first
- avoid relying on rare symbols for core structure
- treat all non-ASCII plant marks as atlas-tested candidates
- avoid dense block glyphs unless explicitly representing soil/shadow mass
- never mix too many glyph families in one organism profile

Open questions:

- Which glyph families render best in current Kitty/Ghostty fonts?
- Should greenhouse atlas tests include screenshot fixtures?
- Should glyph palettes be grouped by species or by visual material?

### Color Role Vocabulary V0

Label: color-role-vocabulary-v0

Description:

Use semantic color roles rather than hard-coding colors in organism concepts.

Candidate roles:

- `greenhouse_frame_muted`
- `glass_edge_dim`
- `bench_wood_dark`
- `tray_shadow`
- `plant_stem_primary`
- `plant_leaf_primary`
- `plant_growth_highlight`
- `lichen_dry`
- `lichen_damp`
- `label_dim`
- `label_selected`
- `selection_marker`

Rules:

- core organism data should reference semantic roles, if anything
- actual color values belong to palette/render configuration
- debug overlays must remain visually distinguishable from greenhouse frame
- selected labels should be readable without becoming bright HUD elements

Open questions:

- Should color roles live in docs first, then Rust constants later?
- Should greenhouse roles reuse existing BTAS/YAM palette names directly?
- Should weather/environment influence color roles or only organism state?

### First Visual Review Surface

Label: first-visual-review-surface

Description:

Before implementing a full greenhouse mode, YAM may benefit from a visual review
surface for static greenhouse glyph and layout candidates.

Candidate review surface options:

- markdown fixture in docs
- plain-text art file under assets
- dev-only route in sparse sandbox
- screenshot/golden fixture after visual contract stabilizes

Preferred order:

1. plain-text sketch in docs or assets
2. dev-only sparse sandbox render
3. deterministic render fixture
4. screenshot/golden fixture only after the visual contract stabilizes

Non-goals:

- no screenshot/golden test before art direction stabilizes
- no large asset pipeline for first static mock
- no opaque ANSI blob as the source of truth

Open questions:

- Should static art source live under `assets/greenhouse/`?
- Should docs carry the canonical sketch before runtime assets exist?
- Should review surfaces show glyph names and color roles side-by-side?

### Risks Introduced By Pass 4

Label: pass-4-risks

Risk: greenhouse becomes visually boxy

Mitigation:

- use light frame hints, not heavy panel borders
- preserve empty air and dark negative space
- keep plants visually softer than fixtures

Risk: labels dominate the scene

Mitigation:

- default to hidden or compact labels
- expand labels only for selected organisms or inspector mode
- keep verbose labels dev-only

Risk: glyph palette expands too quickly

Mitigation:

- start with a small tested palette
- document candidate glyphs before runtime adoption
- group glyphs by material and organism role

Risk: fixtures become pseudo-organisms

Mitigation:

- keep fixture state separate from organism lifecycle state
- allow fixtures to provide anchors only
- inspect fixtures only if explicitly promoted

### Recommendation After Pass 4

Decision status: recommendation, not accepted contract

Recommended next promoted direction:

1. Define `greenhouse_nursery_static_v0` as the first visual staging target.
2. Keep the first room composition small: glass hint, bench, tray, cutting jar,
   optional training frame.
3. Use labels as derived visual/inspector output, not state creators.
4. Start with a conservative glyph palette and semantic color roles.
5. Prefer a plain-text visual review artifact before runtime rendering.

Rationale:

## Brainstorming Pass 5: Curation Loop, Promotion States, And Non-Game Progression

Status: brainstorming
Scope: curation flow, organism promotion/deprecation, pacing, and non-game progression
Implementation status: do not implement directly from this pass

### Conceptual Reference Notes

These notes summarize external conceptual patterns. They are not source
citations and should not be treated as authoritative project contracts or
implementation requirements.

- Living collections are documented, labeled, curated, and periodically revised.
  YAM can borrow this rhythm for greenhouse organisms without becoming a
  database application.
- Accessioning can happen when material enters a collection, even before final
  placement is decided. YAM can similarly assign stable ids to greenhouse
  organisms before they become visually or mechanically stable.
- Deaccessioning preserves records even when living material is removed. YAM can
  borrow the idea as `retired` or `deprecated` status without adding persistence
  or archival storage in v0.
- Collection utilization is often separate from collection existence. YAM can
  separate "organism exists in greenhouse" from "organism is eligible for main
  scene transfer".
- Ratatui redraws UI from current application state, so progression should be
  represented as state transitions and derived render output, not retained
  widget history.

### Curation Loop V0

Label: curation-loop-v0

Description:

Define greenhouse progression as a curation loop rather than a resource economy
or farming game.

Candidate loop:

```text
register → observe → stabilize → review → promote | retain | retire
```

Suggested meanings:

- `register`: create or load a greenhouse organism record/state entry
- `observe`: display or inspect the organism under controlled conditions
- `stabilize`: verify visual behavior, bounds, ids, and state transitions
- `review`: decide whether it remains experimental, becomes a candidate, or is
  retired
- `promote`: mark as eligible for a later main-scene transfer task
- `retain`: keep greenhouse-only for future tests or atmosphere
- `retire`: remove from active display while preserving conceptual record

Non-goals:

- no currency
- no unlock grind
- no inventory loop
- no watering/minigame economy
- no real-time obligation
- no failure punishment

Architecture notes:

- curation status should be symbolic and explicit
- promotion should not automatically mutate the main scene
- retirement should not require persistence in v0
- curation actions may be dev-only at first

Open questions:

- Should `promote` be a status field, a separate transfer note, or both?
- Should `retire` exist before persistent records exist?
- Should curation actions be user-triggered, test-driven, or manually edited in
  fixtures?

### Promotion State Vocabulary

Label: promotion-state-vocabulary

Description:

Separate greenhouse lifecycle from main-scene eligibility.

Candidate fields:

```text
lifecycle_status: registered | propagating | established | observed | retired
stability_class: experimental | visual_stable | behavior_stable | deprecated
transfer_status: greenhouse_only | candidate | approved | blocked
```

Suggested rules:

- lifecycle describes the organism's greenhouse stage
- stability describes confidence in visual/system behavior
- transfer status describes whether it may be considered outside greenhouse
- `approved` does not mean automatically rendered in the main scene
- `blocked` requires a reason

Candidate blocked reasons:

- `visual_noise`
- `hero_obstruction`
- `companion_obstruction`
- `unstable_bounds`
- `palette_conflict`
- `unclear_identity`
- `too_dashboard_like`

Example:

```text
organism_id: gh-0002
species_id: cutting_jar
lifecycle_status: established
stability_class: visual_stable
transfer_status: candidate
transfer_block_reason: none
```

Open questions:

- Should `greenhouse_only` be a transfer status instead of a lifecycle state?
- Should `behavior_stable` require tests or visual review only?
- Should blocked reasons be enum-like vocabulary or free text in docs?

### Main-Scene Transfer Gate V0

Label: main-scene-transfer-gate-v0

Description:

A greenhouse organism must pass explicit gates before it can be considered for
main-scene use.

Candidate gates:

- identity gate: species has a clear role and name
- visual gate: glyph/palette profile is stable
- bounds gate: organism stays within expected footprint
- readability gate: does not obstruct hero, clock, date, weather, or footer
- architecture gate: no simulation truth in render/UI layer
- test gate: deterministic fixture or state test exists, if applicable
- curation gate: transfer status is at least `candidate`

Transfer result options:

- `reject`: not suitable for main scene
- `defer`: needs more greenhouse testing
- `greenhouse_only`: valuable but not transferable
- `candidate`: may receive a bounded implementation task
- `approved`: may be used by a later main-scene contract

Hard rule:

- approval must not directly place the organism in the main scene; it only makes
  the organism eligible for a later scoped task.

Open questions:

- Should transfer review live in `docs/greenhouse.md`, `docs/audit.md`, or a
  dedicated future transfer note?
- Should the first transfer candidate be `seedling_tray`, `cutting_jar`, or
  `training_vine`?
- Should transfer gates be testable assertions or human review checklist items?

### Deprecation / Retirement Vocabulary

Label: deprecation-retirement-vocabulary

Description:

Some greenhouse ideas should be safely retired without deleting their learning
value.

Candidate statuses:

- `active`: visible or available in greenhouse
- `held`: retained but not currently visible
- `retired`: removed from active consideration
- `deprecated`: should not be used for new work
- `superseded`: replaced by a better organism/species/profile

Candidate reasons:

- `too_noisy`
- `too_dashboard_like`
- `poor_font_support`
- `unstable_layout`
- `duplicates_existing_species`
- `conflicts_with_main_scene`
- `not_worth_complexity`

Architecture notes:

- retirement vocabulary is planning-only until persistence exists
- deprecated species should remain documented if they shaped later decisions
- superseded entries should point to the replacement id/name in docs

Open questions:

- Should retired/deprecated entries live only in docs until persistence exists?
- Should deprecation reasons be required for any rejected transfer candidate?
- Should the greenhouse inspector show deprecated organisms in dev mode only?

### Non-Game Progression Rules

Label: non-game-progression-rules

Description:

The greenhouse may evolve over time, but it should not become a farming game,
pet simulator, or productivity dashboard.

Preferred progression style:

- slow curation
- visible refinement
- inspectable state
- ambient growth
- dev-friendly experimentation
- main-scene eligibility review

Avoided progression style:

- daily chores
- timers that punish absence
- resource collection
- unlock trees
- numerical optimization
- arbitrary rarity tiers
- achievement loops

Positive verbs:

- observe
- inspect
- stabilize
- prune
- retain
- promote
- retire
- document

Dangerous verbs:

- grind
- farm
- collect all
- upgrade
- monetize
- level up
- optimize yield

Architecture notes:

- progression should be optional and ambient
- no state change should demand daily user attention
- any timed growth should be deterministic and pausable in dev paths
- user-facing greenhouse should remain a visual/curatorial space first

### Pacing Model V0

Label: pacing-model-v0

Description:

If growth or progression exists, it should be calm, symbolic, and bounded.

Candidate pacing types:

- `static`: no growth; visual review only
- `tick_based`: growth changes after deterministic ticks
- `session_based`: growth changes within current runtime session only
- `fixture_based`: growth stage set by test fixture or mode configuration
- `weather_influenced`: future optional symbolic modifier

Preferred v0 pacing:

- `static` or `fixture_based`

Future pacing candidate:

- `tick_based`, deterministic and pausable

Avoid for now:

- wall-clock growth
- real-time daily growth
- persistent offline progress
- weather-driven mutation

Open questions:

- Should growth first appear only as fixture-defined stages?
- Should the first growth system mutate state, or should render select from
  staged fixtures?
- Should time/weather influence be visual-only before it affects state?

### Curation Inspector V0

Label: curation-inspector-v0

Description:

Extend the inspector idea to show curation status without turning it into a
control dashboard.

Candidate sections:

```text
Identity
  organism_id
  species_id
  display_name

Placement
  origin_space
  zone_anchor
  fixture_anchor

Growth
  growth_stage
  lifecycle_status
  stability_class

Transfer
  transfer_status
  transfer_block_reason
  main_scene_eligible

Notes
  summary_line
  review_note
```

Inspector rules:

- read-only in v0
- derived from state or fixture data
- no direct mutation controls at first
- dev mode may show blocked reasons and raw ids
- visual mode may show only identity and summary

Open questions:

- Should curation inspector be separate from organism journal?
- Should review notes be plain docs text or runtime fields?
- Should transfer gate results be visible in user-facing mode?

### First Curation Fixture Candidates

Label: first-curation-fixture-candidates

Description:

Candidate hand-authored fixture states for early tests and visual review.

Fixture A:

```text
organism_id: gh-0001
species_id: seedling_tray
lifecycle_status: propagating
stability_class: experimental
transfer_status: greenhouse_only
zone_anchor: propagation_bench:left_tray
growth_stage: two_leaf_sprouts
```

Fixture B:

```text
organism_id: gh-0002
species_id: cutting_jar
lifecycle_status: established
stability_class: visual_stable
transfer_status: candidate
zone_anchor: propagation_bench:center_slot
growth_stage: root_fan
```

Fixture C:

```text
organism_id: gh-0003
species_id: training_vine
lifecycle_status: observed
stability_class: experimental
transfer_status: blocked
transfer_block_reason: hero_obstruction
zone_anchor: training_frame:upper_left
growth_stage: overreaching_vine
```

Use cases:

- inspector layout testing
- transfer-gate vocabulary testing
- label rendering review
- deterministic state fixture tests

### Risks Introduced By Pass 5

Label: pass-5-risks

Risk: curation loop becomes game loop

Mitigation:

- keep progression descriptive and optional
- reject resource/currency/unlock mechanics
- preserve greenhouse as visual/curatorial space

Risk: promotion states imply automatic implementation

Mitigation:

- approval only means eligible for a later scoped task
- main-scene transfer requires separate contract or TODO promotion
- no direct mutation of main scene from greenhouse status

Risk: too many status fields too early

Mitigation:

- treat field groups as vocabulary sketches
- implement only after `docs/greenhouse.md` chooses minimum required state
- prefer fixture data before runtime mutation

Risk: deprecation vocabulary implies persistence

Mitigation:

- keep retirement/deprecation documentation-only at first
- do not add archive/storage layer for v0
- use docs/audit notes for rejected or superseded ideas

### Recommendation After Pass 5

Decision status: recommendation, not accepted contract

Recommended next promoted direction:

1. Frame greenhouse progression as curation, not gameplay.
2. Separate `lifecycle_status`, `stability_class`, and `transfer_status` in the
   planning vocabulary.
3. Treat `greenhouse_only`, `candidate`, `approved`, and `blocked` as transfer
   status concepts, not automatic runtime actions.
4. Use static or fixture-based pacing for the first greenhouse prototype.
5. Include curation/transfer fields in the future inspector shape, read-only at
   first.

Rationale:

This preserves the greenhouse as a calm experimental living collection rather
than a farming or collection game. It also creates a safe conceptual bridge
between greenhouse experimentation and later main-scene promotion without
coupling them directly.
