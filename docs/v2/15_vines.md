
# YAM v2 — VINES

---

## PURPOSE

Define the Vines system as the **primary growth engine** of the ecosystem.

This document specifies:

- vine structure
- growth behavior
- interaction with environment and masks
- integration with morphology

---

## CORE PRINCIPLE

Vines are:

> the main generative system that propagates life through space

They are not simple lines.

They are:

> hierarchical, morphology-driven growth systems

---

## POSITION IN ARCHITECTURE

```text
Runtime → Engine → Morphology → Shape → Render → Framebuffer → Emit
             ↑
            Vines live here
```

Vines exist within the **Engine layer**, using Morphology for structure.

---

## CORE MODEL

```text
Vine
  ├── Axes
  ├── Nodes
  ├── Energy
  └── Growth State
```

---

## STRUCTURE

Vines are composed of axes:

```text
Vine:
  axes[]

Axis:
  nodes[]

Node:
  position
  direction
  age
  energy
  children_axes[]
```

---

## GROWTH MECHANISM

Growth occurs at active nodes (tips).

```text
for each active node:
  compute direction
  create new node
  connect via internode
```

---

## DIRECTION SYSTEM

Growth direction is influenced by multiple factors:

```text
direction = normalize(
    base_direction
  + light_bias
  + support_attraction
  + avoidance
  + randomness
)
```

---

### Factors

- light_bias → growth toward open space
- support_attraction → attraction to scaffold / hero edges
- avoidance → avoid dense regions
- randomness → variation

---

## SUPPORT-SEEKING BEHAVIOR

Vines interact with supports:

```text
support_field = scaffold + hero_mask_edges + existing_vines
```

Rule:

```text
if near_support:
  align growth along surface
else:
  explore open space
```

---

## MASK INTERACTION

Vines respect masks indirectly.

Rules:

- cannot grow into blocked regions
- can grow along mask boundaries

Effect:

- vines wrap around hero
- vines follow scaffold contours

---

## CURVATURE

Vines exhibit organic curvature:

```text
direction += perpendicular(direction) * sin(time + phase) * curvature_strength
```

---

## BRANCHING

Branching occurs at nodes:

```text
if energy > threshold and random() < branching_probability:
  spawn new axis
```

Constraints:

- limited branches per node
- reduced branching in dense areas

---

## ENERGY MODEL

Energy controls growth distribution:

```text
energy flows root → branches
branching divides energy
growth consumes energy
```

Effects:

- dominant main axis
- smaller side branches
- natural tapering

---

## DENSITY CONTROL

Prevent overcrowding:

```text
if local_density > threshold:
  reduce growth probability
```

---

## LIFECYCLE INTEGRATION

Node states:

```text
young → active → mature → inactive
```

Behavior:

- active → growth
- mature → branching + organ spawning
- inactive → no growth

---

## MULTI-SCALE BEHAVIOR

| Region | Behavior |
|-------|--------|
| Tip | exploration |
| Mid | branching |
| Base | structural stability |

---

## INTERACTION WITH ENVIRONMENT

```text
growth_rate *= temperature_factor
branching_probability *= humidity_factor
light_bias *= light_factor
```

---

## OUTPUT CONTRACT

Vines output:

```text
morphological structure (nodes + axes)
```

Consumed by:

```text
Shape Language → Rendering
```

---

## DESIGN CONSTRAINTS

Avoid:

- random walk behavior
- uniform branching
- straight-line growth

Prefer:

- guided growth
- structured branching
- organic curvature

---

## FINAL PRINCIPLE

```text
Vines are the system that fills space with life
```

---

## NEXT

→ 16_balance.md
