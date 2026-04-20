
# YAM v2 — LIFECYCLE

---

## PURPOSE

Define the Lifecycle system as the **state machine governing all organism evolution**.

This document specifies:

- lifecycle states
- state transitions
- time integration
- interaction with other systems

---

## CORE PRINCIPLE

Lifecycle is:

> a deterministic state machine driven by time and environment

It defines how organisms:

- appear
- evolve
- degrade
- disappear

---

## POSITION IN ARCHITECTURE

```text
Runtime → Engine → Morphology → Shape → Render → Framebuffer → Emit
             ↑
        Lifecycle lives here
```

Lifecycle is part of the **Engine layer**.

---

## CORE MODEL

Each entity has lifecycle state:

```text
Entity:
  state
  age
  parameters
```

---

## STATE MACHINE

Standard lifecycle:

```text
SEED → GROWTH → MATURITY → AGING → DECAY → DEATH
```

---

## STATE DEFINITIONS

### SEED

- minimal or invisible
- waiting for activation

---

### GROWTH

- rapid change
- structure formation

---

### MATURITY

- stable form
- full visual expression

---

### AGING

- gradual degradation
- color shift

---

### DECAY

- structural breakdown
- fragmentation

---

### DEATH

- removal or transformation

---

## TIME INTEGRATION

Lifecycle progression:

```text
age += delta * growth_rate * environment_factor
```

Rules:

- driven by runtime ticks
- deterministic

---

## TRANSITIONS

Transitions are based on:

```text
age thresholds
environment modifiers
randomness (seeded)
```

Example:

```text
if age > growth_threshold:
  state = MATURITY
```

---

## SYSTEM INTERACTIONS

### Environment

```text
growth_rate *= temperature_factor
decay_rate *= dryness_factor
```

---

### Species

Species defines:

- lifecycle durations
- transition thresholds
- visual mapping

---

### Morphology

Lifecycle controls:

- when growth is active
- when branching occurs

---

### Rendering

Lifecycle maps to visuals:

```text
SEED → dot
GROWTH → expanding pattern
MATURITY → full shape
AGING → color shift
DECAY → broken pattern
DEATH → removed
```

---

## UPDATE MODEL

Lifecycle updates during engine update:

```text
lifecycle.update(entity):
  update age
  evaluate transitions
  update state
```

---

## VARIATION

Variation comes from:

- parameter ranges
- seeded randomness
- environment modulation

---

## PERFORMANCE RULES

- limit active entities
- remove dead entities early
- batch updates where possible

---

## DESIGN CONSTRAINTS

Avoid:

- excessive micro-states
- complex branching logic

Prefer:

- simple transitions
- readable behavior
- predictable progression

---

## FINAL PRINCIPLE

```text
Lifecycle defines when things change
Not how they are rendered
```

---

## NEXT

→ 13_species.md
