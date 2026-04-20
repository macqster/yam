
# YAM v2 — HYBRID RENDERER

---

## PURPOSE

Define the Hybrid Renderer as the **execution layer that converts shape language into framebuffer writes using multiple rendering strategies**.

This document specifies:

- renderer types
- selection logic
- resolution strategies
- performance considerations

---

## CORE PRINCIPLE

The hybrid renderer is:

> a system that dynamically selects the best rendering strategy per shape

It does not:

- define shapes
- modify structure

It only:

> executes visual translation efficiently and consistently

---

## POSITION IN ARCHITECTURE

```text
Shape → Hybrid Renderer → Framebuffer
```

---

## CORE MODEL

```text
HybridRenderer:
  renderers[]
  selection_rules
```

---

## RENDERER TYPES

### 1. BRAILLE RENDERER

Uses high-density braille patterns.

Characteristics:

- high resolution (sub-cell detail)
- smooth curves
- dense visuals

Use cases:

- vines
- fine structure

---

### 2. GLYPH RENDERER

Uses symbolic characters.

Characteristics:

- stylized output
- readable forms
- lower density

Use cases:

- leaves
- ornaments

---

### 3. BLOCK RENDERER

Uses block characters.

Characteristics:

- medium resolution
- solid shapes
- good fill behavior

Use cases:

- background fills
- dense clusters

---

## SELECTION LOGIC

Renderer is chosen per ShapeInstance.

```text
if shape.type == vine:
  use braille
elif shape.type == leaf:
  use glyph
elif shape.type == dense:
  use block
```

---

## CONTEXT-AWARE SELECTION

Selection may depend on:

```text
zoom level
local density
performance mode
terminal size
```

Example:

```text
if density > threshold:
  downgrade to block renderer
```

---

## RESOLUTION STRATEGY

Resolution adapts dynamically:

```text
high detail → braille
medium → glyph
low → simplified symbols
```

Rule:

```text
favor readability over raw detail
```

---

## RENDER CONTRACT

Each renderer must:

```text
input: ShapeInstance
output: list of framebuffer writes
```

---

## WRITE BEHAVIOR

Renderers must:

- respect bounds
- respect masks
- respect z-index
- avoid redundant writes

---

## PERFORMANCE MODES

### High Quality

- full braille usage
- maximum detail

---

### Balanced

- mixed rendering
- adaptive detail

---

### Performance

- simplified glyphs
- reduced updates

---

## FALLBACK STRATEGY

If renderer fails:

```text
fallback → simple glyph renderer
```

---

## DEBUG SUPPORT

Hybrid renderer can expose:

- renderer usage overlay
- per-cell renderer source
- performance stats

---

## DESIGN CONSTRAINTS

Avoid:

- global renderer locking
- mixing logic with rendering

Prefer:

- per-shape decisions
- simple heuristics
- predictable output

---

## FINAL PRINCIPLE

```text
Use the simplest renderer that achieves the desired visual effect
```

---

## NEXT

→ 26_emitter.md
