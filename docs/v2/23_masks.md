
# YAM v2 — MASKS

---

## PURPOSE

Define the Mask system as the **spatial constraint layer** that controls where rendering is allowed.

This document specifies:

- mask structure
- mask types
- interaction with rendering and vines
- integration with layers and framebuffer

---

## CORE PRINCIPLE

Masks are:

> spatial filters applied at render time

They do not:

- contain logic
- own entities

They only:

> allow or block writes to the framebuffer

---

## POSITION IN ARCHITECTURE

```text
Shape → Layer → Renderer → Mask → Framebuffer
```

Masks are applied during the **rendering stage**, at write time.

---

## CORE MODEL

```text
Mask:
  width
  height
  data[][]
```

---

## CELL STATE

Each mask cell defines a constraint:

```text
0 → free (allow write)
1 → blocked (deny write)
```

---

## MASK TYPES

### 1. HERO MASK

Defines the solid region of the hero.

Properties:

- blocks rendering inside silhouette
- defines interaction boundary

Effects:

- vines wrap around hero
- no overlap inside hero body

---

### 2. UI MASK

Defines protected UI regions.

Properties:

- blocks world rendering
- ignored by UI layer

Rule:

```text
world layers respect UI mask
UI layer ignores it
```

---

### 3. BOUNDARY MASK

Defines global limits.

Examples:

- terminal edges
- no-go zones

---

### 4. DEBUG MASK

Optional overlays for testing.

Examples:

- highlight blocked regions
- visualize mask alignment

---

## COMPOSITION

Multiple masks combine into a final mask:

```text
final_mask = hero_mask ∪ ui_mask ∪ boundary_mask
```

Rule:

```text
if any mask blocks → cell is blocked
```

---

## WRITE RULE

During rendering:

```text
if mask[x][y] == blocked:
  skip write
else:
  proceed
```

---

## SOFT VS HARD MASKS

### Hard Mask

- absolute block
- prevents all writes

---

### Soft Mask (future)

- reduces probability or intensity

Example:

```text
if soft_mask[x][y]:
  reduce growth or opacity
```

---

## INTERACTION WITH VINES

Masks affect growth indirectly:

- vines cannot grow into blocked cells
- vines align along mask edges

Effect:

- natural wrapping
- boundary-following behavior

---

## INTERACTION WITH LAYERS

| Layer | Mask Behavior |
|------|-------------|
| World Layers | respect masks |
| UI Layer | ignores masks |

---

## PERFORMANCE RULES

- mask lookup must be O(1)
- masks should be precomputed when possible
- avoid recomputing masks per write

---

## DEBUG CAPABILITIES

Masks can be visualized as:

- overlay grids
- colored regions
- edge highlights

---

## DESIGN CONSTRAINTS

Avoid:

- embedding logic in masks
- dynamic complex recomputation

Prefer:

- simple binary masks
- clear boundaries
- predictable behavior

---

## FINAL PRINCIPLE

```text
Masks define where rendering is allowed
Nothing more
```

---

## NEXT

→ 24_shape_language.md
