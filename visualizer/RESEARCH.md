# YAM Visualizer — Rendering & Architecture Learnings (Deep Study Summary)

## Context

This document summarizes architectural insights gathered during iterative development of the YAM terminal visualizer, with cross-analysis of external projects:

- Ansizalizer (Zebbeni)
- Buddy (terminal renderer approach)
- Internal YAM scaffold + vines + mask system

Goal:
Improve **visual coherence, rendering correctness, and system architecture** without rewriting the project.

---

# 1. CURRENT YAM MODEL (BASELINE)

## Rendering structure
- Layered system:
  - Hero (Chafa → braille)
  - Vines (procedural growth, graph-based)
  - Scaffold (mask-guided pathing)
  - Panel

## Core mechanics
- Binary masks:
  - `hero_mask`
  - `trunk_mask`
- Pathfinding:
  - BFS / mask-guided routing
- Glyph-based rendering:
  - direct writes (`|`, `/`, `\\`, braille)

## Observed problems
- Scaffold:
  - technically correct, visually fragmented
- Trunk mask:
  - binary → produces unnatural geometry
- Rendering:
  - discrete, not continuous
- Layers:
  - compete instead of blending

---

# 2. KEY INSIGHT — ROOT PROBLEM

> The system models structure, not appearance.

Current:
- “Where should the path go?”

Needed:
- “What should this area look like?”

---

# 3. ANSIZALIZER — WHAT WE LEARNED

Source:
- https://github.com/Zebbeni/ansizalizer

## Core idea
Rendering is a **pipeline**, not direct drawing.

### Pipeline stages
1. Sampling
2. Color mapping
3. Glyph selection

## Key features
- Unicode block rendering (half/quarter/full)
- Custom character sets
- Alpha-aware rendering
- Dithering + resampling

## What matters for YAM

### 1. Glyph abstraction
Characters are interchangeable representation units.

### 2. Density-based rendering
pixel density → glyph

Instead of:
logic → glyph

### 3. Alpha handling
Edges should be smooth and approximated — not binary.

## Extracted principle

> Rendering should approximate a continuous field using discrete glyphs.

---

# 4. BUDDY — WHAT WE LEARNED

Source:
- https://github.com/JVSCHANDRADITHYA/buddy

## Core idea
Terminal cell = **two pixels encoded via color + glyph**

Example:
▀
fg = top pixel
bg = bottom pixel

## Key techniques

### 1. Dual-channel rendering
- foreground color
- background color

→ doubles vertical resolution

### 2. Area averaging
Instead of nearest-neighbor mapping, average area contribution per cell.

## Extracted principle

> Color carries most of the visual information — glyph is just a carrier.

---

# 5. CRITICAL GAP IN YAM

## Missing concepts

### ❌ Density field
No representation of how “full” a cell is.

### ❌ Soft masks
Only binary inclusion/exclusion.

### ❌ Unified render buffer
Each layer writes independently.

## Result
- fragmentation
- aliasing
- unnatural scaffold shapes

---

# 6. CORRECT MODEL (SYNTHESIS)

Replace:
geometry → glyph

With:
field → glyph

---

# 7. TARGET ARCHITECTURE

## 7.1 Unified render field

Cell:
    density: float   # 0.0 → 1.0
    color: tuple
    priority: int

All systems write into this.

---

## 7.2 Density → glyph mapping

Define mapping from density to braille characters (low → high density).

---

## 7.3 Soft trunk mask

Replace binary mask checks with a distance field.

---

## 7.4 Scaffold model

Current:
- BFS path through mask

Target:
- central trunk spine
- fork node
- controlled branch divergence

---

## 7.5 Dual-color cells (optional)

Use foreground/background color split for higher vertical resolution.

---

# 8. WHY CURRENT SCAFFOLD LOOKS WRONG

- follows mask ✔
- but lacks structural coherence ❌

Because:
- binary mask
- unconstrained pathfinding

---

# 9. WHAT ACTUALLY FIXES IT

### NOT
- more BFS tuning

### BUT
- structural constraints
- density-based rendering
- better mask geometry

---

# 10. CONFIG INSIGHT

Some scaffold config values may be partially ineffective because trunk mask dominates structure.

Config should influence selection within mask, not override it.

---

# 11. FINAL SYNTHESIS

## Ansizalizer
- pipeline
- density mapping
- glyph abstraction

## Buddy
- terminal cell physics
- color-first rendering
- sampling correctness

## YAM direction
- merge both

---

# 12. FINAL PRINCIPLE

Stop drawing paths. Start sampling fields.

---

# 13. FUTURE DIRECTIONS

1. Density field + glyph mapping
2. Soft trunk mask
3. Scaffold as ridge extraction
4. Dual-color rendering
5. Layer unification

---

# 14. LIMITATIONS

- Terminal resolution limits remain
- Braille is still best density glyph

---

# 15. SUMMARY

System is:
- technically correct
- visually incomplete


Next evolution:
- from graph-based → field-based rendering

---

# 16. CODEX-INGESTIBLE IMPLEMENTATION REPORT

## Purpose

This section translates the research findings into actionable, machine-ingestible guidance for automated agents (Codex) and future development passes.

---

## SYSTEM STATE SUMMARY

Current renderer type:
- Graph-based (discrete path + symbol placement)

Target renderer type:
- Field-based (continuous density + glyph approximation)

Core transition:
```
paths → fields
symbols → density-mapped glyphs
binary masks → distance fields
```

---

## VERIFIED WORKING COMPONENTS

- Config loading pipeline (visualizer.json → runtime)
- Mask alignment system (hero_mask, trunk_mask)
- Scaffold mask-guided routing (topologically correct)
- Layer ordering (scaffold → vines → hero → panel)

---

## IDENTIFIED FAILURE MODES

1. Scaffold fragmentation
   - Cause: BFS pathfinding without structural bias

2. Mask-driven geometry artifacts
   - Cause: binary trunk_mask

3. Visual discontinuity
   - Cause: direct glyph writes without density abstraction

4. Layer competition
   - Cause: no unified render buffer

---

## REQUIRED ARCHITECTURAL UPGRADES

### 1. Introduce Render Field

Add intermediate structure:

```
Cell:
    density: float
    color: tuple
    priority: int
```

All rendering systems write into this field before final glyph pass.

---

### 2. Implement Density → Braille Mapping

Add function:

```
def density_to_braille(d: float) -> str
```

Mapping requirement:
- monotonic density progression
- use braille Unicode block (U+2800–U+28FF)

---

### 3. Replace Binary Mask with Distance Field

Instead of:
```
(x, y) in trunk_mask
```

Use:
```
distance_to_mask(x, y)
```

Behavior:
- 0 = inside mask
- increasing values = outside mask

Used for:
- scaffold routing
- vines attraction

---

### 4. Scaffold Generation Model

Replace:
- unconstrained BFS path

With:
- vertical trunk spine extraction
- single fork node selection
- directional branch divergence

Constraints:
- must remain inside trunk_mask domain
- must be visually continuous

---

### 5. Optional Dual-Color Cell Upgrade

Introduce:
```
glyph = "▀"
fg_color = top
bg_color = bottom
```

Effect:
- doubles vertical resolution
- improves smoothness of gradients and edges

---

## CONFIG INTERPRETATION RULES

Scaffold config values must:
- influence selection within trunk_mask
- NOT override trunk_mask geometry

Examples:
- base_x → bias base selection
- fork_y → bias fork location
- reach → bias branch endpoints

---

## ACCEPTANCE TESTS

1. Changing `fork_y` shifts fork position visibly
2. Changing `base_x` moves trunk laterally
3. Changing reach values affects branch spread
4. Scaffold remains continuous (no fragmented segments)
5. Mask scaling/offset visibly affects scaffold shape

---

## NON-GOALS

- Full renderer rewrite
- Removal of existing vines system
- Replacement of braille rendering

---

## IMPLEMENTATION PRIORITY

1. Density → glyph mapping
2. Soft mask (distance field)
3. Scaffold spine + fork logic
4. Unified render buffer
5. Dual-color rendering (optional)

---

## FINAL DIRECTIVE

All future rendering logic should move toward:

> Sampling-based representation of visual fields

NOT:

> Discrete symbolic path construction

---
