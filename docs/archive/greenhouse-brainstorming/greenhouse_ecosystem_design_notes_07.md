
# Greenhouse Ecosystem Design Notes 07

## Sensor and Gauge Elements

### Purpose of this note

Notes 04–06 defined the greenhouse architectural shell, plant-support elements, and lighting elements.

Note 07 defines **sensor and gauge elements**: thermometers, hygrometers, soil moisture probes, light meters, CO₂ / air-quality indicators, pH / EC readouts, warning LEDs, control panels, bay-local sensor tags, and frame-level status readouts.

This note answers:

```text
How does each lab visibly measure, report, and warn about its environment?
```

Sensors and gauges are the greenhouse’s self-reading layer. They make the controlled environment legible before the user sees actuators such as fans, vents, heaters, foggers, pumps, or valves.

---

## Why sensors / gauges come next

The current design sequence is:

```text
04 Boundary / shell / lab frame
05 Plant-support elements
06 Lighting elements
07 Sensor and gauge elements
08 Climate-control actuators
09 Water / nutrient infrastructure
```

Sensors should come before actuators because they define what the greenhouse knows about itself.

Real greenhouse climate-control logic commonly centers on measuring and managing environmental factors such as:

- temperature
- humidity
- ventilation / airflow
- light intensity
- CO₂ concentration
- irrigation / watering state
- nutrient condition
- pH and EC, especially in irrigation or fertigation systems
- outside weather or external conditions

Commercial greenhouse-control systems often describe sensors as the input layer for climate computers, automation systems, actuator controls, lighting, irrigation, and energy management. The design implication for YAM is clear: sensors are not decorative meters. They are the visible evidence that the lab is a controlled environment.

---

## Core sensor model

Every sensor / gauge element should answer seven questions:

1. **Variable** — What does it measure?
2. **Scope** — Is the reading greenhouse-global, lab-local, bay-local, or plant-local?
3. **Display form** — Is it analog, digital, symbolic, textual, or a tiny status marker?
4. **Attachment** — Is it mounted on a wall, rail, pot, tray, pipe, panel, shelf, or greenhouse frame?
5. **State** — Is the reading normal, warning, fault, offline, stale, or calibrating?
6. **Action relationship** — Which actuator or maintenance response could later use this reading?
7. **Readability level** — Should the user read exact values or only infer state?

Design rule: a sensor should usually be attached to something physical. Avoid floating readouts unless they belong to the greenhouse frame / HUD.

---

## Scope levels

### 1. Greenhouse-frame level

Global readings that persist across lab switches.

Possible elements:

- global clock
- outside weather capsule
- global alert count
- greenhouse climate summary
- active lab status marker
- overall system OK / WARN / FAULT indicator

Example:

```text
╔═ YAM GREENHOUSE ═════════════════════════════ 08:21:46  OK:03 ═╗
║ [Propagation] [Climate] [Grow Bay] [Vines] [Utility] [Archive] ║
```

Use frame-level readings sparingly. They should orient the user, not replace lab-local instruments.

### 2. Lab-local level

Readings for the active lab as a whole.

Possible elements:

- wall-mounted temperature gauge
- wall-mounted humidity gauge
- CO₂ / air-quality marker
- light-level meter
- climate control panel
- lab status corner

Example:

```text
┌─ Grow Bay Lab ───────────────────────────── OK ─┐
│ temp ○  hum ○  light ●                         │
```

Lab-local readings are the main sensor layer for early prototypes.

### 3. Bay-local level

Readings tied to one grow station, pot, tray, lamp, or hydro bucket.

Possible elements:

- bay moisture tag
- pot probe
- lamp intensity marker
- local warning LED
- bay number / plant tag

Example:

```text
╭────╮        ╭────╮        ╭────╮
 ╲  ╱          ╲  ╱          ╲  ╱
[p1 42%]      [p2 dry!]     [p3 ok]
```

Bay-local readings preserve the HighGrow-like station logic: each bay can have its own condition.

### 4. Plant-local level

Readings directly attached to a plant or support object.

Possible elements:

- soil probe in pot
- specimen jar tag
- tray cell state
- leaf / stress marker, later

Plant-local readings should be minimal at first. Too many tiny readouts can turn the scene into a spreadsheet.

---

## Primary sensor / gauge element families

### 1. Analog gauge

Purpose: retro, readable, HighGrow-like environmental instrument.

Best labs:

- Climate Lab
- Grow Bay Lab
- Utility Lab
- greenhouse frame side rail, if used

Support logic:

- usually wall-mounted or rail-mounted
- can summarize temperature, humidity, pressure, airflow, or CO₂ state
- excellent for visual identity
- good bridge from HighGrow’s left-side gauge rail

Terminal forms:

```text
╭────╮
│╲   │
│ ╲  │
╰────╯
```

```text
 ○ temp
```

```text
[○ 21°]
```

State variants:

| State | Visual cue |
|---|---|
| Normal | centered or stable needle |
| Low | needle left / low marker |
| High | needle right / high marker |
| Warning | `!` beside dial |
| Fault | broken needle or `x` marker |
| Calibrating | small `~` or moving marker |

Design rule: analog gauges should communicate state more than exact numeric precision.

### 2. Thermometer

Purpose: core temperature readout.

Best labs:

- all labs
- strongest in Climate, Grow Bay, Propagation

Support logic:

- wall-mounted or frame-level
- may pair with humidity gauge
- may later drive heaters, vents, fans, shade, and alarm states

Terminal forms:

```text
🌡 21°
```

```text
T:21°
```

```text
│°│ temp
│█│
╰─╯
```

State variants:

| State | Visual cue |
|---|---|
| Cool | low bar / low marker |
| Stable | normal value |
| Hot | high bar / `!` |
| Rapid change | arrow marker |
| Sensor fault | `T:x` |

Design rule: temperature is a high-priority lab-local reading, but exact numeric display can be optional in visual mode.

### 3. Hygrometer / humidity gauge

Purpose: humidity and moisture-air readout.

Best labs:

- Propagation Lab
- Climate Lab
- Grow Bay Lab
- Archive / Specimen Lab

Support logic:

- pairs naturally with thermometer
- important for humidity domes, foggers, vents, and condensation states
- can be analog or compact digital

Terminal forms:

```text
H:63%
```

```text
○ hum
```

```text
[hum 63]
```

State variants:

| State | Visual cue |
|---|---|
| Dry | low marker / sparse dots |
| Stable | normal value |
| Humid | condensation dot cue |
| Too humid | `!` / fog marker |
| Fault | `H:x` |

Design rule: humidity can also be shown indirectly through condensation / fog motifs, especially in Propagation and Archive labs.

### 4. Soil moisture probe

Purpose: pot-local or bay-local substrate moisture readout.

Best labs:

- Grow Bay Lab
- Propagation Lab
- Vines Lab
- Utility Lab, as test equipment

Support logic:

- attaches to pot, tray, bed, or hydro medium
- directly connects plant-support elements to environmental state
- ideal for bay-local status tags

Terminal forms:

```text
[p1 42%]
```

```text
╭──╮
│░░│╿ probe
╰──╯
```

```text
M:42
```

State variants:

| State | Visual cue |
|---|---|
| Dry | `dry`, crack marker, low value |
| OK | compact `ok` or stable value |
| Wet | drip marker |
| Overwatered | `!` plus tray / puddle cue |
| Probe fault | missing probe or `M:x` |

Design rule: soil moisture is probably the first plant-local sensor to prototype because it maps directly onto pots and trays.

### 5. Light meter / PAR-style indicator

Purpose: connects sensor logic to Note 06 lighting.

Best labs:

- Grow Bay Lab
- Climate Lab
- Propagation Lab
- Vines Lab

Support logic:

- may be lab-local or bay-local
- can show whether lighting coverage is adequate
- can detect misaligned lamps, shading, or night mode

Terminal forms:

```text
L:ok
```

```text
lux ●
```

```text
[light ▂▅▇]
```

State variants:

| State | Visual cue |
|---|---|
| Dark | empty / low bar |
| Low | dotted marker |
| OK | stable dot / mid bar |
| High | dense marker / warning |
| Shaded | shade glyph paired with low reading |
| Sensor fault | `L:x` |

Design rule: light readings should align with visible lamp state where possible. If lamps are on but light reads low, that becomes meaningful.

### 6. CO₂ / air-quality indicator

Purpose: make the Climate Lab and advanced Grow Bay feel technical.

Best labs:

- Climate Lab
- Grow Bay Lab, optional
- greenhouse frame / global summary, optional

Support logic:

- lab-local by default
- can pair with ventilation, fans, vents, and gas handling later
- should stay visually compact

Terminal forms:

```text
CO₂ ok
```

```text
air ●
```

```text
[CO₂  ppm]
```

State variants:

| State | Visual cue |
|---|---|
| Low | down marker |
| OK | stable dot |
| High | `!` |
| Venting | arrow / airflow cue |
| Fault | `CO₂:x` |

Design rule: CO₂ is useful for Climate Lab identity, but not required for the first Grow Bay prototype.

### 7. pH / EC nutrient readout

Purpose: bridge from sensors into future water / nutrient infrastructure.

Best labs:

- Utility Lab
- Climate Lab
- Grow Bay Lab with hydro bucket
- Propagation Lab, optional technical variant

Support logic:

- usually tied to reservoir, hydro bucket, irrigation line, or nutrient channel
- should not appear without water / nutrient infrastructure context
- becomes more important in Note 09

Terminal forms:

```text
pH 6.2
EC 1.4
```

```text
[pH] [EC]
```

```text
tank: pH ok / EC !
```

State variants:

| State | Visual cue |
|---|---|
| OK | compact stable marker |
| Too low | down marker |
| Too high | up marker |
| Drift | `~` marker |
| Fault | `pH:x` / `EC:x` |
| Needs calibration | small wrench / `cal` marker |

Design rule: pH / EC should be mostly Utility / hydro-specific. Do not clutter simple pot labs with nutrient instrumentation too early.

### 8. Warning LED / alarm marker

Purpose: compact status communication.

Best labs:

- all labs
- strongest in Climate and Utility
- useful in greenhouse frame tab strip

Support logic:

- can appear on control panels, lab tabs, equipment, pots, lamp fixtures, or status corners
- communicates abnormal state without long text
- should be tiny and consistent

Terminal forms:

```text
● OK
! WARN
x FAULT
```

```text
[!]
```

```text
panel: o o ●
```

State variants:

| State | Visual cue |
|---|---|
| OK | small steady dot |
| Warning | `!` marker |
| Fault | `x` marker |
| Offline | hollow dot / blank marker |
| Maintenance | wrench / `m` marker |

Design rule: warnings should be state markers, not walls of text.

### 9. Control panel

Purpose: concentrated local dashboard embedded in the lab world.

Best labs:

- Climate Lab
- Utility Lab
- Grow Bay Lab, as small side panel
- Propagation Lab, compact variant

Support logic:

- hosts multiple small readouts
- can connect sensors to actuator state later
- should mount on wall, service panel, or equipment rack
- useful for Climate Lab’s identity

Terminal forms:

```text
┌─ panel ─┐
│ T 21°   │
│ H 63%   │
│ L ok    │
└─────────┘
```

```text
[ T:21 H:63 L:ok ]
```

```text
╞══ control ══╡
  o o ●  T/H
```

State variants:

| State | Visual cue |
|---|---|
| Normal | compact values |
| Warning | one line with `!` |
| Fault | panel corner `x` |
| Manual override | `M` marker |
| Auto mode | `A` marker |
| Calibrating | `cal` marker |

Design rule: control panels should concentrate data so individual gauges do not overwhelm the lab.

### 10. Bay-local sensor tag

Purpose: tie environmental state to a specific plant station.

Best labs:

- Grow Bay Lab
- Propagation Lab
- Vines Lab
- Archive Lab, as specimen label variant

Support logic:

- combines identity + small state readout
- can attach to pot, lamp rail, tray, shelf, or bay divider
- ideal for three-station HighGrow-like layouts

Terminal forms:

```text
[p1 ok]
[p2 dry!]
[p3 wet]
```

```text
╭─ B1 ─╮
│ M ok │
╰──────╯
```

```text
tag: B2 / M:42 / L:ok
```

State variants:

| State | Visual cue |
|---|---|
| Normal | `ok` |
| Dry | `dry!` |
| Wet | `wet!` |
| Low light | `L-` |
| Overlight | `L!` |
| Unknown | `?` |

Design rule: bay tags are the safest early way to show plant-local condition without rendering complex plant physiology.

---

## Sensor and gauge mapping by lab

| Lab | Sensor / gauge emphasis |
|---|---|
| Propagation Lab | thermometer, hygrometer, humidity-dome state, tray moisture, gentle light meter |
| Climate Lab | full control panel, temp / humidity / CO₂ / light gauges, warning LEDs, outside-condition readout |
| Grow Bay Lab | temp / humidity pair, three bay moisture tags, lamp/light status marker, lab status corner |
| Vines Lab | wall moisture markers, light meter, airflow marker, trellis-local tags |
| Utility Lab | pump / tank status, pH / EC, pressure gauge, warning LEDs, maintenance panel |
| Archive / Specimen Lab | specimen tags, jar humidity / condensation state, small temp/humidity readout, quiet alarm marker |

---

## Frame-level vs lab-local vs plant-local readings

### Frame-level readings

Use for:

- clock
- active lab tabs
- global alert count
- outside weather / day-night context
- overall greenhouse mode

Avoid putting too many exact values in the frame.

### Lab-local readings

Use for:

- temperature
- humidity
- light summary
- CO₂ / air quality in technical labs
- lab status corner
- local control panel

This is the main readout layer.

### Plant-local / bay-local readings

Use for:

- soil moisture
- pot / tray condition
- lamp coverage status
- plant station warning
- specimen identity

Keep these compact.

---

## Minimal Grow Bay prototype sensor set

For the first Grow Bay Lab, include only:

```text
1. one wall-mounted temperature / humidity gauge pair
2. three tiny bay-local moisture tags
3. one light-status marker tied to the lamp rail
4. one lab status corner: OK / WARN / FAULT
```

Prototype sketch:

```text
┌─ Grow Bay Lab ───────────────────────────── OK ─┐
│ temp ○  hum ○      L:ok                         │
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

This preserves HighGrow’s gauge-and-station logic while keeping the first prototype readable.

---

## Sensor state vocabulary

### Basic sensor states

| State | Meaning |
|---|---|
| OK | reading is normal |
| Low | variable is below desired range |
| High | variable is above desired range |
| Warning | reading deserves attention |
| Fault | sensor or connected system is broken |
| Offline | no current reading |
| Stale | reading exists but is old |
| Calibrating | sensor is temporarily adjusting |
| Manual override | user/control state suppresses automation |

### Compact markers

```text
ok   normal
!    warning
x    fault
?    unknown / missing
~    drift / calibrating
↑    high
↓    low
●    active / stable
○    passive / analog / inactive
```

Design rule: markers should be consistent across sensors, actuators, and lab tabs.

---

## Terminal readability rules

### Prefer paired short labels

Good:

```text
T:21 H:63
```

Better for atmospheric mode:

```text
temp ○  hum ○
```

Avoid large dashboard blocks unless the lab is specifically the Climate Lab.

### Use gauges as landmarks

A few wall-mounted gauges help the lab feel functional. Too many turn the scene into visual static.

### Use exact numbers only when valuable

For idle visualizer mode, `ok`, `dry!`, or `hot!` may be more readable than exact values.

For debug / simulation mode, exact values can appear in panels or tags.

### Tie warnings to objects

Bad:

```text
WARNING: SOIL MOISTURE LOW
```

Better:

```text
[p2 dry!]
```

### Keep the HighGrow gauge-rail idea, but adapt it

HighGrow’s left gauge rail is useful as inspiration. YAM can adapt it as:

- lab-local wall instruments
- compact side strip
- frame-level status capsule
- Climate Lab instrument wall

Do not copy the exact desktop-app UI chrome.

---

## Relationship to next notes

### Note 08 — Climate-Control Actuators

Sensors define what the greenhouse knows.

Actuators define how the greenhouse responds.

Expected actuator families:

- vents
- fans
- louvers
- heaters
- foggers
- humidifiers
- cooling pads
- ducts
- circulation fans
- shade / curtain motors

### Note 09 — Water / Nutrient Infrastructure

Sensor groundwork also prepares the water / nutrient layer.

Expected infrastructure families:

- tanks
- reservoirs
- pumps
- valves
- pipes
- drip lines
- misting nozzles
- drain trays
- nutrient bottles
- pH / EC-linked reservoirs

---

## Open design questions

1. Should the first Grow Bay prototype show exact numeric temperature / humidity, or only symbolic gauge state?
2. Should the gauge rail be global, lab-local, or only used in Climate Lab?
3. Should bay-local tags attach to pots, lamp rail, or bay dividers?
4. Should sensor warnings appear in lab tabs, status corners, or both?
5. Should pH / EC be deferred until water / nutrient infrastructure is designed?
6. Should sensor state be visually simulated even before real plant state exists?
7. Should YAM support display modes: atmospheric, readable, debug?

---

## Working conclusion

Sensor and gauge elements should be designed before climate-control actuators.

The essential sensor grammar is:

```text
sensor / gauge
└── variable measured
    └── scope: frame / lab / bay / plant
        └── state marker
            └── possible actuator or maintenance response later
```

For the first greenhouse prototype, the Grow Bay Lab only needs a small set of sensors:

- wall-mounted temp / humidity gauge pair
- three bay-local moisture tags
- light status marker tied to the lamp rail
- lab status corner

This gives the greenhouse a readable self-monitoring layer while preserving the clean room + lamp + pot composition established in earlier notes.
