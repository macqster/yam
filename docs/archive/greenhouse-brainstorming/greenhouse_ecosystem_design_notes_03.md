
# Greenhouse Ecosystem Design Notes 03

## Greenhouse element families — next design layer

### Purpose of this note

Notes 01–02 established the main architectural direction:

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

The next logical layer is to define reusable **in-world element families** that labs can compose from.

This note should not yet design a single finished lab. Instead, it should define the groups of objects that can appear across labs, what they communicate, and which group should be designed first.

---

## Why element families come next

A lab should not be a loose pile of props. It should be a composed sub-world built from recurring greenhouse systems.

Real controlled-environment greenhouse logic commonly revolves around:

- enclosed or semi-enclosed growing structures
- light control
- temperature control
- humidity control
- CO₂ / air-quality control
- ventilation / airflow
- irrigation and water management
- nutrient delivery
- sensors and monitoring
- actuators such as vents, heaters, fans, cooling systems, shades, valves, and lighting

For YAM, these real greenhouse domains translate naturally into visible world-element families.

Design implication: each element family should answer a functional question:

```text
Where is this lab?
What holds the plants?
What lights them?
What measures the environment?
What changes the environment?
What moves water / nutrients?
What lets the user understand or navigate the space?
```

---

## Recommended sequence

### 1. Boundary / shell elements first

Start with architecture: walls, corners, floor, roofline, partitions, panels, ceiling rails, and structural frames.

Reason: labs need to feel like sub-worlds before they become prop collections.

Boundary and shell elements define:

- lab silhouette
- depth and enclosure
- material character
- active viewport area
- tabbed frame relationship
- possible hanging / mounting points
- places where gauges, vents, pipes, and lights can attach

This should be the first specific element group to design.

### 2. Plant-support elements second

After shell geometry, define what physically holds plants:

- pots
- trays
- beds
- shelves
- trellises
- hanging baskets
- hydro buckets
- specimen jars
- terrarium vessels

Reason: plant-support objects define where plants can exist and how they anchor to the lab.

This preserves HighGrow’s readable discrete plant-station logic while allowing later labs to expand beyond exactly three pots.

### 3. Sensor / gauge language third

Then define the greenhouse readout grammar:

- wall thermometer
- hygrometer
- light meter
- soil moisture marker
- pressure gauge
- digital status panel
- warning LED
- bay label
- lab status sign

Reason: the greenhouse should communicate environmental state spatially and visually, not only through menus.

This is where YAM can build a distinctive mix of old analog instrumentation, compact glyph gauges, warning lights, and diegetic status tags.

### 4. Climate-control actuators fourth

Once sensors exist, define the things that respond to them:

- fans
- vents
- louvers
- heaters
- foggers
- humidifiers
- shade cloth
- cooling pads
- ducts
- autovents
- lamp dimmers

Reason: sensors measure; actuators change conditions. Together they make the lab feel like an active controlled environment.

This family is especially valuable because it can make labs feel alive before sophisticated plant growth behavior exists.

### 5. Water / nutrient infrastructure fifth

Then design the greenhouse metabolism:

- reservoir
- water tank
- pump
- valve
- tube
- drip line
- misting nozzle
- drain tray
- nutrient bottle
- substrate bin
- runoff channel

Reason: water and nutrient infrastructure can visually connect multiple labs, making the greenhouse feel like one coherent system.

This may deserve its own dedicated note because pipes, tanks, tubes, and valves can become a major visual grammar.

---

## Immediate recommended target

The next focused design note should be:

```text
Note 04 — Boundary, Shell, and Lab Frame Elements
```

Reason: after establishing that labs are switchable greenhouse sub-worlds, the next question should be:

```text
What makes a lab visibly a lab?
```

Note 04 should cover:

1. common greenhouse frame
2. lab tab strip
3. active lab viewport
4. back wall / side wall / floor grammar
5. ceiling rails and hanging points
6. glass / plastic / concrete / utility-wall variants
7. per-lab architectural silhouettes
8. rules for keeping architecture readable in terminal cells

Then Note 05 can cover:

```text
Plant-Support Elements: pots, trays, beds, shelves, trellises, hydro buckets, specimen jars
```

This keeps the work architectural, systematic, and implementation-friendly.

---

## Element family overview

### 1. Boundary and shell elements

Purpose: define the lab as a contained sub-world.

Elements:

- greenhouse outer frame
- tab strip
- active lab viewport
- back wall
- side wall
- floor plane
- ceiling beam
- roofline
- corners
- glass panels
- plastic sheeting
- concrete wall
- panel seams
- partitions
- service wall
- mounting rails

Questions to answer:

- Is this lab enclosed, open, narrow, tall, deep, shelf-like, or cabinet-like?
- What material language identifies it?
- Where can lights, pipes, vents, and gauges attach?
- Does the shell create foreground / midground / background depth?

### 2. Plant-support elements

Purpose: physically anchor plants and growth systems.

Elements:

- clay pot
- nursery tray
- humidity dome
- raised bed
- shelf
- trellis
- string line
- hanging basket
- hydro bucket
- nutrient channel
- glass jar
- terrarium case

Questions to answer:

- How many plants can this support?
- Is it for seedlings, mature plants, vines, hydroponics, or specimens?
- Does it create a discrete plant station or a continuous growth surface?
- Can it show state through dryness, dampness, overflow, cracks, or labels?

### 3. Lighting elements

Purpose: establish growth control, bay identity, and visual hierarchy.

Elements:

- overhead lamp
- lamp bar
- grow panel
- reflector hood
- hanging chain
- power cable
- visible light cone
- shade cloth
- dimmer marker
- warning flicker

Questions to answer:

- Is the light per-plant, per-shelf, or room-wide?
- Is it active, dimmed, off, overheating, misaligned, or broken?
- Does it create a readable cone / beam in terminal glyphs?
- Does it help divide the lab into bays?

### 4. Sensor and gauge elements

Purpose: make environment state visible inside the world.

Elements:

- thermometer
- hygrometer
- light meter
- soil moisture probe
- CO₂ / air-quality marker
- pressure gauge
- analog dial
- digital panel
- bay status label
- warning LED
- small alarm light

Questions to answer:

- Is the readout global, lab-local, bay-local, or plant-local?
- Is it diegetic or HUD-like?
- Can it be read at a glance in a terminal viewport?
- Does it support warning states without clutter?

### 5. Climate-control elements

Purpose: visibly change or imply control over the lab environment.

Elements:

- vent
- louver
- fan
- duct
- heater
- cooling pad
- fogger
- humidifier
- misting head
- circulation fan
- shutter
- autovent arm

Questions to answer:

- What environmental variable does this element affect?
- Is it passive, active, automated, broken, or manually adjustable?
- Does it create motion cues such as fan spin, mist, airflow, or heat shimmer?
- Does it live on a wall, ceiling, floor, pipe, shelf, or plant bay?

### 6. Water / nutrient infrastructure

Purpose: define the greenhouse’s internal metabolism.

Elements:

- reservoir
- tank
- pump
- valve
- pipe
- flexible tube
- drip line
- misting nozzle
- drain tray
- runoff channel
- nutrient bottle
- substrate bin
- filter housing

Questions to answer:

- Does it belong to one lab or connect labs together?
- Is it visible as background infrastructure or foreground interactable equipment?
- Can it show flow, blockage, leak, empty state, or pressure?
- Does it support propagation, hydroponics, vines, or utility maintenance?

### 7. Work / maintenance props

Purpose: show that the greenhouse is a maintained working place.

Elements:

- watering can
- scissors / pruners
- tags
- notebook
- tool rack
- gloves
- bucket
- soil bag
- substrate bin
- spare pots
- brush
- small stool

Questions to answer:

- Does this prop imply a real maintenance action?
- Does it explain the lab’s purpose?
- Is it foreground clutter or background identity?
- Should it be static, stateful, or interactable later?

### 8. Navigation / identity markers

Purpose: help the user understand where they are inside the greenhouse frame.

Elements:

- lab tabs
- room sign
- bay number
- specimen tag
- plant label
- status banner
- selected-lab marker
- selected-bay marker
- alert badge

Questions to answer:

- Is this global navigation or local identification?
- Does it belong to the frame or to the active lab?
- Can it be visually consistent across all labs?
- Does it support keyboard switching later?

---

## Per-lab element emphasis

| Lab | Strongest element families |
|---|---|
| Propagation Lab | plant-support, sensors, humidity control, small lighting, labels |
| Climate Lab | gauges, sensors, vents, ducts, fans, heaters, control panels |
| Grow Bay Lab | pots, overhead lamps, light cones, bay labels, local gauges |
| Vines Lab | trellises, wall frames, hanging lines, vertical sensors, pruning tags |
| Utility Lab | tanks, pipes, pumps, valves, tool racks, spare containers |
| Archive / Specimen Lab | shelves, jars, cases, labels, small lights, observation panels |

---

## Design rules

- Do not design random props first.
- Start with shell / boundary so labs have spatial identity.
- Every element should imply a greenhouse function.
- Prefer reusable element families over one-off decoration.
- Keep the persistent greenhouse frame separate from lab-local contents.
- Use diegetic controls where possible.
- Preserve HighGrow’s useful layout grammar: room + plant stations + lamps + gauges + clock.
- Translate references into YAM’s terminal-native architectural language.
- Keep elements readable at small terminal sizes.
- Avoid overloading the first prototype with all systems at once.

---

## Working conclusion

The next practical design pass should be **Boundary, Shell, and Lab Frame Elements**.

That pass should define the visual and structural grammar that makes each lab feel like a contained sub-world inside the persistent greenhouse frame.

Once that exists, plant-support elements can be placed inside a coherent architecture instead of floating in empty space.
