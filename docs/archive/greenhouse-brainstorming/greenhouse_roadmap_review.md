
# Greenhouse Roadmap Review

## Main Assessment

The updated greenhouse roadmap is strongly coherent. It now reads less like a wishlist and more like a controlled expansion contract.

It does four important things well:

```text
1. separates brainstorming from authorization
2. locks functional-space-first as the first work order
3. keeps greenhouse state out of render/UI ownership
4. defers plant systems until room/site/environment contracts exist
```

The strongest operational reading is:

```text
The first greenhouse pass is a pure data contract for a functional room container,
not a render contract, species contract, or growth contract.
```

That gives Codex a clear boundary.

## What Looks Especially Strong

### 1. Implementation Gates Are Well Ordered

The current ordering is healthy:

```text
docs/readiness
→ functional-space contract
→ room/access/zone/fixture vocabulary
→ symbolic environment
→ flora storage decision
→ static visual review
→ inert state
→ growth probe
→ render
→ inspection
→ transfer
```

This avoids the common trap of rendering a pretty greenhouse first and then discovering that nothing owns the room model.

### 2. Reference Ingests Are Properly Bounded

`cbonsai`, `HighGrow`, `Viridi`, `asciiquarium`, and the scientific plant-growth lineage are all admitted, but none of them is allowed to hijack scope.

This matters because the technical plant-modeling references are much larger than YAM's immediate needs. OpenAlea is an open plant-modeling ecosystem with analysis, visualization, functioning, and growth tooling. L-Py is specifically an L-system simulation framework for modeling plant architecture development.

Sources:

```text
OpenAlea: https://openalea.readthedocs.io/
OpenAlea paper/index: https://inria.hal.science/hal-00831811v1
L-Py docs: https://lpy.readthedocs.io/
L-Py paper: https://www.frontiersin.org/journals/plant-science/articles/10.3389/fpls.2012.00076/full
```

Roadmap interpretation is correct:

```text
these references justify future attachment points,
not first-pass plant engines.
```

### 3. YAM / Ratatui Boundary Is Correct

The roadmap's insistence that render layers visualize state but do not own simulation truth matches Ratatui's immediate-mode model.

Ratatui redraws the UI every frame based on application state; widgets are not permanent authoritative state objects.

Source:

```text
https://ratatui.rs/concepts/rendering/
```

So this architectural rule is correct:

```text
state owns truth
render derives view
UI owns presentation selection only
```

### 4. Chafa Boundary Is Correct

The roadmap correctly keeps Chafa in the hero/source-art/rendering bucket rather than making it greenhouse-state infrastructure.

Chafa is a terminal graphics tool/library for converting image data, including animated GIFs, into ANSI/Unicode terminal art and other terminal graphics formats.

Sources:

```text
https://hpjansson.org/chafa/
https://github.com/hpjansson/chafa
```

Roadmap interpretation is correct:

```text
greenhouse rooms should not become opaque ANSI blobs.
```

## Places To Watch Carefully

### 1. The Document Is Becoming Dense

The roadmap is still coherent, but it is approaching the point where a dedicated accepted-contract doc may become useful.

Do not split yet unless implementation is authorized.

Suggested threshold:

```text
keep greenhouse-roadmap.md while work is planning-only
split docs/greenhouse.md once core::greenhouse data shapes are authorized
```

Suggested split:

```text
greenhouse-roadmap.md → strategy, phases, gates, references
greenhouse.md         → accepted data contract and invariants
```

### 2. Room And Zone Need A Sharp Distinction

`Room` and `Zone` are close enough to confuse future implementation.

Recommended invariant before code starts:

```text
Room = selectable/internal greenhouse space with one environment profile.
Zone = named functional sub-area inside a room.
```

This should become non-negotiable before `GreenhouseRoom` / `GreenhouseZone` structs are generated.

### 3. AccessPath Needs A No-Pathfinding Warning

`AccessPath` is a tempting term. It must not imply player movement, collision, routing, or pathfinding.

Safe invariant:

```text
AccessPath is layout semantics, not movement simulation.
```

### 4. Substrate Is Slightly Ambiguous

The symbolic environment model currently includes fields like:

```text
light
humidity
temperature
water
airflow
substrate
outside-weather influence
```

That is fine, but `substrate` can belong either to the room environment or to a planting site / fixture.

Recommended current interpretation:

```text
room substrate = dominant medium / floor-bed character
site substrate = later planting-site modifier
```

No immediate rewrite is required, but this should be watched before code starts.

### 5. Inert Greenhouse State Must Precede Runtime Visual Work

The phase order is defensible if early phases are documentation/design phases.

But once implementation starts, the first code-bearing slice should still be inert state before any runtime visual slice.

Codex-safe reading:

```text
docs sketch may happen before inert state
runtime visual review must not happen before inert state
```

## Recommended Operational Contract

The roadmap is ready to act as the greenhouse authority doc for the next planning/code-prep cycle.

Current contract:

```text
YAM 0.4 greenhouse begins as a pure functional-space model.
```

It owns:

```text
greenhouse state
rooms
access paths
zones
fixtures
planting sites
symbolic environment profiles
read-only inspection refs
```

It does not yet own:

```text
plant lifecycle
growth ticks
species catalogs
persistence
crop/care gameplay
selectable runtime greenhouse world
render-owned greenhouse truth
```

## Best Next Discussion Target

The next useful design target is not more inspiration.

It should be:

```text
What is the minimal greenhouse_nursery room profile?
```

Not code yet — just canonical first-room data prose:

```text
id
display name
role
bounds assumption
access paths
zones
fixtures
1–3 planting sites
environment profile
inspection refs
```

## Bottom Line

The roadmap is implementation-safe if read as a staged contract:

```text
first:   functional greenhouse space
later:   plant occupants and growth feel
never by default: crop simulation, lifecycle sprawl, management mechanics
```

This is the right posture for YAM 0.4.
