

# YAM v2 — OVERVIEW

---

## PURPOSE

Define what YAM is at a high level and establish a clear mental model for the entire system.

This document is the entry point for all other documentation.

---

## SYSTEM IDENTITY

YAM is a:

> terminal-based, real-time ecosystem engine rendered via ANSI output

It is not:
- a static renderer
- a dashboard
- a CLI tool with UI

It is:
- a simulation system
- running continuously
- projected into a terminal surface

---

## CORE IDEA

YAM renders a **living environment**.

The system:
- simulates organisms
- evolves them over time
- translates them into visual structures
- renders them into a terminal framebuffer

---

## HIGH-LEVEL ARCHITECTURE

YAM consists of four major layers:

```text
Runtime (Bubble Tea)
  ↓
Engine (simulation)
  ↓
Rendering (framebuffer + layers)
  ↓
Output (ANSI emitter)
```

---

## RUNTIME MODEL (BUBBLE TEA)

YAM runs inside a Bubble Tea application.

Bubble Tea follows the Model–Update–View architecture:

- Model → full application state
- Update → processes events and mutates state
- View → renders output from state citeturn0search0

This makes Bubble Tea:

> a deterministic event-driven runtime

for YAM.

---

## CORE LOOP

At runtime, YAM executes a continuous loop:

```text
receive input / tick
→ update simulation
→ render frame
→ emit ANSI
→ repeat
```

This loop is controlled by Bubble Tea’s event system.

---

## SYSTEM LAYERS

### 1. Runtime

Responsible for:
- event handling
- input
- mode switching
- UI composition

Implemented via Bubble Tea.

---

### 2. Engine (Simulation)

Responsible for:
- ecosystem state
- organisms (vines, structures, etc.)
- lifecycle
- environment

Produces:
- structured world state

---

### 3. Rendering

Responsible for:
- translating state → visual representation
- writing into framebuffer
- resolving layering and masks

Produces:
- framebuffer (cells)

---

### 4. Output

Responsible for:
- converting framebuffer → ANSI stream
- writing to terminal

---

## KEY CONCEPTS

### 1. Framebuffer

Terminal is treated as a 2D surface:

```text
cell(x, y) = char + fg + bg + z
```

All rendering writes into this buffer.

---

### 2. Layers

All visual elements are rendered as layers:

- hero
- vines
- ornaments
- particles
- UI

Layers do not print.

They write into framebuffer.

---

### 3. Determinism

Given:
- seed
- config
- time index

YAM should produce identical output.

---

### 4. Separation of Concerns

Strict separation:

```text
simulation ≠ rendering ≠ output
```

---

## MODES

YAM operates in multiple runtime modes:

- Visualizer → main ecosystem view
- Greenhouse → controlled simulation environment
- Debug → inspection and diagnostics

Modes change behavior, not architecture.

---

## PRODUCT IDENTITY

YAM is best described as:

> a terminal-native living ecosystem

Characteristics:
- ambient
- evolving
- reactive
- readable

---

## FINAL PRINCIPLE

```text
Bubble Tea controls execution
YAM defines the world
Renderer projects it
Terminal displays it
```

---

## NEXT

Continue with:

→ 01_architecture.md
