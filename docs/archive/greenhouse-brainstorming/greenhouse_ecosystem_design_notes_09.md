
# Greenhouse Ecosystem Design Notes 09

## Water / Nutrient Infrastructure

### Purpose of this note

Notes 04–08 defined the greenhouse shell, plant-support layer, lighting layer, sensor / gauge layer, and climate-control actuator layer.

Note 09 defines **water / nutrient infrastructure**: the visible in-world systems that store, move, filter, dose, deliver, drain, recover, and monitor water or nutrient solution.

This note answers:

```text
How does the greenhouse visibly store, move, dose, drain, recycle, and monitor water / nutrients?
```

This is the greenhouse’s metabolism layer.

---

## Why water / nutrient infrastructure follows actuators

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

Note 08 already introduced climate equipment that overlaps with water systems:

- foggers need water supply
- humidifiers may need reservoirs
- cooling pads need water circulation
- hydro buckets and channels need pumps / lines
- drains collect excess moisture

Note 09 separates the water / nutrient network itself from climate actuation.

Real greenhouse water systems commonly deliver both water and nutrients according to crop needs. Drip irrigation references describe system components such as water sources, pumps, filters, pressure gauges, valves, dosing units, mainlines, sub-main lines, distribution pipes, fittings, driplines, connectors, sensors, and controllers. Greenhouse fertigation specifically means applying fertilizer through irrigation water, so the water layer naturally connects to nutrient dosing and pH / EC monitoring.

Design implication for YAM:

```text
reservoir → pump → filter → valve / manifold → line → emitter → pot / tray → drain / runoff → optional recovery
```

Water infrastructure should create visible connection, not clutter.

---

## Core water-system model

Every water / nutrient element should answer eight questions:

1. **Role** — storage, movement, filtration, routing, dosing, delivery, drainage, recovery, or monitoring?
2. **Medium** — plain water, nutrient solution, runoff, mist supply, cooling-pad water, or hydroponic solution?
3. **Attachment** — tank, wall, pipe, ceiling rail, floor, pot, tray, shelf, pump station, or reservoir?
4. **Flow direction** — where does water come from and where does it go?
5. **State** — full, low, flowing, blocked, leaking, dosing, draining, stale, contaminated, offline, or fault?
6. **Scope** — whole greenhouse, one lab, one bay, one shelf, one pot, or one hydro unit?
7. **Sensor relationship** — moisture, pH, EC, pressure, flow, reservoir level, or drainage volume?
8. **Lab fit** — which labs use it naturally?

Design rule: a water element should make flow legible without turning the scene into a pipe maze.

---

## Primary water / nutrient element families

### 1. Reservoir / tank

Purpose: store water or nutrient solution.

Best labs:

- Utility Lab
- Climate Lab
- Grow Bay Lab, as small local tank
- Propagation Lab, as compact supply tank

Support logic:

- usually floor-based or wall-adjacent
- can supply pumps, misting lines, cooling pads, drip lines, or hydro buckets
- gives Utility Lab a strong infrastructure anchor
- should support level state

Terminal forms:

```text
╭─────╮
│ H₂O │
│████ │
╰─────╯
```

```text
[tank]
```

```text
╔═ reservoir ═╗
║ water 80%  ║
╚════════════╝
```

State variants:

| State | Visual cue |
|---|---|
| Full | high fill mark |
| OK | mid fill / stable label |
| Low | low fill + `!` |
| Empty | blank interior / `x` |
| Nutrient mix | `N` or pH / EC tag |
| Contaminated | `!` / dirty texture |
| Offline | disconnected tube |

Design rule: tanks should usually live in Utility or Climate contexts. Avoid putting large tanks inside every grow room.

### 2. Pump

Purpose: move water or nutrient solution through the system.

Best labs:

- Utility Lab
- Grow Bay Lab, compact inline variant
- Climate Lab
- Propagation Lab, if misting line is present

Support logic:

- connects reservoir to lines
- can be inline, floor-mounted, or tank-mounted
- behaves like an actuator but belongs to water infrastructure
- can show active / blocked / fault state

Terminal forms:

```text
[pump]───
```

```text
▣→ water
```

```text
tank ║──▣── line
```

State variants:

| State | Visual cue |
|---|---|
| Off | pump box only |
| On | arrow / active dot |
| Low pressure | weak arrow / `↓` |
| Blocked | `!` at outlet |
| Fault | `x` |
| Priming | `~` marker |

Design rule: pumps should sit between source and distribution. They should not appear without connected lines.

### 3. Filter

Purpose: prevent emitters, nozzles, driplines, and cooling pads from clogging.

Best labs:

- Utility Lab
- Grow Bay Lab, inline small form
- Climate Lab
- Propagation Lab, if misting nozzles are used

Support logic:

- usually inline after pump or before manifold
- small but important infrastructure cue
- pairs with blocked emitter / pressure fault states

Terminal forms:

```text
──[filter]──
```

```text
──[≋]──
```

```text
pump ▣──[F]── valve
```

State variants:

| State | Visual cue |
|---|---|
| Clean | normal inline module |
| Clogged | `!` / darkened module |
| Bypass | alternate line marker |
| Maintenance | wrench / `m` |
| Missing | line break |

Design rule: filters make water systems feel real. Use them sparingly as inline modules, especially in Utility Lab.

### 4. Valve / manifold

Purpose: route water to labs, bays, shelves, or individual emitters.

Best labs:

- Utility Lab
- Grow Bay Lab
- Propagation Lab
- Climate Lab

Support logic:

- splits one line into multiple lines
- can show open / closed / fault state
- excellent terminal glyph object
- can map cleanly to the three-bay Grow Bay layout

Terminal forms:

```text
──◇── valve
```

```text
main ──┬── bay 1
       ├── bay 2
       └── bay 3
```

```text
[manifold]
  ├─ p1
  ├─ p2
  └─ p3
```

State variants:

| State | Visual cue |
|---|---|
| Open | connected line |
| Closed | blocked branch |
| Auto | `A` marker |
| Manual | `M` marker |
| Leaking | drip near valve |
| Stuck | `!` |
| Fault | `x` |

Design rule: manifolds are high-value for making multiple plant stations feel connected to one greenhouse system.

### 5. Pipe / tube / hose

Purpose: visible water path.

Best labs:

- all labs
- strongest in Utility, Grow Bay, Propagation, Climate

Support logic:

- connects tanks, pumps, filters, valves, emitters, misting lines, hydro buckets, and drains
- may be rigid pipe or flexible tube
- should express flow direction only when needed
- can be background infrastructure or foreground connection

Terminal forms:

```text
──────── pipe
```

```text
══════ rigid mainline
```

```text
╎╎╎ flexible tube
```

```text
───o─── connector
```

State variants:

| State | Visual cue |
|---|---|
| Idle | line only |
| Flowing | subtle arrow / moving dot |
| Leaking | drip mark below |
| Blocked | `!` at segment |
| Disconnected | broken line |
| Flexible | curved / dotted cue |

Design rule: pipes and tubes should explain relationships. They should not become decorative spaghetti.

### 6. Drip line / emitter

Purpose: deliver water to pots, trays, beds, or hydro media in controlled small amounts.

Best labs:

- Grow Bay Lab
- Propagation Lab
- Vines Lab
- Climate Lab, as test line

Support logic:

- belongs above, beside, or inside plant-support objects
- pairs directly with soil moisture tags from Note 07
- works well for HighGrow-like plant stations
- can show clogged / dry / active state compactly

Terminal forms:

```text
drip ──┬──┬──┬──
       ·  ·  ·
```

```text
───o───o───o── emitters
```

```text
╭──╮
│░░│ ← · emitter
╰──╯
```

State variants:

| State | Visual cue |
|---|---|
| Off | line only |
| Dripping | `·` under emitter |
| Flowing | arrow or repeated dots |
| Clogged | missing drip + `!` |
| Overwatering | multiple drip marks / tray puddle |
| Misaligned | drip not over support |

Design rule: drip emitters are the best first water-delivery element for Grow Bay Lab.

### 7. Misting line / nozzle

Purpose: feed propagation mist, fogger, humidifier, or cooling mist.

Best labs:

- Propagation Lab
- Climate Lab
- Grow Bay Lab, optional
- Archive / Specimen Lab, rare subtle variant

Support logic:

- attaches to overhead pipe, shelf, ceiling rail, or side wall
- connects directly to fogger / humidifier actuator logic from Note 08
- supports visible microclimate state

Terminal forms:

```text
pipe ───┬───┬───
        ··· ···
```

```text
nozzle ˅  · · ·
```

```text
mist: · · ·
```

State variants:

| State | Visual cue |
|---|---|
| Off | nozzle only |
| Low mist | sparse dots |
| Active | repeated dots |
| Heavy mist | denser dots / fog cue |
| Clogged | one dry nozzle + `!` |
| Leaking | drip when off |

Design rule: mist should remain localized unless the lab intentionally enters a fogged state.

### 8. Drain tray / runoff channel

Purpose: collect excess water and ground plant supports.

Best labs:

- Grow Bay Lab
- Propagation Lab
- Utility Lab
- Climate Lab
- Archive / Specimen Lab, subtle specimen-tray form

Support logic:

- sits under pots, trays, beds, hydro buckets, or shelves
- shows water leaving the root zone
- visually grounds plant supports
- can lead to drain / recovery system later

Terminal forms:

```text
┄┄┄┄┄┄ drain / runoff channel
```

```text
[pot] [pot] [pot]
════════ tray ════════
```

```text
╘════════════════╛ runoff tray
```

State variants:

| State | Visual cue |
|---|---|
| Dry | faint seam |
| Wet | denser seam / small dots |
| Draining | arrows toward drain |
| Overflow | `!` / puddle marks |
| Blocked | water accumulates near one point |
| Recovered | line returns toward reservoir |

Design rule: the drain tray is the simplest way to show water leaving the plant station. It should be part of the first Grow Bay prototype.

### 9. Nutrient bottle / dosing injector

Purpose: introduce nutrients, pH adjusters, or additive solutions into irrigation water.

Best labs:

- Utility Lab
- Climate Lab
- Grow Bay Lab, technical variant
- Propagation Lab, optional later

Support logic:

- should usually attach to reservoir, pump line, or dosing manifold
- connects to pH / EC readouts from Note 07
- makes fertigation visible without simulating full chemistry yet

Terminal forms:

```text
[N]── injector ── line
```

```text
bottle A  bottle B
  │         │
  └──[dose]─┘
```

```text
[pH-] [pH+] [N]
```

State variants:

| State | Visual cue |
|---|---|
| Available | bottle present |
| Dosing | arrow into line |
| Low | low fill / `!` |
| Empty | `x` |
| Wrong mix | `!` with pH / EC warning |
| Manual | `M` marker |
| Auto | `A` marker |

Design rule: nutrient bottles and injectors should mostly live in Utility / Climate contexts until water infrastructure is otherwise legible.

### 10. pH / EC reservoir readout

Purpose: monitor solution quality.

Best labs:

- Utility Lab
- Climate Lab
- Grow Bay Lab with hydro bucket / channel

Support logic:

- belongs on reservoir, dosing panel, hydro line, or control panel
- connects Note 07 sensors to Note 09 infrastructure
- should be exact only in readable/debug mode

Terminal forms:

```text
pH 6.2  EC 1.4
```

```text
[tank: pH ok / EC ok]
```

```text
[ pH ] [ EC ]
```

State variants:

| State | Visual cue |
|---|---|
| OK | stable values / `ok` |
| pH low | `pH↓` |
| pH high | `pH↑` |
| EC low | `EC↓` |
| EC high | `EC↑` |
| Drift | `~` |
| Calibration needed | `cal` |
| Sensor fault | `x` |

Design rule: pH / EC should be visually tied to a water container or line. Do not show it as free-floating text in simple labs.

### 11. Hydro bucket / channel connector

Purpose: connect hydroponic plant-support elements to the water / nutrient system.

Best labs:

- Grow Bay Lab
- Utility Lab
- Climate Lab
- Vines Lab, optional technical variant

Support logic:

- links hydro buckets / channels from Note 05 to reservoirs, pumps, drain lines, or return lines
- can show recirculating or drain-to-waste logic
- supports later technical lab modes

Terminal forms:

```text
╭────╮── tube
│H₂O │
╰─┬──╯
  └── return
```

```text
reservoir → pump → bucket → drain
```

```text
╞═o══o══o═╡── return
```

State variants:

| State | Visual cue |
|---|---|
| Connected | tube present |
| Flowing | arrow / dot movement |
| Low level | `!` on bucket |
| Blocked return | `!` at drain line |
| Leaking | drip below connector |
| Disconnected | broken tube |

Design rule: hydro support objects should not appear technical unless they visually connect to the water network.

---

## Closed-loop / recirculation concept

YAM does not need to simulate full water chemistry at first, but the visual system should allow two future modes:

### 1. Drain-to-waste

```text
reservoir → pump → emitter → pot / tray → drain → waste
```

Useful for simple Grow Bay / propagation setups.

### 2. Recirculating system

```text
reservoir → pump → hydro channel / bucket → return line → reservoir
```

Useful for hydroponic / technical labs.

Design rule: keep early visuals compatible with both modes. Do not force every first prototype into a complex loop.

---

## Water / nutrient mapping by lab

| Lab | Water / nutrient emphasis |
|---|---|
| Propagation Lab | nursery tray watering, humidity dome moisture, misting line, small drain tray, heat-mat-safe moisture |
| Climate Lab | test irrigation line, mist / humidity feed, cooling-pad water feed, pH / EC panel, valve / relay integration |
| Grow Bay Lab | drip line, three emitters, pot moisture tags, shared runoff tray, simple H₂O status |
| Vines Lab | drip line to base pots, wall-mounted moisture markers, optional hanging basket drip / catch tray |
| Utility Lab | main tank, pump, filter, manifold, nutrient bottles, pH / EC readout, hose, drain / recovery line |
| Archive / Specimen Lab | small jar / terrarium moisture state, tiny mist / humidity support, specimen-tray drainage |

---

## Minimal Grow Bay prototype water set

For the first Grow Bay Lab, keep it intentionally small:

```text
1. one shared drain tray / floor runoff seam
2. three short drip-line markers, one per pot
3. one small tube or manifold line above / beside the pots
4. optional water-status marker: H₂O ok / low / blocked
5. no complex fertigation yet
```

Prototype sketch:

```text
┌─ Grow Bay Lab ───────────────────────────── OK ─┐
│ temp ○  hum ○      L:ok      fan ◉  vent ▦     │
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

This adds visible greenhouse metabolism without turning the first Grow Bay prototype into a full plumbing diagram.

---

## Water state vocabulary

### Basic states

| State | Meaning |
|---|---|
| OK | system available and normal |
| Low | reservoir or water availability low |
| Flowing | water is moving |
| Dripping | emitter active |
| Blocked | line, filter, valve, or emitter clogged |
| Leaking | unintended water escape |
| Overflow | runoff exceeds tray / drain capacity |
| Dosing | nutrients / pH adjuster being injected |
| Recovered | runoff returns to reservoir |
| Fault | pump / valve / sensor failure |

### Compact markers

```text
H₂O ok   water normal
H₂O low  low supply
·        drip / small mist
···      mist / active small droplets
→        flow direction
!        blocked / leak / warning
x        fault
N        nutrient dosing
pH       acidity / alkalinity readout
EC       nutrient conductivity readout
```

Design rule: water-state markers should reuse the broader OK / WARN / FAULT language from sensors and actuators.

---

## Attachment and flow rules

### Tanks feed pumps

```text
tank ║──▣ pump── line
```

### Pumps feed filters or manifolds

```text
pump ▣──[F]──◇ valve
```

### Manifolds feed multiple bays

```text
main ──┬── p1
       ├── p2
       └── p3
```

### Emitters should align with plant supports

```text
·
╭──╮
│░░│
╰──╯
```

### Drains should sit below plant supports

```text
[pot] [pot] [pot]
┄┄┄┄┄ drain seam ┄┄┄┄┄
```

### pH / EC belongs to tanks, reservoirs, lines, or panels

```text
[tank: pH ok / EC ok]
```

Design rule: water infrastructure should express source → route → delivery → drainage.

---

## Terminal readability rules

### Avoid pipe spaghetti

A few clear lines are better than many decorative connections.

### Use flow only when meaningful

Do not animate or mark every pipe. Flow markers should appear when a system is active, faulty, or selected.

### Keep the first Grow Bay simple

One drip line and one drain seam are enough.

### Use Utility Lab for dense infrastructure

If a system needs tanks, pumps, filters, valves, nutrient bottles, and pH / EC panels, it probably belongs in Utility Lab first.

### Align water with plant stations

Drip / emitters should reinforce the bay structure already created by lamps and pots.

### Preserve layer order

Water infrastructure should not visually erase plants, pots, gauges, or lamps.

Suggested layering:

```text
Background: wall pipes / mainlines / tanks
Midground: manifolds / drip lines / sensors
Foreground: pots / trays / drain seams / active drips
```

---

## Relationship to Note 10 — Work / Maintenance Props and Tools

Water systems naturally imply maintenance objects:

- watering can
- hose coil
- bucket
- filter cartridge
- spare tubing
- nutrient bottle crate
- pH / EC calibration solution
- towel / mop
- tool rack
- repair tags
- maintenance notebook

Note 10 should probably cover these as **Work / Maintenance Props and Tools**, so they remain separate from functional water infrastructure.

---

## Open design questions

1. Should the first Grow Bay prototype show drip lines immediately, or start with only drain trays and moisture tags?
2. Should water infrastructure be simulated, decorative, or status-driven in the first implementation?
3. Should Utility Lab contain the main greenhouse reservoir from the beginning?
4. Should pH / EC readouts be hidden until hydro or fertigation systems exist?
5. Should visible pipes connect across labs, or should each lab abstract its local water network?
6. Should the greenhouse support recirculating and drain-to-waste visual modes?
7. Should leakage / overflow states be animated, or static warning marks first?
8. Should nutrient dosing be represented as bottles, injector panel, or both?

---

## Working conclusion

Water / nutrient infrastructure should make the greenhouse feel internally connected and metabolically alive.

The essential water grammar is:

```text
source
└── movement
    └── filtration / routing
        └── delivery
            └── plant support
                └── drainage / recovery
```

For the first greenhouse prototype, the Grow Bay Lab only needs a minimal water layer:

- one shared drain / runoff seam
- one simple drip line
- three emitter markers aligned with pots
- optional compact `H₂O ok / low / blocked` marker

This adds believable greenhouse function without overloading the established shell + support + light + sensor + actuator composition.
