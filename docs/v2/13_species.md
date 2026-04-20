
# YAM v2 — SPECIES

---

## PURPOSE

Define the Species system as the **template layer for organism behavior and appearance**.

This document specifies:

- how organisms are defined
- how behavior is reused
- how variation is controlled

---

## CORE PRINCIPLE

A species is:

> a reusable definition of behavior, structure, and appearance

It is not an organism itself.

---

## POSITION IN ARCHITECTURE

```text
Runtime → Engine → Morphology → Shape → Render → Framebuffer → Emit
             ↑
          Species lives here
```

Species is part of the **Engine layer**.

---

## CORE MODEL

```text
Species:
  name
  lifecycle_model
  growth_model
  renderer_binding
  parameters
  aesthetics
  spawn_rules
```

---

## ORGANISM VS SPECIES

```text
Species → defines behavior
Organism → executes behavior
```

Example:

```text
species: vine_sprawl
organism: instance of vine_sprawl at position (x, y)
```

---

## COMPONENTS

### 1. Lifecycle Model

Defines:

- state machine
- durations
- transitions

Example:

```text
lifecycle_model: leaf_v1
```

---

### 2. Growth Model

Defines spatial behavior:

- direction bias
- branching rules
- density
- interaction rules

Example:

```text
growth_model: vine_sprawl_v1
```

---

### 3. Renderer Binding

Defines how the species is rendered:

```text
renderer: braille | block | glyph
shape: leaf_oval_v1
```

---

### 4. Parameters

Defines tunable values:

```text
growth_speed
branching_rate
max_size
decay_rate
randomness
```

Supports ranges:

```text
growth_speed: 0.8–1.2
```

---

### 5. Aesthetics

Defines visual identity:

```text
color_palette
shape_bias
density
symmetry
```

---

### 6. Spawn Rules

Defines how organisms appear:

```text
spawn_on_vine
spawn_at_node
random_ground_spawn
```

---

## VARIATION MODEL

Variation is controlled, not random.

Sources:

- parameter ranges
- seeded randomness
- environment influence

Rule:

```text
variation must preserve recognizability
```

---

## COMPOSITION PRINCIPLE

Species should be composed from reusable parts.

Avoid:

- monolithic species definitions

Prefer:

- shared lifecycle models
- shared growth models
- shared shape systems

---

## INTERACTION WITH OTHER SYSTEMS

### Lifecycle

Species defines lifecycle parameters.

---

### Environment

Species reacts via modifiers:

```text
growth_speed *= environment_factor
```

---

### Morphology

Species influences:

- node spacing
- branching angles
- axis persistence

---

### Rendering

Species selects:

- renderer type
- shape language

---

## GREENHOUSE INTEGRATION

Greenhouse mode allows:

- selecting species
- spawning instances
- modifying parameters live
- observing lifecycle

---

## DESIGN CONSTRAINTS

Avoid:

- over-generalization
- deeply nested definitions

Prefer:

- small building blocks
- explicit parameters
- readable structure

---

## FINAL PRINCIPLE

```text
Species define possibility
Organisms express it
```

---

## NEXT

→ 14_morphology.md
