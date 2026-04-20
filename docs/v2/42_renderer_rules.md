
# YAM v2 — RENDERER RULES (BTAS INTEGRATION)

---

## PURPOSE

Define the Renderer Rules as the **decision layer that translates BTAS rendering principles into concrete per-cell behavior**.

This document specifies:

- glyph selection logic
- brightness mapping
- edge detection
- contrast enforcement

---

## CORE PRINCIPLE

Renderer rules are:

> deterministic decisions that convert structure + theme into final visual output

They are:

- local (per-cell or small neighborhood)
- consistent (same input → same output)

---

## RENDER DECISION PIPELINE

For each ShapeInstance cell:

```text
1. classify (edge / interior / empty)
2. compute density
3. compute lighting
4. select glyph
5. apply color
6. write to framebuffer
```

---

## 1. CELL CLASSIFICATION

Each cell is classified:

```text
EMPTY → no structure
EDGE → boundary of structure
INTERIOR → inside structure
```

---

### Edge Detection

```text
is_edge = neighbor_empty_count > 0
```

---

## 2. DENSITY EVALUATION

Density is derived from local neighborhood:

```text
local_density = count(active_neighbors) / max_neighbors
```

---

### Density Buckets

```text
LOW     → sparse
MEDIUM  → structured
HIGH    → dense cluster
```

---

## 3. LIGHTING MODEL

Global light direction:

```text
light_dir = (-1, -1)  // top-left
```

---

### Lighting Factor

```text
light_factor = dot(normal, light_dir)
```

Approximation:

```text
if cell faces light:
  brighter
else:
  darker
```

---

## 4. GLYPH SELECTION RULES

### Structural (vines)

```text
if EDGE:
  use strong glyph (│ ─ ╱ ╲)

if INTERIOR:
  use reduced glyph or none
```

---

### Density Influence

```text
LOW density → thin glyphs or gaps
MEDIUM → normal structural glyphs
HIGH → blockier / filled glyphs
```

---

### BTAS Constraint

```text
prefer fewer, stronger glyphs over many weak ones
```

---

## 5. BRIGHTNESS RULES

```text
base = theme.dark

if EDGE:
  brightness += high

if HIGH density:
  brightness += medium

if in shadow:
  brightness -= medium
```

---

## 6. COLOR APPLICATION

```text
default → neutral dark gray
edges → highlight_warm or accent_blue
interior → desaturated
```

---

### Accent Rule

```text
only apply accent if:
  EDGE && high contrast region
```

---

## 7. SHADOW MASSING

Interior suppression:

```text
if INTERIOR and density high:
  reduce glyph visibility
  darken color
```

This creates large shadow shapes.

---

## 8. CONTRAST ENFORCEMENT

Ensure separation:

```text
if neighbor brightness difference < threshold:
  increase edge brightness
```

---

## 9. NOISE SUPPRESSION

```text
if rapid glyph variation in area:
  stabilize to dominant glyph
```

---

## 10. DEPTH RULE

```text
foreground → brighter + sharper
background → darker + simplified
```

---

## 11. FINAL WRITE RULE

```text
write(x, y, glyph, fg_color, bg_color, z)
```

---

## PSEUDO-CODE (REFERENCE)

```text
for cell in shape:
  type = classify(cell)
  density = compute_density(cell)
  light = compute_light(cell)

  glyph = select_glyph(type, density)
  brightness = compute_brightness(type, density, light)
  color = apply_theme(brightness, type)

  framebuffer.write(cell, glyph, color)
```

---

## DESIGN CONSTRAINTS

Avoid:

- per-cell randomness
- inconsistent glyph switching
- low contrast output

Prefer:

- stable patterns
- strong silhouettes
- minimal variation

---

## FINAL PRINCIPLE

```text
Renderer decisions must reinforce silhouette, contrast, and clarity
```

---

## NEXT

→ implementation (Go renderer module)
