# YAM v2 — ARCHITECTURE

---

## PURPOSE

Define the complete system structure of YAM and how all subsystems interact.

This document is the single source of truth for:

- data flow
- system boundaries
- execution model

---

## CORE PRINCIPLE

YAM is a **layered, unidirectional system**.

```text
Simulation → Morphology → Shape → Render → Framebuffer → Emit
```

Rules:

- no backward dependencies
- no cross-layer shortcuts
- each layer consumes only previous layer output

---

## HIGH-LEVEL FLOW

```text
Bubble Tea Runtime
  ↓
Update (events + time)
  ↓
Engine (simulation)
  ↓
Render Pipeline
  ↓
Framebuffer
  ↓
ANSI Emitter
  ↓
Terminal
```

---

## SYSTEM LAYERS

### 1. RUNTIME (Bubble Tea)

Role:

- controls execution loop
- handles input
- manages modes
- routes messages

Contract:

- does not contain simulation logic
- does not perform rendering logic

---

### 2. ENGINE (SIMULATION)

Sub-systems:

- ecosystem
- environment
- lifecycle
- species
- vines
- balance

Responsibilities:

- update world state
- evolve organisms
- apply environmental influence

Output:

- structured simulation state

---

### 3. MORPHOLOGY

Role:

- convert abstract growth into structure

Defines:

- nodes
- internodes
- axes

Output:

- spatial structure

---

### 4. SHAPE LANGUAGE

Role:

- convert structure into glyph-level forms

Defines:

- glyph patterns
- orientation rules
- lifecycle-based variations

Output:

- renderable shapes

---

### 5. RENDERING

Sub-systems:

- layer system
- renderer implementations (braille / block / glyph)
- mask system

Responsibilities:

- translate shapes → framebuffer cells
- apply masks
- resolve layering via z-buffer

Output:

- framebuffer

---

### 6. FRAMEBUFFER

Definition:

```text
Cell = (char, fg, bg, z)
```

Responsibilities:

- store final visual state
- resolve write conflicts via z

Rule:

- framebuffer is the single visual truth

---

### 7. EMITTER

Responsibilities:

- convert framebuffer → ANSI string
- handle cursor control
- output to terminal

Rule:

- no logic, only projection

---

## EXECUTION MODEL

Each frame follows:

```text
Update()
  → Engine update
  → Morphology build
  → Shape generation

Render()
  → clear framebuffer
  → layers write

Emit()
  → framebuffer → ANSI
```

---

## DATA FLOW CONTRACT

```text
Runtime → Engine → Morphology → Shape → Render → Framebuffer → Emit
```

Rules:

- forward-only
- no mutation outside owning layer
- explicit data passing only

---

## OWNERSHIP MODEL

| System | Owns |
|------|------|
| Engine | simulation state |
| Morphology | structure |
| Shape | glyph representation |
| Renderer | cell mapping |
| Framebuffer | final visual state |
| Runtime | control flow |

---

## MODES

Modes alter behavior, not architecture:

- Visualizer → full ecosystem
- Greenhouse → controlled simulation
- Debug → inspection overlays

Routing occurs in Runtime.

---

## RENDER PIPELINE (DETAIL)

```text
clear framebuffer

HeroLayer
VinesLayer
OrnamentLayer
ParticlesLayer
UILayer

z-buffer resolves conflicts
```

Rules:

- layers do not print
- layers write to framebuffer only
- z defines final visibility

---

## MASK INTEGRATION

Masks are applied during framebuffer writes:

```text
if mask[x][y] == blocked:
  discard write
```

Rules:

- masks align with framebuffer space
- masks are part of rendering, not side logic

---

## DETERMINISM

System must be reproducible:

Given:

- seed
- config
- time index

Output:

- identical frame

---

## PERFORMANCE RULES

- Update() must remain fast
- View() must remain pure
- heavy work → async commands

---

## FINAL PRINCIPLE

```text
Runtime drives execution
Engine defines behavior
Renderer defines appearance
Framebuffer defines truth
Emitter delivers output
```

---

## NEXT

→ 02_runtime_bubbletea.md
