
# Greenhouse Ecosystem Design Notes 06

## Lighting Elements

### Purpose of this note

Notes 01–05 established the greenhouse ecosystem as:

```text
Greenhouse Frame
└── switchable Labs
    ├── shell / boundary layer
    ├── plant-support elements
    └── functional equipment families
```

Note 06 defines **lighting elements**: lamps, lamp bars, reflectors, chains, cables, light cones, shade cloth, curtains, and lighting states.

This note answers:

```text
How does the greenhouse visibly deliver, shape, block, schedule, and communicate light?
```

Lighting is one of the strongest element families for YAM because it is both functional and architectural. It divides space into plant stations, explains growth conditions, creates atmosphere, and gives the terminal scene a clear vertical hierarchy.

---

## Why lighting comes next

Plant-support elements define where plants live. Lighting defines what each plant station receives from above.

This directly follows the HighGrow-inspired grammar:

```text
lamp
 ↓
light cone
 ↓
pot / plant anchor
```

Lighting also connects several greenhouse systems:

- growth support
- photoperiod / schedule
- heat load
- shading / cooling
- energy use
- lab identity
- warning states

Real greenhouse sources distinguish lighting strategies such as **supplemental lighting** for increasing photosynthetic light during low-radiation periods and **photoperiodic lighting** for influencing plant daylength response. Greenhouse lighting is also commonly considered together with shade / curtain systems, because light control affects temperature, humidity, energy use, and plant stress.

Design implication: YAM lighting should not be decorative glow. It should be visible infrastructure with states, attachment logic, and readable consequences.

---

## Core lighting model

Every lighting element should answer six questions:

1. **Source** — What emits or modifies light?
2. **Attachment** — Where is it mounted?
3. **Coverage** — Does it serve one plant, one shelf, one bay, or the whole lab?
4. **State** — Is it on, off, dimmed, overheated, shaded, broken, or scheduled?
5. **Visual field** — Does it cast a cone, bar, panel wash, dotted shimmer, or no visible beam?
6. **System role** — Is it growth light, photoperiod cue, inspection light, warning light, or shading / light-blocking equipment?

---

## Primary lighting element families

### 1. Overhead grow lamp

Purpose: default per-plant or per-bay light source.

Best labs:

- Grow Bay Lab
- Vines Lab
- Climate Lab, as test equipment
- Propagation Lab, in smaller form

Support logic:

- usually mounted to a ceiling rail, chain, bracket, or arm
- pairs naturally with one pot / bucket below
- can create visible cone
- strongest direct carryover from HighGrow layout grammar

Terminal forms:

```text
╭────╮
│LED │
╰────╯
```

```text
.-====-.
```

```text
╭────╮
╰─┬──╯
  │ chain
```

Light cone forms:

```text
 ╲    ╱
  ╲  ╱
   \/
```

```text
  ·  ·
 ·    ·
  ·  ·
```

```text
 ╲░░╱
  ╲╱
```

States:

| State | Visual cue |
|---|---|
| Off | lamp hood only, no cone |
| Low | sparse cone, dotted beam |
| On | clear cone |
| High | denser cone, stronger boundary |
| Overheated | `!`, flicker marker, unstable cone |
| Misaligned | cone offset from pot |
| Broken | tilted fixture, split cable, no cone |

Design rule: overhead lamps should visibly attach to rails or chains. Never let them float.

### 2. Lamp bar / shelf light

Purpose: even horizontal lighting for trays, shelves, propagation racks, or bench surfaces.

Best labs:

- Propagation Lab
- Archive / Specimen Lab
- Grow Bay Lab, as bench lighting
- Climate Lab, as controlled test strip

Support logic:

- mounted under shelf, across rack, or along ceiling rail
- serves multiple small plants or tray cells
- communicates uniform low-profile lighting
- better for seedlings than large single lamps

Terminal forms:

```text
━━━━━━━━━━━━━━━━ lamp bar
```

```text
[ LED ][ LED ][ LED ]
```

```text
┄┄┄┄┄┄┄┄ soft shelf wash
```

States:

| State | Visual cue |
|---|---|
| Off | plain bar |
| On | underline / glow wash below |
| Segment failure | one broken section |
| Low | dotted wash |
| Propagation mode | soft continuous bar over trays |

Design rule: lamp bars should read as shelf / rack infrastructure, not as floating decoration.

### 3. Grow panel / LED board

Purpose: technical controlled-environment light source.

Best labs:

- Climate Lab
- Grow Bay Lab
- Utility Lab, as stored equipment
- Propagation Lab, if advanced setup

Support logic:

- rectangular panel or board
- can hang over one bay or span several stations
- good for modern lab feeling
- can expose grid / diode pattern

Terminal forms:

```text
┌────────┐
│ ······ │
│ ······ │
└────────┘
```

```text
╔═ LED PANEL ═╗
║ · · · · ·  ║
╚════════════╝
```

States:

| State | Visual cue |
|---|---|
| Active | diode dots visible |
| Dimmed | fewer dots |
| Fault | one dark patch / `!` |
| Test mode | panel label / diagnostic marker |
| Heat risk | warning corner |

Design rule: panels are useful for Climate Lab and technical Grow Bay variants, but the first prototype can start with simpler overhead lamps.

### 4. Reflector hood

Purpose: shape the lamp visually and imply directed light.

Best labs:

- Grow Bay Lab
- Utility Lab, as stored hardware
- Climate Lab

Support logic:

- pairs with lamp source
- makes light direction explicit
- can visually separate old-style lamp fixtures from modern LED panels

Terminal forms:

```text
  .-====-.
 /        \
```

```text
╭────────╮
╰╲____╱─╯
```

States:

| State | Visual cue |
|---|---|
| Clean | crisp hood |
| Dusty | stipple on hood |
| Tilted | asymmetric hood |
| Hot | warning marker |
| Missing bulb | empty hood |

Design rule: reflectors are part of the light fixture, not separate props.

### 5. Chains, cables, hooks, and tracks

Purpose: attachment grammar for lighting elements.

Best labs:

- all labs

Support logic:

- explains how lamps are physically mounted
- allows adjustable height
- provides strong vertical rhythm
- connects lighting to the shell layer from Note 04

Terminal forms:

```text
━━━━━━━━━━━━━━━━ rail
   │      │      │
 ╭────╮ ╭────╮ ╭────╮
```

```text
hook:  ┐
chain: │
cable: ╎
track: ═══════
```

States:

| State | Visual cue |
|---|---|
| Fixed | short straight mount |
| Adjustable | chain / pulley cue |
| Loose cable | wavy or offset cable |
| Broken mount | tilted lamp / `!` |
| Rail-mounted | lamp slides along horizontal track |

Design rule: attachment elements should be drawn before lamps in the design model. They prevent lighting from feeling pasted onto the scene.

### 6. Visible light cones / washes

Purpose: communicate what area a light affects.

Best labs:

- Grow Bay Lab
- Climate Lab
- Propagation Lab, in soft form
- Archive Lab, as small spotlights

Support logic:

- bridges source and plant support
- can show coverage, intensity, misalignment, and mode
- creates atmosphere and depth

Cone / wash types:

| Type | Meaning |
|---|---|
| Narrow cone | one pot / specimen |
| Wide cone | bay / bench area |
| Shelf wash | propagation tray coverage |
| Dotted cone | low / gentle light |
| Dense cone | high intensity |
| Broken cone | obstruction or malfunction |
| Offset cone | misaligned fixture |

Terminal forms:

```text
╲    ╱
 ╲  ╱
  \/
```

```text
╲ · · ╱
 ╲ · ╱
  \/
```

```text
┄┄┄┄┄┄┄┄ soft horizontal wash
```

Design rule: light cones should be used sparingly. They are high-visibility elements and can easily overwhelm plant glyphs.

### 7. Shade cloth / screen / curtain

Purpose: block, soften, retain, or redirect light and heat.

Best labs:

- Climate Lab
- Grow Bay Lab
- Propagation Lab
- greenhouse frame / global exterior edge

Support logic:

- attaches to roofline, ceiling track, side wall, or glazing
- can be manual or automated
- doubles as climate-control element
- ties lighting to temperature, humidity, and energy behavior

Real-world grounding:

- Shade cloth and screen systems are commonly used with ventilation and humidity control to manage greenhouse heat and radiation.
- Energy / thermal curtains can reduce heat loss at night and can also be used for summer shading or light control.
- Curtains may be manually operated or automated with tracks, mechanical drives, and controllers.

Terminal forms:

```text
▒▒▒▒▒▒▒▒▒▒ shade screen
```

```text
╱╲╱╲╱╲╱╲ folded curtain
```

```text
track ═════════════
shade ▒▒▒▒▒▒▒────── partially drawn
```

States:

| State | Visual cue |
|---|---|
| Open | stored at one side / top |
| Partially drawn | screen covers part of viewport |
| Closed | dense screen span |
| Torn | gap / irregular edge |
| Automated | rail + motor marker |
| Night curtain | closed screen with low-light state |

Design rule: shade cloth should usually be architectural and large-scale. It should not be confused with a small prop.

### 8. Blackout / light-abatement curtain

Purpose: control photoperiod or light leakage.

Best labs:

- Climate Lab
- Propagation Lab
- Archive / Specimen Lab
- greenhouse global frame, if nighttime exterior logic is used

Support logic:

- related to shade cloth but visually darker / more sealing
- indicates scheduled darkness, night mode, or controlled experiment
- may affect humidity / temperature state later

Terminal forms:

```text
████████ blackout curtain, use very sparingly
```

```text
╞══ sealed curtain ══╡
```

States:

| State | Visual cue |
|---|---|
| Open | curtain bunched at side |
| Closed | sealed dark band |
| Leaking light | small slits / gaps |
| Scheduled | clock marker |
| Manual override | handle marker |

Design rule: blackout curtains should be rare and purposeful because dense blocks can visually dominate the terminal scene.

### 9. Warning / inspection / status lights

Purpose: small non-growth lighting signals.

Best labs:

- Climate Lab
- Utility Lab
- Grow Bay Lab
- greenhouse frame

Support logic:

- not plant growth lights
- communicate equipment state, alarms, lab selection, or mode
- useful for compact state communication

Terminal forms:

```text
● OK
● WARN
● OFF
```

```text
[!] lamp heat
```

```text
panel:  o o ●
```

States:

| State | Visual cue |
|---|---|
| OK | small steady indicator |
| Warning | blinking / `!` marker |
| Fault | `x` or dark indicator |
| Active mode | selected light |
| Maintenance | wrench / dot pairing |

Design rule: status lights should be readable but tiny. They should not compete with growth lamps.

---

## Lighting by lab

| Lab | Lighting emphasis |
|---|---|
| Propagation Lab | soft lamp bars, shelf lighting, humidity-dome glow, low-intensity gentle wash |
| Climate Lab | test panels, status lights, shade / curtain systems, diagnostic lighting |
| Grow Bay Lab | three overhead lamps, visible cones, lamp rail, pot-aligned coverage |
| Vines Lab | vertical / angled lighting, hanging fixtures, trellis-edge highlights |
| Utility Lab | inspection lights, stored lamps, warning LEDs, service-panel lights |
| Archive / Specimen Lab | small spotlights, jar / case lighting, quiet observation glow |

---

## Lighting state vocabulary

### Basic states

| State | Meaning |
|---|---|
| Off | fixture exists but emits no light |
| On | normal active light |
| Dimmed | reduced intensity |
| Scheduled | obeying photoperiod / timed cycle |
| Manual override | user / tool state changes light |
| Overheated | light creates heat warning |
| Misaligned | coverage misses plant support |
| Broken | fixture or cable fault |
| Shaded | light blocked or softened by screen |
| Night mode | growth / lab lights mostly off, indicators remain |

### Visual state markers

```text
!    warning / heat / fault
*    active / scheduled marker
·    soft / low light
░    medium beam / wash
▒    shade / screen
╲╱   cone boundary
┄    subtle wash / floor light
```

Design rule: state should be layered onto fixture + beam + support alignment, not represented only by text.

---

## Attachment and alignment rules

### 1. Every lamp needs an attachment point

Bad:

```text
     ╭────╮
     │LED │
     ╰────╯
```

Better:

```text
━━━━━━━━━━━━ rail
     │
   ╭────╮
   │LED │
   ╰────╯
```

### 2. Per-plant lamps should align with plant supports

Good Grow Bay grammar:

```text
╭────╮        ╭────╮        ╭────╮
 ╲  ╱          ╲  ╱          ╲  ╱
 [pot]          [pot]          [pot]
```

Misalignment can become a meaningful state later:

```text
╭────╮
   ╲  ╱
 [pot]      !
```

### 3. Shelf lights belong under shelves or over trays

```text
━━━━━━━━━━━━━━━━ shelf
┄┄┄┄┄┄┄┄┄┄ light wash
[ o o o o ] tray
```

### 4. Shade cloth belongs to tracks, glazing, or roofline

```text
track ═════════════════════
shade ▒▒▒▒▒▒▒▒──── partially drawn
```

### 5. Light should not erase architecture

Light cones should overlay or interrupt space without destroying the structural readability of walls, rails, supports, and pots.

---

## Terminal readability rules

### Keep beams sparse

Dense light fills can quickly become visual noise. Prefer outlines, dots, or short wash marks.

### Use light to divide space

In Grow Bay, light cones should reinforce bay divisions.

### Use different forms for different scales

- pot lamp = cone
- shelf light = horizontal wash
- panel = rectangular diode grid
- shade cloth = broad screen texture
- status light = single dot / marker

### Keep warning signals small

A light fault should use a tiny `!`, dark patch, or broken cone. Do not flood the lab with warning text.

### Reserve dense shade glyphs

Glyphs like `▒` or `█` are strong. Use them only for shade / blackout states, not ordinary wall texture.

---

## Minimal Grow Bay lighting prototype

Required elements:

1. one ceiling / lamp rail
2. three hanging lamp fixtures
3. three visible light cones
4. three pot-aligned coverage zones
5. simple lamp states: off / on / warning

Prototype sketch:

```text
┌─ Grow Bay Lab ───────────────────────────── OK ─┐
│ ╞══╪══╪══╪══╪══╪══╪══╪══╪══╡ back wall       │
│ ━━━━━━━━━━━━━━━━━ lamp rail ━━━━━━━━━━━━━━━━━ │
│      │                  │                  │   │
│   ╭────╮             ╭────╮             ╭────╮ │
│    ╲  ╱               ╲  ╱               ╲  ╱  │
│    ╭──╮               ╭──╮               ╭──╮  │
│    │░░│               │░░│               │░░│  │
│    ╰──╯               ╰──╯               ╰──╯  │
│ ┄┄┄┄┄┄┄┄┄ drain / floor seam ┄┄┄┄┄┄┄┄┄┄      │
└────────────────────────────────────────────────┘
```

Optional first-state variations:

```text
Off:       lamp hood only, no cone
Low:       dotted cone
On:        clear cone
Warning:   `!` near fixture or broken cone
```

---

## Later prototype extensions

After the basic Grow Bay lighting grammar works, expand to:

1. Propagation Lab shelf lights
2. Climate Lab panel + warning lights
3. Vines Lab angled / vertical lighting
4. Shade cloth / screen in Climate or Grow Bay Lab
5. Night mode / photoperiod state
6. Lamp height / cone-width variations
7. Misalignment state
8. Heat warning state

---

## Open design questions

1. Should visible light cones always render, or only in active / debug / night views?
2. Should light intensity be represented by cone density, cone width, color, or small numeric labels?
3. Should photoperiod be simulated visibly through the global clock and lamp schedule?
4. Should shade cloth be a lab-local object or a greenhouse-frame/global object?
5. Should lamp height be adjustable in the model, or visually fixed at first?
6. Should warning lights live in the lab world, the greenhouse frame, or both?
7. Should Grow Bay lamps be old reflector-style lamps, modern LED panels, or a hybrid?

---

## Working conclusion

Lighting should be designed as a structural system, not as atmospheric decoration.

The essential lighting grammar is:

```text
mount / rail
└── fixture
    └── beam / wash / cone
        └── plant-support coverage
```

For the first greenhouse prototype, implement or sketch the **Grow Bay Lab** lighting stack first: rail, three lamps, three cones, three pot-aligned zones.

This preserves the best HighGrow layout grammar while translating it into YAM’s terminal-native architecture.
