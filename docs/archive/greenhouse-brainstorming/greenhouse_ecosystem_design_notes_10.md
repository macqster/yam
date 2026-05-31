
# Greenhouse Ecosystem Design Notes 10

## Work / Maintenance Props and Tools

### Purpose of this note

Notes 04–09 defined the greenhouse shell, plant supports, lighting, sensors, climate actuators, and water / nutrient infrastructure.

Note 10 defines **work / maintenance props and tools**: the visible in-world objects that show the greenhouse is a tended, inspected, cleaned, repaired, and adjusted working environment.

This note answers:

```text
What objects make the greenhouse feel actively maintained rather than merely displayed?
```

These elements should not be random decoration. They should imply care, inspection, cleaning, repair, calibration, watering, pruning, labeling, and routine operation.

---

## Why maintenance props come next

The current design sequence is:

```text
04 Boundary / shell / lab frame
05 Plant-support elements
06 Lighting elements
07 Sensor and gauge elements
08 Climate-control actuators
09 Water / nutrient infrastructure
10 Work / maintenance props and tools
```

After the greenhouse has functional systems, the next layer should show the human / operator side of the system:

- tools used to care for plants
- supplies used to maintain water and nutrient systems
- cleaning objects used to keep the greenhouse healthy
- labels and notebooks used to track experiments
- spare parts used to repair infrastructure
- calibration supplies used to trust readings

Real greenhouse maintenance guidance commonly emphasizes routine inspection of frames, glazing, ventilation, heating, irrigation, benches, floors, and walkways. Cleaning guidance also emphasizes removing debris and organic matter, washing or sanitizing surfaces, clearing algae or grime, and maintaining light transmission. For YAM, these practices translate into visible working objects: brushes, buckets, hose coils, filter cartridges, spare tubing, tags, notebooks, and tool racks.

Design implication:

```text
functional system → maintenance object → implied human care loop
```

---

## Core maintenance-prop model

Every maintenance prop should answer seven questions:

1. **Task implied** — watering, pruning, cleaning, labeling, measuring, calibrating, repairing, inspecting, or storing?
2. **Lab fit** — which lab naturally contains it?
3. **Attachment / placement** — shelf, rack, hook, bench, floor, bucket, wall, tray, or panel?
4. **State** — clean, used, empty, full, missing, dirty, broken, selected, warning, or stored?
5. **Relationship** — which functional system does it support?
6. **Clutter risk** — is it useful information or just noise?
7. **Interactivity potential** — is it decorative, selectable, or later actionable?

Design rule: maintenance props should be sparse and purposeful. One well-placed tool rack is better than scattered visual clutter.

---

## Primary maintenance prop families

### 1. Watering can / hand watering tool

Purpose: simple manual watering cue.

Best labs:

- Propagation Lab
- Grow Bay Lab
- Vines Lab
- Utility Lab, stored variant

Support logic:

- implies manual override or low-tech care
- useful before complex irrigation simulation exists
- can sit on bench, floor, shelf, or hook
- pairs with dry pot / tray state

Terminal forms:

```text
[can]
```

```text
╭───╮__
│H₂O│  )
╰───╯
```

```text
can · · ·
```

State variants:

| State | Visual cue |
|---|---|
| Full | `H₂O` or fill mark |
| Empty | blank / low marker |
| In use | small drip marks |
| Stored | on shelf / hook |
| Missing | placeholder / `?` |

Design rule: the watering can is a friendly fallback object, but it should not replace the visible irrigation infrastructure from Note 09.

### 2. Hose coil

Purpose: manual water line / utility-room maintenance cue.

Best labs:

- Utility Lab
- Grow Bay Lab, edge / background variant
- Climate Lab, service area

Support logic:

- implies manual washdown, filling, or emergency watering
- pairs with water tank, pump, floor drain, and bucket
- can be wall-hung or floor-coiled

Terminal forms:

```text
@ hose
```

```text
(@@@)
```

```text
wall hook ┐
          @
```

State variants:

| State | Visual cue |
|---|---|
| Stored | coil on hook |
| In use | line extends from coil |
| Leaking | drip marker |
| Tangled | irregular coil |
| Missing nozzle | broken line end |

Design rule: hose coils belong mostly in Utility contexts. Avoid making every lab look like a storage room.

### 3. Bucket / catch pail

Purpose: water collection, cleaning, leak response, or general work container.

Best labs:

- Utility Lab
- Grow Bay Lab
- Propagation Lab
- Climate Lab

Support logic:

- simple floor object
- pairs with leak / overflow / cleaning state
- can be empty, filled, dirty, or catching drips

Terminal forms:

```text
[bucket]
```

```text
╭──╮
│~~│
╰──╯
```

```text
 drip ·
     ╭──╮
     │~~│
     ╰──╯
```

State variants:

| State | Visual cue |
|---|---|
| Empty | blank bucket |
| Full | water line |
| Dirty | stipple / residue |
| Catching leak | drip above bucket |
| Overflowing | `!` |

Design rule: buckets are useful state props because they explain leaks and maintenance without adding UI text.

### 4. Pruners / scissors / cutters

Purpose: plant care, pruning, trimming, harvesting, or vine maintenance cue.

Best labs:

- Vines Lab
- Grow Bay Lab
- Propagation Lab
- Utility Lab, stored tool rack

Support logic:

- should usually be on a bench, hook, tray, or tool rack
- connects strongly to plant growth management
- especially important for Vines Lab

Terminal forms:

```text
✂
```

```text
[cut]
```

```text
tool rack: ✂  ⌁  tag
```

State variants:

| State | Visual cue |
|---|---|
| Stored | on rack / hook |
| In use | near plant / selected marker |
| Dirty | small residue marker |
| Missing | empty rack slot |
| Maintenance | sharpen / repair marker |

Design rule: pruning tools are a high-value cue for active growth management, but should not be scattered randomly.

### 5. Plant labels / tags / stakes

Purpose: identity, bay mapping, specimen tracking, and experiment notation.

Best labs:

- all labs
- strongest in Propagation, Grow Bay, Archive, Vines

Support logic:

- attaches to pot, tray, shelf, specimen jar, trellis, or bay divider
- bridges visual world and plant identity
- can carry state or metadata compactly

Terminal forms:

```text
[tag]
```

```text
╿ P1
```

```text
[p2 dry!]
```

```text
jar 07
```

State variants:

| State | Visual cue |
|---|---|
| Named | short label |
| Unknown | `?` |
| Warning | label plus `!` |
| Selected | bracket / underline |
| Archived | small index number |
| Faded | dim marker |

Design rule: labels are both maintenance props and navigation / identity markers. Keep them compact.

### 6. Notebook / clipboard / log sheet

Purpose: human-readable inspection, schedule, or experiment record cue.

Best labs:

- Utility Lab
- Climate Lab
- Archive / Specimen Lab
- Propagation Lab

Support logic:

- belongs on bench, shelf, wall clipboard, or control panel
- suggests routine observation and data collection
- can connect to tasks, schedules, or plant history later

Terminal forms:

```text
[log]
```

```text
┌────┐
│note│
└────┘
```

```text
clip: T/H/L ok
```

State variants:

| State | Visual cue |
|---|---|
| Current | visible label / date marker |
| Old | folded / archived marker |
| Warning | `todo!` |
| Selected | bracket / highlight |
| Missing | empty clipboard |

Design rule: log objects are useful for atmosphere and future interactions, but should not become full text panels in idle view.

### 7. Tool rack / wall hooks

Purpose: organize tools and reduce clutter.

Best labs:

- Utility Lab
- Grow Bay Lab, small side rack
- Propagation Lab
- Vines Lab

Support logic:

- wall-mounted or shelf-mounted
- hosts scissors, tags, hose nozzle, brush, gloves, spare clips
- makes props feel stored instead of scattered

Terminal forms:

```text
╞══ tool rack ══╡
  ✂  tag  brush
```

```text
hooks: ┐ ┐ ┐
       ✂ tag hose
```

State variants:

| State | Visual cue |
|---|---|
| Organized | evenly spaced tools |
| Missing tool | empty hook |
| Crowded | compressed tools |
| Maintenance | `!` or wrench marker |

Design rule: tool racks should be the default home for maintenance props. This keeps labs readable.

### 8. Gloves / apron / towel

Purpose: soft human-care cue, cleaning cue, or wet-work cue.

Best labs:

- Utility Lab
- Propagation Lab
- Grow Bay Lab
- Climate Lab

Support logic:

- small object, best on hook / shelf / bench
- can imply recent work without adding machinery
- useful for softening technical labs

Terminal forms:

```text
[glv]
```

```text
towel ~
```

```text
hook: glove
```

State variants:

| State | Visual cue |
|---|---|
| Clean | simple label |
| Wet | drip mark |
| Dirty | stipple |
| Hanging | on hook |
| Missing | empty hook |

Design rule: soft props should be rare, quiet accents.

### 9. Brush / broom / scraper / cleaning kit

Purpose: greenhouse sanitation and light-transmission maintenance cue.

Best labs:

- Utility Lab
- Propagation Lab
- greenhouse frame edge
- Grow Bay Lab, small stored variant

Support logic:

- connects to cleaning glazing, benches, floors, algae, dirt, and debris
- should be stored on rack or leaned against service wall
- can support seasonal maintenance state later

Terminal forms:

```text
brush
```

```text
broom /
```

```text
kit [clean]
```

State variants:

| State | Visual cue |
|---|---|
| Stored | on rack / leaning |
| In use | near dirt / algae marker |
| Dirty | residue / stipple |
| Missing | empty rack slot |

Design rule: cleaning props help communicate that greenhouse health depends on maintenance, not just watering.

### 10. Spare pots / trays / substrate bag

Purpose: supplies for planting, transplanting, and propagation setup.

Best labs:

- Utility Lab
- Propagation Lab
- Grow Bay Lab, small shelf variant

Support logic:

- should be stacked or stored, not scattered
- connects to plant-support elements from Note 05
- useful for Utility Lab and Propagation Lab identity

Terminal forms:

```text
[pots]
```

```text
╭──╮╭──╮╭──╮
╰──╯╰──╯╰──╯
```

```text
[soil]
```

State variants:

| State | Visual cue |
|---|---|
| Available | stack present |
| Low stock | `!` |
| Empty stack | placeholder |
| Spilled soil | small pile / dots |
| Selected | bracket |

Design rule: supply props should reinforce future planting workflows without cluttering active grow stations.

### 11. Spare tubing / filter cartridge / repair parts

Purpose: maintenance support for water and climate infrastructure.

Best labs:

- Utility Lab
- Climate Lab
- Grow Bay Lab, tiny side shelf variant

Support logic:

- connects directly to Notes 08–09
- explains repairs to pumps, filters, emitters, fans, ducts, and sensors
- works best in labeled bins or shelves

Terminal forms:

```text
[tube]
```

```text
[filter spare]
```

```text
parts: tube F valve
```

State variants:

| State | Visual cue |
|---|---|
| Stored | in bin / shelf |
| Low stock | `!` |
| Used | item removed |
| Wrong part | `?` |
| Repair active | near fault marker |

Design rule: spare parts belong primarily to Utility / Climate labs and should be grouped.

### 12. pH / EC calibration supplies

Purpose: make sensor maintenance visible.

Best labs:

- Utility Lab
- Climate Lab
- Archive / Specimen Lab, rare technical variant

Support logic:

- supports pH / EC readouts from Notes 07 and 09
- should sit near meters, reservoir panel, or testing bench
- implies that readings require maintenance and trust checks

Terminal forms:

```text
[cal]
```

```text
pH cal / EC cal
```

```text
bench: meter + cal
```

State variants:

| State | Visual cue |
|---|---|
| Available | bottle / marker present |
| Needed | `cal!` |
| Expired | `x` |
| In use | arrow to meter |
| Stored | on testing shelf |

Design rule: calibration supplies should not appear in simple Grow Bay mode unless pH / EC instruments are visible.

### 13. Pest / disease inspection card

Purpose: routine observation and problem-detection cue.

Best labs:

- Grow Bay Lab
- Propagation Lab
- Vines Lab
- Archive / Specimen Lab

Support logic:

- can be represented as sticky card, inspection tag, or tiny warning note
- supports future pest / disease logic without implementing it now
- should be subtle and not over-suggest problems by default

Terminal forms:

```text
[card]
```

```text
sticky: ?
```

```text
scan tag
```

State variants:

| State | Visual cue |
|---|---|
| Clear | neutral tag |
| Watch | `?` |
| Alert | `!` |
| Replaced | new card marker |
| Archived | tag number |

Design rule: inspection cards are useful future hooks, but should remain quiet in the first prototype.

---

## Maintenance props by lab

| Lab | Maintenance emphasis |
|---|---|
| Propagation Lab | labels, trays, humidity-dome cloth, small watering can, brush, notebook, seedling tags |
| Climate Lab | clipboard, calibration supplies, spare sensors, relay labels, maintenance warnings, filter / fan service tags |
| Grow Bay Lab | watering can, bay labels, pruners, small tool rack, drip repair tag, bucket for leak response |
| Vines Lab | pruners, clips, hooks, tie cord, ladder / stool cue, inspection cards, wall tags |
| Utility Lab | hose coil, bucket, tool rack, spare pots, tubing, filters, nutrient bottles, calibration supplies, cleaning kit |
| Archive / Specimen Lab | specimen labels, notebook, small brush, jar cloth, inspection cards, quiet calibration / observation tools |

---

## Minimal Grow Bay prototype maintenance set

For the first Grow Bay Lab, include only a few quiet props:

```text
1. three compact bay labels / plant tags
2. one small wall tool rack or side shelf
3. one pruning tool or scissors marker
4. one watering can or bucket, but not both at first
5. optional notebook / log marker only if there is room
```

Prototype sketch:

```text
┌─ Grow Bay Lab ───────────────────────────── OK ─┐
│ temp ○  hum ○      L:ok      fan ◉  vent ▦     │
│ tools: ✂ tag        H₂O ok                       │
│ ╞══╪══╪══╪══╪══╪══╪══╪══╪══╡ back wall       │
│ ━━━━━━━━━━━━━━━━━ lamp rail ━━━━━━━━━━━━━━━━━ │
│        drip ──────┬─────────┬─────────┬──      │
│                 · │       · │       · │        │
│   [p1 ok]       ╭──╮      ╭──╮      ╭──╮       │
│                 │░░│      │░░│      │░░│       │
│                 ╰──╯      ╰──╯      ╰──╯       │
│ ┄┄┄┄┄┄┄ drain / runoff channel ┄┄┄┄┄┄┄┄┄      │
└────────────────────────────────────────────────┘
```

This adds a human-care layer without destroying the clean greenhouse systems composition.

---

## Maintenance state vocabulary

### Basic states

| State | Meaning |
|---|---|
| Stored | prop is present and organized |
| In use | prop is active or selected |
| Missing | expected prop is absent |
| Low stock | supply is nearly gone |
| Dirty | cleaning or sanitation needed |
| Wet | recent watering / leak / cleaning |
| Broken | tool or part unusable |
| Needs calibration | sensor-maintenance action needed |
| Watch | inspection needed |
| Clear | no issue noted |

### Compact markers

```text
ok     normal / available
!      attention needed
x      missing / broken
?      watch / unknown
cal    calibration needed
m      maintenance
wet    wet / recently used
low    low stock
```

Design rule: maintenance markers should reuse the existing OK / WARN / FAULT vocabulary where possible.

---

## Placement rules

### Store tools on racks, shelves, hooks, or benches

Bad:

```text
✂       brush      tag       hose
```

Better:

```text
╞══ tool rack ══╡
  ✂  tag  brush
```

### Keep wet-work objects near water infrastructure

```text
hose @      bucket      drain ┄┄┄
```

### Keep calibration supplies near meters / panels

```text
[pH] [EC]   [cal]
```

### Keep plant labels near supports

```text
[p1 ok]   ╭──╮
          │░░│
          ╰──╯
```

### Use Utility Lab for dense storage

The Grow Bay should not become a storeroom. Utility Lab is where dense maintenance objects belong.

---

## Terminal readability rules

### Props should clarify, not decorate randomly

Every maintenance prop should imply a task.

### Prefer grouped storage

Grouped props are more readable than scattered props.

### Keep first prototype sparse

One rack, one tool, one label family is enough.

### Avoid text-heavy notebooks

A notebook can exist as `[log]`; it does not need to display a paragraph.

### Use props as future interaction hooks

Some props may later become selectable actions:

- water
- prune
- inspect
- calibrate
- clean
- repair
- label
- transplant

But early visuals can show them as static affordance objects.

---

## Relationship to future notes

### Note 11 — Navigation / Identity Markers

Maintenance props overlap with identity markers, especially labels, tags, plaques, bay IDs, specimen IDs, and lab signs.

Note 11 should probably define:

- lab signs
- active lab markers
- bay numbers
- plant names
- specimen tags
- warning badges
- selected-object markers
- tab-state rules

### Later note — Workflows / Care Actions

Once all element families exist, a later note can define care verbs:

```text
inspect
water
prune
clean
repair
calibrate
transplant
label
```

These should come after visual objects exist.

---

## Open design questions

1. Should maintenance props be selectable, or purely atmospheric in the first prototype?
2. Should tool presence imply available actions later?
3. Should the Grow Bay include a watering can, bucket, or neither in the first pass?
4. Should calibration supplies be visible only in Utility / Climate labs?
5. Should cleaning props create visible clean / dirty states in shell panels or glazing later?
6. Should pest / disease inspection cards be deferred to a later plant-health system?
7. Should labels belong to maintenance props, navigation markers, plant supports, or all three depending on context?
8. Should Utility Lab become the main storage / maintenance hub?

---

## Working conclusion

Work / maintenance props and tools should make the greenhouse feel cared for, inspected, and operational.

The essential maintenance-prop grammar is:

```text
functional system
└── maintenance need
    └── visible tool / supply
        └── implied care action
```

For the first Grow Bay prototype, use only a minimal maintenance layer:

- compact bay labels
- one small tool rack / side shelf
- one pruning tool or tag marker
- one watering can or bucket if space allows

This adds human maintenance presence without cluttering the greenhouse’s established shell + support + lighting + sensor + actuator + water composition.
