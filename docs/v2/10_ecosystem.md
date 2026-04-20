
# YAM v2 — ECOSYSTEM

---

## PURPOSE

Define the ecosystem layer as the **core simulation domain** of YAM.

This document specifies:

- what exists in the world
- how entities are structured
- how systems interact
- how the engine updates state

---

## CORE PRINCIPLE

The ecosystem is:

> a structured collection of interacting organisms evolving over time

It is the **source of all simulation data**.

---

## POSITION IN ARCHITECTURE

```text
Runtime → Engine → Morphology → Shape → Render → Framebuffer → Emit
             ↑
        Ecosystem lives here
```

The ecosystem exists entirely inside the **Engine layer**.

---

## CORE MODEL

```text
Ecosystem
 ├── Organisms
 ├── Environment
 └── Systems (lifecycle, balance)
```

---

## ORGANISMS

Organisms are all entities that:

- exist in space
- evolve over time
- participate in lifecycle

### Types

#### 1. Hero

- central visual anchor
- animated (GIF-based)
- defines spatial gravity
- provides mask source

---

#### 2. Scaffold (Structural Flora)

- slow or static structure
- defines primary composition
- acts as support for vines

---

#### 3. Vines (Dynamic Flora)

- primary growth system
- expands through space
- reacts to environment and structure

---

#### 4. Independent Flora

- shrubs, plants, future species
- self-contained lifecycle
- adds diversity

---

## ENVIRONMENT

Environment is global state affecting all organisms.

```text
Environment:
  light
  temperature
  humidity
  time
  spatial constraints
```

Rules:

- environment does not render
- environment does not own organisms
- environment modifies parameters only

---

## TIME

Time is part of ecosystem state.

```text
Time:
  tick
  delta
  phase (day/season future)
```

Rules:

- all systems derive time from runtime ticks
- no direct system time access

---

## LIFECYCLE SYSTEM

All organisms follow lifecycle states:

```text
seed → growth → maturity → aging → decay → death
```

Each organism stores:

```text
state
age
parameters
```

Lifecycle controls:

- visual transitions
- growth activation
- decay and removal

---

## SPECIES SYSTEM

Species define behavior templates.

```text
Species:
  lifecycle_model
  growth_model
  renderer_binding
  parameters
```

Rule:

- species define behavior
- organisms execute behavior

---

## MORPHOLOGY INTERFACE

Ecosystem does not define structure directly.

It outputs:

```text
abstract growth state
```

Morphology converts it into:

```text
nodes / axes / internodes
```

---

## BALANCE SYSTEM

Ensures stability and readability.

Mechanisms:

- density control
- resource competition
- growth suppression
- aging and decay

Rule:

```text
growth must be self-limiting
```

---

## UPDATE MODEL

Ecosystem evolves only during Update():

```text
update():
  environment.update()
  lifecycle.update()
  ecosystem.update()
  balance.update()
```

Rules:

- no mutation outside Update()
- deterministic evolution

---

## DATA CONTRACT

Ecosystem outputs:

```text
world_state
```

This is consumed by:

```text
Morphology → Shape → Rendering
```

---

## DESIGN CONSTRAINTS

Avoid:

- rendering logic in ecosystem
- direct terminal interaction
- uncontrolled randomness

Prefer:

- structured state
- deterministic updates
- local interactions

---

## FINAL PRINCIPLE

```text
Ecosystem defines what exists
Engine defines how it evolves
Rendering defines how it appears
```

---

## NEXT

→ 11_environment.md
