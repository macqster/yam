
# YAM v2 — MORPHOLOGY

---

## PURPOSE

Define the Morphology system as the **structural translation layer** between simulation and rendering.

This document specifies:

- structural primitives
- growth geometry
- spatial logic
- interface with shape language

---

## CORE PRINCIPLE

Morphology is:

> the system that turns abstract growth into spatial structure

It does not decide:

- *when* things grow (lifecycle)
- *what* they are (species)
- *how* they look (rendering)

It defines:

> how they are built in space

---

## POSITION IN ARCHITECTURE

```text
Runtime → Engine → Morphology → Shape → Render → Framebuffer → Emit
                    ↑
             Morphology lives here
```

Morphology sits **between simulation and rendering**.

---

## CORE MODEL

Morphology operates on structural primitives:

```text
Node
Internode
Axis
Organ
```

---

## STRUCTURAL PRIMITIVES

### 1. Node

A node is a point of growth or connection.

```text
Node:
  position
  direction
  age
  energy
  children_axes
```

Role:

- branching point
- attachment point for organs

---

### 2. Internode

A segment connecting two nodes.

```text
Internode:
  start_node
  end_node
  length
  thickness
```

Role:

- defines structural continuity

---

### 3. Axis

A continuous chain of nodes.

```text
Axis:
  nodes[]
  direction_bias
  persistence
```

Types:

- primary axis (main growth)
- secondary axes (branches)

---

### 4. Organ

An attached structure (leaf, flower, etc.)

```text
Organ:
  type
  position
  orientation
  scale
  lifecycle_state
```

---

## GROWTH MODEL

Growth occurs at active nodes.

```text
for each active node:
  compute direction
  create new node
  connect via internode
```

---

## DIRECTION SYSTEM (TROPISMS)

Growth direction is computed as:

```text
direction = normalize(
    base_direction
  + light_bias
  + gravity_bias
  + obstacle_avoidance
  + randomness
)
```

---

### Components

- phototropism → grow toward light / open space
- gravitropism → vertical bias
- thigmotropism → react to surfaces (hero/scaffold)
- randomness → controlled variation

---

## CURVATURE

Introduce natural curvature:

```text
direction += perpendicular(direction) * sin(time + phase) * curvature_strength
```

Effect:

- removes straight lines
- adds organic motion

---

## BRANCHING

Branching occurs at nodes.

```text
if energy > threshold and random() < branching_probability:
  spawn new axis
```

Constraints:

- limit branches per node
- avoid overcrowding

---

## PHYLLOTAXIS (ORGAN PLACEMENT)

Defines how organs attach to nodes.

Modes:

- alternate
- opposite
- whorled

Example:

```text
angle = node_index * 137.5°
organ_direction = rotate(axis_direction, angle)
```

---

## ENERGY MODEL (SIMPLIFIED)

Energy controls growth distribution.

```text
energy flows root → branches
branching divides energy
growth consumes energy
```

Effects:

- main axis dominance
- smaller branches
- natural tapering

---

## DENSITY CONTROL

Prevent overcrowding.

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

- young → growth
- mature → organ spawning
- inactive → no growth

---

## MULTI-SCALE BEHAVIOR

Different parts behave differently:

| Region | Behavior |
|-------|--------|
| Tip | exploration |
| Mid | branching |
| Base | stability |

---

## OUTPUT CONTRACT

Morphology outputs:

```text
structural representation
```

Consumed by:

```text
Shape Language → Rendering
```

---

## DESIGN CONSTRAINTS

Avoid:

- purely random growth
- rigid geometric patterns

Prefer:

- structured rules
- controlled randomness
- organic variation

---

## FINAL PRINCIPLE

```text
Morphology defines structure
Shape defines appearance
```

---

## NEXT

→ 15_vines.md
