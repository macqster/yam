
# YAM v2 — LAYERS

---

## PURPOSE

Define the Layer system as the **logical grouping of renderable elements before framebuffer composition**.

This document specifies:

- layer responsibilities
- z-index mapping
- separation between world and UI
- interaction with render pipeline

---

## CORE PRINCIPLE

Layers are:

> logical containers of rendering intent

They do not:

- control final visibility
- resolve conflicts

They only:

> submit draw operations to the framebuffer

---

## POSITION IN ARCHITECTURE

```text
Shape → Layer → Renderer → Framebuffer
```

Layers exist inside the **Render Pipeline stage**.

---

## CORE MODEL

```text
Layer:
  name
  z_range
  renderer
  enabled
```

---

## LAYER TYPES

### 1. WORLD LAYERS

These represent the ecosystem.

```text
HeroLayer
VinesLayer
OrnamentsLayer
ParticlesLayer
```

---

### 2. UI LAYER

Represents observer instrumentation.

```text
UILayer
```

Rule:

```text
UI is always above world layers
```

---

## Z-INDEX STRATEGY

Layers are assigned z-index ranges.

Example:

```text
HeroLayer        → 100–199
VinesLayer       → 200–299
OrnamentsLayer   → 300–399
ParticlesLayer   → 400–499
UILayer          → 1000–1999
```

Rules:

- each layer owns a z-range
- elements inside layer vary within that range
- no overlap between layer ranges

---

## LAYER RESPONSIBILITIES

### HeroLayer

- renders central anchor
- highest priority among world elements

---

### VinesLayer

- renders structural growth
- main visual density contributor

---

### OrnamentsLayer

- renders leaves, flowers, details
- sits above vines

---

### ParticlesLayer

- renders transient effects
- may overlap other layers

---

### UILayer

- renders panels and overlays
- not affected by masks

---

## EXECUTION MODEL

Layers are processed sequentially:

```text
for layer in layers:
  for shape in layer:
    renderer.write(shape)
```

---

## DECOUPLING RULE

Layers must not depend on each other.

```text
no layer reads another layer
no layer modifies another layer
```

All interaction happens via framebuffer.

---

## MASK BEHAVIOR

World layers:

- respect masks

UI layer:

- ignores masks

---

## ENABLE / DISABLE

Each layer can be toggled:

```text
layer.enabled = true / false
```

Used for:

- debugging
- greenhouse mode
- performance tuning

---

## DEBUG CAPABILITIES

Layer system enables:

- isolating individual layers
- visualizing contributions
- debugging z conflicts

---

## PERFORMANCE CONSIDERATIONS

- skip disabled layers
- batch shapes per layer
- minimize cross-layer duplication

---

## DESIGN CONSTRAINTS

Avoid:

- implicit ordering
- shared state between layers
- mixing UI and world elements

Prefer:

- strict separation
- explicit z-ranges
- simple iteration model

---

## FINAL PRINCIPLE

```text
Layers organize rendering
Framebuffer resolves it
```

---

## NEXT

→ 23_masks.md
