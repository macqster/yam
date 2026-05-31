
# Greenhouse Ecosystem Design Notes 02

## Labs as greenhouse sub-worlds

### Revision

The greenhouse ecosystem should not be modeled as one flat room. It should be modeled as a larger common greenhouse frame containing multiple switchable sub-world rooms called **labs**.

A **lab** is a contained greenhouse chamber / workroom / growth environment. The user should be able to switch between labs like switching between tabs inside a shared greenhouse world-interface.

This keeps the greenhouse coherent as one place while allowing each lab to have its own architecture, environmental profile, plant stations, instruments, props, and visual mood.

---

## Core mental model

```text
╔════════════════════════════ YAM GREENHOUSE ════════════════════════════╗
║ [ Propagation Lab ] [ Climate Lab ] [ Vines Lab ] [ Utility Lab ]      ║
╠═════════════════════════════════════════════════════════════════════════╣
║                                                                         ║
║                  currently selected lab renders here                    ║
║                                                                         ║
║        lamps / gauges / pots / walls / pipes / sensors / plants         ║
║                                                                         ║
╚═════════════════════════════════════════════════════════════════════════╝
```

The outer frame is stable. The inner lab changes.

This gives YAM a useful structure:

- the **greenhouse frame** provides identity, navigation, clock, global status, and common controls
- each **lab** provides local architecture, plant bays, climate settings, props, and visual composition
- lab switching behaves like tab navigation, not like leaving the greenhouse entirely

---

## Why “labs” works better than generic rooms

“Room” is spatial but neutral. “Lab” implies a controlled environment, instrumentation, and experiment-like plant systems.

This fits the greenhouse direction because modern controlled-environment growing is built around deliberate manipulation of growth parameters such as light, nutrients, carbon dioxide, temperature, and humidity. CEA sources describe greenhouses as enclosed structures where these conditions can be fully or partially controlled, and greenhouse climate-control discussions consistently emphasize temperature, humidity, ventilation, light, and CO₂ as the key climate factors.

Design implication: a YAM lab should visibly contain **control surfaces**, **sensors**, **fixtures**, and **plant stations**, not merely background decoration.

---

## Greenhouse frame vs lab contents

### Greenhouse frame responsibilities

The common frame should own elements that persist across lab switches:

- greenhouse title / identity
- lab tabs
- global simulation clock
- global alert indicator
- possibly global weather / outside condition indicator
- current mode or tool state
- stable border / viewport shell
- high-level resource summaries, if needed later

Example:

```text
╔═ YAM GREENHOUSE ═══════════════════════════════════════ 08:21:46 ═╗
║ [Propagation] [Climate] [Vines] [Utility] [Archive]      OK: 03   ║
╠════════════════════════════════════════════════════════════════════╣
║                                                                    ║
║                         active lab area                            ║
║                                                                    ║
╚════════════════════════════════════════════════════════════════════╝
```

### Lab responsibilities

Each lab owns its own local world-elements:

- wall / floor / ceiling structure
- lab-specific lighting
- plant bays / grow slots
- pots, trays, beds, tanks, shelves
- local gauges and instruments
- local climate-control equipment
- pipes, vents, fans, valves, cables
- props that explain the lab’s function
- local warnings, labels, and state readouts

The lab should feel like a self-contained microclimate.

---

## Lab switching as tab navigation

Lab navigation should be visually explicit and low-friction.

Possible tab styles:

```text
[ Propagation ] [ Climate ] [ Vines ] [ Utility ]
```

```text
╭ Propagation ╮╭ Climate ╮╭ Vines ╮╭ Utility ╮
╰─────────────┴──────────┴────────┴─────────╯
```

```text
LABS:  1 Propagation  |  2 Climate  |  3 Vines  |  4 Utility
```

Design rule: lab switching should preserve the feeling of staying inside the same greenhouse system. The active lab changes; the greenhouse frame remains.

---

## Initial lab categories

These are not final features yet. They are architectural categories for organizing in-world elements.

### 1. Propagation Lab

Purpose: early growth, seedlings, small trays, delicate monitoring.

Likely elements:

- seed trays
- humidity dome
- small pots
- heat mat
- thin lamp bars
- misting nozzle
- delicate thermometer / hygrometer
- labels / stake tags
- shallow shelves

Visual character:

- compact
- orderly
- small-scale
- dense with tags and trays

### 2. Climate Lab

Purpose: the clearest expression of environmental control.

Likely elements:

- wall gauges
- thermometer
- hygrometer
- CO₂ / air-quality indicator, if used
- vent fan
- ducting
- heater
- cooler / evaporative pad reference
- humidifier / fogger
- control panel
- warning lights

Visual character:

- instrument-heavy
- pipes, vents, boxes, ducts
- strongest “greenhouse control room” feel

### 3. Grow Bay Lab

Purpose: HighGrow-like 1–3 plant stations with pots and lamps.

Likely elements:

- three plant bays
- overhead lamps
- visible light cones
- pots or grow buckets
- local plant tags
- wall-mounted sensor strip
- irrigation line
- drain tray

Visual character:

- readable triptych
- the best default lab for the first functional prototype
- strong continuity with HighGrow’s layout grammar

### 4. Vines Lab

Purpose: vertical growth, climbing structures, trellis logic, wall occupation.

Likely elements:

- trellis grid
- strings / wires
- wall hooks
- overhead rail
- climbing frames
- hanging pots
- wall moisture meter
- pruning tag markers

Visual character:

- more vertical than the grow bay
- vines interact with architecture
- good place for YAM’s procedural plant-ecosystem identity

### 5. Utility Lab

Purpose: equipment and maintenance world-elements.

Likely elements:

- water tank
- pump
- hose coil
- bucket
- nutrient reservoir
- fuse box
- valves
- spare pots
- tool rack
- soil bag / substrate bin

Visual character:

- practical, infrastructural, slightly cluttered but purposeful
- useful for making the greenhouse feel like a working system

### 6. Archive / Specimen Lab

Purpose: special containers, older specimens, jars, terrarium-like displays, records.

Likely elements:

- specimen shelves
- glass jars
- terrarium cases
- labels
- old notebooks / tags
- small lights
- preservation trays

Visual character:

- quieter, museum-like, observational
- good for long-lived plants or strange ecosystem artifacts later

---

## In-world element groups to design next

This note introduces lab structure. The next design pass should focus on specific in-world element groups inside labs.

Suggested groups:

1. **Boundaries and shell elements**
   - walls, corners, floors, ceiling beams, glass panels, plastic sheeting, partitions

2. **Plant-support elements**
   - pots, trays, beds, shelves, trellises, hanging baskets, hydro buckets

3. **Lighting elements**
   - lamps, lamp bars, chains, cables, reflectors, light cones, shade cloth

4. **Climate-control elements**
   - vents, fans, heaters, foggers, humidifiers, ducts, autovents, louvers

5. **Sensor and gauge elements**
   - thermometers, hygrometers, dial gauges, digital readouts, warning LEDs, control panels

6. **Water / nutrient infrastructure**
   - tanks, reservoirs, pumps, valves, tubes, misting nozzles, drip lines, drain trays

7. **Work / maintenance props**
   - watering cans, scissors, tags, notebooks, soil bags, tool racks, buckets, gloves

8. **Navigation / identity markers**
   - lab tabs, room signs, bay numbers, specimen tags, status banners

---

## Design constraints

- A lab is a sub-world, not just a menu page.
- The greenhouse frame persists across lab switches.
- Labs should be visually distinct but governed by the same structural grammar.
- Every in-world object should imply a function: monitoring, growing, supporting, controlling, watering, storing, or navigating.
- Avoid random decorative clutter.
- Prefer diegetic controls and gauges where possible.
- HighGrow is a useful reference for layout grammar, especially room + 1–3 plant stations + lamps + gauges + clock.
- YAM should translate that grammar into terminal-native architecture rather than copying the old desktop UI literally.

---

## Working architectural thesis

The greenhouse is the persistent frame-world.

Labs are switchable controlled-environment sub-worlds inside that frame.

Each lab is defined by:

1. its **purpose**
2. its **local architecture**
3. its **plant-support infrastructure**
4. its **climate-control infrastructure**
5. its **sensor / gauge language**
6. its **props and maintenance affordances**
7. its **visual rhythm**

This provides a scalable structure for designing architectural elements without collapsing everything into one overloaded room.
