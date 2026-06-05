# Main-Scene Scaffold

<!-- cspell:ignore baubotanik -->

This document is the owning note for YAM's main-scene hero support scaffold.
It is a composition and ownership brief, not an implementation batch.

The goal is to keep hero-support ideation grounded in the actual current
main-scene contract: world-attached content, read-only render layers, and
careful authored composition rather than dashboard chrome or generic background
decoration.

Related contracts:

- [scene-model.md](scene-model.md) owns world/screen behavior and scene
  categories
- [rendering.md](rendering.md) owns visible layer and composition rules
- [architecture.md](architecture.md) owns ownership boundaries
- [vines.md](vines.md) owns vine-specific readiness and future phases
- [../TODO.md](../TODO.md) owns active execution work

Coordinate source note:

- the local brainstorming note at
  `/Users/mcq/Downloads/yam_tree_scaffold_brainstroming.md` now counts as an
  active reference for scaffold coordinate anatomy during this thread; its
  concrete origin/fork/target graph should be distilled here whenever runtime
  geometry is updated so implementation does not depend on unstated local
  memory

## Current Runtime State

The first scaffold runtime slice is now present in a deliberately narrow form:

- `core::scaffold` owns a static main-scene support shape
- the current shape is still support-first: rear seat cradle, back brace, leg
  brace, fork mass, plus a small foreground nesting edge
- the scaffold renders through dedicated read-only scene layers: rear support
  beneath the hero and foreground nesting edge above the hero
- there is still no decorative branch spread or vine-led overgrowth

This is intentional. The current goal is to prove world ownership and support
readability before adding another layer of composition complexity.

## Prototype Workflow

Scaffold experimentation should default to the sandbox world first.

Current recommendation:

- use sandbox as the visual proving ground for scaffold fits and comparisons
- keep hero, companions, and scaffold independently toggleable there through
  UI-owned visibility controls
- keep the sandbox scaffold payload sourced from the same canonical
  `core::scaffold` support shape rather than inventing a second sandbox-only
  scaffold truth
- compare scaffold alone, scaffold plus hero, and fuller composition passes
  before promoting changes into the main scene
- keep those toggles presentation-only; they must not create a second scaffold
  data owner or a sandbox-only projection path

Current mask recommendation:

- do not default to masks for scaffold work yet
- prefer plain world-owned scaffold geometry and ordinary layer ordering first
- consider a mask only if a specific foreground nesting or occlusion read
  cannot be expressed cleanly through geometry and layer order alone

That recommendation is still active after the latest batch: the first
foreground nesting edge now exists as ordinary scaffold geometry above the hero,
so the next review question is whether that no-mask lip reads clearly enough
before any explicit mask seam is entertained.

## Current Role

The scaffold exists to ground the current hero GIF inside the main scene.

It is not just decorative bark or a generic tree prop. Its first job is to
replace the support geometry lost when the original source background and
window frame were removed from the hero asset.

That means the scaffold must explain three pose facts:

- back support
- seat support
- raised-leg brace

If those three reads are weak, the hero risks looking suspended rather than
seated.

## Core Translation Rule

Replace window-frame logic with tree-fork logic.

The useful mapping is:

| Original support logic | Scaffold replacement |
| --- | --- |
| lower sill / ledge | thick seat trunk or cradle branch |
| rear vertical frame | rear branch or back-support sweep |
| right/top frame angle | upper branch aligned with the raised leg |
| dark interior occlusion | dense fork shadow or bark mass |
| foreground frame edge | foreground lip, moss edge, root lip, or small twig mass |

The scaffold should therefore be treated as a functional support silhouette
first and a botanical object second.

## Current Direction

The strongest current direction is a clean single-trunk cradle rather than a
busy multi-branch silhouette.

The scaffold should first read as one continuous load-bearing organism:

- ground anchor
- trunk rise
- seat cradle
- back sweep
- optional leg-side rise

The primary shape should read as an S / hook / cradle curve rather than a set
of unrelated limbs.

## Coordinate Reference

The current coordinate reference is distilled directly from the local
brainstorming note at
`/Users/mcq/Downloads/yam_tree_scaffold_brainstroming.md`.

These coordinates are still authored composition targets, not final procedural
plant anatomy:

| Support element | Coordinate intent | Read |
| --- | --- | --- |
| ground trunk origin | `(-20, -30)` -> `(-20, -24)` | off-world root and primary load path into the fork |
| fork node | `(-20, -24)` | swollen branch collar / knot mass, not a hinge |
| back-support branch | `(-20, -24)` -> roughly `(-65, 10)` | rear back brace under the hero lean |
| seat / butt-ramp anatomical origin | `(-20, -24)` | hidden origin inside the trunk system |
| seat / butt-ramp visible emergence | around `(20, -24)` -> `(50, -10)` | forward cradle under the butt and bent right leg |
| raised-leg branch | `(50, -10)` -> `(25, 25)` | secondary perspective echo for the straight leg |

Interpretation rules from that note remain active:

- the seat ramp should not read like a thin line launched from the fork node;
  trunk thickness and occlusion should let it appear to emerge forward around
  `(20, -24)`
- the fork node should be visibly swollen and load-bearing rather than a clean
  geometric Y
- the support graph should read as `ground trunk -> fork node -> rear back
  brace / forward seat ramp -> raised-leg branch`

## Main Components

### 1. Ground-Trunk / Seat Cradle

This is the most important element.

Rules:

- it should anchor near the lower-right or lower-center world region
- it should rise as a thick trunk mass into the seated pose
- it should bend under the butt, pelvis, and bent right leg
- it may have a flattened or moss-softened contact surface, but should not
  read like a literal bench
- it must survive the hero/scaffold composition as a clear support form

### 2. Rear Back-Brace Sweep

The back support should feel like the same structure continuing upward, not a
separate prop pasted behind the hero.

Rules:

- emerge from the main support mass near the lower back / hip region
- run behind the torso and shoulder area
- stay clear of the face and key silhouette edges
- prefer a knotted or swollen fork transition over a geometric Y

### 3. Leg-Aligned Rise

This element is secondary. It can reinforce the straight-leg read, but only
after the seat and back cradle already work.

Rules:

- echo the raised-leg direction
- stay visually tied to the same trunk system
- remain lighter than the seat cradle
- preserve negative space around the leg rather than tracing every edge

### 4. Foreground Lip / Nesting Edge

A small foreground edge can help the hero feel nested inside the support
instead of pasted onto it.

Allowed forms:

- moss edge
- bark ridge
- root lip
- small twig cluster

This should stay subtle and should not become foreground clutter.

## Perspective Rules

The current safest bias is to preserve the original pose-support perspective
logic from the source asset while translating it into branch geometry.

Rules:

- the seat trunk should follow the old lower-sill directional logic
- the rear sweep should replace the lost left/rear frame support
- the leg-side rise should echo the old right/top angle
- the scaffold should read as foreground-to-background structure, not a flat
  backdrop
- any hero-perspective correction must be coordinated with scaffold fit, not
  done independently

## Visual Direction

The current best first-pass bias is a tree plus hidden-frame hybrid:

- organic enough to read as a living or once-living greenhouse support
- fitted enough to preserve the hero pose support logic
- disciplined enough to avoid decorative branch noise

Useful flavor lanes:

- natural greenhouse fork
- trained / baubotanik-like living support
- hybrid organic support that still quietly preserves the old frame geometry

The hybrid path is the current recommended starting point.

## Layering Rules

The scaffold should not live only behind the hero.

Preferred conceptual stack:

1. background greenhouse scene
2. rear scaffold mass / back-brace sweep
3. hero render
4. small foreground lip / moss / twig edge
5. optional particles later

This keeps the hero visually seated inside the structure rather than floating
in front of a backdrop.

## Implementation Bias

The first scaffold pass should stay:

- static or pre-rendered
- tightly fitted to the current hero envelope
- world-attached
- composition-led
- low-noise
- rear-support-only if that is the cleanest way to preserve ownership and pose
  readability in the first batch

The current second pass may add only a very small foreground nesting edge when
it strengthens the seated read without turning the scaffold into busy
foreground clutter.

It should not yet be:

- fully procedural
- a broad botanical simulation
- a substitute for flora ownership
- cluttered with secondary branches

## Interaction With Vines

The scaffold may prepare future vine placement, but it should not collapse into
vine ownership.

Rules:

- scaffold support logic and vine growth logic stay distinct
- vines may later attach to, overgrow, or frame the scaffold
- the scaffold should remain readable on its own before vine decoration arrives
- future vine enrichment must preserve the hero-support silhouette instead of
  hiding it

## Current Non-Goals

Do not use scaffold work to justify:

- hero asset churn for its own sake
- a second projection or attachment model
- render-owned support truth
- dashboard-style control chrome around the hero
- procedural branch complexity before the pose reads correctly

## Summary Rule

The scaffold is a pose-support cradle first.

If a future scaffold change improves botanical richness but weakens the seated
read, it is moving in the wrong direction.
