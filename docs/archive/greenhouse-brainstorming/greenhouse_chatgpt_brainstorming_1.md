
# Greenhouse Brainstorming 1

## Work-Order Constraint: Functional Space First

For the first greenhouse pass, create a functional greenhouse space before implementing plant systems.

The goal is to define the room, access paths, fixtures, planting sites, environmental zones, and inspection affordances first. Plant organisms, species data, growth state, and lifecycle behavior remain deferred until the functional space contract is stable.

This is an ordering rule only. It does not demote plants as the eventual core content of the greenhouse.

## Why This Distinction Matters Now

This distinction matters because it decides what must be stabilized first.

A greenhouse-space-first pass starts with the spatial and operational container:

```text
room → zones → fixtures → planting sites → environment profile → inspection surface
```

A plant-system-first pass would instead pull the project toward biological state and persistence too early:

```text
organism families → species registry → growth state → lifecycle → persistence → interactions
```

For YAM 0.4, the safer order is to define the nursery room as a stable container, then attach organisms into known sites later.

The greenhouse should therefore begin as an atmospheric, inspectable, functional room model rather than a premature plant simulation.

## Current Vision Sentence

The greenhouse is a nursery / propagation room: a small, inspectable, symbolic environment where plant life can later be understood through fixtures, planting sites, room conditions, and read-only inspection before any game-like progression exists.

## Codex-Safe Interpretation

Do not implement a selectable `WorldKind::Greenhouse` until room, environment, and planting-site ownership are documented.

Do not expand flora storage, species registries, or growth lifecycle behavior during the first greenhouse-space pass unless a later work order explicitly authorizes it.

The first pass should clarify the functional spatial contract that future plant systems will attach to.

## Reference Note: `cbonsai` Still Valid, But Bounded

`cbonsai` remains a valid inspirational reference for the greenhouse vision.

Use it as a mood, tempo, and terminal-organism reference, not as an architecture reference and not as a reason to start plant lifecycle implementation early.

The useful lesson from `cbonsai` is that a terminal organism can feel alive through slow procedural emergence, minimal controls, careful glyph/color choices, and contemplative display.

For YAM 0.4, this reference should stay bounded:

```text
valid reference → terminal plant mood, procedural emergence, live/static display, contemplative pacing
not valid as  → greenhouse architecture, room ownership model, flora storage model, species lifecycle plan
```

`cbonsai` should therefore inform later plant rendering and growth-feel work, while the first greenhouse pass remains focused on functional space: room, access paths, fixtures, planting sites, environmental zones, and inspection affordances.

Codex-safe interpretation: do not turn `cbonsai` inspiration into an implementation mandate during the functional-space pass.

## Reference Pass: Web Exploration Should Be Bounded

External web exploration is useful, but it should be treated as a bounded reference audit rather than open-ended inspiration hunting.

YAM already has direct inspiration sources in its documentation. Any additional web references should therefore be sorted by what they clarify:

```text
terminal-life references       → mood, tempo, procedural organism feel, ambient animation
functional greenhouse sources  → room layout, zones, fixtures, benches, access paths, environmental control
visual/staging references      → descriptive vocabulary for glass, trays, racks, lights, mist, humidity, propagation surfaces
```

The reference pass should not become a feature-expansion pass.

Use external references only when they clarify the greenhouse's functional-space contract, terminal-organism mood, or visual vocabulary. Do not use them to expand YAM 0.4 implementation scope unless a later work order explicitly promotes them.

### Candidate Reference Lanes

`cbonsai` remains the cleanest plant-specific terminal reference. Its useful traits are live/static growth display, configurable pacing, tree positioning, coloring, bases, messages, and screensaver-like repetition. These traits support later plant mood and growth-feel work, not the first greenhouse-space contract.

`asciiquarium` is useful as a broader terminal-ecosystem reference. It demonstrates an ambient ASCII scene with multiple moving organisms sharing one visual environment. Its relevance is ecological staging, motion layering, and playful terminal atmosphere, not plant architecture.

Greenhouse design references are more important for the immediate work order. They commonly organize greenhouse thinking around layout, workflow, benches, propagation areas, environmental control, humidity, light, water access, ventilation, and growing surfaces. These concepts support the current functional-space-first pass.

### Codex-Safe Interpretation

A web reference should be admitted only if it lands in one of these buckets:

```text
1. clarifies the room / zone / fixture / planting-site contract
2. clarifies terminal organism mood without mandating plant lifecycle implementation
3. supplies visual vocabulary for the greenhouse scene
```

Reject or defer references that push toward species registries, growth simulation, persistence, inventory systems, or management-game mechanics during the first greenhouse-space pass.

## Existing YAM Inspiration Sources

YAM already has a mature reference map. New greenhouse references should be admitted only after checking whether they add something not already covered by the existing inspiration set.

### Core Adopted / Structural References

```text
Ratatui        → adopted Rust TUI/render substrate; immediate-mode terminal UI foundation
Chafa          → adopted hero/rendering compiler baseline for terminal image and animation conversion
ansi-to-tui    → adopted ANSI-to-Ratatui conversion seam where needed
tachyonfx      → adopted presentation/effect polish, not state ownership
wttr.in JSON   → adopted weather provider path; YAM owns normalized state and rendering
```

These are infrastructure references. They shape how YAM renders and presents information, but they should not independently expand greenhouse scope.

### UI / Presentation References

```text
Awesome Ratatui / exemplar TUIs → popup, inspector, keyboard dispatch, settings-panel patterns
weather TUIs / wttr.in grammar  → compact condition/fact ordering and terminal weather layout
wego                            → compact weather-display precedent
BTAS / TNBA palette language    → dark-deco palette, restrained semantic color, cel-animation discipline
terminal palette conventions    → ANSI fallback roles and terminal compatibility expectations
Kitty / Ghostty-like terminals  → practical terminal visual baseline
```

These references support mood, layout grammar, palette discipline, and terminal compatibility.

### Offline / Source-Art References

```text
Moebius / MoebiusXBIN           → offline colored ANSI/XBIN source-art workflow
REXPaint                        → text-grid authoring and interchange experiments
perkins                         → braille-oriented offline cleanup / doctoring experiments
LachlanArthur / Braille-ASCII-Art → future braille dot-packing and error-diffusion comparison
Ansizalizer / ansipx            → custom glyph-set and palette experiments; cautionary, not replacement baseline
```

These references are useful for art-pipeline thinking. They should remain behind YAM-owned rendering contracts such as `CellGrid`, scene layers, and explicit render seams.

### Procedural Flora / Growth References

```text
cbonsai                  → terminal plant mood, slow/live emergence, glyph economy, static/live display
rbonsai                  → Rust/crossterm adjacent confirmation of the terminal-bonsai lineage
L-systems                → procedural branching idea bank
space-colonization algorithms → procedural branching/growth idea bank
cellular automata        → growth-system idea bank, only if kept deterministic and art-directable
agent-based growth literature → future flora-behavior inspiration, not first-pass mandate
```

These are the most greenhouse-relevant references, but they remain bounded. They inform later plant behavior, growth feel, and organism rendering. They do not justify implementing plant lifecycle, persistence, or species registries during the functional-space pass.

### Systems / Game-Ancestry References

```text
Dwarf Fortress      → world/system depth and inspectable environment ancestry
Cataclysm: DDA      → inspectable systems, item/state legibility, plant/UI lineage
```

These are lineage references only. They should not push YAM toward management-game mechanics, inventory sprawl, or simulation breadth during YAM 0.4.

### Future Companion / Non-Greenhouse References

```text
Apple MusicKit / MusicKit JS / Music.app automation → possible future yam-music companion path
```

This reference family is not relevant to the greenhouse pass unless a later work order explicitly reopens music integration.

### Greenhouse-Relevant Subset

For the current greenhouse discussion, keep the active reference subset narrow:

```text
cbonsai                                  → terminal plant mood and slow emergence
L-systems / space-colonization / CA      → later plant behavior idea bank
BTAS / TNBA palette language             → dark-deco greenhouse mood and color discipline
Ratatui scene/layer model                → implementation substrate, not creative expansion
Chafa / braille rendering references     → future plant or hero-art handling, not room contract
Dwarf Fortress / Cataclysm: DDA          → inspection/system lineage, dangerous if over-applied
```

### Current Reference Posture

YAM already has enough inspiration sources for the greenhouse to proceed without a large new inspiration hunt.

Use `cbonsai` and procedural-growth literature as bounded future plant references, while the first greenhouse pass stays anchored to functional space: room, access paths, fixtures, zones, planting sites, environment profiles, and inspection affordances.

Do not admit a new external reference unless it clarifies one of the following:

```text
1. functional greenhouse space
2. terminal organism mood
3. visual vocabulary
4. inspection or UI legibility
```

Reject or defer references that duplicate existing YAM inspirations or push the greenhouse toward premature plant simulation, persistence, inventory, or management mechanics.

## Reference Note: HighGrow-Like Room / Climate Abstraction

The greenhouse system may use `HighGrow` as a loose structural reference, but only at the level of room-scale abstraction and compact plant occupancy.

The useful pattern is:

```text
multiple small rooms → each room contains a few plant slots → each room has local care / climate controls
```

For YAM, this should be adapted as:

```text
Greenhouse
  → Room 1..N
    → 1..3 planting sites
    → local environment profile
    → fixture support
    → inspection surface
```

This reference supports the idea that the greenhouse does not need to start as one large undifferentiated biome. It can be modeled as a small set of named functional rooms, each with limited plant capacity and its own environmental affordances.

The first YAM version should stay vague and atmospheric rather than becoming a detailed grow simulator.

### What To Borrow

```text
borrow → multiple rooms
borrow → small number of plant slots per room
borrow → room-local light / water / temperature / humidity affordances
borrow → per-room inspection and status display
borrow → slow day-by-day feeling, if time progression is introduced later
```

### What Not To Borrow Yet

```text
avoid → crop-specific simulation
avoid → detailed fertilizing / pruning / harvesting loop
avoid → species genetics
avoid → yield optimization
avoid → management-game economy
avoid → realistic cultivation instruction
```

### Codex-Safe Interpretation

`HighGrow` is a bounded structural reference for compact room organization and local climate-control affordances.

It does not override the functional-space-first work order.

During the first greenhouse pass, use this reference only to clarify that rooms may contain 1–3 planting sites and may expose local climate/fixture support. Do not implement plant lifecycle, crop-specific mechanics, harvest loops, species genetics, or optimization systems unless a later work order explicitly authorizes them.

## Refined Work Order: HighGrow-Like Rooms, Not HighGrow-Like Content

Use `HighGrow` as a loose precedent for the greenhouse's room model: multiple rooms, each holding a very small number of plants, with room-local environmental support.

The useful abstraction is not the subject matter or detailed care loop. The useful abstraction is the compact structure:

```text
small named room → 1–3 plant sites → local climate/fixture affordances → inspectable status
```

For YAM, this points toward a greenhouse made from several small functional rooms rather than one generic plant area.

Possible room examples should stay generic and non-crop-specific:

```text
Propagation Room
Warm Shelf
Mist Bench
Dry Rack
Glass Corner
Utility Alcove
```

Each room may eventually expose a small local environment profile:

```text
light      → low / filtered / bright / artificial
humidity   → dry / balanced / misted / damp
temperature → cool / mild / warm
water      → none / tray / drip / mist
airflow    → still / vented / fan-assisted
```

For the first functional-space pass, these should be representational and inspectable rather than physically simulated.

### Strict Boundary

Do not import `HighGrow`'s crop-specific loop, realistic cultivation mechanics, harvesting goal, or optimization logic.

For YAM 0.4, the greenhouse should feel like a symbolic terminal greenhouse with compact rooms and local climate affordances, not a real-world cultivation simulator.

### Implementation-Order Reading

This reinforces the existing work-order constraint:

```text
first  → room model, room names, access paths, fixtures, planting sites, inspection affordances
later  → plant occupants, growth feel, organism rendering, lifecycle, persistence
never by default → crop simulation, yield optimization, realistic cultivation instruction
```

## Uploaded HighGrow 4.20 Artifact: Safe Reference Extraction

The uploaded `HighGrow_420.exe` should be treated as a reference artifact only. Do not run it as part of YAM work.

The accompanying `HighGrow_420.txt` is more useful than the executable for brainstorming because it gives a compact feature summary of the original program.

Useful extracted references from the text file:

```text
multiple grow rooms      → confirms multi-room organization as a valid structural precedent
room editing             → pots, soil, lights, reflectors as fixture/support vocabulary
up to ninety plants      → confirms original scale, but YAM should deliberately compress this
vacation mode            → possible later passive-care / ambient-state idea
magnifying glass         → strong precedent for inspect / closer-look affordance
MP3/MIDI player          → ignore for greenhouse; belongs, if anywhere, to separate music companion concerns
alarm clock              → possible later reminder/status affordance, not greenhouse core
```

The strongest new YAM idea from the uploaded artifact is:

```text
Magnifying Glass → Inspect Mode
```

This fits YAM better than the original care loop. Inspection can be atmospheric, read-only, and scene-native without becoming a management system.

### Refined YAM Translation

```text
Room
  → planting sites
  → fixtures/supports
  → environment profile
  → inspection affordance
```

The uploaded artifact strengthens the HighGrow-like-room reference in two specific ways:

```text
1. greenhouse rooms may have editable/support fixtures in concept, even if first pass only renders or describes them
2. each room or planting site may later support an inspect/zoom affordance
```

### Boundary From Uploaded Artifact

Do not copy the original program's crop-specific subject matter, realistic lifecycle framing, harvesting objective, fertilizing/pruning loop, or optimization logic.

For YAM, the useful inheritance is spatial and interface-oriented:

```text
multi-room structure
few visible sites per room
local fixture vocabulary
local environment affordances
inspect / closer-look interaction
```

### Codex-Safe Interpretation

Treat `HighGrow_420.txt` as a bounded reference note for room organization, fixture vocabulary, and inspect-mode precedent.

Do not run or reverse-engineer the executable.

Do not implement HighGrow-like plant care, crop simulation, harvest logic, genetics, or optimization mechanics during the YAM 0.4 functional-space pass.

## Reference Note: `Viridi` / Small-Pot Slow-Care Mood

`Viridi` is a valid greenhouse-adjacent inspiration, but it should be bounded as a mood, pacing, and small-container care reference rather than a systems architecture reference.

The useful public framing is that `Viridi` asks the player to nurture a small pot of succulents that grow in real time, functioning as a safe haven and a quiet place to return to for a moment of peace. It supports checking in on plants as a meditative moment, forgiving maintenance, naming favorite plants, special attention, flowering, a nursery, and occasional free seedlings.

For YAM, the useful translation is not a full succulent simulator. The useful translation is:

```text
small contained plant scene → real-time / slow-time presence → gentle check-in → minimal care/status → peaceful companion window
```

This is especially compatible with YAM because YAM is already a terminal companion scene rather than a goal-driven management game.

### What To Borrow

```text
borrow → calm companion-window mood
borrow → small contained ecology
borrow → slow real-time or pseudo-real-time presence
borrow → check-in rather than command-heavy play
borrow → plant naming / personal attachment later, if persistence exists
borrow → flowering or visible reward as rare atmosphere, not optimization
borrow → forgiving maintenance language if care is ever added
borrow → nursery vocabulary as a gentle acquisition / selection metaphor later
```

### What Not To Borrow Yet

```text
avoid → monetized nursery / purchase model
avoid → full species catalog pressure
avoid → death/failure pressure during the first greenhouse-space pass
avoid → detailed watering loop as core gameplay
avoid → achievements / collection pressure
avoid → turning greenhouse into an idler or management economy
```

### Relationship To Existing References

`Viridi` complements `cbonsai` and `HighGrow` without replacing either:

```text
cbonsai  → procedural terminal plant emergence and glyph economy
HighGrow → multi-room / local fixture / inspect-mode structural precedent
Viridi   → calm small-container care mood, check-in cadence, personal attachment
```

For the first greenhouse pass, `Viridi` should influence tone and inspection language, not architecture. The room model still comes first.

### Codex-Safe Interpretation

Treat `Viridi` as a bounded reference for calm greenhouse mood, slow plant presence, gentle check-in cadence, and future personal attachment affordances.

Do not use `Viridi` to justify implementing plant lifecycle, species catalogs, watering gameplay, death/failure pressure, achievements, collection systems, monetized nursery mechanics, or management loops during YAM 0.4.

The first greenhouse pass remains functional-space-first: room model, access paths, fixtures, planting sites, environment profiles, and inspection affordances.

## Research Note: Simulated Plant-Growth Software Lineage

The Wikipedia page on simulated plant growth is useful as an index, not as the sole authority. Its listed software section points toward a serious procedural / scientific plant-modeling lineage that should be recorded separately from the softer mood references such as `cbonsai`, `HighGrow`, and `Viridi`.

Source index:

```text
https://en.wikipedia.org/wiki/Simulated_growth_of_plants
```

### Verified / Useful Software References

```text
OpenAlea
  role: open-source plant modelling platform / ecosystem
  source: https://openalea.readthedocs.io/
  source: https://inria.hal.science/hal-00831801v1
  YAM use: future reference for plant architecture tooling ecosystems; too heavy for YAM 0.4

L-Py
  role: Python L-system simulation framework for modeling plant architecture development
  source: https://lpy.readthedocs.io/
  source: https://github.com/openalea/lpy
  source: https://www.frontiersin.org/journals/plant-science/articles/10.3389/fpls.2012.00076/full
  YAM use: strong later reference for rule-based plant grammar, morphology, and procedural growth; not first greenhouse pass

Branching: L-system Tree
  role: small Java applet / source-code demo of botanical tree growth using L-systems
  source: https://www.mizuno.org/applet/branching/
  YAM use: minimal reference for simple branching grammar and readable growth demonstrations

Arbaro
  role: Java tree generator implementing Weber & Penn's “Creation and Rendering of Realistic Trees”
  source: https://arbaro.sourceforge.net/
  source: https://sourceforge.net/projects/arbaro/
  source: https://macs4plants.cirad.fr/projects/weberpenn/
  YAM use: reference for parameterized tree/form profiles; useful later if plant forms need profile-driven silhouettes

AmapSim
  role: CIRAD / AMAP whole-plant architecture simulation lineage
  source: https://web.archive.org/web/20131001215755/http://umramap.cirad.fr/amap2/logiciels_amap/index.php?page=amapsim
  YAM use: reference for separating plant architecture simulation from external applications; likely archival only

GreenLab
  role: functional-structural plant model for plant structure establishment and production
  source: https://www.quantitative-plant.org/model/GreenLab
  source: https://greenlab.cirad.fr/GLUVED/html/P2_GLab/GL_intro.html
  YAM use: strong cautionary reference; scientifically rich, but far beyond YAM's intended greenhouse scope

PowerPlant / pplant
  role: parametric plant description / plant-modeling tool lineage
  source: https://sourceforge.net/projects/pplant/
  YAM use: later reference for parameterized plant descriptions; not renderer or runtime target

ONETREE
  role: art/system reference where a local CO2 meter influences virtual tree growth rate, according to the Wikipedia index
  source: https://en.wikipedia.org/wiki/Simulated_growth_of_plants
  YAM use: interesting environment-signal-to-growth metaphor, but too niche and not implementation-ready
```

### Additional Strong Lineage Reference

```text
Algorithmic Botany / L-studio
  role: formal L-system / plant-modeling research lineage and historical tool ecosystem
  source: https://algorithmicbotany.org/
  source: https://algorithmicbotany.org/papers/lstudio.fsmp2004.pdf
  YAM use: later theoretical reference for grammar-based plant generation and environment-aware plant models
```

### YAM Classification

These sources should be treated as a deferred technical lineage:

```text
cbonsai / Viridi / HighGrow
  → mood, room scale, check-in cadence, inspect mode, terminal plant feel

OpenAlea / L-Py / Arbaro / AmapSim / GreenLab / PowerPlant / Algorithmic Botany
  → future plant form, morphology, rule grammar, architectural growth, and environment-hook references
```

### What This Adds To The Greenhouse Vision

This research strengthens the idea that future YAM plants can eventually be described through explicit plant-form grammars or parameter profiles rather than ad-hoc sprites.

Possible future vocabulary:

```text
plant profile
branching grammar
growth rule
morphology profile
organism silhouette
environment hook
inspection view
```

However, these terms should remain future-facing until the functional greenhouse space is stable.

### Strict Boundary

Do not import scientific plant simulation scope into YAM 0.4.

Do not implement L-system engines, biomass allocation, organogenesis, species registries, plant lifecycle, persistence, growth rules, or functional-structural plant models during the first functional-space pass.

The immediate greenhouse work remains:

```text
room model
access paths
fixtures
planting sites
environment profiles
inspection affordances
```

### Codex-Safe Interpretation

Use the simulated-growth software lineage as a deferred reference bucket for future plant morphology and growth-grammar work.

For YAM 0.4, these sources only justify making the greenhouse room/site/environment contract clean enough that future plant generators could attach to it later.
