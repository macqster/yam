
# Greenhouse Ecosystem Design Notes 05

## Plant-Support Elements

### Purpose of this note

Notes 01–04 established the greenhouse ecosystem as a persistent frame-world containing switchable lab sub-worlds, then defined the boundary / shell / lab-frame layer.

The next design layer is **plant-support elements**: the in-world objects that physically hold, position, organize, label, and contextualize plants inside each lab.

This note answers:

```text
Where do plants actually live inside the labs?
```

Plant-support elements are not decorative props. They define root position, scale, access, local state, and future interaction affordances.

---

## Why plant-support elements come next

The current design sequence is:

```text
01 HighGrow layout grammar
02 Greenhouse frame + switchable labs
03 Greenhouse element families
04 Boundary / shell / lab frame
05 Plant-support elements
```

Now that the architectural container exists, the next practical step is to decide how plants attach to the world.

Real greenhouse layout logic supports this step. Greenhouse planning commonly accounts for growing zones, work areas, access paths, airflow, water access, drainage, benches, vertical use, and climate-equipment placement. For YAM, this means supports must be designed as part of the lab’s spatial system, not placed randomly.

Design implication: the plant support is the bridge between architecture and plant simulation.

---

## Core definition

A **plant-support element** is any in-world object or structure that can host plant life or plant-related growth systems.

It may be:

- a single-plant container
- a multi-plant tray
- a continuous bed
- a shelf / bench surface
- a wall-climbing structure
- a hanging structure
- a hydroponic / technical container
- a specimen enclosure

Every support should answer at least four questions:

1. **Anchor logic** — Does it hold one plant, many plants, a vine system, or a specimen?
2. **Spatial logic** — Is it floor-based, shelf-based, wall-mounted, hanging, or bench-integrated?
3. **State logic** — Can it show dry soil, wet tray, overflow, cracks, algae, condensation, or blocked drainage?
4. **Lab mapping** — Which labs should use it?

---

## Primary plant-support families

### 1. Basic pot

Purpose: default readable plant anchor.

Best labs:

- Grow Bay Lab
- Vines Lab
- Propagation Lab, in small form
- Utility Lab, as spare equipment

Support logic:

- usually one plant per pot
- floor, bench, shelf, or hanging variants possible
- clear root / stem origin
- excellent for early prototype because it is compact and legible

Terminal forms:

```text
[pot]
```

```text
╭──╮
│░░│
╰──╯
```

```text
  ╱╲
 ╱░░╲
 ╲__╱
```

Possible state variants:

| State | Visual cue |
|---|---|
| Dry | sparse soil glyphs, cracked top, lighter texture |
| Wet | denser soil, tray mark, small drip cue |
| Overwatered | overflow mark, puddle / tray line |
| Rootbound | side crack, tight root glyph, stressed marker |
| Tagged | small stake label or hanging tag |
| Empty | pot without plant anchor |

Design rule: the basic pot should be the canonical first plant anchor.

### 2. Nursery tray

Purpose: compact support for many small starts or seedlings.

Best labs:

- Propagation Lab
- Utility Lab, as stored trays
- Archive / Specimen Lab, as small organized samples

Support logic:

- many tiny cells in one object
- good for repeated small glyphs
- communicates early growth better than individual pots
- works naturally with shelves and humidity domes

Terminal forms:

```text
[ o o o o ]
```

```text
┌─────────┐
│ o o o o │
│ o o o o │
└─────────┘
```

```text
╞═╪═╪═╪═╡
│·│·│·│·│
╘═╧═╧═╧═╛
```

Possible state variants:

| State | Visual cue |
|---|---|
| Empty | cell grid only |
| Seeded | dots in cells |
| Sprouted | tiny marks `,`, `'`, `·/` |
| Patchy | some empty cells |
| Damp | tray underline or condensation marker |
| Overcrowded | dense cell marks |

Design rule: nursery trays are high-value for Propagation Lab because they establish scale and early-growth logic.

### 3. Humidity dome

Purpose: show a controlled seedling microclimate.

Best labs:

- Propagation Lab
- Climate Lab, as test chamber
- Archive / Specimen Lab, for delicate samples

Support logic:

- usually sits over a nursery tray or small pots
- visually encodes humidity and protection
- can show condensation state
- useful as a bridge between plant support and climate-control systems

Terminal forms:

```text
  .─────────.
 /  o o o o  \
└─────────────┘
```

```text
╭───────────╮
│ · · · · · │
└─[tray]────┘
```

Possible state variants:

| State | Visual cue |
|---|---|
| Clear | light outline only |
| Humid | dots / beads inside dome |
| Fogged | faint fill / stipple |
| Open | broken or lifted top line |
| Overhumid | dense condensation marks |

Design rule: the dome should read as a micro-lab inside the lab.

### 4. Raised bed / bench planter

Purpose: wider continuous planting surface.

Best labs:

- Grow Bay Lab
- Propagation Lab, as bench surface
- Vines Lab, as base planter

Support logic:

- supports multiple plants along a horizontal span
- creates a stronger floor / workbench layer
- can host plants at intervals
- works well with overhead lamp bars

Terminal forms:

```text
┬────────────────────┬
│░░░░░░░░░░░░░░░░░░░░│
┴────────────────────┴
```

```text
▔▔▔▔▔▔▔▔▔▔▔▔ bench edge
[ soil / pots / tray anchors ]
```

Possible state variants:

| State | Visual cue |
|---|---|
| Dry bed | cracked or sparse soil texture |
| Irrigated | drip-line markers |
| Overgrown | plants crossing station boundaries |
| Divided | bay separators / labels |
| Draining | runoff seam beneath |

Design rule: raised beds and benches are useful when a lab should feel less like three isolated pots and more like a working greenhouse surface.

### 5. Shelf / rack

Purpose: organize supports vertically and efficiently.

Best labs:

- Propagation Lab
- Archive / Specimen Lab
- Utility Lab
- Vines Lab, as partial structure

Support logic:

- hosts trays, pots, jars, tools, or specimen cases
- exploits vertical space
- creates strong lab silhouette
- provides attachment points for lamp bars and labels

Terminal forms:

```text
┬────────┬────────┬────────┬
│ tray   │ pot    │ jar    │
├────────┼────────┼────────┤
│ tray   │ tray   │ tools  │
┴────────┴────────┴────────┴
```

```text
━━━━━━━━━━━━━━━━ shelf
  [tray]   [pot]   [jar]
━━━━━━━━━━━━━━━━ shelf
```

Possible state variants:

| State | Visual cue |
|---|---|
| Empty | shelf lines only |
| Loaded | visible tray / pot / jar units |
| Crowded | compressed objects, warning marker |
| Lit | lamp bar under shelf |
| Wet | drip marks below shelf |

Design rule: shelf / rack is both support and architecture. It should be treated as a structural element.

### 6. Trellis grid

Purpose: support climbing and wall-interacting plant growth.

Best labs:

- Vines Lab
- Grow Bay Lab, as optional rear support
- Archive / Specimen Lab, for trained specimens

Support logic:

- wall-mounted or frame-mounted
- converts wall into plant-support surface
- allows vertical growth paths
- creates strong visual identity for Vines Lab

Terminal forms:

```text
╞══╪══╪══╪══╪══╡
│  │  │  │  │  │
╞══╪══╪══╪══╪══╡
│  │  │  │  │  │
```

```text
# # # # #
 # # # #
# # # # #
```

Possible state variants:

| State | Visual cue |
|---|---|
| Empty | clean grid |
| Light growth | occasional tendril marks |
| Dense growth | vines obscure grid |
| Pruned | cut markers / gaps |
| Guided | clips, string ties, tag markers |

Design rule: the trellis should be part of the lab shell and plant-support system simultaneously.

### 7. Hanging basket / suspended planter

Purpose: use ceiling rails and vertical space.

Best labs:

- Vines Lab
- Propagation Lab, as optional small hanging starts
- Archive / Specimen Lab
- Grow Bay Lab, as advanced variation

Support logic:

- attaches to ceiling rail or hook
- creates foreground / midground depth
- good for trailing vines
- reinforces the ceiling / rail grammar from Note 04

Terminal forms:

```text
   │
  ╭┴╮
  │░│
  ╰─╯
```

```text
  chain
    │
  \___/
  (░░░)
```

Possible state variants:

| State | Visual cue |
|---|---|
| Empty | basket only |
| Planted | center sprout / stem |
| Trailing | hanging vine marks |
| Dry | sparse soil / wilt marker |
| Dripping | drip below basket |
| Mis-hung | tilted basket / warning marker |

Design rule: hanging supports should visibly attach to ceiling rails. They should not float.

### 8. Hydro bucket / grow bucket

Purpose: technical controlled-environment plant container.

Best labs:

- Grow Bay Lab
- Climate Lab
- Utility Lab
- Vines Lab, if technical vine setup is desired

Support logic:

- usually one plant per bucket
- implies water / nutrient infrastructure
- can attach to tubes, reservoirs, pumps, or drains
- strong technical-lab cue

Terminal forms:

```text
╭────╮
│ H₂O│── tube
╰────╯
```

```text
[ bucket ]───o tube
```

```text
╭─────╮
│░││░│  net pot / medium cue
╰─┬───╯
  ╰── tube
```

Possible state variants:

| State | Visual cue |
|---|---|
| Active | tube attached, stable marker |
| Low water | warning mark / low fill line |
| Blocked | crossed tube / `!` marker |
| Leaking | drip below bucket |
| Aerated | small bubble marks |
| Disconnected | loose tube |

Design rule: hydro buckets should visually connect to water / nutrient infrastructure, even if the system is not simulated yet.

### 9. Hydro channel / nutrient rail

Purpose: continuous technical support for multiple plants.

Best labs:

- Climate Lab
- Grow Bay Lab
- Utility Lab
- Propagation Lab, if technical tray logic is used

Support logic:

- multiple plant sites along a horizontal channel
- strongly implies controlled feeding
- works well with pipes, pumps, and drain returns

Terminal forms:

```text
╞═o══o══o══o═╡
```

```text
┌────────────────┐
│ o   o   o   o  │── tube
└────────────────┘
```

Possible state variants:

| State | Visual cue |
|---|---|
| Empty | holes only |
| Planted | small stems at holes |
| Flowing | subtle arrow / ripple marks |
| Blocked | warning at one segment |
| Dry | broken flow mark |

Design rule: hydro channels are useful for technical labs, but probably not required for the first Grow Bay prototype.

### 10. Specimen jar / terrarium case

Purpose: contained observation support for small or unusual specimens.

Best labs:

- Archive / Specimen Lab
- Propagation Lab, as small contained microclimate
- Climate Lab, as experiment vessel

Support logic:

- one small specimen or micro-ecosystem
- emphasizes containment, observation, labels, and long-term archival state
- can show condensation, sealed/open status, and specimen tag

Terminal forms:

```text
 ╭──╮
 │··│
 │╿ │
 ╰──╯
```

```text
┌────────┐
│ case A │
│  ·╿·   │
└────────┘
```

Possible state variants:

| State | Visual cue |
|---|---|
| Clear | clean outline |
| Condensed | bead marks |
| Sealed | cap / top lock marker |
| Open | broken lid line |
| Labeled | tag plaque |
| Overgrown | specimen touches glass |

Design rule: specimen containers should support quiet, observational labs rather than main grow-bay readability.

---

## Prototype priority

For implementation usefulness, start with five support types:

```text
1. basic pot
2. nursery tray
3. shelf / bench
4. trellis
5. hydro bucket
```

These five cover the first useful set of labs:

| Support | Enables |
|---|---|
| Basic pot | Grow Bay Lab, Vines Lab, simple plant anchors |
| Nursery tray | Propagation Lab |
| Shelf / bench | Propagation Lab, Archive Lab, Utility Lab organization |
| Trellis | Vines Lab |
| Hydro bucket | technical Grow Bay / Climate / Utility cue |

Do not start with every possible support. The first prototype only needs enough vocabulary to make labs structurally distinct.

---

## Required support set for first Grow Bay Lab prototype

The first Grow Bay prototype from Note 04 only requires:

1. three basic pot anchors
2. optional three small pot labels
3. one shared floor / drain baseline
4. optional tray line under pots
5. optional hydro-bucket variant for later

Minimal sketch:

```text
┌─ Grow Bay Lab ───────────────────────────── OK ─┐
│ ━━━━━━━━━━━━━ lamp rail ━━━━━━━━━━━━━━━━━━━━━━ │
│     ╭────╮        ╭────╮        ╭────╮         │
│      ╲  ╱          ╲  ╱          ╲  ╱          │
│     ╭──╮           ╭──╮           ╭──╮         │
│     │░░│           │░░│           │░░│         │
│     ╰──╯           ╰──╯           ╰──╯         │
│ ┄┄┄┄┄┄┄┄┄ drain / floor seam ┄┄┄┄┄┄┄┄┄┄      │
└────────────────────────────────────────────────┘
```

HighGrow’s most useful plant-support idea is the clearly readable station: one pot under one lamp. YAM can preserve that grammar while making the supports terminal-native and expandable.

---

## Lab mapping matrix

| Support element | Propagation | Climate | Grow Bay | Vines | Utility | Archive |
|---|---:|---:|---:|---:|---:|---:|
| Basic pot | yes | optional | primary | primary | spare | optional |
| Nursery tray | primary | test item | optional | no | stored | optional |
| Humidity dome | primary | test chamber | no | no | stored | optional |
| Raised bed / bench planter | optional | no | optional | optional | no | no |
| Shelf / rack | primary | optional | optional | optional | primary | primary |
| Trellis grid | no | no | optional | primary | no | optional |
| Hanging basket | optional | no | optional | primary | no | optional |
| Hydro bucket | no | optional | optional | optional | primary | no |
| Hydro channel | optional | optional | optional | no | primary | no |
| Specimen jar / terrarium | optional | optional | no | no | stored | primary |

---

## Terminal readability rules

### Use supports as anchors, not clutter

A plant support should define where growth begins. It should not be visually louder than the plant unless the plant is absent.

### Keep small / medium / large forms

Each support should have at least three possible drawing scales:

- **small**: fits cramped lab or background shelf
- **medium**: default interactable object
- **large**: foreground / focus object

Example for pot:

```text
small:  [p]
medium: ╭──╮ / │░░│ / ╰──╯
large:  wider planter with soil, label, and tray
```

### Preserve attachment logic

- pots sit on floor, shelf, bench, or tray
- trays sit on shelves / benches
- hanging baskets attach to hooks or rails
- trellises attach to wall or frame
- hydro buckets attach to tubes / drains
- specimen jars sit on shelves / cases

### Show state sparingly

Support state should be readable but not noisy.

Preferred state signals:

- tiny `!` marker for problem state
- drip mark for leak / overwater
- crack mark for dry / damaged pot
- dot texture for soil / condensation
- short tube segment for hydro connection
- tag marker for label

### Avoid unsupported floating objects

Every plant-support object should visibly belong to a surface or mount.

---

## Open design questions

1. Should Grow Bay Lab start with simple pots only, or include hydro-bucket variants from the beginning?
2. Should plant supports be individually selectable objects, or only structural anchors for plant rendering at first?
3. Should nursery trays be rendered as one object with many cells, or as many tiny plant anchors?
4. Should trellis supports be part of the Vines Lab shell or separate placeable supports?
5. Should support state be derived from plant state, container state, or both?
6. Should labels belong to support objects, plant objects, or a separate identity-marker layer?

---

## Working conclusion

Plant-support elements should be the next concrete object family after lab shell design.

They provide the physical grammar for plant life:

1. where plants root
2. how many plants a lab can host
3. how plant scale is communicated
4. how water / substrate / support state is shown
5. how each lab’s purpose becomes visible

The immediate prototype should start with **basic pot**, **nursery tray**, **shelf / bench**, **trellis**, and **hydro bucket**. This is enough to differentiate the Grow Bay, Propagation, Vines, Utility, and Archive directions without overbuilding the system.

The next design note should likely cover **Lighting Elements**: overhead lamps, lamp bars, chains, cables, reflectors, visible light cones, shade cloth, and lamp states.
