
# YAM v2 — TEXTMODE GRAPHICAL DESIGN

---

## PURPOSE

Define the Textmode Graphical Design system as the **medium-specific design layer** governing how visuals are composed using characters on a fixed grid.

This document establishes rules derived from:

- ASCII / ANSI art practices
- BBS / 16colo scene aesthetics
- terminal-native constraints

---

## CORE PRINCIPLE

Textmode graphics are:

> composition using discrete symbols on a fixed grid

Not:

- pixel rendering
- image approximation

---

## 1. GRID AS CANVAS

All rendering occurs on a **fixed-width grid**.

```text
framebuffer = design surface
```

Rules:

- every cell is equal size
- no sub-cell positioning
- layout is absolute

ASCII art fundamentally relies on monospaced grids where each character occupies identical space citeturn0search1

---

## 2. CHARACTER AS PRIMITIVE

Characters are not pixels.

Each glyph represents:

- shape
- direction
- visual weight

```text
│ ─ ╱ ╲ = structure
█ ▓ ░ = fill
. , ' = detail
```

---

## 3. SHAPE OVER DENSITY

Reject pure brightness mapping.

Avoid:

```text
image → grayscale → character
```

Prefer:

```text
structure → glyph family
```

ASCII systems traditionally map brightness to character density, but this produces noisy results citeturn0search0

---

## 4. COMPOSITION OVER SHADING

Textmode art is built from:

- lines
- blocks
- empty space

Not:

- gradients
- soft transitions

---

## 5. GLYPH FAMILIES

Define controlled glyph sets:

### Structural

```text
│ ─ ╱ ╲ ┌ ┐ └ ┘
```

### Fill

```text
█ ▓ ▒ ░
```

### Detail

```text
. , ' `
```

### Accent

```text
* + x
```

Rule:

```text
each region uses limited glyph family
```

---

## 6. REGION CONSISTENCY

Within a region:

- glyph usage must be stable
- avoid rapid switching

```text
stability > variation
```

---

## 7. NEGATIVE SPACE

Empty cells are intentional.

```text
empty = design element
```

Rules:

- preserve spacing
- avoid overfilling

---

## 8. COLOR AS STRUCTURE

ANSI systems allow limited color sets:

- 16 foreground
- 8 background citeturn0search10

Rules:

- color indicates hierarchy
- color supports shape
- color is not decoration

---

## 9. LAYERED THINKING

Even without explicit z-buffer:

```text
background
midground
foreground
```

Each layer uses:

- different density
- different brightness

---

## 10. BLOCK COMPOSITION

Prefer grouping:

```text
clusters > scattered cells
```

This improves:

- readability
- silhouette

---

## 11. ALIGNMENT DISCIPLINE

Rules:

- vertical and horizontal lines must align cleanly
- avoid broken diagonals
- preserve geometric coherence

---

## 12. NOISE CONTROL

Terminal output easily becomes noisy.

Avoid:

- too many glyph types
- flickering patterns

Prefer:

- dominant glyphs per region
- smooth transitions between regions

---

## 13. SYMBOLIC READABILITY

Glyphs should communicate structure.

Example:

```text
/ \  → slope
│   → vertical growth
```

---

## 14. TEXTMODE IDENTITY

This system must align with ANSI art traditions:

- constraint-driven design
- deliberate placement
- visual clarity over realism

ANSI art uses extended character sets and color control sequences to enhance composition beyond basic ASCII citeturn0search2

---

## DESIGN CONSTRAINTS

Avoid:

- photorealistic conversion
- gradient-heavy output
- uncontrolled randomness

Prefer:

- structural clarity
- compositional intent
- stable visual language

---

## FINAL PRINCIPLE

```text
Textmode graphics are composed, not approximated
```

---

## NEXT

→ integrate with renderer + UI panels
