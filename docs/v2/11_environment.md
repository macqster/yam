
# YAM v2 — ENVIRONMENT

---

## PURPOSE

Define the Environment system as the **global, time-evolving state** that influences all organisms.

This document specifies:

- time model
- environmental parameters
- system influence on ecosystem
- integration with engine and runtime

---

## CORE PRINCIPLE

The environment is:

> a global modifier of behavior, not a controller

It does not create or own organisms.

It influences them.

---

## POSITION IN ARCHITECTURE

```text
Runtime → Engine → Morphology → Shape → Render → Framebuffer → Emit
             ↑
        Environment lives here
```

Environment is part of the **Engine layer**.

---

## CORE MODEL

```text
Environment
  ├── Time
  ├── Weather
  └── Global Factors
```

---

## TIME SYSTEM

### Multi-scale time

```text
Time:
  tick
  delta
  growth_time
  world_time
```

---

### Time domains

#### 1. Frame Time (fast)

- animation
- rendering cadence

---

#### 2. Growth Time (medium)

- vine extension
- lifecycle progression

---

#### 3. World Time (slow)

- day/night cycle (future)
- seasonal drift (future)

---

### Time rules

- driven by runtime ticks
- no direct system time usage
- deterministic progression

---

## WEATHER SYSTEM

### Definition

Weather is short-term variation of environmental conditions.

---

### State

```text
Weather:
  light
  temperature
  humidity
  wind
  precipitation
```

---

### Behavior

Weather changes gradually over time.

Rules:

- no abrupt changes
- values remain within defined ranges

---

## GLOBAL FACTORS

Environment exposes normalized modifiers:

```text
light_factor
humidity_factor
temperature_factor
wind_factor
```

Range:

```text
0.0 → minimal influence
1.0 → maximal influence
```

---

## SYSTEM INFLUENCE

Environment affects other systems by **modulating parameters**.

---

### Lifecycle

```text
growth_rate *= temperature_factor
decay_rate *= dryness_factor
```

---

### Vines / Growth

```text
direction += light_bias
branching_probability *= humidity_factor
```

---

### Morphology

```text
internode_length *= temperature_factor
leaf_size *= humidity_factor
```

---

### Rendering

```text
brightness = light_factor
color_shift = environment_palette
```

---

## DESIGN RULE

```text
Environment modifies parameters
It does not contain logic of other systems
```

---

## UPDATE MODEL

Environment updates during engine update phase:

```text
environment.update():
  update time
  update weather
  compute factors
```

---

## GREENHOUSE MODE

Environment can be overridden.

```text
greenhouse:
  fixed time
  fixed weather
  deterministic values
```

Used for:

- testing
- tuning
- isolation

---

## DEBUG CONTROL

Environment supports:

- time freeze
- manual parameter override
- factor visualization

---

## DESIGN CONSTRAINTS

Avoid:

- over-simulation
- excessive parameters
- sudden changes

Prefer:

- smooth transitions
- few strong signals
- subtle influence

---

## FINAL PRINCIPLE

```text
Environment shapes behavior
But never dictates it
```

---

## NEXT

→ 12_lifecycle.md
