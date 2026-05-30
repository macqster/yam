# Greenhouse Roadmap

This document is the preliminary roadmap for the future greenhouse architecture
and environment. It is a planning contract, not an implementation batch.

The goal is to make greenhouse work deliberate before creative expansion begins.
Large new organisms, rooms, mechanics, and worlds should wait until the
architecture below has enough shape to hold them.

## ChatGPT Brainstorming Reference

Use this document as the local reference before asking ChatGPT or another
external brainstorming partner for greenhouse ideas.

The useful output is not a full feature spec. The useful output is a set of
ideas that can later be translated into YAM-owned data and contracts:

- room concepts
- species concepts
- fixture and support ideas
- environment presets
- small terminal-cell visual motifs
- inspection text and organism-journal flavor
- lifecycle events that can be represented by existing or planned state

The brainstorming prompt should preserve these constraints:

- YAM is a Rust/Ratatui terminal ecosystem, not a dashboard.
- The main scene is a curated live visualizer.
- The greenhouse is a separate simulation world or world-internal room model.
- Rooms, plants, fixtures, environment values, journals, and inspection surfaces
  are the natural design units.
- Ideas should be calm, inspectable, botanical, and compatible with terminal
  cells.
- Ideas should avoid hidden UI state, broad mechanics, render-owned simulation,
  or large new runtime systems before the architecture is ready.

Good prompts should ask for options in small catalogs. For example:

- five greenhouse room concepts with role, mood, fixtures, environment profile,
  and first organism fit
- five plant-family concepts with morphology, lifecycle hooks, inspection
  facts, and terminal-rendering constraints
- five fixture/support concepts with attachment behavior, visual texture, and
  room placement
- five event/journal concepts that make plant lifecycles feel alive without
  requiring broad mechanics

## Role

- Own the greenhouse expansion roadmap.
- Keep greenhouse planning grounded in current YAM infrastructure.
- Name implementation gates before runtime work starts.
- Prepare a clean brief for later external creative input.

Related contracts:

- [architecture.md](architecture.md) owns module boundaries and data flow.
- [scene-model.md](scene-model.md) owns world, screen, and scene behavior.
- [rendering.md](rendering.md) owns visible layer and UI/render contracts.
- [vines.md](vines.md) owns vine-specific readiness and future phases.
- [TODO.md](../TODO.md) owns execution items derived from this roadmap.

## Current Baseline

The current repo already has the first seams that greenhouse work should use:

- `WorldKind::profile()` owns selectable worlds, loading labels, composition,
  grid, camera defaults, guide plans, population plans, and capabilities.
- `Boot` is explicitly not selectable; `MainScene` and `Sandbox` are the current
  selectable worlds.
- `Sandbox` is the first sparse alternate space and should be treated as a dry
  trial surface for future room and guide work.
- `core::organism` owns the first shared identity, species registry, lifecycle,
  stats, and per-instance journal vocabulary.
- `FloraState` is still vine-shaped in storage, but exposes family-count and
  organism-identity adapters.
- `core::spatial` owns the active projection, anchor, guide-index, and resolver
  vocabulary used by the central render paths.
- Scene and render layers are read-only visualizers of world/flora/spatial
  state.

## North Star

YAM should grow into two connected simulation spaces:

- the main scene: a curated live visualizer with hero, companions, scaffold,
  vines, and selected flora forms
- the greenhouse: a separate simulation world with rooms, growing sites,
  environment profiles, multiple plant families, lifecycle tuning, and
  inspectable organism histories

The greenhouse should feel like a place, not an admin dashboard. It can have
inspection and dev tooling, but the primary architecture is still world space,
rooms, organisms, and environment state.

## Hard Rules

- Greenhouse is a separate world or a room model inside a world, not panel chrome
  over the main scene.
- Room selection belongs to greenhouse/world state, not to side-by-side UI tabs.
- World selection extends `WorldKind::SELECTABLE` and `WorldKind::profile()`.
- Organism runtime state stays in world/flora state; species defaults stay in a
  registry; journals stay per organism.
- Environment values are simulation inputs first, not styling flags.
- Render layers visualize derived geometry and state; they do not own growth,
  room selection, inspection truth, or environment mutation.
- Creative content must arrive through room, species, organism, environment, or
  fixture data rather than bespoke render shortcuts.

## Domain Model

The first greenhouse model should stay small and inspectable:

- `Greenhouse world`: the selectable world profile plus greenhouse-specific
  state.
- `Room`: a named internal space with a role, bounds, environment profile, and
  active planting sites.
- `Fixture`: stable world-attached support geometry such as benches, pots,
  bowls, trellises, walls, shelves, or lab surfaces.
- `Environment`: coarse room inputs such as light, moisture, temperature,
  airflow, medium, season, and time scale.
- `Planting site`: an attachment or bounded area where one or more organisms can
  root, climb, sprawl, or be inspected.
- `Organism population`: independent plant instances keyed by organism identity
  and species id.
- `Lifecycle`: per-organism and per-organ state derived from species rules,
  environment, growth tips, and elapsed ticks.
- `Inspection`: a mode or popup that reads registry, journal, state, and derived
  geometry without becoming the owner of those facts.

## Roadmap

### Phase 0: Planning Gate

Goal: make the expansion understandable before adding runtime behavior.

Work:

- keep this roadmap current as the greenhouse owner document
- keep the active backlog execution-shaped
- keep `docs/audit.md` risk-focused
- keep `known_issues.md` empty unless a concrete active issue appears

Acceptance gate:

- docs checks pass
- no runtime greenhouse behavior is added
- future implementation slices can point to this roadmap instead of scattered
  chat memory

### Phase 1: World Contract Slice

Goal: prove the greenhouse can be named by the world profile contract before it
has visible content.

Work:

- decide the first world vocabulary, probably `WorldKind::Greenhouse` with
  internal rooms rather than separate top-level worlds for every room
- define candidate profile fields for room plan, environment plan, population
  plan, and inspection capabilities
- add tests that the new profile is selectable, has no main-scene companions,
  owns its loading label, and declares only capabilities it can actually support

Acceptance gate:

- the world contract can describe greenhouse intent without UI-local toggles
- render layers can branch by profile/composition/capability
- no greenhouse world variant is added before room and environment ownership are
  named

### Phase 2: Room And Environment Storage

Goal: add inert greenhouse state with no visible output.

Work:

- introduce a small room/environment model, likely in `core::greenhouse` once
  code starts
- define `GreenhouseState`, `GreenhouseRoom`, `EnvironmentProfile`, and
  `PlantingSite` or equivalent names
- keep room selection inside greenhouse/world state
- start with coarse per-room environment values before any cell-level simulation
- add deterministic fixtures only as data, not as a new render shortcut

Acceptance gate:

- greenhouse state can be constructed, inspected, and tested without rendering
- environment values are typed enough for future growth dispatch
- no room data is stored in UI state

### Phase 3: Flora Storage Generalization

Goal: make room for multiple plant families before a second family lands.

Recommended direction:

- move from a vine-only top-level store toward a small enum-backed organism
  store when the second family is ready
- preserve typed accessors for vine tests and existing render code during the
  transition
- keep organism identity, species id, lifecycle state, stats, and journal id on
  every instance
- keep the species registry read-heavy and reusable; do not store per-instance
  history in it

Acceptance gate:

- family counts and organism identities still work across all stored families
- growth and render code can query the right family without downcasting through
  UI or render state
- adding the first greenhouse plant does not create another ad hoc vector beside
  the current vine storage without an explicit store decision

### Phase 4: Growth Dispatch And Lifecycle

Goal: make organism updates species-aware and environment-aware.

Work:

- keep `systems/growth` as the mutation owner for flora tick behavior
- dispatch growth by organism family and species profile
- feed growth from room/environment state when the organism lives in the
  greenhouse
- record lifecycle and growth events into the organism journal
- keep deterministic seeds and explicit tick pacing

Acceptance gate:

- same input state and tick sequence produce the same growth state
- render layers still receive derived geometry only
- tests cover lifecycle transitions, journal events, and no-op behavior for
  unsupported species or missing environments

### Phase 5: First Greenhouse Room Render

Goal: visualize one room after storage and simulation contracts exist.

Work:

- render a sparse greenhouse room through the existing scene pipeline
- start with fixtures, planting sites, guide linework, and organism silhouettes
- keep room rendering world-attached and camera-aware
- keep HUD/footer and modal vocabulary unchanged
- use sandbox to test dense linework or fixture shapes before promotion

Acceptance gate:

- room content renders only when the active world profile asks for it
- main-scene hero and companion composition remains untouched
- footer, debug, and modal invariance tests stay green under greenhouse camera
  movement

### Phase 6: Inspection And Authoring

Goal: expose greenhouse state without turning UI into simulation ownership.

Recommended order:

- start with per-organism inspect popups for quick reading
- add room/environment debug readouts only after state exists
- reserve a dedicated registry or journal mode for later, when organism count
  makes popups too small
- keep mutation behind explicit dev tooling

Acceptance gate:

- inspection reads registry, organism state, journal, room, and environment facts
- routine inspection works without crowding the footer
- dev edits are explicit and persist only through the existing save/dirty-state
  discipline

### Phase 7: Main Scene Bridge

Goal: let greenhouse work enrich the main scene only after the system is proven.

Work:

- promote selected flora concepts from greenhouse prototypes into the main scene
- keep the scaffold stable and nearly fixed
- add richer vines, monstera-like leaves, or other plant families only through
  shared organism, flora, spatial, and render contracts
- keep main-scene enrichment curated and comparatively static

Acceptance gate:

- main scene remains a composed visualizer, not a management surface
- new visible organisms have world/HUD/overlay ownership assigned before render
- no greenhouse implementation detail leaks into main-scene-only shortcuts

### Phase 8: Creative Expansion

Goal: ask for creative input after constraints are clear.

Work:

- gather room mood, species concept, fixture, naming, and ritual ideas
- translate usable ideas into species profiles, room profiles, environment
  presets, visual briefs, and inspection text
- reject ideas that require render-owned simulation, hidden UI state, or broad
  mechanics before the storage and growth contracts exist

Acceptance gate:

- creative additions become data or bounded implementation slices
- new ideas are traced back to this roadmap, a contract doc, or a backlog item
- the greenhouse grows as an environment with rules, not as a pile of one-off
  decorations

## First Implementation Sequence

When implementation starts, the rational order is:

1. Add or refine contract docs and tests for greenhouse world profile fields.
2. Add inert room/environment data.
3. Decide and implement multi-family flora storage.
4. Add one inert non-vine organism profile and instance shape.
5. Add deterministic growth dispatch for that organism with environment input.
6. Render the first sparse room.
7. Add per-organism inspection.
8. Then invite broader creative expansion.

Stop if a slice requires render-owned growth, UI-owned room state, a second
projection system, raster masks without a promoted mask contract, or main-scene
visual changes before the greenhouse contracts are ready.

## Open Decisions

- Should the first visible greenhouse be a single `Greenhouse` world with
  internal rooms, or should `Lab` become a separate world later? Current bias:
  one greenhouse world first, lab as a later room or mode if it proves distinct.
- Should greenhouse environments start per room or per planting site? Current
  bias: per room first, planting-site modifiers later.
- Should inspection begin as popups or a dedicated registry mode? Current bias:
  popups first, registry mode later if population size justifies it.
- Should species definitions remain static Rust fixtures or move to structured
  files? Current bias: static fixtures for the first two families, structured
  data once the schema stabilizes.
- Should Lua or another script layer enter species authoring? Current bias:
  no runtime scripting until Rust-owned species and growth contracts are proven.

## Creative Brief For Later

Use this brief when asking for external creative input. Paste the prompt as-is
or adapt it for a narrower room, species, fixture, or lifecycle pass.

```text
YAM is a Rust/Ratatui terminal ecosystem with a dark-deco, compact, readable
scene style. It has a main visualizer scene and is preparing a separate
greenhouse simulation world. The greenhouse should feel like a real place with
rooms, benches, pots, bowls, supports, environment profiles, multiple plant
families, lifecycle history, and quiet inspection tools. It must not become a
generic dashboard or panel UI.

Please propose creative greenhouse directions that can be expressed as room
profiles, species profiles, fixture ideas, environment presets, inspect text,
or small visual motifs. Favor ideas that work in terminal cells, support
inspectable plant lifecycles, and respect a calm botanical/lab atmosphere.
Avoid ideas that require hidden UI state, freeform dashboard panels, render-owned
simulation, or large mechanics before the architecture is ready.
```

Useful follow-up prompts:

- Turn the strongest room ideas into compact `RoomProfile` sketches with role,
  environment, fixtures, planting sites, and inspection notes.
- Turn the strongest plant ideas into compact `SpeciesProfile` sketches with
  habit, axes, organs, lifecycle hooks, journal events, and terminal-cell visual
  constraints.
- Turn the strongest fixture ideas into world-attached support sketches with
  placement rules, attachment behavior, and visual texture.
- Sort all ideas into `now`, `after storage`, `after growth dispatch`, and
  `later atmosphere` buckets.
