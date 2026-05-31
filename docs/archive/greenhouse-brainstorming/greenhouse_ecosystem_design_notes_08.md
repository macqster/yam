
# Greenhouse Ecosystem Design Notes 08

## Climate-Control Actuators

### Purpose of this note

Notes 04–07 defined the greenhouse shell, plant-support layer, lighting layer, and sensor / gauge layer.

Note 08 defines **climate-control actuators**: the visible in-world devices that respond to measured conditions and change the lab environment.

This note answers:

```text
How does each lab visibly change temperature, humidity, airflow, ventilation, cooling, and climate state?
```

Sensors and gauges define what the greenhouse knows. Actuators define how the greenhouse responds.

---

## Why actuators follow sensors

The current design sequence is:

```text
04 Boundary / shell / lab frame
05 Plant-support elements
06 Lighting elements
07 Sensor and gauge elements
08 Climate-control actuators
09 Water / nutrient infrastructure
```

This order matters because actuator behavior becomes meaningful only after the scene has a sensor/readout language.

Real greenhouse climate systems typically combine sensors, controllers, and actuators. Modern descriptions of greenhouse climate systems explicitly group sensors for humidity, temperature, light, and CO₂ with actuators for ventilation, heating, cooling, shading systems, artificial lighting, irrigation, and energy management. Greenhouse heating / ventilation / cooling guidance also describes systems such as fans, evaporative cooling pads, fogging, and heating as ways to keep the crop environment within acceptable ranges.

Design implication for YAM:

```text
sensor → decision / controller → actuator → visible room state
```

Actuators should be visible infrastructure, not random props. A fan, vent, heater, fogger, cooling pad, shade motor, or duct should visibly attach to the shell, rail, wall, pipe, ceiling, or control panel.

---

## Core actuator model

Every climate actuator should answer eight questions:

1. **Variable affected** — temperature, humidity, airflow, ventilation, light / heat load, or air quality?
2. **Trigger relationship** — which sensor or control state could activate it?
3. **Attachment** — wall, roofline, ceiling rail, duct, pipe, floor, shelf, or control box?
4. **Coverage** — whole lab, one bay, one shelf, one wall, or one plant station?
5. **State** — off, on, low, high, automatic, manual, blocked, fault, maintenance?
6. **Motion cue** — fan spin, open louver, airflow marks, mist, heat shimmer, moving curtain?
7. **Visual footprint** — small marker, wall object, broad surface, large infrastructure?
8. **Lab fit** — which lab uses it naturally?

Design rule: an actuator should imply a physical mechanism and an environmental consequence.

---

## Primary actuator families

### 1. Vent / louver / roof opening

Purpose: passive or automated air exchange.

Best labs:

- Grow Bay Lab
- Climate Lab
- Propagation Lab
- greenhouse frame / exterior shell

Support logic:

- belongs on wall, roofline, side panel, or glazing
- pairs naturally with temperature, humidity, and CO₂ sensors
- can be passive, manual, or motorized
- creates visible open / closed state

Terminal forms:

```text
vent ▦
```

```text
┌────┐
│////│ louver
└────┘
```

```text
roof:  /── open
      /  \
```

State variants:

| State | Visual cue |
|---|---|
| Closed | solid grille / flat louver |
| Partially open | angled slats |
| Open | gap / raised roofline |
| Automated | small motor marker |
| Blocked | `!` beside vent |
| Fault | `x` or stuck-open / stuck-closed cue |

Design rule: vents should be part of the wall / roof shell, not freestanding objects.

### 2. Exhaust fan

Purpose: remove hot, humid, or stale air from the lab.

Best labs:

- Climate Lab
- Grow Bay Lab
- Utility Lab
- greenhouse exterior wall

Support logic:

- usually wall-mounted or duct-mounted
- pairs with vents / inlets
- strong visible motion cue
- can link to high temperature, high humidity, or air-quality warning

Terminal forms:

```text
fan ◉
```

```text
╭────╮
│ ◉  │
╰────╯
```

```text
▦→◉ exhaust
```

State variants:

| State | Visual cue |
|---|---|
| Off | static fan glyph |
| Low | small motion mark |
| High | stronger spin / airflow arrows |
| Reversing | opposite arrow |
| Blocked | `!` at grille |
| Fault | `x` / broken blade cue |

Design rule: exhaust fans should be mounted into a wall, duct, or service panel. They should not float in the lab.

### 3. Circulation fan

Purpose: mix air inside the lab and reduce stagnant pockets.

Best labs:

- Grow Bay Lab
- Vines Lab
- Propagation Lab
- Climate Lab

Support logic:

- may hang from ceiling, mount to wall, or sit on a rack
- affects internal airflow more than air exchange
- makes the room feel alive with a small animation opportunity
- can support uniform temperature / humidity logic

Terminal forms:

```text
◉
```

```text
fan ◉  ~~~
```

```text
  ┐
  ◉  ceiling fan / bracket cue
```

State variants:

| State | Visual cue |
|---|---|
| Off | static glyph |
| On | spin marker / airflow trail |
| Oscillating | alternating airflow direction |
| Too strong | warning near plants |
| Fault | stuck glyph / `x` |

Design rule: circulation fans are good low-cost “room is alive” elements, but should remain small so they do not dominate the lab.

### 4. Adjustable air inlet

Purpose: admit outside air into the greenhouse or lab.

Best labs:

- Climate Lab
- Grow Bay Lab
- greenhouse frame

Support logic:

- pairs with exhaust fans
- belongs on wall / side panel / exterior-facing shell
- can be passive opening, motorized inlet, or simple grille

Terminal forms:

```text
inlet ▤
```

```text
← ▦ air
```

```text
┌────┐
│ ↙↙ │
└────┘
```

State variants:

| State | Visual cue |
|---|---|
| Closed | flat panel |
| Open | arrows / gap |
| Filtered | mesh cue |
| Blocked | `!` |
| Motorized | small actuator box |

Design rule: inlets should visually pair with fans or vents so airflow has a believable path.

### 5. Heater

Purpose: raise temperature or prevent cold stress.

Best labs:

- Propagation Lab
- Climate Lab
- Grow Bay Lab
- Archive / Specimen Lab, as subtle thermal protection

Support logic:

- floor, wall, pipe, bench, or mat-based variants
- pairs with thermometer and low-temperature state
- can produce heat shimmer or warning marker
- should be visually contained and not look like random furniture

Terminal forms:

```text
heater ▤
```

```text
[heat]
  ~~~
```

```text
mat ▔▔▔ under tray
```

State variants:

| State | Visual cue |
|---|---|
| Off | heater box only |
| On | subtle `~~~` heat shimmer |
| High | stronger shimmer / `!` |
| Fault | `x` / cold warning persists |
| Mat heat | warm line under tray |

Design rule: heating can be represented as a small box, pipe, or mat. Avoid making it visually louder than lamps unless there is a warning state.

### 6. Cooling pad / evaporative wall

Purpose: cool incoming air through evaporative cooling.

Best labs:

- Climate Lab
- Utility Lab
- Grow Bay Lab, as wall infrastructure

Support logic:

- normally works with exhaust fans drawing air through wet pads
- belongs on a wall surface, often opposite fans
- can also visually imply humidity increase
- bridges climate control and water infrastructure

Terminal forms:

```text
cool pad ▒▒▒
```

```text
╞▒▒▒▒▒▒╡ wet pad wall
```

```text
air → ▒▒▒ → lab
```

State variants:

| State | Visual cue |
|---|---|
| Dry / inactive | faint panel |
| Wet / active | denser panel + drip / flow cue |
| Cooling | arrows through pad |
| Clogged | `!` / broken flow |
| Winter-closed | cover panel |

Design rule: cooling pads are wall-scale infrastructure, not small devices. They should occupy part of a service wall or exterior-facing side.

### 7. Fogger / humidifier / misting actuator

Purpose: raise humidity, cool air, or support delicate propagation conditions.

Best labs:

- Propagation Lab
- Climate Lab
- Grow Bay Lab, optional
- Archive / Specimen Lab, very subtle variant

Support logic:

- attaches to pipe, nozzle line, shelf, ceiling rail, or small humidifier unit
- pairs with humidity sensors and fog / condensation states
- visually communicates microclimate immediately

Terminal forms:

```text
nozzle · · ·
```

```text
pipe ───┬───
        ··· mist
```

```text
[humid] ~~~
```

State variants:

| State | Visual cue |
|---|---|
| Off | nozzle / unit only |
| Low | sparse dots |
| On | visible mist dots |
| Heavy | denser fog / reduced clarity |
| Clogged | missing mist + `!` |
| Overhumid | fog + humidity warning |

Design rule: mist glyphs can quickly obscure the room. Use them as localized cues, not full-screen fog, unless the lab is explicitly in a fogged state.

### 8. Shade / curtain motor

Purpose: move shade cloth, thermal curtain, or blackout screen.

Best labs:

- Climate Lab
- Grow Bay Lab
- Propagation Lab
- greenhouse frame / roofline

Support logic:

- connects Note 06 lighting elements to climate-control actuation
- affects radiation, heat load, and sometimes night insulation
- belongs on track, roofline, glazing, or side curtain rail

Terminal forms:

```text
motor ▣══ track ═════
shade ▒▒▒▒──── open
```

```text
[shade M] ▒▒▒────
```

```text
╱╲╱╲ curtain folds
```

State variants:

| State | Visual cue |
|---|---|
| Open | shade stored at side / top |
| Partially drawn | partial screen span |
| Closed | full shade span |
| Moving | arrow along track |
| Jammed | `!` at motor |
| Manual override | `M` marker |

Design rule: shade motors are actuators; shade cloth itself is a lighting / shell element. Keep the distinction clear.

### 9. Duct / air-distribution tube

Purpose: distribute air through the lab.

Best labs:

- Climate Lab
- Utility Lab
- Grow Bay Lab
- Propagation Lab, as small ducting

Support logic:

- can run along ceiling, wall, or service rail
- pairs with fans, heaters, cooling pads, or dehumidification later
- provides excellent architectural infrastructure
- can show airflow with subtle arrows or vents

Terminal forms:

```text
duct ════════════
```

```text
══════╤══════╤══════
      ↓      ↓
```

```text
perforated tube: ═·═·═·═·═
```

State variants:

| State | Visual cue |
|---|---|
| Idle | duct only |
| Active | subtle arrows / outlet marks |
| Blocked | `!` at segment |
| Leaking | airflow escaping at break |
| Filtered | filter box segment |

Design rule: ducting is a shell/infrastructure bridge. It should help make labs feel engineered rather than cluttered.

### 10. Control relay / actuator box

Purpose: make automation visible.

Best labs:

- Climate Lab
- Utility Lab
- Grow Bay Lab, small variant

Support logic:

- receives or represents controller output
- connects sensor panel to physical actuators
- can show auto/manual/fault state
- useful when multiple actuators would otherwise feel disconnected

Terminal forms:

```text
┌─ relay ─┐
│ fan ●   │
│ vent ○  │
│ heat x  │
└─────────┘
```

```text
[ACT: fan● vent○ heat○]
```

```text
box ▣── fan
```

State variants:

| State | Visual cue |
|---|---|
| Auto | `A` marker |
| Manual | `M` marker |
| Output active | `●` |
| Output inactive | `○` |
| Fault | `x` |
| Maintenance | wrench / `m` |

Design rule: relay boxes are not required in every lab. They are strongest in Climate and Utility contexts.

---

## Actuator mapping by lab

| Lab | Actuator emphasis |
|---|---|
| Propagation Lab | heat mats, small humidifier / mist nozzle, gentle circulation fan, small vent |
| Climate Lab | full actuator wall: vents, fans, ducts, heater, cooling pad, fogger, shade motor, relay box |
| Grow Bay Lab | wall vent, small circulation fan, optional heater, optional shade state, actuator status marker |
| Vines Lab | circulation fan, wall vent, trellis airflow markers, optional shade curtain |
| Utility Lab | pumps / relay boxes / ducting / service fans / heating equipment stored or connected |
| Archive / Specimen Lab | very subtle climate protection: small fan, heater, humidity control, alarm marker |

---

## Actuator state vocabulary

### Basic actuator states

| State | Meaning |
|---|---|
| Off | present but inactive |
| On | active normal operation |
| Low | reduced output |
| High | strong output |
| Auto | controlled by sensor / schedule |
| Manual | user / maintenance override |
| Blocked | airflow, vent, pad, or nozzle obstructed |
| Fault | actuator cannot operate correctly |
| Maintenance | actuator available but being serviced |
| Moving | transitional state such as vent opening or shade drawing |

### Compact markers

```text
●    active
○    inactive / passive
!    warning / blocked
x    fault
A    auto
M    manual override
~    heat / airflow / humidity motion
→    air direction
···  mist / fog
▒    shade / pad / screen surface
```

Design rule: actuator markers should reuse the sensor state vocabulary from Note 07 whenever possible.

---

## Attachment rules

### Vents belong to walls or rooflines

```text
wall │ vent ▦ │
roof /── open
```

### Fans belong to walls, ducts, brackets, or rails

```text
│ fan ◉ │
```

### Foggers belong to pipes or nozzle lines

```text
pipe ───┬───
        ···
```

### Cooling pads occupy wall surfaces

```text
╞▒▒▒▒▒▒╡ wet pad wall
```

### Shade motors belong to tracks

```text
motor ▣══ track ═════
shade ▒▒▒────
```

### Heaters belong to floors, walls, benches, pipes, or mats

```text
[heat] ~~~
mat ▔▔▔ under tray
```

Design rule: do not allow climate-control equipment to float. Every actuator should be anchored to the shell, a support, or an infrastructure line.

---

## Minimal Grow Bay prototype actuator set

For the first Grow Bay Lab, keep the actuator set small:

```text
1. one wall vent / louver
2. one small circulation fan
3. optional heater or heat warning marker
4. optional shade / curtain state inherited from Note 06
5. one actuator status marker tied to the sensor panel
```

Prototype sketch:

```text
┌─ Grow Bay Lab ───────────────────────────── OK ─┐
│ temp ○  hum ○      L:ok      fan ◉  vent ▦     │
│ ╞══╪══╪══╪══╪══╪══╪══╪══╪══╡ back wall       │
│ ━━━━━━━━━━━━━━━━━ lamp rail ━━━━━━━━━━━━━━━━━ │
│      │                  │                  │   │
│   ╭────╮             ╭────╮             ╭────╮ │
│    ╲  ╱               ╲  ╱               ╲  ╱  │
│   [p1 ok]            [p2 dry!]          [p3 ok]│
│    ╭──╮               ╭──╮               ╭──╮  │
│    │░░│               │░░│               │░░│  │
│    ╰──╯               ╰──╯               ╰──╯  │
│ ┄┄┄┄┄┄┄┄┄ drain / floor seam ┄┄┄┄┄┄┄┄┄┄      │
└────────────────────────────────────────────────┘
```

This preserves the established Grow Bay composition while adding the first evidence that the room can actively regulate its climate.

---

## Later prototype extensions

After the minimal Grow Bay actuator pass works, expand to:

1. Climate Lab actuator wall
2. Propagation heat mat + misting line
3. Vines Lab airflow / fan logic
4. Cooling pad + exhaust fan pairing
5. Shade motor and moving curtain state
6. Fan speed or oscillation animation
7. Vent open / closed animation
8. Fogging / condensation state
9. Heater-on / heat-warning state
10. Relay panel connecting sensors to actuators

---

## Relationship to Note 09 — Water / Nutrient Infrastructure

Some actuators overlap with water systems:

- foggers need water supply
- cooling pads need recirculating water
- humidifiers may need reservoir state
- hydro buckets / channels may require pumps and valves
- drain trays and runoff channels respond to moisture state

Note 08 should define these as climate actuators only when their primary visible role is air / humidity / temperature control.

Note 09 should separately define the greenhouse’s water and nutrient metabolism: reservoirs, tanks, pumps, pipes, valves, drip lines, misting supplies, nutrient bottles, pH / EC reservoirs, and drains.

---

## Open design questions

1. Should vents and fans be represented as separate objects, or combined into compact airflow modules?
2. Should actuator state be visible in atmospheric mode, or mostly in readable / debug modes?
3. Should fan spin / airflow animate, or use static glyph variants first?
4. Should shade motors belong to the lighting system, climate system, or both?
5. Should fogging be allowed in the first visual prototype, or deferred because it can obscure plants?
6. Should Climate Lab be the first place where complex actuator panels appear?
7. Should Grow Bay Lab stay minimal: one fan + one vent only?
8. Should actuator state be driven by simulated sensors eventually, or remain decorative/status-only in early versions?

---

## Working conclusion

Climate-control actuators should make the greenhouse feel like a responsive controlled environment.

The essential actuator grammar is:

```text
sensor reading
└── controller / decision state
    └── actuator output
        └── visible room consequence
```

For the first greenhouse prototype, the Grow Bay Lab only needs a small actuator set:

- one wall vent / louver
- one small circulation fan
- optional heat or shade state
- one actuator status marker tied to the sensor panel

This gives the room a visible climate-response layer without overwhelming the established shell + plant-support + lighting + gauge composition.
