
# Greenhouse Ecosystem Design Notes

## HighGrow 4.20 — Architecture and UI Layout Orientation

### Purpose of this note

Use HighGrow 4.20 as a layout / interaction reference for YAM’s greenhouse ecosystem design, especially for architectural and structural world-elements. The goal is not to copy its subject matter or exact Windows-era look, but to extract useful spatial grammar: rooms, plant bays, pots, lamps, gauges, environmental controls, labels, and simulation-time affordances.

HighGrow’s basic setup is especially relevant because public descriptions frame it around three selected seeds planted in three pots inside a virtual grow room, with care actions including watering, fertilizing, light-height / photoperiod adjustment, pruning, and harvesting. Public descriptions also identify three available room locations: Workshop, Basement, and Attic.

### Core layout read

HighGrow’s screen is effectively a small functional stage:

- Top menu / toolbar: menus, room selector, plant selector, small tool/status icons, large digital clock.
- Left vertical meter rail: circular analog-style environmental gauges plus a magnifier/tool affordance.
- Central room stage: static room background, 1–3 plant stations, overhead lamps, pots, props.
- Three grow lanes: left / center / right plant columns, each with a pot, plant, label, and lamp.
- Room identity: Basement, Workshop, Attic, etc. Each room has a different architectural background and mood.
- Simulation temporality: the large visible time readout makes the room feel like a running system rather than a static scene.

The important pattern is “room as dashboard”: a compact greenhouse chamber where monitoring, infrastructure, and plant stations are all visible at once.

---

## Elements worth extracting for YAM

### 1. Room shell / backdrop

HighGrow uses static room backgrounds such as concrete basement walls, workshop walls, attic slopes, exposed beams, pipes, floor surfaces, and miscellaneous utility props.

For YAM, this should become terminal-native structural texture rather than photo-collage:

```text
╔════════════════════════════════════════════════════════════╗
║ wall texture / panel seams / pipes / vents / old brackets ║
║                                                            ║
║        lamp cone        lamp cone        lamp cone         ║
║                                                            ║
║        pot slot         pot slot         pot slot          ║
╚════════════════════════════════════════════════════════════╝
```

Potential YAM translations:

- low-contrast wall panels
- beams / rafters / angled attic rooflines
- pipes, cable runs, drain channels
- vent grates and small fan housings
- shelf rails and brackets
- concrete / glass / plastic sheeting textures
- room identity encoded through architecture, not decorative clutter

Design rule: the room shell should support the greenhouse system and atmosphere without competing with plant readability.

### 2. 1–3 plant bay layout

HighGrow’s strongest reusable idea is the 1–3 plant room. Each room reads as a simple triptych: left plant, center plant, right plant. This is highly legible and maps cleanly to a simulation model.

YAM concept:

```text
[ sensor tag ]     [ sensor tag ]     [ sensor tag ]
   ╭─────╮            ╭─────╮            ╭─────╮
   │lamp │            │lamp │            │lamp │
  /       \          /       \          /       \
     pot                pot                pot
```

Possible model name:

- `GrowRoom`
- `PlantStation`
- `GrowSlot`
- `Bay`

Each station can own or reference:

- pot / tray / planter object
- soil or substrate state
- plant occupant
- local light source
- local moisture / nutrient / temperature modifiers
- label / tag
- optional tool affordance
- status flags such as dry, overheated, shaded, recovering, dormant

### 3. Overhead lamps and visible light cones

The overhead lamps are the most valuable architectural element to emulate. They define each grow bay, imply environmental control, and visually connect the top infrastructure to the plant below.

Terminal sketches:

```text
      ╭────────╮
      │  LED   │
      ╰────────╯
       ╲      ╱
        ╲    ╱
         ╲  ╱
```

```text
      .-====-.
       \    /
        \  /
         \/
```

Potential lamp states:

| State | Visual cue |
|---|---|
| Off | lamp hood only, no cone |
| Low | faint or sparse cone |
| On | clear cone |
| Overheat | warning marker or unstable/flickering cone |
| Bloom / grow mode | different cone cadence or cone density |
| Broken / misaligned | asymmetric cone, dangling cable, tilted fixture |

Design rule: light cones are useful because they are both atmospheric and functional. They should become first-class greenhouse infrastructure, not merely decoration.

### 4. Pots as primary anchors

HighGrow’s pots make every plant station legible even when the plant is tiny. The pot is the stable base, the interaction target, and the visual anchor for growth.

YAM should treat pots as structural world-elements rather than as part of plant sprites.

Potential pot / container types:

| Type | Meaning |
|---|---|
| Clay pot | baseline readable planter |
| Nursery tray | propagation / early growth |
| Hanging pot | vertical space / vines |
| Hydro bucket | technical controlled room |
| Raised bed | wider growth footprint |
| Crate / bin planter | improvised greenhouse mood |
| Glass jar / terrarium vessel | small contained microclimate |

Pot responsibilities:

- define plant root position / anchor
- indicate scale
- carry soil / moisture state
- support labels or tags
- expose interaction affordances
- optionally leak state visually through cracks, damp marks, tray overflow, dry soil glyphs

### 5. Side climate-monitor rail

HighGrow’s left rail of circular analog gauges is one of its most reusable UI ideas. It makes environmental state spatial and persistent instead of hiding it in menus.

YAM translation should probably avoid literal bitmap dials and use compact terminal-native glyph gauges.

Example:

```text
╭────╮
│╲   │  temp
│ ╲  │  21°
╰────╯

╭────╮
│  ╱ │  humidity
│ ╱  │  63%
╰────╯

╭────╮
│ ◌  │  soil
│ 42 │
╰────╯
```

Possible gauges:

- temperature
- humidity
- light intensity
- soil moisture
- airflow / ventilation
- CO₂ / air quality, if simulation warrants it
- nutrient / substrate condition
- stress / warning state

Design rule: monitoring should be readable at a glance and visually integrated into the room. Avoid burying core environmental state in text-only menus.

### 6. Top status strip / toolbar semantics

HighGrow’s toolbar is visually dated but semantically useful. It exposes room selection, plant selection, tool/status icons, and time.

YAM should preserve the semantic structure, not the literal desktop-app chrome.

Possible status strip:

```text
ROOM: Workshop Bay     MODE: Observe     PLANT: Ivy-03     TIME 08:21
```

or:

```text
[Room: Workshop] [Bay: 2/3] [Climate: stable] [08:21:46]
```

Useful status strip fields:

- current room / chamber
- selected bay or plant
- current mode / tool
- climate state summary
- simulation clock
- alert count or warning badge

### 7. Plant labels / tags

HighGrow uses blunt floating name labels above plants. These are effective but visually utilitarian.

YAM should translate them into more diegetic tags:

```text
╭─ IVY-03 ─╮
```

or:

```text
   ┌─────────┐
   │ Ivy-03  │
   └─┬─────┬─┘
```

Preferred direction:

- hanging tags attached to lamp chains or shelf rails
- small stake tags near pots
- optional debug labels for development mode
- labels fade or collapse when not needed

Design rule: labels should aid orientation without turning the room into a spreadsheet.

### 8. Background functional props

HighGrow’s screenshots include watering cans, shovels, fans, pipes, wheelbarrow shapes, concrete walls, attic beams, and workshop clutter. These make rooms feel inhabited and functional.

YAM should include props only when they imply greenhouse function.

High-value prop candidates:

- watering can
- hose coil
- wall thermometer
- hygrometer
- pressure gauge
- analog timer
- fan
- vent grate
- small heater
- humidifier / misting head
- pipe elbow
- fuse box / controller box
- cable run
- shelf bracket
- seed tray
- bucket
- drain channel
- lamp chain

Avoid purely random clutter unless it clarifies room identity.

---

## Emulation priority

### Tier 1 — definitely emulate

- 1–3 plant room layout
- distinct grow stations / bays
- overhead lamps with visible light cones
- pots / trays as stable plant anchors
- side climate-monitor rail
- room identity through architecture
- large simulation time / clock readout

These form a strong greenhouse base without requiring complex plant biology immediately.

### Tier 2 — adapt carefully

- floating plant labels → convert to tags, stakes, or subtle HUD labels
- photo-real backgrounds → convert to terminal-native walls, beams, pipes, panels
- circular analog dials → convert to compact glyph gauges
- toolbar icons → convert to compact textual status segments
- room dropdown → convert to room tabs, chamber names, or viewport zones

### Tier 3 — avoid as direct copy

- literal Windows 2000 application chrome
- flat photo-collage composition
- gray floating labels as-is
- exact identical lamp repetition without variation
- opaque plant-care menus unless YAM becomes a full care simulator

HighGrow is useful as layout grammar, not as a visual style to reproduce literally.

---

## Proposed YAM greenhouse chamber direction

Functional / dashboard-heavy version:

```text
┌──────────────────────────────────────────────────────────────┐
│ Room: Workshop Bay        Climate: Stable        08:21:46     │
├───────┬──────────────────────────────────────────────┬───────┤
│ TEMP  │        ╭────╮      ╭────╮      ╭────╮        │ ALERT │
│ 21°   │        │LED │      │LED │      │LED │        │  OK   │
│ HUM   │         ╲  ╱        ╲  ╱        ╲  ╱         │       │
│ 63%   │          \/          \/          \/          │       │
│ SOIL  │                                                        │
│ 42%   │        [pot]       [pot]       [pot]        │       │
└───────┴──────────────────────────────────────────────┴───────┘
```

More atmospheric / room-native version:

```text
╔════════════════════ greenhouse bay: workshop ═══════════════════╗
║ pipe ─────┐        chain       chain       chain        vent ▦  ║
║           │       .-====-.    .-====-.    .-====-.              ║
║   21°     │        ╲    ╱      ╲    ╱      ╲    ╱       fan ◉   ║
║   63%     │         ╲  ╱        ╲  ╱        ╲  ╱                ║
║                    ┌────┐      ┌────┐      ┌────┐              ║
║                    │pot │      │pot │      │pot │              ║
╚══════════════════════════════════════════════════════════════════╝
```

Recommended stance: use the atmospheric / room-native version as the default aesthetic, with dashboard elements embedded diegetically where possible.

---

## Working design thesis

YAM’s greenhouse should begin as a functional room system before becoming a plant-detail system.

The essential architectural grammar:

1. A room shell defines place and mood.
2. Plant bays define the simulation slots.
3. Pots define growth anchors.
4. Lamps define environmental control and visual hierarchy.
5. Gauges define monitoring and feedback.
6. Props define greenhouse function.
7. The clock defines simulation time.

This gives the greenhouse a coherent “controlled environment” identity while leaving room for later plant ecology, growth stages, climate behaviors, and multi-room navigation.
