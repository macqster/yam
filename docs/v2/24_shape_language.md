# YAM v2 — SHAPE LANGUAGE

---

## PURPOSE

Define the Shape Language as the **visual grammar that translates morphology into glyph-based representation**.

This document specifies:

- glyph systems
- shape construction rules
- lifecycle-driven variation
- consistency across renderers

---

## CORE PRINCIPLE

Shape language is:

> a deterministic mapping from structure to visual form

It does not:

- simulate behavior
- control structure

It defines:

> how structure is expressed visually

---

## POSITION IN ARCHITECTURE

```text
Morphology → Shape → Render → Framebuffer
```

Shape Language sits between **structure and rendering**.

---

## CORE MODEL

```text
ShapeInstance:
  position
  orientation
  scale
  glyph_set
  color
  density
  lifecycle_state
  z_index
```

---

## GLYPH SYSTEM

Shape language is built on curated glyph sets.

### Categories

```text
Structural Glyphs
Leaf Glyphs
Detail Glyphs
Particle Glyphs
```

---

### Structural Glyphs

Used for vines and axes:

```text
│ ─ ╱ ╲ ⎜ ⎟
```

Goal:

- continuity
- direction readability

---

### Leaf Glyphs

Used for organs:

```text
• ○ ● ◦ ◉
```

Goal:

- organic feel
- variation with density

---

### Detail Glyphs

Used for texture:

```text
. , ' `
```

Goal:

- break uniformity
- add softness

---

### Particle Glyphs

Used for transient effects:

```text
* ✦ +
```

---

## GLYPH RESOLUTION TIERS

Different levels of detail:

```text
LOW → simple symbols
MID → mixed glyphs
HIGH → braille / dense glyphs
```

Rule:

```text
resolution adapts to context and density
```

---

## ORIENTATION RULES

Glyphs align with structure direction.

```text
vertical → │
horizontal → ─
diagonal → ╱ ╲
```

---

## SHAPE CONSTRUCTION

Shapes are built from morphology.

```text
node → anchor point
internode → line glyphs
organ → cluster of glyphs
```

---

## DENSITY MAPPING

Density controls glyph choice.

```text
low density → sparse glyphs
high density → filled glyphs
```

---

## LIFECYCLE MAPPING

Visuals evolve with lifecycle:

```text
SEED → minimal dot
GROWTH → expanding lines
MATURITY → full shapes
AGING → dim / fragmented
DECAY → broken / sparse
```

---

## COLOR MAPPING

Colors are derived from:

- species palette
- environment factors
- lifecycle state

Example:

```text
growth → bright green
aging → desaturated
```

---

## VARIATION

Variation is controlled:

- glyph alternation
- slight positional jitter
- density fluctuation

Rule:

```text
variation must remain coherent
```

---

## CONSISTENCY RULES

- same structure → same glyph logic
- avoid random symbol switching
- preserve directional readability

---

## HYBRID SUPPORT

Shape language must support multiple renderers:

```text
braille → high detail
block → medium detail
glyph → stylized detail
```

Rule:

```text
shape logic is renderer-agnostic
```

---

## DEBUG CAPABILITIES

Shape system can expose:

- raw structure view
- glyph overlays
- density visualization

---

## DESIGN CONSTRAINTS

Avoid:

- overly complex glyph sets
- unreadable symbol clusters
- excessive randomness

Prefer:

- minimal sets
- strong visual identity
- consistent mapping

---

## FINAL PRINCIPLE

```text
Shape language defines how the system looks
Not how it behaves
```

---

## NEXT

→ 25_hybrid_renderer.md
