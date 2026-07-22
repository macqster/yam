# HighGrow 4.20 Reference Analysis

Status: reference analysis
Scope: external plant-simulation reference, distilled from a real released game
Repo baseline: YAM `0.3.9`
Snapshot note: source game is HighGrow 4.20 (Slick Software, 2005), already
credited in `README.md`'s acknowledgements as a greenhouse/plant-simulation
reference nod
Implementation status: do not implement from this file directly; promote
individual ideas into `docs/greenhouse-roadmap.md`, `TODO.md`, or an explicit
task the same way brainstorming material in this directory is already handled

## Purpose

`README.md` already cites HighGrow as design-reference lineage alongside
Viridi, OpenAlea, L-Py, Arbaro, GreenLab, PowerPlant, and Algorithmic Botany.
This document makes that citation concrete: it distills HighGrow's actual
mechanics (extracted from its installed help manual and resource strings,
not run or played) into a form comparable to YAM's own greenhouse vocabulary,
so future greenhouse work can borrow specific, proven mechanics rather than
re-deriving them from scratch.

Only the mechanical/structural patterns are of interest here — the
simulation's specific subject matter is incidental to why it's a useful
reference, exactly as with the other citations in `README.md`.

## Source And Method

HighGrow 4.20 was installed locally via Wine (Sikarugir) for inspection.
Content was extracted by:

- converting the bundled `HighGrow.chm` help manual to plain HTML with
  `extract_chmLib` (from Homebrew's `chmlib`), then stripping tags
- extracting readable ASCII/UTF-16LE string runs from `Comments.dll` and
  `Robbie.dll` (in-game commentary and character-dialogue text resources)

No gameplay was observed directly; this is a text-only distillation of the
manual and resource strings.

## Distilled Mechanics

### Room / fixture structure

- Base unit is a **grow room**: a fixed-capacity container holding exactly
  3 pots, each with its own independent lamp. Capacity is a hard constraint
  ("space limitations"), not a soft suggestion.
- Additional rooms are unlocked as discrete units (not a continuous space),
  each contributing a fixed +3 planting capacity.
- Each room has independently configurable: light type (2 options), light
  strength (2 wattages), light reflector (10 options, affecting beam spread),
  pot (30 options, cosmetic + soil-capacity), soil type (5 archetypes, each
  with explicit advantages/disadvantages/improvement-tip triplet), and which
  status gauges are visible.
- Rooms are visually a static background image with fixtures composited on
  top — not directly transferable to YAM's terminal rendering, but the
  underlying pattern (fixed background + independently-configured overlay
  fixtures per room) maps cleanly onto `core::greenhouse`'s room/zone/fixture
  vocabulary.
- Lights are explicitly **not cross-influential** between pots in the same
  room — a deliberate realism-for-flexibility tradeoff, so a room can hold
  plants at different lifecycle stages simultaneously without one plant's
  light schedule forcing all others into the same stage.

### Soil archetype pattern (directly reusable data shape)

Each of the 5 soil types is defined by exactly three fields: Advantages,
Disadvantages, To-Improve (a corrective tip). This is a clean, small,
data-driven pattern worth considering for YAM's own environment-profile
model — it gives every soil choice a legible tradeoff and a player-facing
corrective action, without needing a deep chemistry simulation underneath.

### Per-organism state and logging (matches YAM's existing direction closely)

- Tracked per-plant, per-day: health, height, mass, potency, moisture,
  soil pH, nutrient level — surfaced as both a tabular "Growth Log" and a
  "Growth Chart" (graph).
- Health trends up over time by default; decline is a signal something is
  wrong (over/under-watering, lamp-tip contact), not a natural baseline.
- Potency is genetically dominated (seed/strain-determined) with
  environment (temperature, humidity) acting as a secondary modifier —
  explicitly stated in the manual as a small effect compared to variety.
  This maps directly onto YAM's existing `core::organism` species-registry
  bias (genetics-first, environment-as-modifier) already implied by having
  a `SpeciesRegistry` distinct from per-instance state.
- Harvest produces a persistent artifact (a "certificate" with the final
  stats) — a natural fit for `OrganismJournal`'s per-instance event history.

### Growth-graph / pruning mechanic (directly relevant to YAM's flora model)

- Clipping a growing tip causes **two new tips to sprout from the nearest
  leaf axil** — a literal branch-on-prune rule. Pruning upper tips lets
  lower branches "catch up," flattening the plant's canopy shape.
  This is a concrete, already-proven instance of exactly the kind of rule
  YAM's own architecture docs describe wanting ("growth tips/meristems,"
  "hybrid graph-plus-segment model: plant state is a growth graph of
  organs, geometry emitted as segments under deterministic rules") — worth
  treating as a validated reference rule rather than a novel one to invent.
- Pruning has an explicit cooldown (roughly a week) and is disallowed on
  seedlings — a recovery-time gate on a structural mutation, not an
  instant, repeatable action.

### Lighting / photoperiod as a reversible stage-transition control

- A single input (daily light hours) drives a real lifecycle transition:
  long photoperiod keeps a plant vegetative indefinitely; ~2 weeks under a
  reduced photoperiod (≤12h/day) triggers flowering; increasing light again
  **reverts flowering back to vegetative growth**. The transition is
  reversible, not a one-way gate — a useful precedent if YAM wants
  lifecycle stages to be player/environment-steerable rather than strictly
  time-locked.
- Photoperiod is capped just under 24h (23h45m) rather than allowing a
  literal "always on" state — worth noting as a boundary-condition
  precedent (avoiding a degenerate full-day edge case) if YAM models a
  similar daily-cycle input.

### Idle/offline catch-up pattern (directly relevant to any tick-while-closed model)

- Growth is calculated once per real-world calendar day, on next launch
  (including catching up multiple missed days at once), not via a live
  background process while the game is closed. Visiting more than once in
  the same day does not add extra growth, but hands-on actions (water,
  fertilize, prune, adjust light) apply immediately regardless of the daily
  growth-calc cadence.
- "Vacation Mode" pre-schedules watering/fertilizing amounts and timing,
  plus automatic light-height adjustment as the plant grows, and — like
  regular growth — resolves entirely in a single catch-up pass on next
  launch, needing no process to stay running.
- System-clock tampering is explicitly defended against: a day's growth,
  once calculated, cannot be recalculated by rewinding the clock.
- This whole pattern (deterministic once-per-day batch catch-up, idempotent
  per calendar day, no live background process required) is a clean,
  proven shape for any future YAM greenhouse tick model that needs to
  survive the app being closed for arbitrary real-world time.

### Ambient/companion features (lower priority, but consistent with YAM's existing companion vocabulary)

- A caretaker NPC ("Rasta Robbie") animates on-screen during Vacation Mode
  and for an opt-in alarm-clock feature, driven by a simple named
  animation-frame + synced-sound resource format ("Toon" files) rather than
  a general animation engine.
- A magnifying-glass gauge-panel toggle gives a click-and-hold, drag-able
  circular zoom lens over the scene — a lightweight "inspect closer"
  affordance that doesn't require a separate modal or full-screen zoom
  mode, conceptually adjacent to YAM's existing pointer-probe/debug-inspect
  surfaces.
- Room lighting brightness responds to how many of the room's lamps are
  currently on (0/1/2/3 lit) — light state is simultaneously a visual fact
  and a mechanical one (the same reflector/intensity setting that looks
  brighter also grows plants faster), not two separate systems to keep in
  sync.

## What Not To Carry Over

- No literal NPK-gram fertilizer chemistry or numeric soil-pH simulation —
  YAM's greenhouse direction is explicitly a "calm, inspectable, botanical"
  symbolic environment, not a chemistry sim; the *shape* of the
  soil-archetype and nutrient-role patterns is useful, the literal units
  are not.
- No install-limiting/anti-piracy behavior (HighGrow "actively prevents
  multiple installations" to cap total plants per user) — an artifact of
  2005 freeware distribution concerns, not a design pattern.
- No photographic-background room compositing — YAM's rendering contract
  (`docs/rendering.md`, `docs/scene-model.md`) is terminal/world-space
  native; only the underlying "fixed room + independently configured
  fixture set" structure is the transferable part, not the presentation
  technique.

## Reading Rules For Future Sessions

- Treat this file as reference analysis, not an implementation contract or
  brainstorming prompt to expand further.
- Do not begin implementing any mechanic listed here without first checking
  it against the current state of `docs/greenhouse-roadmap.md`'s gate
  checklist and locked first-pass decisions — greenhouse implementation
  gates take precedence over anything in this file.
- If a specific mechanic here gets adopted, promote it explicitly into
  `docs/greenhouse-roadmap.md` (or `TODO.md` for execution tracking) rather
  than treating its presence in this file as sufficient authorization.
