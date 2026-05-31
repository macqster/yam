
# Greenhouse Ecosystem Design Notes 11

## Navigation / Identity Markers

### Purpose of this note

Notes 04–10 defined the greenhouse shell, plant supports, lighting, sensors, climate actuators, water / nutrient infrastructure, and maintenance props.

Note 11 defines **navigation / identity markers**: the visual language that tells the user where they are, what they are looking at, what is selected, what is warning, and how objects are named or indexed.

This note is written with **Note 12 — Plant Organism Logging / Journaling System** in mind.

This note answers:

```text
How does the greenhouse visibly identify labs, bays, plant supports, organisms, specimens, warnings, selections, and future journal links?
```

Identity markers are the bridge between visual greenhouse objects and future plant records.

---

## Why identity markers come before plant journaling

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

This order matters.

A journaling system needs stable identities to attach records to:

- greenhouse
- lab
- bay / station
- plant support
- plant organism
- specimen
- event
- warning
- observation
- care action

If visual identity is not designed first, the journal has nowhere clean to point.

Real greenhouse and horticultural recordkeeping practices commonly depend on reliable location, crop, date, activity, and observation records. Greenhouse IPM scouting guidance also emphasizes maps, logical units, regular scouting patterns, sticky cards, flags, labels, and records so crop information can be communicated consistently. For YAM, this translates into a visual requirement: every meaningful thing that can be logged later should have a stable visible identity now.

Design implication:

```text
visible marker → stable object identity → journal record target
```

---

## Core identity-marker model

Every navigation / identity marker should answer eight questions:

1. **Target** — What does this marker identify?
2. **Scope** — Greenhouse, lab, bay, object, plant, specimen, warning, or journal entry?
3. **Persistence** — Is the marker permanent, session-local, temporary, or event-based?
4. **Placement** — Frame, tab strip, lab plaque, bay tag, pot tag, wall sign, shelf label, object badge, or journal link?
5. **State** — active, inactive, selected, warning, archived, unknown, locked, future, or hidden?
6. **Density** — short symbol, short label, numeric ID, full name, or debug identifier?
7. **Interaction** — navigates, selects, opens journal, filters, warns, or only labels?
8. **Journal readiness** — Can Note 12 attach a log entry to this marker cleanly?

Design rule: identity markers should be compact, stable, and reusable. They should never become uncontrolled text clutter.

---

## Identity hierarchy

YAM should use a clear identity hierarchy:

```text
Greenhouse
└── Lab
    └── Bay / station / shelf / wall zone
        └── Support object
            └── Plant organism / specimen
                └── Observation / event / journal entry
```

Example IDs:

```text
GH-01
GH-01/GROW
GH-01/GROW/B2
GH-01/GROW/B2/POT
GH-01/GROW/B2/IVY-003
GH-01/GROW/B2/IVY-003/LOG-2026-05-31
```

The user-facing display does not need to show long IDs. Long IDs are for internal identity and future journal anchoring.

Visible display can be compact:

```text
Grow Bay
B2
[p2 dry!]
Ivy-003
log +1
```

---

## Primary marker families

### 1. Greenhouse title / global identity marker

Purpose: identify the persistent greenhouse frame.

Best placement:

- top border
- title bar
- status frame
- optional footer

Support logic:

- persists across lab switches
- establishes the greenhouse as the stable world container
- can carry global mode, clock, and alert summary

Terminal forms:

```text
╔═ YAM GREENHOUSE ═════════════════════════════ 08:21:46 ═╗
```

```text
YAM GREENHOUSE  |  day 003  |  OK
```

```text
GH-01  [labs]  [journal]
```

State variants:

| State | Visual cue |
|---|---|
| Normal | stable title |
| Warning | global `!` marker |
| Maintenance | `m` / tool marker |
| Journal activity | `log +1` |
| Debug | visible full ID |

Design rule: the global marker should remain stable and should not be visually replaced by lab-local labels.

### 2. Lab tab marker

Purpose: navigate between lab sub-worlds.

Best placement:

- greenhouse frame tab strip
- top row under title
- optional compact mode footer

Support logic:

- tabs are navigation, not props
- active tab shows current lab
- inactive tabs show available lab destinations
- warning markers can indicate offscreen lab issues

Terminal forms:

```text
[Propagation] [Climate] [Grow Bay] [Vines] [Utility] [Archive]
```

```text
LABS: 1 Prop  2 Climate  3 Grow*  4 Vines!  5 Utility
```

```text
╭ Grow Bay ╮╭ Vines ! ╮╭ Utility ╮
╰──────────┴──────────┴──────────╯
```

State variants:

| State | Visual cue |
|---|---|
| Active | brackets, underline, highlight, `*` |
| Inactive | plain / dim label |
| Warning | `!` on tab |
| Journal update | `+` / `log` marker |
| Locked / future | faint label / `…` |
| Hidden | omitted in compact mode |

Design rule: lab tabs belong to the greenhouse frame, not the lab world.

### 3. Lab plaque / local title marker

Purpose: identify the active lab inside the viewport.

Best placement:

- inner lab frame title
- top-left of lab shell
- local status corner

Support logic:

- helps the active lab feel like a named room
- can carry local status
- can later link to lab-level journal entries

Terminal forms:

```text
┌─ Grow Bay Lab ───────────────────────────── OK ─┐
```

```text
[ Grow Bay Lab | stable | log +1 ]
```

```text
GROW BAY  BAYS: 3  STATE: OK
```

State variants:

| State | Visual cue |
|---|---|
| Normal | lab name + OK |
| Warning | lab name + `!` |
| Selected | stronger plaque / underline |
| Journal update | `log +1` |
| Archived | dim / archive marker |

Design rule: the lab plaque is the best place for lab-level status, not plant-level details.

### 4. Bay / station marker

Purpose: identify repeated plant stations.

Best placement:

- near pot / hydro bucket / support object
- under lamp cone
- on bay divider
- as compact tag row

Support logic:

- central to HighGrow-like 1–3 plant layout
- creates stable attachment targets for sensors, water, lighting, and journal records
- can carry short status such as `ok`, `dry!`, or `log`

Terminal forms:

```text
[B1]  [B2]  [B3]
```

```text
[p1 ok]   [p2 dry!]   [p3 ok]
```

```text
╭─ B2 ─╮
│ dry! │
╰──────╯
```

State variants:

| State | Visual cue |
|---|---|
| Empty | `B2 empty` |
| Occupied | `B2 plant` / plant short name |
| Warning | `B2!` |
| Selected | brackets / caret / underline |
| Journal update | `B2 log+` |
| Unknown | `B2 ?` |

Design rule: bay markers should be stable even if the plant changes. The bay is a location; the plant is an organism.

### 5. Plant-support marker

Purpose: identify pots, trays, shelves, hydro buckets, jars, and trellis zones.

Best placement:

- attached to the support object
- short label below / beside support
- rack / shelf label
- jar / case placard

Support logic:

- support identity is distinct from organism identity
- support marker can hold state: empty, occupied, wet, cracked, low water, disconnected
- supports future journal entries about containers or infrastructure, not just plants

Terminal forms:

```text
pot-02
```

```text
[tray A]
```

```text
jar 07
```

```text
hydro B2
```

State variants:

| State | Visual cue |
|---|---|
| Empty | `empty` |
| Occupied | plant short name or occupied marker |
| Wet / dry | compact moisture state |
| Fault | `!` / `x` |
| Selected | bracket / underline |
| Archived | index marker |

Design rule: support labels should not be confused with plant organism labels. This distinction is important for Note 12.

### 6. Plant organism marker

Purpose: identify the living individual or colony being tracked.

Best placement:

- attached to pot tag
- beside bay tag
- in selected-object panel
- on specimen card
- hidden in atmospheric mode if too dense

Support logic:

- plant organism identity is persistent across movement between supports
- the journal should attach primarily to organism identity, not just position
- plant marker can be short in visual mode and expanded in journal mode

Terminal forms:

```text
Ivy-003
```

```text
[ivy03 ok]
```

```text
P: ivy03
```

```text
specimen: FERN-011
```

State variants:

| State | Visual cue |
|---|---|
| Healthy | `ok` |
| Watch | `?` |
| Warning | `!` |
| New log | `log+` |
| Moved | arrow / new bay marker |
| Archived / dead | dim / archive marker |

Design rule: organism marker should be stable across time. If the plant moves from B1 to B3, the organism ID should remain the same.

### 7. Specimen / archive marker

Purpose: identify archived, contained, or observational organisms / materials.

Best placement:

- specimen jar
- terrarium case
- archive shelf
- specimen plaque

Support logic:

- Archive / Specimen Lab needs quieter, more museum-like identity markers
- markers can use catalog IDs rather than friendly plant names
- future journal entries may include observations, provenance, preservation state, or experiment notes

Terminal forms:

```text
jar 07
```

```text
case A-12
```

```text
spec: F-011
```

```text
[ARCH 04]
```

State variants:

| State | Visual cue |
|---|---|
| Cataloged | index label |
| Unknown | `?` |
| Observation due | `obs!` |
| Sealed | lock / cap marker |
| Archived | dim label |
| New note | `log+` |

Design rule: specimen markers should feel indexed and calm, not alarm-heavy unless there is a real issue.

### 8. Warning / alert badge

Purpose: compactly mark abnormal state.

Best placement:

- lab tab
- lab plaque
- bay tag
- sensor panel
- actuator object
- plant-support object
- journal marker

Support logic:

- warnings must be attached to a target
- warning hierarchy matters: global, lab, bay, object, organism
- should reuse existing sensor / actuator / water marker vocabulary

Terminal forms:

```text
!
```

```text
[!]
```

```text
B2 dry!
```

```text
Vines!
```

State variants:

| State | Visual cue |
|---|---|
| Watch | `?` |
| Warning | `!` |
| Fault | `x` |
| Maintenance | `m` |
| Resolved | `ok` |
| Journaled | `log` / check marker |

Design rule: warnings should never float. A warning badge should always answer: warning on what?

### 9. Selection marker

Purpose: show current focus or interaction target.

Best placement:

- around object
- before label
- under label
- in status corner
- optionally in selected-object panel later

Support logic:

- critical for future navigation / inspection / journaling
- can mark selected lab, bay, support, plant, sensor, actuator, or journal entry
- should be visually distinct from warning

Terminal forms:

```text
> B2
```

```text
[B2]
```

```text
╭─ selected ─╮
```

```text
B2▸ ivy03
```

State variants:

| State | Visual cue |
|---|---|
| Hover / focus | caret / pointer |
| Selected | bracket / stronger outline |
| Multi-selected | plus / count marker |
| Inspecting | magnifier / `i` marker |
| Journal open | `J` marker |

Design rule: selection should be a navigation state, not a health state.

### 10. Journal-link marker

Purpose: prepare the visual system for Note 12.

Best placement:

- plant organism tag
- bay tag
- lab plaque
- selected-object marker
- archive specimen label
- status corner

Support logic:

- indicates that an object has records or new observations
- can show count of new notes or recent events
- should not open the journal visually yet; Note 12 defines the journal system

Terminal forms:

```text
log
```

```text
log+1
```

```text
J
```

```text
ivy03 ✎
```

State variants:

| State | Visual cue |
|---|---|
| Has journal | `J` / `log` |
| New entry | `log+` |
| Observation due | `obs!` |
| Archived record | `arc` |
| No record | no marker |

Design rule: journal markers should be small. The visualizer should show that a record exists, not display the record inline.

### 11. Unknown / placeholder marker

Purpose: mark incomplete, unnamed, future, or unclassified objects.

Best placement:

- future labs
- empty supports
- unidentified plants
- uncalibrated sensors
- unknown specimens

Support logic:

- useful during development and future gameplay states
- helps avoid fake precision
- can also indicate TODO / implementation placeholder

Terminal forms:

```text
?
```

```text
[unknown]
```

```text
B3 empty
```

```text
spec ?
```

State variants:

| State | Visual cue |
|---|---|
| Unknown | `?` |
| Empty | `empty` |
| Future | `…` |
| Locked | lock / dim marker |
| Unnamed | `new` |

Design rule: unknown markers are preferable to misleading labels.

---

## Marker mapping by lab

| Lab | Identity emphasis |
|---|---|
| Propagation Lab | tray IDs, cell groups, seedling batches, humidity-dome labels, sowing-log markers |
| Climate Lab | control-panel labels, sensor IDs, actuator status badges, experiment labels, warning hierarchy |
| Grow Bay Lab | B1 / B2 / B3 station markers, pot/support tags, organism names, warning badges, journal links |
| Vines Lab | trellis zone markers, vine organism IDs, pruning tags, wall-sector labels, inspection cards |
| Utility Lab | tank IDs, pump / valve labels, spare-part bins, calibration labels, maintenance status markers |
| Archive / Specimen Lab | specimen IDs, jar / case labels, catalog plaques, archived-log markers, observation-due badges |

---

## Marker density modes

YAM should eventually support at least three density modes.

### 1. Atmospheric mode

Minimal identity, mostly visual scene.

```text
Grow Bay     OK
[p1] [p2!] [p3]
```

Use for idle visualizer mode.

### 2. Readable mode

More labels and state markers.

```text
Grow Bay Lab | OK | log+1
[B1 ivy03 ok] [B2 basil02 dry!] [B3 empty]
```

Use for normal inspection mode.

### 3. Debug / journal mode

Fuller IDs and record hooks.

```text
GH-01/GROW/B2/POT -> organism BASIL-002 -> logs: 14, latest: dry substrate
```

Use for development, diagnostics, and Note 12 journal integration.

Design rule: the same object identity should exist in all modes; only the display density changes.

---

## Minimal Grow Bay prototype identity set

For the first Grow Bay Lab prototype, include:

```text
1. greenhouse title / clock / global status
2. lab tab strip with active Grow Bay marker
3. local Grow Bay plaque
4. B1 / B2 / B3 bay markers
5. compact pot / plant status tags
6. warning badge support
7. optional journal-link marker placeholder: log / log+1
```

Prototype sketch:

```text
╔═ YAM GREENHOUSE ═════════════════════════════ 08:21:46  OK ═╗
║ [Propagation] [Climate] [Grow Bay*] [Vines] [Utility] [Archive] ║
╠═══════════════════════════════════════════════════════════════════╣
║ ┌─ Grow Bay Lab ───────────────────────── OK  log+1 ──────────┐ ║
║ │ temp ○  hum ○      L:ok      fan ◉  vent ▦                  │ ║
║ │ tools: ✂ tag        H₂O ok                                  │ ║
║ │ ╞══╪══╪══╪══╪══╪══╪══╪══╪══╡ back wall                    │ ║
║ │ ━━━━━━━━━━━━━━━━━ lamp rail ━━━━━━━━━━━━━━━━━              │ ║
║ │        drip ──────┬─────────┬─────────┬──                  │ ║
║ │                 · │       · │       · │                    │ ║
║ │   [B1 ivy03 ok]  ╭──╮ [B2 basil02 dry!] ╭──╮ [B3 empty]    │ ║
║ │                 │░░│      │░░│          │░░│               │ ║
║ │                 ╰──╯      ╰──╯          ╰──╯               │ ║
║ │ ┄┄┄┄┄┄┄ drain / runoff channel ┄┄┄┄┄┄┄┄┄                 │ ║
║ └─────────────────────────────────────────────────────────────┘ ║
╚═══════════════════════════════════════════════════════════════════╝
```

This sketch is intentionally dense to show all identity hooks. The real idle view should probably use a lighter density.

---

## Journal-readiness rules for Note 12

Note 11 should leave the system ready for journaling without defining the journal itself.

### 1. Bay identity must be stable

A bay is a location.

```text
B2 remains B2 even if the plant changes.
```

### 2. Organism identity must move with the plant

A plant organism is a living record target.

```text
ivy03 can move from B1 to B3 and keep the same journal.
```

### 3. Support identity is separate from organism identity

A pot can be replaced or cleaned without deleting the plant’s journal.

```text
B2 / pot-04 / basil02 are related, but not the same identity.
```

### 4. Journal markers should signal records, not display records

```text
ivy03 log+1
```

means a journal entry exists or has changed. The entry itself belongs in Note 12.

### 5. Warnings should be loggable events later

A warning badge should be convertible into a journal event:

```text
B2 dry! → journal event: moisture low observed at timestamp
```

### 6. Care actions should become journal events later

Maintenance objects from Note 10 imply future verbs:

```text
water → watered event
prune → pruning event
inspect → observation event
calibrate → calibration event
repair → maintenance event
```

Note 12 should build on this.

---

## Identity marker state vocabulary

### Basic states

| State | Meaning |
|---|---|
| Active | currently visible or selected lab / object |
| Inactive | available but not selected |
| Selected | current interaction target |
| Empty | valid location with no organism / object |
| Occupied | support or bay contains an organism / specimen |
| Warning | attention needed |
| Fault | broken / failed system |
| Unknown | not identified yet |
| New | newly added item or organism |
| Logged | record exists |
| New log | record has new entry |
| Archived | historical / inactive record |
| Future | planned but not implemented |

### Compact markers

```text
*      active
>      focus / selected
!      warning
x      fault
?      unknown
+      new / added
J      journal exists
log+   new journal entry
arc    archived
…      future / not implemented
```

Design rule: markers should be shared across labs. A warning in Grow Bay and a warning in Utility should not use unrelated visual language.

---

## Placement rules

### Put navigation in the frame

```text
[Propagation] [Climate] [Grow Bay*] [Vines]
```

### Put lab identity in the lab plaque

```text
┌─ Grow Bay Lab ───────── OK ─┐
```

### Put station identity near stations

```text
[B1]     [B2]     [B3]
```

### Put organism identity on plant tags or selected panels

```text
[B2 basil02 dry!]
```

### Put journal markers beside identity, not in the middle of scene art

```text
ivy03 log+1
```

### Put warnings on their targets

Bad:

```text
WARNING!
```

Better:

```text
[B2 dry!]
```

---

## Terminal readability rules

### Prefer stable short names

Use `B1`, `B2`, `B3`, `ivy03`, `jar07`, `pumpA` rather than long prose.

### Avoid label saturation

Labels should clarify the scene, not cover the scene.

### Use density modes

Do not force debug IDs into atmospheric idle view.

### Separate health, identity, and selection

- identity: `ivy03`
- health/state: `dry!`
- selection: `>` or brackets
- journal: `log+`

### Keep IDs machine-friendly but display user-friendly

Internal identity can be hierarchical and exact. Display identity can be short.

---

## Relationship to Note 12 — Plant Organism Logging / Journaling System

Note 12 should define the actual record system that attaches to the identities established here.

Expected Note 12 topics:

- plant organism record model
- journal entry types
- observation events
- care-action events
- sensor-derived events
- warning / incident events
- movement / transplant events
- growth-stage entries
- photo / frame snapshot references, if later applicable
- per-organism timeline
- lab / bay / support cross-references
- atmospheric vs readable journal display
- minimal idle-view journal indicators

Note 11 should not yet define full journal storage or entry schema. It should only ensure the visual world has stable targets for those records.

---

## Open design questions

1. Should plant organism IDs be user-facing names, internal numeric IDs, or both?
2. Should the first Grow Bay prototype show plant names immediately or only bay labels?
3. Should journal markers appear in atmospheric mode, or only readable / debug modes?
4. Should lab tabs show offscreen warnings and journal updates?
5. Should selected-object identity appear inside the scene, in a side panel, or in the frame footer?
6. Should support identity be visible by default, or only when selected?
7. Should `B1 / B2 / B3` be global per lab or unique across the greenhouse?
8. Should warnings automatically become journal events in Note 12?
9. Should Archive / Specimen Lab use catalog-style IDs distinct from Grow Bay plant names?
10. Should unknown organisms be allowed, or should every living object require a record before rendering?

---

## Working conclusion

Navigation / identity markers should make the greenhouse legible and prepare it for organism journaling.

The essential identity grammar is:

```text
place
└── support
    └── organism / specimen
        └── state marker
            └── optional journal link
```

For the first greenhouse prototype, use a minimal identity layer:

- greenhouse title and global status
- lab tabs with active marker
- local lab plaque
- B1 / B2 / B3 station markers
- compact plant / support status tags
- target-attached warning badges
- small placeholder journal marker such as `log` or `log+1`

This gives Note 12 a clean foundation: plant organism logs can attach to stable identities instead of floating text or ambiguous scene objects.
