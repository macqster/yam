# YAM v2 — BTAS RENDERING RULES

---

## PURPOSE

Define BTAS-inspired rendering rules as a **concrete visual behavior layer** applied on top of Theme and Shape systems.

This document translates aesthetic principles into **actionable rendering logic**.

---

## CORE PRINCIPLE

BTAS rendering is:

> light painted onto darkness

Instead of:

- drawing shapes on light background

We do:

```text
start from darkness → add light
```

This mirrors BTAS production where backgrounds were painted using light colors on black surfaces citeturn0search0

---

## 1. BACKGROUND DOMINANCE

```text
background = near-black (always dominant)
```

Rules:

- empty space stays dark
- do NOT fill space unnecessarily
- darkness is a feature, not absence

---

## 2. SILHOUETTE FIRST

Rendering priority:

```text
1. silhouette readability
2. internal detail
3. micro texture
```

Implementation:

- ensure vine shapes read clearly at distance
- prefer bold continuous forms

---

## 3. EDGE HIGHLIGHTING

BTAS uses edge lighting instead of gradients.

```text
edge = highlight
interior = suppressed
```

---

### Implementation

```text
if cell is edge:
  increase brightness
  use accent color
else:
  reduce brightness
```

---

## 4. SHADOW MASSING

Shadows are not gradients — they are blocks.

```text
large dark regions > detailed shading
```

Rules:

- collapse interior detail into shadow
- prefer large unified dark areas

---

## 5. MINIMAL DETAIL PRINCIPLE

```text
less detail = stronger image
```

Avoid:

- noisy glyphs
- excessive dithering

Prefer:

- single clear glyph over multiple weak ones

---

## 6. ANGULARITY & STRUCTURE

BTAS style favors:

- strong angles
- geometric flow

Implementation:

- bias vine growth toward clear directional lines
- reduce chaotic curvature in high-contrast areas

---

## 7. CONTRAST-BASED DEPTH

Depth is expressed through contrast, not blur.

```text
foreground → high contrast
midground → medium
background → very dark
```

---

## 8. COLOR AS ACCENT ONLY

Color is sparse and intentional.

```text
90% neutral/dark
10% accent
```

Rules:

- use accent colors for highlights only
- never saturate entire scene

---

## 9. DENSITY → LIGHT MAPPING

Density drives brightness.

```text
low density → almost invisible
mid density → readable
high density → highlighted
```

---

## 10. LINE ECONOMY

Inspired by animation constraints:

```text
fewer lines → stronger clarity
```

BTAS simplified designs to reduce line complexity citeturn0search2

---

## 11. NOISE SUPPRESSION

Terminal rendering can become noisy quickly.

Rules:

- avoid mixing too many glyph types
- stabilize glyph patterns per region
- prefer consistency over variation

---

## 12. LIGHT DIRECTION (OPTIONAL ADVANCED)

Introduce global light bias:

```text
light_direction = top-left (default)
```

Effect:

- one side of vines highlighted
- opposite side darkened

---

## 13. RENDERING STACK ORDER

Apply rules in order:

```text
1. base darkness
2. silhouette
3. edge highlight
4. density brightness
5. accent color
```

---

## 14. FAILURE MODES (WHAT TO AVOID)

Avoid:

- evenly lit scenes
- full brightness everywhere
- noisy glyph fields
- low contrast visuals

---

## FINAL PRINCIPLE

```text
BTAS style is not about detail
It is about controlled contrast and strong shapes
```

---

## NEXT

→ integrate into renderer + test presets
