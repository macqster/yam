

# Tuning Map

Date: 2026-04-16
Scope: visualizer engine + config interaction
Purpose: provide a fast diagnostic guide for where to tune when visual issues appear

---

## Core Principle

Do NOT start tuning randomly.

Always identify:
1. which phase is responsible
2. which state is involved
3. which parameters influence that phase

---

## Phase → Problem Mapping

| Problem Type | Likely Phase | First Area To Inspect |
|---|---|---|
| Growth stops early / feels dead | Structural Growth | tip life, branching, movement rules |
| Trunk path looks wrong | Structural Growth (route state) | trunk phase logic, movement bias |
| Leaves appear in wrong places | Host Discovery | mature-host detection, connectivity logic |
| Canopy density feels off | Spatial Shaping | enrichment, thinning, density bias |
| Scene feels unbalanced (one side heavy) | Spatial Shaping | spatial bias / field logic |
| Top edge forms a hard line | Spatial Shaping | top-run breakup, directional shaping |
| Trunk gets visually buried | Spatial Shaping | lower-right thinning, density suppression |
| Leaves look wrong (style, glyphs) | Ornament Reconstruction | ornament/glyph logic only |
| Debug overlay mismatch | Debug / Layout | layout + mask + debug sync |

---

## Quick Diagnostic Guide

### Problem: Canopy too sparse

Check in order:
1. Structural Growth
   - are enough tips surviving?
   - is branching happening?
2. Host Discovery
   - are mature hosts being detected?
3. Spatial Shaping
   - is thinning too aggressive?

Do NOT:
- increase leaf rendering density in ornament

---

### Problem: Canopy too dense / noisy

Check:
- Spatial Shaping
  - enrichment strength
  - thinning rules
  - suppression passes

Then check:
- Host Discovery (too many hosts generated?)

Do NOT:
- reduce glyph density as primary fix

---

### Problem: Hard horizontal line at top

Check:
- Spatial Shaping
  - top-run breakup logic
  - directional bias (sag / droop)

---

### Problem: Trunk not readable

Check:
- Spatial Shaping
  - lower-right thinning
  - density suppression near base

Then check:
- Structural Growth
  - trunk path correctness

---

### Problem: Growth avoids areas unexpectedly

Check:
- Layout
  - mask correctness
  - allowed_cells
- Structural Growth
  - movement constraints

Do NOT:
- “force” growth via ornament layer

---

### Problem: Leaves appear detached or random

Check:
- Host Discovery
  - mature host detection
  - neighborhood connectivity

Then:
- Spatial Shaping
  - jitter / repositioning

---

### Problem: Scene feels unbalanced (left/right/top bias)

Check:
- Spatial Shaping
  - enrichment zones
  - field bias logic

---

### Problem: Visual flicker / instability

Check:
- use of randomness
- ensure deterministic pseudo-random logic is used where possible

---

## Parameter Categories (Mental Model)

Think of parameters as belonging to one of these groups:

### Structural Parameters
Affect actual growth:
- tip life
- branching
- movement

### Host Parameters
Affect where foliage can exist:
- maturity thresholds
- connectivity rules

### Field / Shaping Parameters
Affect distribution:
- enrichment
- thinning
- directional bias
- breakup / suppression

### Ornament Parameters
Affect appearance only:
- glyph choice
- variation
- styling

---

## Golden Rules

### 1. Fix at the correct layer

- structure problem → fix in Structural Growth
- placement problem → fix in Host Discovery
- density / balance problem → fix in Spatial Shaping
- visual style problem → fix in Ornament

---

### 2. Do not compensate across layers

Bad pattern:
- weak growth → “add more leaves” in ornament

Correct pattern:
- fix growth or host generation

---

### 3. One change at a time

Always:
- change one parameter group
- observe result
- avoid stacking adjustments blindly

---

### 4. Use debug overlays first

Before tuning:
- confirm mask alignment
- confirm growth boundaries
- confirm host distribution

---

## Known Special Cases

These are not generic behavior and should be treated carefully:

- upper-left enrichment bias
- lower-right trunk readability thinning
- top-run breakup logic
- global horizontal suppression
- prototype hanging stems

If adjusting these:
- verify they do not break overall balance

---

## One-Sentence Rule

> Always tune the phase that owns the problem, not the layer that exposes it visually.
