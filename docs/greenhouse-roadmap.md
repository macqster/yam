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
- First-pass ideas should favor functional space: room, access paths, zones,
  fixtures, planting sites, environment profile, and inspection affordances
  before plant lifecycle systems.
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

## 0.4 Readiness Snapshot

Last checked: 2026-07-21 (baseline verification, spatial ownership, and
dependency inventory rows below re-verified directly with a working Rust
toolchain; other rows carried forward from the 2026-05-31 pass).

Current status:

- baseline verification is green: `bash scripts/verify.sh` passed with docs
  checks, ownership checks, clippy, cargo check, and `255` tests
- `known_issues.md` has no active tracked issues
- dependency inventory is clean: `cargo tree -d` reports nothing to print
- world profile/selectability tests cover `Boot`, `MainScene`, and `Sandbox`,
  including `Boot` staying non-selectable
- `scripts/check.sh` guards `core`/`systems` module-boundary ownership; the
  former `crate::scene::coords` isolation guard was removed 2026-07-21 when
  the compatibility module it guarded was retired (zero remaining call sites
  outside its own tests), so `core::spatial` is now the sole spatial resolver
  with no compatibility layer left to isolate
- `FloraState` still stores only vines, but family-count and organism-identity
  adapters are tested and remain the current bridge toward a future enum-backed
  family store decision
- Chafa/cache fallback tests cover missing GIF decode, unavailable Chafa,
  placeholder non-cacheability, and cache freshness

No new guard or test was added during this readiness pass because the inspected
0.4 gates already have matching coverage or an explicit open decision recorded
below. The next implementation-prep work should resolve contract shape, not
start visible greenhouse behavior.

## 0.4 Gate Checklist

| Gate | Status | Evidence | Remaining Work |
| --- | --- | --- | --- |
| Docs aligned | Ready | `docs/greenhouse-roadmap.md` owns greenhouse strategy and operation; `TODO.md` carries execution pointers only | Keep future updates in owning docs and log each batch |
| Verification green | Ready | `bash scripts/verify.sh` passed with docs checks, guard checks, clippy, cargo check, and `255` tests (re-verified 2026-07-21 with a working toolchain) | Re-run before each implementation batch |
| Spatial ownership stable | Ready | `scene::coords` compatibility module retired 2026-07-21 (zero remaining external call sites); all active render paths consume `core::spatial` directly with no intermediate compatibility layer | Masks and organism guidance can layer on `core::spatial` when needed; no further coords-migration work remains |
| Flora storage decision | Decision-biased | `FloraState` adapters are tested; enum-backed family store is the current first-pass bias | Lock the enum-backed shape before a second plant family lands |
| Greenhouse/world contract | Contract-ready, not runtime-ready | Functional-space contract lives below; `WorldKind::profile()` is the future world seam | Add pure data tests before any visible `WorldKind::Greenhouse` variant |
| Hero/render fallback hardened | Prep-ready | Chafa fallback/cache tests cover missing GIF, unavailable Chafa, placeholder cache rejection, and cache freshness | Keep offline `CellGrid` / editor work deferred |

## Locked First-Pass Decisions

- Keep the greenhouse contract in this roadmap for now; create
  `docs/greenhouse.md` only after inert state exists or this roadmap becomes too
  crowded to remain readable.
- The first code-bearing greenhouse slice has now landed as a pure
  `core::greenhouse` data module with construction and invariant tests. It does
  not render, tick growth, mutate flora, or add a selectable world.
- The first visible greenhouse should eventually be a named
  `WorldKind::Greenhouse`; the sandbox may support visual review, but it should
  not become the hidden owner of greenhouse state.
- The first room identity stays `greenhouse_nursery`: a nursery / propagation
  room with botanical-lab discipline and later conservatory atmosphere.
- The first visual artifact stays a docs/plain-text room sketch until pure data
  ownership is tested; no screenshot or golden art lock should lead the design.
- The first environment model is symbolic and room-level: light, humidity,
  temperature, water, airflow, substrate, and outside-weather influence, with
  outside weather disabled by default.
- First inspection is read-only and closer-look oriented. Mutation controls,
  care loops, and edit surfaces wait until read-only inspection is stable.
- First room capacity stays tiny: one to three planting sites per room.
- First species/profile data, when plant work is promoted later, starts as
  static Rust fixtures. Structured files wait until the schema stabilizes.
- First flora-storage generalization remains biased toward an enum-backed family
  store. A generic registry or broader store must beat that bias with a concrete
  simplification.

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
- The first greenhouse pass is functional-space-first: room, access paths,
  zones, fixtures, planting sites, environment profile, and read-only inspection
  affordances must be stable before plant systems attach to them.
- Do not add a new plant family, lifecycle loop, species catalog, persistence
  model, or visible greenhouse runtime before the 0.4 pre-expansion gates are
  green.
- Do not weaken `scripts/verify.sh`, `scripts/check.sh`, projection isolation,
  or existing architecture guardrails to make greenhouse work easier.

## Domain Model

The first greenhouse model should stay small and inspectable:

- `Greenhouse world`: the selectable world profile plus greenhouse-specific
  state.
- `Room`: a named internal space with a role, bounds, environment profile, and
  active planting sites.
- `Access path`: readable movement/attention space inside a room; it gives
  fixtures and planting sites breathing room before interaction exists.
- `Zone`: a named functional sub-area such as propagation bench, glass frame,
  mist bench, warm shelf, dry rack, utility alcove, specimen shelf, lamp zone,
  or inspection marker.
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

## Functional-Space Contract

This contract is the first implementation target once greenhouse code is
explicitly authorized. It is a pure data contract for a functional room
container. It is not a render contract, species contract, or growth contract.

Implementation placement:

- `src/core/greenhouse.rs` now owns the first pure data slice
- it is exposed through `core/mod.rs` with invariant tests in place
- attach it to `WorldState` only after construction and invariant tests pass
- keep render, UI, systems, weather, and terminal modules out of the first data
  slice

Identifier policy:

- use stable string-like ids for room, access path, zone, fixture, planting
  site, environment profile, and inspection records
- use human-readable planning ids such as `greenhouse_nursery`,
  `propagation_bench`, `left_tray`, and `inspection_marker`
- do not reuse `OrganismId` for rooms, fixtures, or planting sites

Minimum data owners:

| Owner | Owns | Must Not Own |
| --- | --- | --- |
| `GreenhouseState` | rooms, active room id, greenhouse-level capability flags | UI tabs, render layers, organism lifecycle, persistence policy |
| `GreenhouseRoom` | room id, display name, role, bounds, access paths, zones, fixtures, planting sites, environment profile, inspection records | growth ticks, species registry, render ordering |
| `AccessPath` | readable attention/movement space inside a room | pathfinding, player movement, layout panels |
| `GreenhouseZone` | named functional sub-area and its anchor/bounds | organism lifecycle state |
| `Fixture` | stable world-attached support geometry and material/inspection hints | growth mutation, species identity |
| `PlantingSite` | bounded or anchored occupancy site and capacity | plant state, care loop, yield logic |
| `EnvironmentProfile` | symbolic room-level light, humidity, temperature, water, airflow, substrate, outside-weather influence | weather widget state, numeric simulation |
| `InspectionRef` | read-only target reference, label, and short inspection text | mutation controls, journal ownership |

First nursery fixture vocabulary:

- `glass_frame`
- `propagation_bench`
- `left_tray`
- `cutting_jar_slot`
- `inspection_marker`
- `training_frame`
- `specimen_shelf`
- `substrate_strip`
- `lamp_zone`

First invariant tests, when code starts:

- a nursery state can be constructed with one active room
- room ids are stable and unique inside the greenhouse state
- zones, fixtures, planting sites, and inspection records resolve to the active
  room
- a room has one to three planting sites
- environment profile values are symbolic and outside-weather influence is off
  by default
- inspection records are read-only and target rooms, zones, fixtures, planting
  sites, or later organisms without owning those targets
- no greenhouse type imports render, UI, terminal, weather-provider, or systems
  modules

Current landed shape:

- `GreenhouseState::nursery()` constructs one inert `greenhouse_nursery` room
- stable string-like ids exist for room, access path, zone, fixture, planting
  site, environment profile, and inspection reference
- the nursery carries the first bounded functional-space vocabulary for access
  paths, zones, fixtures, planting sites, symbolic environment, and read-only
  inspection references
- greenhouse state is still not attached to `WorldState` and still has no
  visible world/render surface

## ChatGPT Preflight Check Ingest

The ChatGPT preflight note is accepted as a readiness checkpoint for the 0.4
transition. It is not a feature request and does not authorize new visible
greenhouse behavior.

Pre-expansion gates:

- docs aligned
- verification green
- spatial ownership stable
- flora storage decision made
- greenhouse/world contract decided
- hero/render pipeline failure modes hardened

Accepted constraints:

- keep the repo in pre-feature readiness mode until the gates above are green
- fix verification before feature expansion if `bash scripts/verify.sh` fails
- keep `TODO.md` execution-oriented, `docs/audit.md` risk-focused,
  `docs/LOG.md` historical, and `known_issues.md` limited to concrete active
  issues
- make small, reviewable patches and preserve existing checks

Architecture decision bias from the preflight:

- `core::spatial` is the canonical resolver; `scene::coords` was retired 2026-07-21 rather than kept as a compatibility shim once it had no remaining external callers
- before a new plant family lands, prefer an enum-backed family store as the
  first `FloraState` generalization unless inspection proves another shape is
  simpler
- the first greenhouse contract should bias toward a later
  `WorldKind::Greenhouse`, a `greenhouse_nursery` / propagation-room identity,
  per-room symbolic environment, read-only popup inspection, and static Rust
  fixtures for the first profiles
- missing GIF, temp-write failure, Chafa failure, and cache miss paths must keep
  known fallbacks before hero/render work is treated as stable for expansion

Suggested tiny enabling tests, when implementation begins:

- world profile and selectability invariants
- `Boot` remains non-selectable
- main scene / sandbox profile invariants
- projection isolation guardrails
- `FloraState` family identity and count adapters
- `SpeciesRegistry` and `OrganismJournal` basics
- greenhouse room, zone, fixture, environment, planting-site, and inspection
  fixture tests once those data shapes exist

## ChatGPT Brainstorming 1 Ingest

The ChatGPT brainstorming note is accepted as candidate planning material. Its
strongest contribution is an ordering rule: functional greenhouse space comes
before plant systems.

Stable takeaways:

- First greenhouse pass order:
  `room -> zones -> fixtures -> planting sites -> environment profile -> inspection surface`.
- Plant-system work stays deferred until the functional room contract is stable:
  organism families, species registry expansion, growth state, lifecycle,
  persistence, and interactions are later work.
- Current vision sentence: the greenhouse is a nursery / propagation room, a
  small inspectable symbolic environment where plant life can later be
  understood through fixtures, planting sites, room conditions, and read-only
  inspection before any game-like progression exists.
- `cbonsai` remains valid as mood, tempo, terminal-organism feel, and glyph
  economy; it is not a greenhouse architecture reference and does not mandate
  plant lifecycle implementation.
- External web/reference exploration should be bounded to three lanes:
  functional greenhouse space, terminal organism mood, and visual/staging
  vocabulary. Reject references that push species registries, growth simulation,
  persistence, inventory, or management loops into the first pass.
- `HighGrow` is a bounded structural reference for compact room organization:
  multiple small rooms, one to three planting sites per room, local
  climate/fixture affordances, and a magnifying-glass-like inspect/closer-look
  precedent. Do not copy crop simulation, harvesting, genetics, fertilizing,
  pruning, or optimization loops.
- `Viridi` is a bounded mood reference for calm small-container care,
  slow-time presence, gentle check-ins, peaceful companion-window language, and
  later personal attachment. Do not copy monetized nursery loops, achievement
  pressure, species-catalog pressure, death/failure pressure, or watering
  gameplay into the first pass.
- `asciiquarium` can be treated as a terminal-ecosystem atmosphere reference
  for shared ambient scene staging, not as plant architecture.
- OpenAlea, L-Py, Arbaro, AmapSim, GreenLab, PowerPlant / pplant, ONETREE, and
  Algorithmic Botany / L-studio belong in a deferred technical lineage bucket
  for future plant form, morphology, rule grammar, and environment-hook work.
  They do not authorize L-system engines, biomass allocation, organogenesis,
  species registries, plant lifecycle, persistence, or functional-structural
  plant models during the first functional-space pass.

Useful room and environment vocabulary:

- possible rooms: Propagation Room, Warm Shelf, Mist Bench, Dry Rack, Glass
  Corner, Utility Alcove
- room-local environment fields: light (`low`, `filtered`, `bright`,
  `artificial`), humidity (`dry`, `balanced`, `misted`, `damp`), temperature
  (`cool`, `mild`, `warm`), water (`none`, `tray`, `drip`, `mist`), and airflow
  (`still`, `vented`, `fan-assisted`)

Deferred plant vocabulary to preserve for later:

- plant profile
- branching grammar
- growth rule
- morphology profile
- organism silhouette
- environment hook
- inspection view

## Ecosystem Design Notes Ingest

The ecosystem design note series (`01` through `12`) is now accepted as
candidate architectural source material. It should shape the greenhouse
contract, naming, and staging rules, but it still does not authorize runtime
greenhouse implementation by itself.

Stable takeaways:

- The broader greenhouse can eventually be modeled as a stable greenhouse frame
  containing switchable internal labs, but the first implementation pass should
  still start smaller: one inert room container under `greenhouse_nursery`
  before any tab-strip, multi-lab navigation, or greenhouse-global chrome is
  promoted into runtime code.
- `AccessPath` is now more concrete: it should represent readable movement and
  work space between supports, benches, trays, and infrastructure, not player
  movement logic or generic panel spacing.
- The first room should read as a controlled container, not a loose decorative
  box. Useful first-pass shell vocabulary is roofline, rails, back wall, side
  wall, floor plane, glazing/panels, and mounting points.
- Plant-support vocabulary is now clearer: the first room should prefer a small
  support family such as nursery tray, bench surface, humidity dome, specimen
  jar slot, and optional training support before larger hydro, hanging, or
  archive-heavy structures appear.
- Lighting should be modeled as attached infrastructure with stable relation to
  supports: lamp, shelf light, panel, chain/rail attachment, and optional
  light-cone vocabulary are all valid, but floating glow or detached lighting
  props are not.
- Sensor and gauge ideas are useful, but the first pass should keep them as
  bounded inspection and room-language references. Prefer attached local tags,
  panel readouts, or one greenhouse-level summary later rather than filling the
  first room with exact numeric readouts.
- Climate actuators and water infrastructure are valid later element families,
  but the first room should only preserve their attachment logic and vocabulary
  for future use. Dense fans, ducts, pipes, valves, pumps, and dosing lines
  belong to later labs or later room revisions, not the first sparse nursery.
- Maintenance props are best treated as sparse coherence markers and later
  interaction hooks. Favor one or two purposeful objects, such as labels,
  notebook/clipboard hints, or a tool rack, rather than clutter.
- Identity hierarchy is now clearer and should be preserved as a contract:
  greenhouse identity, lab/room identity, bay or planting-site identity,
  support identity, organism identity, warning identity, and journal-link
  identity are separate layers.
- The current roadmap bias still holds: support identity must stay separate
  from organism identity, and future journal markers should signal records
  without becoming the records themselves.
- Future journaling work should stay attached to stable targets in this order:
  organism, support, planting site/bay, room/lab, then greenhouse. The first
  runtime preparation remains a read-only inspection target model, not a full
  timeline UI.

First-pass implications for YAM:

- treat the note series as a vocabulary bank for room, support, lighting,
  identity, and later journal concepts
- keep the first runtime greenhouse slice constrained to a single inert nursery
  room with tiny capacity and symbolic environment
- delay greenhouse-frame chrome, multi-lab switching, dense gauges, dense
  infrastructure, and broad journaling UI until the core room/state contract
  proves itself

Useful preserved greenhouse/lab vocabulary:

- frame/lab candidates: `Propagation Lab`, `Climate Lab`, `Grow Bay Lab`,
  `Vines Lab`, `Utility Lab`, `Archive / Specimen Lab`
- room-shell cues: `back_wall`, `side_wall`, `floor_plane`, `roofline`,
  `ceiling_rail`, `glazing_panel`, `partition_panel`
- support cues: `nursery_tray`, `humidity_dome`, `bench_planter`,
  `specimen_jar`, `shelf_rack`, `trellis_grid`
- identity cues: `lab_plaque`, `bay_tag`, `support_tag`, `organism_tag`,
  `warning_badge`, `journal_link`

## Roadmap Review Ingest

The roadmap review note reinforces the current planning direction and is now
accepted as a validation source rather than a second roadmap.

Stable takeaways:

- The current ordering is still the right one:
  `docs/readiness -> functional-space contract -> room/access/zone/fixture vocabulary -> symbolic environment -> flora storage decision -> inert state -> growth probe -> render -> inspection -> transfer`.
- The first greenhouse pass should remain a pure data contract for a
  functional room container, not a render contract, species contract, or
  growth contract.
- Reference ingests remain bounded. `HighGrow`, `Viridi`, `cbonsai`,
  `asciiquarium`, and plant-modeling lineage justify future attachment points,
  not first-pass runtime scope.
- The Ratatui boundary remains correct: render layers visualize state; they do
  not own greenhouse truth.

## Prior Brainstorming Ingest

The earlier local greenhouse brainstorming source has been ingested as
candidate material, not as an implementation contract. It should inform future
prompts, planning docs, and bounded later promotions, but no runtime behavior
should be implemented from the source note directly.

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

Goal: decide the first functional-space contract shape before adding a world
variant, visible room, or plant system.

Tasks:

- decide whether the first implementation contract stays here or graduates into
  a future `docs/greenhouse.md`
- decide whether the first runtime boundary is a hidden/dev-only room, sparse
  sandbox route, or named `WorldKind::Greenhouse`
- define what `Greenhouse` owns beyond `WorldKind::profile()`: room,
  access paths, zones, environment, fixture, planting sites, inspection, and
  later curation capability
- record that the first pass is a functional room container, not a lifecycle or
  species implementation pass
- plan profile tests before adding a world variant

Gate:

- no selectable greenhouse world exists before room and environment ownership
  are named
- no UI-local world toggle is needed
- no plant family, growth lifecycle, or persistence work begins before the
  room/site/environment/inspection contract is stable

### Phase 2: Room, Access Path, Zone, And Fixture Vocabulary

Goal: define greenhouse as world space, not panel chrome.

Tasks:

- define candidate room vocabulary such as `GreenhouseState`,
  `GreenhouseRoom`, `GreenhouseZone`, `EnvironmentProfile`, `Fixture`, and
  `PlantingSite`
- define access paths as readable room-space corridors / attention lanes before
  interaction exists
- preserve first zones: `propagation_bench`, `glass_frame`,
  `inspection_marker`, `training_frame`, `substrate_bed`, `specimen_shelf`,
  and `lamp_zone`
- keep first room capacity intentionally tiny: one to three planting sites per
  room unless a later contract proves more is needed
- preserve generic room candidates such as Propagation Room, Warm Shelf, Mist
  Bench, Dry Rack, Glass Corner, and Utility Alcove
- decide whether fixtures own rectangles, anchors, or both
- keep fixtures separate from organism lifecycle state

Gate:

- room/access/zone/fixture ownership is pure data
- room selection stays inside greenhouse/world state, not UI tabs

### Phase 3: Symbolic Environment Model

Goal: keep environment state small and useful for later growth dispatch.

Tasks:

- define symbolic light, humidity, temperature, substrate, and outside-weather
  influence fields
- consider room-local water and airflow fields if they clarify fixture/site
  status without becoming numeric simulation
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
- treat enum-backed family store as the current bias for the first
  generalization, unless inspection proves an organism registry or explicit
  multi-family structure is simpler
- keep organism identity, species id, journal id, lifecycle state, stats, and
  family vocabulary on every organism
- decide whether first greenhouse species profiles are Rust fixtures or
  structured data
- keep `seedling_tray` and `cutting_jar` as first candidate profiles
- do not implement this phase during the first functional-space pass unless a
  later work order explicitly promotes plant-system work

Gate:

- no ad hoc top-level organism vector is added beside `FloraState::vines`
  without a storage decision
- no plant-system-first implementation bypasses the room/site/environment
  contract

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

- start with read-only room, zone, fixture, planting-site, and later
  per-organism inspection
- treat the HighGrow-like magnifying glass as a bounded closer-look precedent,
  not as an editing or care-loop mandate
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

- ask first for small catalogs of room, access-path, fixture, environment,
  planting-site, inspection, and visual-vocabulary ideas
- ask for species, journal, and lifecycle ideas only as deferred candidate
  material until storage and growth gates are ready
- sort ideas into `now`, `after storage`, `after growth dispatch`, and
  `later atmosphere`
- promote only bounded ideas into root TODO or an owning contract
- reject or park ideas that require hidden UI state, render-owned simulation,
  broad mechanics, or main-scene mutation

Gate:

- creative output becomes data, contract text, fixtures, or small tasks
- the greenhouse remains a place with rules, not a pile of decorations

Immediate next tasks:

- keep brainstorming ingests candidate-only
- run the 0.4 pre-expansion verification baseline before implementation work
- decide whether `docs/greenhouse.md` is needed before inert state work begins
- decide and document the functional-space-first room/access/zone/fixture/site
  contract
- decide the first visual review artifact for `greenhouse_nursery_static_v0`
- decide the first flora storage generalization direction, with enum-backed
  family store as current bias
- identify first pure data tests for room, access path, zone, fixture,
  environment, planting site, and inspectable refs

Stop conditions:

- render-owned simulation truth
- UI-owned room, environment, or organism state
- a second projection system
- plant-system-first greenhouse work before the functional-space contract is
  stable
- a selectable greenhouse world without room/environment ownership
- another plant-family store beside `FloraState::vines` without a storage
  decision
- screenshot/golden art locks before the visual vocabulary stabilizes
- main-scene visual changes before transfer gates exist
- crop simulation, harvest loops, genetics, yield optimization, or realistic
  cultivation instruction imported from references
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
greenhouse simulation world. The first greenhouse pass is functional-space
first: room layout, access paths, zones, benches, pots, bowls, supports,
planting sites, symbolic environment profiles, and quiet read-only inspection
affordances before plant lifecycle systems. It must not become a generic
dashboard or panel UI.

Please propose creative greenhouse directions that can be expressed as room
profiles, access-path ideas, fixture ideas, planting-site sketches, environment
presets, inspect text, or small visual motifs. Favor ideas that work in terminal
cells, support later inspectable plant lifecycles, and respect a calm
botanical/lab atmosphere.
Avoid ideas that require hidden UI state, freeform dashboard panels, render-owned
simulation, crop/care gameplay, or large mechanics before the architecture is
ready.
```

Useful follow-up prompts:

- Use the accepted brainstorming ingests as source context, but keep all
  suggestions non-binding until promoted into roadmap, TODO, or a contract doc.
- Turn the strongest room ideas into compact `RoomProfile` sketches with role,
  access paths, zones, environment, fixtures, planting sites, and inspection
  notes.
- Turn the strongest plant ideas into compact `SpeciesProfile` sketches with
  habit, axes, organs, lifecycle hooks, journal events, and terminal-cell visual
  constraints, but treat them as deferred material unless plant-system work has
  been explicitly promoted.
- Turn the strongest fixture ideas into world-attached support sketches with
  placement rules, attachment behavior, and visual texture.
- Sort all ideas into `now`, `after storage`, `after growth dispatch`, and
  `later atmosphere` buckets.
