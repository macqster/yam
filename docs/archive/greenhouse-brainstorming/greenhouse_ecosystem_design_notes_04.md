
# Greenhouse Ecosystem Design Notes 04

## Boundary, Shell, and Lab Frame Elements

### Purpose of this note

This note defines the architectural layer for the YAM greenhouse ecosystem: the persistent greenhouse frame, the switchable lab viewport, and the boundary / shell elements that make each lab feel like a contained sub-world.

The goal is to design the greenhouse architecture before filling rooms with props or plants.

Notes 01–03 established this hierarchy:

```text
Greenhouse Frame
└── switchable Labs
    ├── Propagation Lab
    ├── Climate Lab
    ├── Grow Bay Lab
    ├── Vines Lab
    ├── Utility Lab
    └── Archive / Specimen Lab
```

Note 04 answers the next question:

```text
What makes a lab visibly a lab?
```

---

## Reference logic from real greenhouse design

Real greenhouse design is not just decoration around plants. It is an organized structure that balances enclosure, light, drainage, ventilation, climate equipment, work areas, growing areas, and circulation.

Useful real-world references for YAM translation:

- Greenhouse planning commonly separates **growing area**, **work area**, and **connecting pathways**. This supports YAM’s greenhouse-frame + lab-subworld model.
- Greenhouse design considers location, orientation, drainage, structure, foundation, flooring, glazing, ventilation, and climate-control equipment. For YAM, these become visual shell categories.
- Common greenhouse structure types include freestanding / single greenhouses, Quonset forms, gable forms, and ridge-and-furrow / gutter-connected systems. For terminal design, this suggests distinct roofline and frame silhouettes.
- Greenhouse coverings / glazing are part of function, not decoration: glass, plastic film, panels, shade cloth, and insulated layers all affect light, heat, and visibility.
- Environmental control requires architecture to support sensors and actuators: vents, fans, heaters, cooling pads, shades, lights, humidity controls, CO₂ handling, irrigation, and drainage.

Design translation: YAM labs should be drawn as **controlled containers**, not empty boxes. Walls, floors, rooflines, rails, panels, and openings should imply what the lab can do.

---

## Core architecture model

### 1. Persistent greenhouse frame

The greenhouse frame is the stable outer interface that remains visible while switching labs.

Responsibilities:

- greenhouse identity / title
- lab tabs
- simulation clock
- global status / alert summary
- stable viewport border
- optional outside weather / exterior condition marker
- optional current mode / tool state

Sketch:

```text
╔═ YAM GREENHOUSE ═══════════════════════════════════════ 08:21:46 ═╗
║ [Propagation] [Climate] [Grow Bay] [Vines] [Utility] [Archive]   ║
╠════════════════════════════════════════════════════════════════════╣
║                                                                    ║
║                          active lab viewport                       ║
║                                                                    ║
╚════════════════════════════════════════════════════════════════════╝
```

Design rule: the frame should remain stable. Lab switching changes the interior world, not the whole application identity.

### 2. Active lab viewport

The active lab viewport is the interior area where one lab renders.

Responsibilities:

- local shell / architecture
- wall and floor structure
- ceiling / roofline / overhead mounts
- plant-support objects
- lab-local instruments and equipment
- local labels and warnings

Sketch:

```text
╔═ YAM GREENHOUSE ═══════════════════════════════════════ 08:21:46 ═╗
║ [Propagation] [Climate] [Grow Bay] [Vines] [Utility] [Archive]   ║
╠════════════════════════════════════════════════════════════════════╣
║  ┌──────────────────────── active lab shell ──────────────────┐  ║
║  │ wall / rail / pipe / gauge / lamp mount / floor / objects   │  ║
║  └─────────────────────────────────────────────────────────────┘  ║
╚════════════════════════════════════════════════════════════════════╝
```

Design rule: the lab shell is local. It can vary strongly by lab, but it should always sit inside the common greenhouse frame.

---

## Boundary / shell element families

### 1. Outer frame

Purpose: make the greenhouse feel like one persistent place.

Elements:

- top border
- bottom border
- left / right rails
- greenhouse title
- clock area
- alert marker
- lab-tab row
- optional exterior weather capsule

Terminal vocabulary:

```text
╔ ═ ╗ ║ ╚ ╝ ╠ ╣ ╦ ╩ ╬
┌ ─ ┐ │ └ ┘ ├ ┤ ┬ ┴ ┼
```

Possible style directions:

- heavy box frame for greenhouse identity
- lighter inner frames for labs
- high-contrast active lab marker
- low-contrast inactive tabs
- compact top bar at small widths

Design rule: the outer frame is UI-like, but it should still feel like greenhouse hardware: a window frame, not a generic app border.

### 2. Lab tab strip

Purpose: switch between lab sub-worlds while preserving the greenhouse frame.

Sketch options:

```text
[Propagation] [Climate] [Grow Bay] [Vines] [Utility] [Archive]
```

```text
╭ Propagation ╮╭ Climate ╮╭ Grow Bay ╮╭ Vines ╮
╰─────────────┴──────────┴──────────┴────────╯
```

```text
LABS:  1 Propagation  |  2 Climate  |  3 Grow Bay  |  4 Vines
```

Tab states:

| State | Meaning | Visual cue |
|---|---|---|
| Active | currently visible lab | bracket, highlight, underline, thicker edge |
| Inactive | available lab | dimmer label |
| Warning | lab has alert | small `!`, `*`, or warning LED |
| Locked / future | not implemented yet | faint label or dotted border |

Design rule: tabs belong to the greenhouse frame, not the lab content.

### 3. Inner lab frame

Purpose: define the active lab as a contained sub-world.

Elements:

- inner border
- lab name plaque
- local status corner
- local alert marker
- viewport crop edge
- background shell boundary

Sketch:

```text
┌─ Grow Bay Lab ───────────────────────────────────── stable ─┐
│                                                              │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

Variants:

- full rectangular inner frame
- partial frame only at top / sides
- structural greenhouse beams instead of abstract border
- no explicit inner frame if wall / floor architecture is strong enough

Design rule: the inner frame should not duplicate the outer frame too aggressively. It should help readability without creating a nested-box prison.

### 4. Back wall

Purpose: provide the main architectural background and mounting surface.

Possible wall types:

| Wall type | Best lab fit | Visual function |
|---|---|---|
| Glass panel wall | Propagation, Grow Bay | light, openness, greenhouse identity |
| Concrete / basement wall | Utility, Climate | heavy infrastructure, service mood |
| Pegboard / service wall | Utility, Climate | tool and instrument mounting |
| Trellis wall | Vines | plant interaction with architecture |
| Shelf wall | Archive, Propagation | dense small objects / specimens |
| Plastic-sheet partition | Propagation, Climate | controlled microclimate |
| Insulated panel wall | Climate | controlled-environment lab feeling |

Terminal wall motifs:

```text
│  │  │  │  │     vertical glass ribs
┆  ┆  ┆  ┆  ┆     faint plastic / screen texture
╞══╪══╪══╡     greenhouse panel grid
┬─┬─┬─┬─┬     shelf / rail structure
░░░░░░░░░     dense wall texture, use sparingly
```

Design rule: back walls should be low-contrast and non-dominant. They create place, but should not drown plants and gauges.

### 5. Side walls and corners

Purpose: create enclosure and depth.

Potential patterns:

```text
╱│                 angled left wall / perspective cue
 │
 │
```

```text
│\                 angled right wall / perspective cue
│ \
│  \
```

```text
╔══════╗
║      ║             flat contained lab bay
║      ║
╚══════╝
```

Use side walls for:

- vertical pipes
- cable runs
- wall-mounted gauges
- lab signs
- vents
- small shelves
- corner shadows

Design rule: side walls should create depth with minimal glyph cost. Avoid complex pseudo-3D unless it remains readable at small terminal sizes.

### 6. Floor plane

Purpose: ground the lab and provide locations for pots, tanks, buckets, drains, and floor equipment.

Floor variants:

| Floor type | Meaning |
|---|---|
| Simple baseline | minimal prototype, clean readable space |
| Tile grid | controlled lab / utility room |
| Concrete slab | basement / service room |
| Drain channel | irrigation / wet room |
| Raised bench surface | propagation / grow bay |
| Grate floor | hydro / utility / drainage-heavy lab |

Terminal sketches:

```text
──────────────────────────────────── floor baseline
┄┄┄┄┄┄┄┄┄┄┄┄ drainage seam
┬─┬─┬─┬─┬─┬─ bench / shelf edge
▔▔▔▔▔▔▔▔▔▔ raised platform
```

Design rule: floor grammar should anchor objects and imply drainage / work surface, but should not become visual noise.

### 7. Ceiling / roofline

Purpose: identify lab type and support hanging infrastructure.

Roofline variants:

| Roofline | Meaning |
|---|---|
| Flat ceiling rail | controlled lab / workroom |
| Gable roof | classic greenhouse identity |
| Quonset arc | tunnel / hoop-house identity |
| Attic slope | inherited HighGrow-like room identity |
| Shelf underside | propagation rack / archive shelf |

Terminal sketches:

```text
        /\
       /  \          gable / greenhouse roof
──────/────\──────
```

```text
    .-''''''''-.
 .-'            '-.   Quonset / hoop tunnel cue
```

```text
━━━━━━━━━━━━━━━━━━   ceiling rail / lamp rail
   │      │      │    hanging points
```

Use ceiling / roofline for:

- lamp chains
- hanging baskets
- misting nozzles
- shade cloth
- cable trays
- ventilation slots

Design rule: ceiling elements are high-value because they create vertical hierarchy and explain lamps / misting / vents.

### 8. Structural rails and mounting points

Purpose: provide a believable attachment grammar for equipment.

Elements:

- horizontal rails
- vertical posts
- shelf standards
- lamp tracks
- pipe brackets
- cable trays
- trellis rails
- hanging hooks

Sketch:

```text
━━━━━━━━━━━━━━━━━━━━ rail
   │       │       │ hang points
   ╰─ lamp ╯       ╰─ sensor
```

```text
│╞════╪════╪════╡│ wall-mounted equipment rail
```

Design rule: mounted equipment should not float. Rails and brackets make gauges, lamps, sensors, and pipes feel integrated.

### 9. Glazing, panels, and partitions

Purpose: communicate greenhouse enclosure and light logic.

Materials:

- glass panes
- polycarbonate panels
- plastic sheeting
- shade cloth
- mesh / screen
- insulated panels
- curtain partitions

Terminal motifs:

```text
╞══╪══╪══╪══╡  rigid pane grid
┆  ┆  ┆  ┆  ┆  soft plastic partition
▒▒▒▒▒▒▒▒▒▒▒▒  shade cloth / dense screen, use carefully
╱╲╱╲╱╲╱╲╱╲  folded plastic / curtain hint
```

State variants:

- clear
- fogged
- cracked
- shaded
- open
- sealed
- condensation beads

Design rule: glazing is both architectural and environmental. It can imply light, humidity, insulation, and separation between labs.

---

## Per-lab shell silhouettes

### Propagation Lab

Shell identity:

- shelf-like
- compact
- shallow depth
- delicate partitions
- small repeated trays
- low lamp bars

Suggested structure:

```text
┌─ Propagation Lab ───────────────────────────── humid ─┐
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│   [tray] [tray] [tray]      ┆ plastic dome ┆          │
│ ───────────────────────────────────────────────────── │
└───────────────────────────────────────────────────────┘
```

Architectural emphasis:

- shelves
- trays
- humidity domes
- small labels
- thin lamp rails
- light plastic partitions

### Climate Lab

Shell identity:

- service wall
- gauges and panels
- ducts and vents
- strong vertical / horizontal infrastructure

Suggested structure:

```text
┌─ Climate Lab ───────────────────────────── stable ─┐
│ ╞══ panel rail ═════════════════════════════════╡  │
│  ○ temp   ○ hum   ○ CO₂       [control panel]     │
│  vent ▦       duct ═══════════ fan ◉              │
└───────────────────────────────────────────────────┘
```

Architectural emphasis:

- back wall as instrument board
- vents and fan housings
- control panel
- service ducts
- wall-mounted sensor rail

### Grow Bay Lab

Shell identity:

- HighGrow-like 1–3 plant stations
- overhead lamp rail
- clear bay divisions
- grounded pots

Suggested structure:

```text
┌─ Grow Bay Lab ───────────────────────────── OK ─┐
│ ━━━━━━━━━━━━━ lamp rail ━━━━━━━━━━━━━━━━━━━━━━ │
│     ╭────╮        ╭────╮        ╭────╮         │
│      ╲  ╱          ╲  ╱          ╲  ╱          │
│     [pot]          [pot]          [pot]        │
└────────────────────────────────────────────────┘
```

Architectural emphasis:

- lamp rail
- three bays
- pots / buckets
- local tags
- drain tray / floor line

### Vines Lab

Shell identity:

- vertical grid
- trellis wall
- hanging lines
- wall-to-plant interaction

Suggested structure:

```text
┌─ Vines Lab ───────────────────────────── climb ─┐
│ ╞══╪══╪══╪══╪══╪══╪══╪══╪══╡ trellis wall     │
│    │  │  │  │  │  │  │  │     hanging lines    │
│   [pot]       [hanging]        [wall planter]   │
└─────────────────────────────────────────────────┘
```

Architectural emphasis:

- trellis grid
- vertical supports
- hooks / strings
- hanging pots
- wall moisture markers

### Utility Lab

Shell identity:

- infrastructure room
- tanks, pipes, pumps, valves
- service-wall logic
- floor drains

Suggested structure:

```text
┌─ Utility Lab ───────────────────────── service ─┐
│ tank ║     pipe ═════ valve ◇ ═════ pump ▣       │
│      ║                                           │
│ hose coil @       bucket      drain ┄┄┄┄┄┄┄     │
└──────────────────────────────────────────────────┘
```

Architectural emphasis:

- tanks and reservoirs
- wall pipes
- valves
- floor drain
- tool rack
- spare containers

### Archive / Specimen Lab

Shell identity:

- quiet shelves
- jars / terrariums
- labels
- museum-like observation grid

Suggested structure:

```text
┌─ Specimen Lab ─────────────────────── archive ─┐
│ ┬────────┬────────┬────────┬────────┬──────── │
│ │ jar 01 │ case A │ tag 7  │ jar 02 │ notes  │
│ ┴────────┴────────┴────────┴────────┴──────── │
└────────────────────────────────────────────────┘
```

Architectural emphasis:

- shelving
- specimen cases
- small lights
- placards
- observation labels

---

## Depth model

Each lab should ideally have three visual layers:

```text
Background: wall / panels / large structural texture
Midground: rails / pipes / gauges / shelves / lamps
Foreground: pots / plant supports / tanks / interactable equipment
```

Design rule: keep high-frequency texture in the background faint. Reserve strong contrast for foreground functional objects.

---

## Readability rules for terminal architecture

### Use stable silhouettes

Each lab should be recognizable from outline before details are read.

Examples:

- Grow Bay = three lamps + three anchors
- Climate = gauges + ducts + panel wall
- Vines = vertical trellis grid
- Utility = tank + pipes + valves
- Propagation = shelves + trays + domes
- Archive = shelves + jars / cases

### Avoid texture overload

Do not fill every empty cell. Empty space is useful for readability and animation.

### Prefer attachment logic

Equipment should attach to rails, walls, pipes, shelves, or floors.

Bad:

```text
      ○ gauge floating in empty air
```

Better:

```text
│╞══ ○ temp ══╡│ mounted gauge on wall rail
```

### Separate frame and lab chrome

The greenhouse frame should remain stable and recognizable. Lab interiors can change, but should not visually destroy the frame.

### Use material motifs sparingly

A material should be suggested, not rendered exhaustively.

Examples:

- three or four vertical ribs imply glass
- a few dotted columns imply plastic sheeting
- one drain seam implies wet floor
- one rail implies mounting infrastructure

---

## Minimal prototype recommendation

The first implementation-oriented prototype should probably contain:

1. greenhouse outer frame
2. tab strip
3. active lab title / status corner
4. one shell silhouette: Grow Bay Lab
5. one back wall cue
6. one ceiling lamp rail
7. three hanging lamp positions
8. one floor / drain baseline
9. empty pot anchor positions, even before plants are rendered

This directly follows the HighGrow-inspired grammar while fitting the new lab-based architecture.

Prototype sketch:

```text
╔═ YAM GREENHOUSE ═══════════════════════════════════════ 08:21:46 ═╗
║ [Propagation] [Climate] [Grow Bay] [Vines] [Utility] [Archive]   ║
╠════════════════════════════════════════════════════════════════════╣
║ ┌─ Grow Bay Lab ─────────────────────────────────────── OK ────┐ ║
║ │ ╞══╪══╪══╪══╪══╪══╪══╪══╪══╡ glass / panel back wall        │ ║
║ │ ━━━━━━━━━━━━━━━━━ lamp rail ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │ ║
║ │      ╭────╮              ╭────╮              ╭────╮          │ ║
║ │       ╲  ╱                ╲  ╱                ╲  ╱           │ ║
║ │      [pot]                [pot]                [pot]         │ ║
║ │ ┄┄┄┄┄┄┄┄┄┄ drain / floor seam ┄┄┄┄┄┄┄┄┄┄┄┄                  │ ║
║ └──────────────────────────────────────────────────────────────┘ ║
╚════════════════════════════════════════════════════════════════════╝
```

---

## Open design questions

1. Should the inner lab frame always be drawn, or should some labs use only architectural edges?
2. Should lab tabs be literal UI tabs, greenhouse plaques, or a hybrid terminal control strip?
3. Should labs occupy the full available interior, or should the greenhouse frame include persistent side rails?
4. Should the Grow Bay Lab remain the default lab because it is most HighGrow-readable?
5. Should shell motifs be static, or can some elements animate subtly, such as fan spin, condensation, flicker, or airflow?
6. Should the greenhouse frame represent an actual physical frame/window, or mostly an app-level interface?

---

## Working conclusion

Boundary, shell, and lab-frame elements should be designed before plant-support props or plant growth detail.

The architectural layer gives each lab:

1. containment
2. identity
3. attachment points
4. depth
5. material language
6. functional plausibility
7. readable navigation inside the shared greenhouse frame

The recommended first shell prototype is the **Grow Bay Lab** because it directly expresses the HighGrow-inspired room grammar: a contained room, three plant stations, lamps, wall structure, and environmental control affordances.
