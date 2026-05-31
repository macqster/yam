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

## Brainstorming Session 1 Ingest

The first external brainstorming source has been ingested as candidate material,
not as an implementation contract. It should inform future prompts, planning
docs, and small TODO promotions, but no runtime behavior should be implemented
from the source note directly.

Stable takeaways:

- The first greenhouse identity should bias toward a nursery / propagation room
  wrapped in botanical-lab discipline, with a later path toward a fuller
  conservatory atmosphere.
- The first prototype name can stay `greenhouse_nursery` while planning, with
  `greenhouse_nursery_static_v0` as the first visual staging target.
- The first visible room should be sparse: a glass or roof hint, a propagation
  bench, one seedling tray, one cutting jar, optional training frame, and enough
  empty air that the room reads as a place rather than a panel.
- The safest first organism concepts are `seedling_tray` and `cutting_jar`;
  `weather_lichen` and `training_vine` are later candidates after environment
  and transfer rules are better defined.
- Greenhouse zones should be world-space concepts such as `propagation_bench`,
  `glass_frame`, `inspection_marker`, `training_frame`, `substrate_bed`,
  `specimen_shelf`, and `lamp_zone`.
- Labels should be derived presentation. They can be hidden, compact, selected,
  inspector-level, or dev/debug-level, but they must not create records or own
  organism truth.
- The first environment model should remain symbolic: filtered or artificial
  light, balanced or damp humidity, mild temperature, tray/water/bark/scaffold
  substrate, and no outside-weather coupling by default.
- Greenhouse progression should be curation, not gameplay: register, observe,
  stabilize, review, promote, retain, retire, and document are useful verbs;
  chores, currency, unlocks, daily obligations, and yield optimization are not.
- Transfer into the main scene needs explicit gates for identity, visual
  stability, bounds, readability, architecture, tests, and curation status.

Candidate vocabulary to preserve:

- `lifecycle_status`: `registered`, `propagating`, `established`, `observed`,
  `retired`
- `stability_class`: `experimental`, `visual_stable`, `behavior_stable`,
  `deprecated`
- `transfer_status`: `greenhouse_only`, `candidate`, `approved`, `blocked`
- `transfer_block_reason`: `visual_noise`, `hero_obstruction`,
  `companion_obstruction`, `unstable_bounds`, `palette_conflict`,
  `unclear_identity`, `too_dashboard_like`

Candidate fixture states for future tests and visual review:

```text
organism_id: gh-0001
species_id: seedling_tray
lifecycle_status: propagating
stability_class: experimental
transfer_status: greenhouse_only
zone_anchor: propagation_bench:left_tray
growth_stage: two_leaf_sprouts
```

```text
organism_id: gh-0002
species_id: cutting_jar
lifecycle_status: established
stability_class: visual_stable
transfer_status: candidate
zone_anchor: propagation_bench:center_slot
growth_stage: root_fan
```

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

Candidate room sketch:

```text
[glass / roof hint]
[lamp line, optional]
[training frame or specimen shelf]
[propagation bench with tray + cutting jar]
[floor/substrate strip]
```

Candidate interaction grammar, for later promotion only:

- `Tab` / `Shift+Tab`: move between inspectable organisms
- `Enter`: open selected organism inspector
- `Esc`: close inspector or return to greenhouse
- `l`: toggle compact labels
- `d`: toggle dev overlay
- `Space`: pause or resume growth ticks, if growth exists

None of these bindings are accepted runtime contracts yet. They are a compact
vocabulary for later UI planning.

## Operation Plan

The greenhouse expansion should proceed as one staged operation. The root
[TODO.md](../TODO.md) should only carry active execution pointers; the detailed
greenhouse stage order lives here.

Status tags for future work:

- `docs` - docs-only promotion, alignment, or cleanup
- `inspect` - research or decision work before implementation
- `model` - pure data vocabulary or state shape
- `test` - deterministic test or fixture work
- `render` - visual review or runtime visualization
- `ui` - inspection, selection, labels, or dev surface work
- `gate` - stop/check condition before moving to the next stage
- `defer` - intentionally later work

### Phase 0: Intake And Framing

Goal: keep the idea space documented and bounded.

Tasks:

- keep this roadmap as the strategic and operational greenhouse source
- keep root `TODO.md` limited to active greenhouse execution pointers
- ingest future brainstorming as distilled candidate material, not transcripts
- keep `docs/audit.md` focused on risks and open gates

Gate:

- roadmap, root TODO, audit, and log agree on greenhouse posture
- no runtime greenhouse behavior is added from brainstorming alone

### Phase 1: Greenhouse Contract Decision

Goal: decide the first contract shape before adding a world variant or visible
room.

Tasks:

- decide whether the first implementation contract stays here or graduates into
  a future `docs/greenhouse.md`
- decide whether the first runtime boundary is a hidden/dev-only room, sparse
  sandbox route, or named `WorldKind::Greenhouse`
- define what `Greenhouse` owns beyond `WorldKind::profile()`: room,
  environment, fixture, population, inspection, and curation capability
- plan profile tests before adding a world variant

Gate:

- no selectable greenhouse world exists before room and environment ownership
  are named
- no UI-local world toggle is needed

### Phase 2: Room, Zone, And Fixture Vocabulary

Goal: define greenhouse as world space, not panel chrome.

Tasks:

- define candidate room vocabulary such as `GreenhouseState`,
  `GreenhouseRoom`, `GreenhouseZone`, `EnvironmentProfile`, `Fixture`, and
  `PlantingSite`
- preserve first zones: `propagation_bench`, `glass_frame`,
  `inspection_marker`, `training_frame`, `substrate_bed`, `specimen_shelf`,
  and `lamp_zone`
- decide whether fixtures own rectangles, anchors, or both
- keep fixtures separate from organism lifecycle state

Gate:

- room/zone/fixture ownership is pure data
- room selection stays inside greenhouse/world state, not UI tabs

### Phase 3: Symbolic Environment Model

Goal: keep environment state small and useful for later growth dispatch.

Tasks:

- define symbolic light, humidity, temperature, substrate, and outside-weather
  influence fields
- prefer room-level environment first; planting-site modifiers can come later
- keep outside weather disabled by default
- avoid numeric simulation until a species rule needs it

Gate:

- environment can be read by future systems without reaching into UI or weather
  widget rendering

### Phase 4: Flora And Species Prep

Goal: prepare organism storage before adding a second plant family.

Tasks:

- decide how `FloraState` grows beyond `vines`: enum-backed family store,
  organism registry, or another small explicit structure
- keep organism identity, species id, journal id, lifecycle state, stats, and
  family vocabulary on every organism
- decide whether first greenhouse species profiles are Rust fixtures or
  structured data
- keep `seedling_tray` and `cutting_jar` as first candidate profiles

Gate:

- no ad hoc top-level organism vector is added beside `FloraState::vines`
  without a storage decision

### Phase 5: Static Fixtures And Visual Review

Goal: review greenhouse composition before runtime mechanics.

Tasks:

- decide the first visual review artifact: docs sketch, plain-text asset,
  dev-only sandbox route, or deterministic render fixture
- keep `greenhouse_nursery_static_v0` sparse: glass/roof hint, optional lamp
  line, training frame or specimen shelf, propagation bench with tray and
  cutting jar, and substrate strip
- keep glyph palettes small and material-specific
- treat color roles semantically, not as core organism state

Gate:

- visual review is not the canonical simulation source
- no opaque ANSI blob becomes canonical source material

### Phase 6: Inert Greenhouse State

Goal: add state that can be constructed and tested without visible behavior.

Tasks:

- add greenhouse state only after phases 1-4 identify owners
- include room, environment, zones, fixtures, organisms, inspectable refs,
  label visibility, and growth pause only if each field has an owner
- prove inspectable refs resolve to organisms, clusters, or zones
- keep presentation selection in UI state, not room data

Gate:

- inert greenhouse state is deterministic
- no render path changes are required

### Phase 7: Growth Dispatch Probe

Goal: introduce the smallest deterministic greenhouse growth behavior.

Tasks:

- define lifecycle, stability, and transfer status separately before mutation
  code uses them
- start with fixture-based or static stages before tick-based mutation
- keep growth deterministic and pausable when it begins
- record growth and lifecycle events in organism journals

Gate:

- growth systems mutate world/flora state only
- unsupported species or missing environments produce no-op behavior

### Phase 8: First Runtime Room Render

Goal: render the first sparse greenhouse room after data ownership is proven.

Tasks:

- draw room fixtures through the existing scene pipeline
- keep room content world-attached and camera-aware
- branch by world profile/composition/capability, not UI-local toggles
- preserve main-scene hero, companion, footer, modal, and debug invariance

Gate:

- greenhouse rendering is visible only when the active world profile asks for it
- no greenhouse renderer owns growth, room selection, or inspectable truth

### Phase 9: Inspection, Labels, And Dev Surface

Goal: expose greenhouse state without creating a dashboard.

Tasks:

- start with read-only per-organism inspection
- keep labels derived from state: hidden, compact, selected, inspector, or debug
- keep selection state presentation-oriented and separate from organism truth
- decide whether compact labels are world-adjacent or screen-stabilized

Gate:

- label toggles do not mutate organism state
- mutation controls wait until read-only inspection is stable

### Phase 10: Curation And Transfer Gates

Goal: bridge greenhouse experiments toward the main scene without coupling them.

Tasks:

- preserve separate lifecycle, stability, and transfer status vocabulary if
  implemented
- preserve blocked reasons such as `visual_noise`, `hero_obstruction`,
  `companion_obstruction`, `unstable_bounds`, `palette_conflict`,
  `unclear_identity`, and `too_dashboard_like`
- treat `approved` as eligible for later work, not automatic placement
- keep retirement/deprecation docs-only until persistence exists

Gate:

- greenhouse status cannot mutate the main scene directly
- transfer review is explicit and inspectable

### Phase 11: Creative Expansion Loop

Goal: invite more creative input once the operational gates are legible.

Tasks:

- ask for small catalogs of room, species, fixture, environment, journal, and
  lifecycle ideas
- sort ideas into `now`, `after storage`, `after growth dispatch`, and
  `later atmosphere`
- promote only bounded ideas into root TODO or an owning contract
- reject or park ideas that require hidden UI state, render-owned simulation,
  broad mechanics, or main-scene mutation

Gate:

- creative output becomes data, contract text, fixtures, or small tasks
- the greenhouse remains a place with rules, not a pile of decorations

Immediate next tasks:

- keep the first brainstorming ingest candidate-only
- decide whether `docs/greenhouse.md` is needed before inert state work begins
- decide the first visual review artifact for `greenhouse_nursery_static_v0`
- decide the first flora storage generalization direction
- identify first pure data tests for room, zone, fixture, species, and
  inspectable refs

Stop conditions:

- render-owned simulation truth
- UI-owned room, environment, or organism state
- a second projection system
- a selectable greenhouse world without room/environment ownership
- another plant-family store beside `FloraState::vines` without a storage
  decision
- screenshot/golden art locks before the visual vocabulary stabilizes
- main-scene visual changes before transfer gates exist
- gameplay loops such as chores, currency, unlock grinding, daily obligations,
  or yield optimization

Handoff checklist:

- root `TODO.md` updated only for active execution pointers
- this roadmap updated for strategy, accepted takeaways, operation stages, and
  creative-input reference
- `docs/audit.md` updated only for current risks or open gates
- `docs/LOG.md` appended with a historical note
- docs checks passing, and full verification when the batch touches code or
  cross-contract behavior

## Open Decisions

- Should the first visible greenhouse be a single `Greenhouse` world with
  internal rooms, or should `Lab` become a separate world later? Current bias:
  one greenhouse world first, lab as a later room or mode if it proves distinct.
- Should the first greenhouse identity be lab, conservatory, or nursery? Current
  bias: nursery / propagation room first, with botanical-lab discipline and
  later conservatory atmosphere.
- Should greenhouse environments start per room or per planting site? Current
  bias: per room first, planting-site modifiers later.
- Should inspection begin as popups or a dedicated registry mode? Current bias:
  popups first, registry mode later if population size justifies it.
- Should species definitions remain static Rust fixtures or move to structured
  files? Current bias: static fixtures for the first two families, structured
  data once the schema stabilizes.
- Should greenhouse progression have game-like mechanics? Current bias: no;
  use curation, inspection, review, and promotion vocabulary instead.
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

- Use the Brainstorming Session 1 ingest as source context, but keep all
  suggestions non-binding until promoted into roadmap, TODO, or a contract doc.
- Turn the strongest room ideas into compact `RoomProfile` sketches with role,
  environment, fixtures, planting sites, and inspection notes.
- Turn the strongest plant ideas into compact `SpeciesProfile` sketches with
  habit, axes, organs, lifecycle hooks, journal events, and terminal-cell visual
  constraints.
- Turn the strongest fixture ideas into world-attached support sketches with
  placement rules, attachment behavior, and visual texture.
- Sort all ideas into `now`, `after storage`, `after growth dispatch`, and
  `later atmosphere` buckets.
