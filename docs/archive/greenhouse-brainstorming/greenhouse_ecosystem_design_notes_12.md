
# Greenhouse Ecosystem Design Notes 12

## Plant Organism Logging / Journaling System

### Purpose of this note

Notes 04–11 defined the greenhouse shell, plant supports, lighting, sensors, climate actuators, water / nutrient infrastructure, maintenance props, and identity markers.

Note 12 defines the **plant organism logging / journaling system**: the record layer that attaches observations, care actions, warnings, sensor readings, movements, growth stages, and notes to stable greenhouse identities.

This note answers:

```text
How does YAM remember what happened to each plant organism over time?
```

The journal system should not be a floating text log. It should be anchored to the identity hierarchy from Note 11:

```text
Greenhouse
└── Lab
    └── Bay / station / shelf / wall zone
        └── Support object
            └── Plant organism / specimen
                └── Journal entries
```

The journal is the greenhouse’s memory layer.

---

## Why journaling follows identity markers

The current design sequence is:

```text
04 Boundary / shell / lab frame
05 Plant-support elements
06 Lighting elements
07 Sensor and gauge elements
08 Climate-control actuators
09 Water / nutrient infrastructure
10 Work / maintenance props and tools
11 Navigation / identity markers
12 Plant organism logging / journaling system
```

Note 11 established a critical separation:

```text
bay identity ≠ support identity ≠ organism identity
```

Note 12 builds directly on that separation.

A plant journal needs stable targets:

- the plant organism being tracked
- the support object it currently occupies
- the bay / lab location where an event occurred
- the sensor, actuator, or tool involved
- the time of the event
- the type of event
- the visible consequence or status marker

Real horticultural and greenhouse practice supports this approach. Garden journals commonly record observations about plants, weather, soil, growth, and events over time. Greenhouse IPM scouting guidance emphasizes regular monitoring, greenhouse maps / logical units, plant inspections, pest and disease observations, sticky cards, flags, and records so information can be communicated consistently. Crop scouting guidance also emphasizes accurate documentation of plant health, environmental stress, pest presence, diseases, growth stage, and other field observations.

Design implication:

```text
stable identity → timestamped event → organism history → visible journal marker
```

---

## Core journal model

Every journal entry should answer ten questions:

1. **Target** — Which organism, support, bay, lab, or system does this entry belong to?
2. **Time** — When did it happen?
3. **Entry type** — Observation, care action, sensor event, warning, movement, growth stage, maintenance, or note?
4. **Source** — Manual note, sensor-derived event, scheduled event, tool action, warning state, or system event?
5. **Location** — Which lab / bay / support was involved?
6. **State before** — What was the relevant prior state, if known?
7. **State after** — What changed?
8. **Severity** — Info, watch, warning, fault, resolved, archive?
9. **Display summary** — What short text or marker should be visible in the scene?
10. **Persistence** — Temporary event, permanent organism history, archived record, or debug-only entry?

Design rule: the journal entry should be structured enough for automation but short enough to summarize in terminal UI.

---

## Identity targets

### 1. Organism-level journal

Primary record target.

Examples:

```text
ivy03
basil02
fern011
```

Use for:

- growth observations
- watering events
- pruning events
- health notes
- pest / disease observations
- movement / transplant history
- phenotype notes
- lifecycle events

Rule:

```text
The organism journal moves with the plant.
```

If `ivy03` moves from `B1` to `B3`, its journal remains attached to `ivy03`.

### 2. Support-level journal

Record for pots, trays, hydro buckets, jars, benches, trellis sections, or cases.

Examples:

```text
pot-04
tray-A
jar-07
hydro-B2
```

Use for:

- pot cleaned
- tray replaced
- hydro bucket low water
- jar sealed
- trellis repaired
- support cracked
- substrate changed

Rule:

```text
Support history is separate from organism history.
```

A pot can be replaced without deleting the plant’s record.

### 3. Bay / station journal

Record for stable spatial positions.

Examples:

```text
GROW/B1
GROW/B2
PROP/SHELF-A
VINES/WALL-03
```

Use for:

- repeated dry zone
- light misalignment
- vent airflow issue
- persistent pest finding
- station maintenance
- empty / occupied history

Rule:

```text
Bay history describes a place, not a plant.
```

### 4. Lab-level journal

Record for room-wide events.

Examples:

```text
Grow Bay Lab
Climate Lab
Archive Lab
```

Use for:

- humidity incident
- fan outage
- shade curtain test
- cleaning pass
- global inspection
- sensor calibration

Rule:

```text
Lab entries should summarize room-wide events, not replace organism entries.
```

### 5. Greenhouse-level journal

Record for global events.

Examples:

```text
GH-01 day 003
outside weather event
all-lab maintenance window
```

Use for:

- day / night cycle milestone
- global alert
- greenhouse-wide inspection
- all-lab watering / cleaning
- mode change

Rule:

```text
Greenhouse entries should be rare and high-level.
```

---

## Primary journal entry families

### 1. Observation entry

Purpose: record what was seen.

Best targets:

- organism
- support
- bay
- specimen

Examples:

```text
obs: new leaf visible
obs: lower leaves yellowing
obs: condensation inside jar
obs: vine reached trellis line 03
```

Fields:

| Field | Example |
|---|---|
| target | `ivy03` |
| time | `day 003 / 08:21` |
| location | `GROW/B1` |
| summary | `new leaf visible` |
| severity | `info` |
| source | `manual` |

Visual marker:

```text
ivy03 log+1
```

Design rule: observation entries are the main narrative layer. They should be human-readable.

### 2. Care-action entry

Purpose: record direct maintenance / care actions.

Best targets:

- organism
- support
- bay
- lab

Care verbs:

```text
water
prune
inspect
clean
repair
calibrate
transplant
label
rotate
move
thin
mist
```

Examples:

```text
care: watered ivy03
care: pruned basil02
care: cleaned tray-A
care: calibrated pH meter
care: replaced drip emitter at B2
```

Fields:

| Field | Example |
|---|---|
| action | `water` |
| target | `basil02` |
| tool | `watering can` |
| amount | optional / future |
| location | `GROW/B2` |
| result | `moisture ok` |

Visual marker:

```text
B2 log+
```

Design rule: care actions should connect Note 10 maintenance props to actual recordable events.

### 3. Sensor-derived entry

Purpose: convert sensor state into a record.

Best targets:

- organism
- bay
- support
- lab
- greenhouse

Examples:

```text
sensor: B2 moisture low
sensor: Grow Bay humidity high
sensor: lamp coverage low at B3
sensor: reservoir level low
sensor: pH drift detected
```

Fields:

| Field | Example |
|---|---|
| sensor | `soil moisture` |
| reading | `low` or `42%` |
| threshold | optional |
| target | `GROW/B2` |
| severity | `watch` / `warning` |
| resolved | false |

Visual marker:

```text
[p2 dry!] log
```

Design rule: not every sensor tick should become a journal entry. Only meaningful threshold crossings, state changes, or sampled summaries should be logged.

### 4. Warning / incident entry

Purpose: record abnormal events that need attention or postmortem history.

Best targets:

- bay
- organism
- support
- sensor
- actuator
- lab

Examples:

```text
incident: B2 dry substrate warning
incident: fan fault in Grow Bay
incident: overflow in runoff channel
incident: shade motor jammed
incident: fungus gnat card flagged watch state
```

Fields:

| Field | Example |
|---|---|
| target | `GROW/B2` |
| severity | `warning` |
| cause | `moisture low` |
| status | `open` |
| resolution | optional |
| linked action | optional |

Visual marker:

```text
B2 dry! log
```

Design rule: warning entries should be resolvable. The journal should know if a warning is still open or has been handled.

### 5. Resolution entry

Purpose: close or soften a previous warning / incident.

Best targets:

- warning event
- organism
- bay
- lab
- support

Examples:

```text
resolved: B2 moisture ok after watering
resolved: fan restarted
resolved: emitter unclogged
resolved: pH returned to range
```

Fields:

| Field | Example |
|---|---|
| resolves | `incident-014` |
| action | `watered` |
| result | `moisture ok` |
| time | `day 003 / 09:02` |

Visual marker:

```text
B2 ok log
```

Design rule: resolution entries are important because they keep the journal from becoming only an alarm list.

### 6. Movement / transplant entry

Purpose: record changes in physical location or support.

Best targets:

- organism
- source support
- destination support
- source bay
- destination bay

Examples:

```text
move: ivy03 B1 → B3
transplant: basil02 tray-A → pot-04
archive: fern011 Grow Bay → Archive jar-07
```

Fields:

| Field | Example |
|---|---|
| organism | `ivy03` |
| from | `GROW/B1/pot-01` |
| to | `GROW/B3/pot-08` |
| reason | `space / training / quarantine` |
| time | `day 004 / 12:10` |

Visual marker:

```text
ivy03 moved → B3
```

Design rule: movement entries are why organism identity must be separate from bay identity.

### 7. Growth-stage entry

Purpose: record developmental progress.

Best targets:

- organism
- seedling batch
- specimen

Possible stages:

```text
seeded
germinated
seedling
vegetative
climbing
flowering
fruiting
senescing
dormant
archived
dead / removed
```

Examples:

```text
stage: tray-A seedlings emerged
stage: ivy03 began climbing trellis
stage: basil02 flowering
stage: fern011 archived specimen
```

Fields:

| Field | Example |
|---|---|
| organism | `ivy03` |
| stage | `climbing` |
| previous stage | `vegetative` |
| confidence | optional |
| source | manual / inferred |

Visual marker:

```text
ivy03 stage+ climbing
```

Design rule: growth stages should be broad and readable at first, not botanically over-specified.

### 8. Inspection / scouting entry

Purpose: record a structured check of plant health, pests, disease, or environment.

Best targets:

- lab
- bay
- organism
- sticky card / inspection card
- specimen

Examples:

```text
scout: Grow Bay checked, no pests observed
scout: B2 lower leaves inspected
scout: sticky card VINES-C2 watch state
scout: fungus gnat suspected near propagation tray
```

Fields:

| Field | Example |
|---|---|
| route | `Grow Bay snake pass` |
| target | `GROW/B1-B3` |
| finding | `clear` / `watch` / `warning` |
| organism | optional |
| card | optional |
| followup | optional |

Visual marker:

```text
Grow Bay scout ok
```

Design rule: scouting entries should support systematic checks without requiring a full pest simulation yet.

### 9. Snapshot / frame reference entry

Purpose: prepare for future image / render snapshot references.

Best targets:

- organism
- bay
- lab
- greenhouse

Examples:

```text
snapshot: ivy03 day 003 frame
snapshot: Grow Bay morning state
snapshot: B2 before watering
```

Fields:

| Field | Example |
|---|---|
| target | `ivy03` |
| frame | `render-frame-0031` |
| mode | `atmospheric` / `debug` |
| reason | `growth comparison` |

Visual marker:

```text
ivy03 snap
```

Design rule: snapshot entries are optional future hooks. Do not require image capture for the first journal prototype.

### 10. Freeform note entry

Purpose: allow user-authored notes that do not fit other categories.

Best targets:

- organism
- lab
- bay
- support
- specimen

Examples:

```text
note: likes higher light than expected
note: keep B2 under observation
note: specimen label uncertain
```

Fields:

| Field | Example |
|---|---|
| target | `basil02` |
| text | `watch lower leaves` |
| tags | `watch`, `light` |
| severity | `info` / `watch` |

Visual marker:

```text
basil02 note
```

Design rule: freeform notes are valuable but should not replace structured event types where structure is obvious.

---

## Journal entry severity vocabulary

| Severity | Meaning | Marker |
|---|---|---|
| info | neutral event | `log` |
| observation | normal observation | `obs` |
| care | care action completed | `care` |
| watch | monitor condition | `?` |
| warning | attention needed | `!` |
| fault | equipment / system failure | `x` |
| resolved | issue handled | `ok` |
| archived | historical / inactive | `arc` |

Design rule: severity should align with the marker vocabulary from Notes 07–11.

---

## Journal display modes

### 1. Atmospheric mode

Only tiny journal hints.

```text
[B1 ivy03]   [B2 basil02!]   [B3 empty]
```

Rules:

- do not show full entries
- show `!`, `?`, or `log+` only when useful
- preserve scene readability

### 2. Readable mode

Show short recent summaries.

```text
Grow Bay | log+2
B2 basil02 dry! — moisture low at 08:21
ivy03 — new leaf observed today
```

Rules:

- one-line summaries
- recent-first or target-selected
- no large scrolling panel unless explicitly opened

### 3. Journal mode

Show timeline / record view for selected target.

```text
Journal: basil02
day 003 08:21  warning  moisture low at B2
day 003 08:37  care     watered manually
day 003 08:55  resolved moisture ok
```

Rules:

- target-specific timeline
- structured entries
- filterable by type later

### 4. Debug mode

Show internal IDs and event links.

```text
GH-01/GROW/B2/basil02
entry e-00042 resolves e-00041
source: sensor/moisture-B2
```

Rules:

- useful for development
- not suitable for idle visualizer mode

---

## Minimal first journal prototype

For the first Grow Bay prototype, keep the journal extremely small.

Required:

```text
1. organism record for each occupied bay
2. one recent-entry marker per organism or bay
3. manual observation entry type
4. care-action entry type: watered / pruned / inspected
5. warning entry type: dry / wet / low light / fault
6. simple resolved state
7. selected-object timeline stub
```

Do not start with full pest tracking, full growth models, or chemistry logs.

Minimal example:

```text
╔═ YAM GREENHOUSE ═════════════════════════════ 08:21:46  OK ═╗
║ [Propagation] [Climate] [Grow Bay*] [Vines] [Utility] [Archive] ║
╠═══════════════════════════════════════════════════════════════════╣
║ ┌─ Grow Bay Lab ───────────────────────── OK  log+1 ──────────┐ ║
║ │ temp ○  hum ○      L:ok      fan ◉  vent ▦                  │ ║
║ │ tools: ✂ tag        H₂O ok                                  │ ║
║ │   [B1 ivy03 ok]     [B2 basil02 dry! log]     [B3 empty]    │ ║
║ │                                                             │ ║
║ │ selected: B2 / basil02                                     │ ║
║ │ latest: moisture low observed at 08:21                     │ ║
║ └─────────────────────────────────────────────────────────────┘ ║
╚═══════════════════════════════════════════════════════════════════╝
```

This gives the journal a visible surface without making the greenhouse scene into a text database.

---

## Candidate internal record shape

This is not an implementation mandate, but it is useful for Codex-ingestable planning.

```text
PlantOrganismRecord
- organism_id
- display_name
- species_or_type_optional
- status
- current_lab_id
- current_bay_id
- current_support_id
- created_at
- archived_at_optional
- journal_entries[]
```

```text
JournalEntry
- entry_id
- target_type
- target_id
- timestamp
- entry_type
- severity
- source
- location_lab_id
- location_bay_id_optional
- support_id_optional
- summary
- details_optional
- state_before_optional
- state_after_optional
- resolves_entry_id_optional
- tags[]
```

Design rule: entry schema should support sparse data. Do not require details that early prototypes do not have.

---

## Journal entry type vocabulary

```text
observation
care_action
sensor_event
warning
resolution
movement
transplant
growth_stage
inspection
snapshot
freeform_note
maintenance
archive
```

This vocabulary should remain small at first. More specialized types can be added later only when needed.

---

## Event-source vocabulary

```text
manual
sensor
schedule
system
tool
warning_state
imported
inferred
```

Examples:

```text
manual: user records new leaf
sensor: moisture low threshold crossed
schedule: daily inspection due
system: plant moved between bays
warning_state: fan fault generated incident
inferred: repeated low-light readings suggest shade issue
```

Design rule: always distinguish what was directly observed from what was inferred.

---

## Relationship to visual markers from Note 11

### `log`

Means record exists.

```text
ivy03 log
```

### `log+`

Means new or recent entry exists.

```text
basil02 log+
```

### `obs!`

Means observation is due or watch condition exists.

```text
jar07 obs!
```

### `arc`

Means archived record.

```text
fern011 arc
```

### `! log`

Means active warning has a linked journal entry.

```text
B2 dry! log
```

Design rule: journal markers should summarize journal state, not display journal content directly.

---

## Relationship to sensors and automation

The journal should not record every sensor frame.

Good journal-worthy sensor events:

- threshold crossed
- warning opened
- warning resolved
- repeated abnormal condition
- daily summary sampled
- sensor fault
- calibration event

Bad journal events:

- every tick of temperature
- every tiny humidity fluctuation
- repeated unchanged warning spam
- duplicated status messages

Suggested rule:

```text
Log state changes, not raw noise.
```

---

## Relationship to care actions

Care verbs from Note 10 should become journal event types or action tags.

```text
water     → care_action: watered
prune     → care_action: pruned
inspect   → inspection: checked
clean     → maintenance: cleaned
repair    → maintenance: repaired
calibrate → maintenance: calibrated
transplant→ movement / transplant
label     → metadata update
```

Design rule: if a visible tool implies an action, the journal should eventually be able to record that action.

---

## Relationship to scouting / inspection

YAM does not need a full pest / disease system in the first version, but the journal should prepare for it.

Useful future scouting records:

- lab inspected
- bay inspected
- sticky card checked
- suspicious pest observed
- disease symptom observed
- no issue found
- follow-up needed
- issue resolved

Possible compact forms:

```text
scout ok
watch B2
sticky C2 ?
pest! log
```

Design rule: scouting should be systematic and recordable, but not visually dominant in the first greenhouse prototype.

---

## Terminal readability rules

### Keep journal markers tiny in the scene

The scene should show that a record exists, not the whole record.

### Show full timeline only when selected

Journal mode should be target-specific.

### Prefer recent summaries

Readable mode can show the latest one or two entries only.

### Avoid notification spam

Repeated sensor events should aggregate or update one open warning.

### Separate organism history from location history

A plant’s journal follows the plant. A bay’s journal records the place.

### Preserve the visual greenhouse first

The journal supports the greenhouse. It should not replace the greenhouse view.

---

## Open design questions

1. Should every rendered plant organism require a journal record from the beginning?
2. Should empty bays have their own journal history?
3. Should sensor warnings automatically create journal entries, or only when acknowledged?
4. Should care actions be manual-only at first?
5. Should the journal support freeform user notes in the first implementation?
6. Should journal mode be a side panel, modal overlay, bottom drawer, or separate lab view?
7. Should organism IDs be human names, generated IDs, or both?
8. Should growth-stage entries be manual at first, inferred later, or both?
9. Should archived / dead / removed organisms remain visible in Archive Lab?
10. Should daily summaries exist, or only event-based entries?
11. Should records be persisted in a dedicated greenhouse state file separate from visual config?
12. Should journal entries be exportable as markdown later?

---

## Working conclusion

The plant organism logging / journaling system should make YAM’s greenhouse feel alive over time.

The essential journal grammar is:

```text
stable identity
└── timestamped event
    └── typed entry
        └── short scene marker
            └── optional selected-target timeline
```

For the first greenhouse prototype, the journal should be minimal:

- organism records for occupied bays
- tiny `log` / `log+` markers
- manual observations
- care actions
- warning events
- simple resolution entries
- selected-object latest-entry preview

This creates continuity and memory without overwhelming the terminal scene.
