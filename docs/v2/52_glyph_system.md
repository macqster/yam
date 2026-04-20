
# YAM v2 — GLYPH SYSTEM

---

## PURPOSE

Define the Glyph System as the **formal vocabulary of visual primitives** used to represent structure, density, and detail in the YAM renderer.

This system bridges:

- morphology → shape
- shape → rendering
- rendering → textmode composition

---

## CORE PRINCIPLE

Glyphs are:

> semantic visual units, not pixels or brightness levels

They represent:

- direction
- structure
- mass
- texture

---

## POSITION IN ARCHITECTURE

```text
Morphology → Shape → Glyph System → Renderer → Framebuffer
```

---

## GLYPH MODEL

```text
Glyph:
  char
  family
  weight
  direction
  density_tier
```

---

## GLYPH FAMILIES

Glyphs are grouped into controlled families.

---

### 1. STRUCTURAL FAMILY

Used for:

- vines
- axes
- skeleton

```text
│ ─ ╱ ╲ ┌ ┐ └ ┘
```

Properties:

- directional
- high readability
- low noise

---

### 2. MASS / FILL FAMILY

Used for:

- dense clusters
- shadow massing

```text
█ ▓ ▒ ░
```

Properties:

- high visual weight
- low directionality

---

### 3. DETAIL FAMILY

Used for:

- subtle variation
- soft transitions

```text
. , ' `
```

Properties:

- low weight
- high noise risk

Rule:

```text
use sparingly
```

---

### 4. ACCENT FAMILY

Used for:

- highlights
- particles

```text
* + x
```

Properties:

- high visibility
- rare usage

---

### 5. BRAILLE FAMILY (HIGH RES)

Used for:

- fine detail
- sub-cell resolution

Example:

```text
⣿ ⣶ ⣤ ⣀
```

Properties:

- high density
- smooth gradients

---

## DENSITY TIERS

Each glyph belongs to a density tier.

```text
TIER 0 → empty
TIER 1 → light detail
TIER 2 → structural
TIER 3 → dense
TIER 4 → solid mass
```

---

## GLYPH SELECTION RULES

Glyph choice depends on:

```text
structure type
+ density
+ role (edge / interior)
+ theme
```

---

### Example Mapping

```text
EDGE + LOW density → │ ─ ╱ ╲
EDGE + HIGH density → █ or strong block
INTERIOR + HIGH density → ▓ ▒
INTERIOR + LOW density → (empty or suppressed)
```

---

## DIRECTIONAL CONSISTENCY

Structural glyphs must match direction.

```text
vertical → │
horizontal → ─
diagonal → ╱ ╲
```

Rule:

```text
never mismatch direction and glyph
```

---

## REGION STABILITY

Within a region:

```text
dominant glyph family must remain consistent
```

Avoid:

- rapid glyph switching
- mixed families in small area

---

## GLYPH WEIGHT HIERARCHY

```text
empty < detail < structure < fill < block
```

Used for:

- depth
- emphasis
- layering

---

## THEME INFLUENCE

Theme affects:

- which families are preferred
- how density maps to glyph weight

Example (BTAS):

```text
prefer structure + block
avoid excessive detail glyphs
```

---

## NOISE CONTROL

Rules:

- limit glyph variety per region
- smooth transitions between tiers
- suppress detail in dense areas

---

## GLYPH TRANSITIONS

Transitions between densities must be gradual.

```text
░ → ▒ → ▓ → █
```

Avoid abrupt jumps unless intentional (edges).

---

## PERFORMANCE CONSIDERATIONS

- predefine glyph sets
- avoid dynamic random selection
- use lookup tables for mapping

---

## DESIGN CONSTRAINTS

Avoid:

- large uncontrolled glyph sets
- random glyph usage
- brightness-only mapping

Prefer:

- small curated families
- deterministic mapping
- structural clarity

---

## FINAL PRINCIPLE

```text
Glyphs are a language
Not a fallback for pixels
```

---

## NEXT

→ renderer implementation / Go module
