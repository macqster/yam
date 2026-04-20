
# YAM v2 — THEMES

---

## PURPOSE

Define the Theme system as the **visual identity layer** controlling color, contrast, glyph style, and lighting.

Themes provide a **coherent, constrained aesthetic** applied on top of the rendering system.

---

## CORE PRINCIPLE

A theme is:

> a constrained visual language applied consistently across the system

It does not:

- affect simulation
- affect structure
- affect logic

It only:

> defines how the system is perceived

---

## POSITION IN ARCHITECTURE

```text
Shape → Renderer → Theme → Framebuffer → Emitter
```

---

## CORE MODEL

```text
Theme:
  palette
  contrast_profile
  glyph_style
  lighting_model
  density_mapping
```

---

## 1. PALETTE

Themes use **restricted color palettes**.

### BTAS-inspired palette (default)

```text
background      → #0b0c10
primary_dark    → #2d2926
mid_gray        → #7c878e
accent_blue     → #003da5
accent_gold     → #f1be48
highlight_warm  → #efbe7d
```

---

### Rules

```text
limit palette size
prefer strong contrast
avoid gradient dependence
```

---

## 2. CONTRAST PROFILE

Controls brightness distribution.

### BTAS profile

```text
dark background
high contrast edges
minimal mid-tones
```

---

### Implementation

```text
low density → darker tones
high density → brighter tones
edges → accent highlights
```

---

## 3. GLYPH STYLE

Controls glyph selection bias.

### BTAS style

Prefer:

- bold glyphs
- strong silhouettes
- clean edges

Avoid:

- noisy symbols
- thin, unreadable glyphs

---

## 4. LIGHTING MODEL

Themes define perceived lighting.

```text
light = direction + intensity
```

---

### Effects

- highlight edges
- emphasize silhouettes
- suppress interior noise

---

### Example mapping

```text
brightness = light_factor × contrast_curve
```

---

## 5. DENSITY MAPPING

Maps structure density to visual weight.

```text
low density → sparse glyphs + dark colors
high density → dense glyphs + brighter tones
```

---

## 6. DEPTH VIA CONTRAST

Simulate depth without gradients.

```text
foreground → brighter
midground → neutral
background → darker
```

---

## 7. THEME VARIANTS

System supports multiple themes:

```text
btas_dark_deco (default)
noir_minimal
high_contrast_debug
monochrome
```

---

## 8. INTEGRATION WITH SYSTEM

Themes influence:

- color selection
- glyph bias
- brightness scaling

They do NOT influence:

- morphology
- lifecycle
- growth

---

## 9. RUNTIME SWITCHING

Themes can be changed dynamically.

```text
:theme btas
:theme monochrome
```

---

## 10. DEBUG SUPPORT

Themes may expose:

```text
contrast visualization
color usage heatmap
density-to-color mapping
```

---

## PERFORMANCE RULES

- avoid per-cell heavy computation
- precompute palette mappings
- cache color conversions

---

## DESIGN CONSTRAINTS

Avoid:

- large color palettes
- heavy gradients
- inconsistent glyph styles

Prefer:

- minimal palettes
- strong silhouettes
- high contrast

---

## FINAL PRINCIPLE

```text
Theme defines mood through constraint
Not complexity
```

---

## NEXT

→ 41_layout_presets.md
