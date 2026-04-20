
# YAM v2 — RENDER PIPELINE

---

## PURPOSE

Define the Render Pipeline as the **process that transforms simulation output into a final framebuffer state**.

This document specifies:

- pipeline stages
- layer orchestration
- renderer roles
- execution flow

---

## CORE PRINCIPLE

The render pipeline is:

> a deterministic, ordered sequence of transformations from structure to pixels

It does not:

- simulate behavior
- own state

It only:

> converts data into visual output

---

## POSITION IN ARCHITECTURE

```text
Runtime → Engine → Morphology → Shape → Render → Framebuffer → Emit
                                   ↑
                           Render Pipeline
```

---

## HIGH-LEVEL FLOW

```text
Input: morphological + lifecycle state

→ Shape Generation
→ Layer Rendering
→ Framebuffer Writes
→ Z-Resolution

Output: framebuffer
```

---

## PIPELINE STAGES

### 1. SHAPE GENERATION

Input:

```text
nodes / axes / organs
lifecycle state
species parameters
```

Output:

```text
ShapeInstances
```

Each ShapeInstance contains:

```text
position
orientation
scale
glyph set
color
z_index
```

---

### 2. LAYER ASSIGNMENT

Shapes are grouped into logical layers:

```text
HeroLayer
VinesLayer
OrnamentsLayer
ParticlesLayer
UILayer
```

Rule:

```text
layers are logical only
z-index defines final order
```

---

### 3. RENDERERS

Renderers convert shapes into cell writes.

Types:

```text
BrailleRenderer
GlyphRenderer
BlockRenderer
```

Each renderer:

```text
shape → set of framebuffer writes
```

---

### 4. FRAMEBUFFER WRITES

Renderers write to framebuffer:

```text
write(x, y, char, fg, bg, z)
```

Rules:

- must respect bounds
- must respect masks
- must use z-index

---

### 5. Z-RESOLUTION

Conflicts resolved via framebuffer:

```text
if incoming_z >= current_z:
  overwrite
```

---

### 6. FINALIZATION

After all layers:

```text
framebuffer is complete
```

Passed to emitter.

---

## EXECUTION ORDER

Per frame:

```text
clear framebuffer

render HeroLayer
render VinesLayer
render OrnamentsLayer
render ParticlesLayer
render UILayer
```

---

## HYBRID RENDERING

Different systems may use different renderers.

Example:

```text
vines → braille
leaves → glyph
particles → symbols
```

Rule:

```text
renderer choice is per-shape, not global
```

---

## MASK HANDLING

Masks are applied during writes:

```text
if mask[x][y] == blocked:
  skip write
```

---

## PERFORMANCE RULES

- minimize redundant writes
- avoid full recomputation where possible
- batch operations per renderer

---

## DEBUG PIPELINE

Pipeline can expose:

- shape overlays
- layer isolation
- z-index visualization

---

## DESIGN CONSTRAINTS

Avoid:

- logic in render stage
- cross-layer dependencies
- implicit ordering

Prefer:

- pure transformation stages
- explicit z-control
- simple data flow

---

## FINAL PRINCIPLE

```text
Render pipeline converts structure into visuals
Without altering the underlying system
```

---

## NEXT

→ 22_layers.md
