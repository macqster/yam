# YAM Visualizer — Rendering & Architecture Learnings (Deep Study Summary)

## Context

Back to the canonical reference list:

- [../SOURCE_INDEX.md](../SOURCE_INDEX.md)

This document summarizes architectural insights gathered during iterative development of the YAM terminal visualizer, with cross-analysis of external projects:

- Ansizalizer (Zebbeni)
- Buddy (terminal renderer approach)
- Internal YAM scaffold + vines + mask system

Goal:
Improve **visual coherence, rendering correctness, and system architecture** without rewriting the project.

Focused extracts from this archive:

- [../SOURCE_INDEX.md](../SOURCE_INDEX.md)
- [EXTERNAL_INSPIRATIONS.md](EXTERNAL_INSPIRATIONS.md)
- [EXTERNAL_TERMINAL_LIBRARIES.md](EXTERNAL_TERMINAL_LIBRARIES.md)
- [EXTERNAL_TUI_TOOLKITS.md](EXTERNAL_TUI_TOOLKITS.md)
- [EXTERNAL_ASCII_ART.md](EXTERNAL_ASCII_ART.md)
- [EXTERNAL_COMPARISON_TABLE.md](EXTERNAL_COMPARISON_TABLE.md)
- [EXTERNAL_BINDINGS_AND_FRAMEWORKS.md](EXTERNAL_BINDINGS_AND_FRAMEWORKS.md)
- [EXTERNAL_YAM_SHOULD_COPY.md](EXTERNAL_YAM_SHOULD_COPY.md)
- [TERMINAL_RENDERING.md](TERMINAL_RENDERING.md)
- [PALETTE_DITHERING.md](PALETTE_DITHERING.md)
- [WORKFLOW_REPLAY.md](WORKFLOW_REPLAY.md)

Terminology note:

- this document predates the current nomenclature normalization
- places that say `trunk_mask` are referring to the present-day support-mask / scaffold-guidance concept

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

# 16. TERMINAL VISUAL ECOSYSTEM SURVEY

This section records the broader ANSI / terminal-visual tooling takeaways from the external research note.

## 16.1 Core ecosystem pattern

Most terminal image tools follow the same broad pipeline:

```text
input -> preprocess -> resize/color reduce -> glyph map -> ANSI output
```

The important conclusion is that visual quality is decided mostly before ANSI emission. Rendering matters, but preprocessing, palette choice, symbol choice, and thresholding matter more.

## 16.2 Chafa is a signal-compression system

Chafa is not just a renderer. It is a compression step that turns a source image or frame into a constrained terminal signal.

Useful control axes:

- symbol set selection
- truecolor vs palette-constrained output
- perceptual color space choice
- dithering mode
- preprocessing and thresholding
- background handling

Practical implication for YAM:

- hero quality should be tuned at the source/conversion layer
- layout and masking should not try to compensate for a bad hero footprint
- the renderer should be treated as the last stage of a larger visual pipeline

## 16.3 Glyph density matters more than glyph identity

The useful glyph spectrum is:

```text
ASCII -> block -> braille
```

Tradeoff:

- ASCII: readable, low fidelity
- block: stronger color fill, coarser detail
- braille: highest density, best for terminal-native detail

Useful synthesis:

- treat glyphs as density carriers
- map continuous fields to discrete glyphs
- mix glyph classes when structure, fill, and detail need different treatments

## 16.4 Dithering is visual style, not just correction

Dithering affects:

- texture
- perceived density
- stability over time
- directional artifacts

Useful models:

- Floyd–Steinberg for detail
- Atkinson for softer diffusion
- Bayer / ordered dithering for predictable structure
- serpentine diffusion to reduce directional bias

Practical implication for YAM:

- dithering choices should be scene-specific
- debug and presentation modes may want different style profiles
- noise is not always a defect; it can be part of the look

## 16.5 Palette and color space are first-class design decisions

Useful palette sources:

- curated palettes
- sampled palettes
- manual palettes

Useful color-space lesson:

- perceptual spaces matter
- RGB distance is not always what the eye sees

Practical implication for YAM:

- if color becomes a major part of the scene language, add explicit palette and perceptual tuning hooks
- do not assume raw RGB matching is enough for good terminal output

## 16.6 The terminal is a constrained display, not a neutral canvas

Constraints to keep in mind:

- character cells are not square
- glyph appearance depends on font support
- ANSI escape overhead can become a performance factor
- truecolor support is terminal-dependent
- gamma and brightness differ by terminal

Practical implication for YAM:

- test against the real baseline terminal
- do not treat output as device-independent
- build for readable structure first, then refine density

## 16.7 The ecosystem is fragmented

The survey reinforces that the ANSI ecosystem is split across:

- converters
- editors
- viewers
- TUI frameworks
- recording/replay tools

What is still missing is a unified scene system that combines:

- authoring
- preprocessing
- compositing
- procedural behavior
- runtime rendering

Practical implication for YAM:

- YAM’s architectural direction remains valid
- the project should continue emphasizing scene composition, mask/field logic, and runtime behavior instead of trying to become a generic converter

## 16.8 Authoring and replay are valuable abstractions

Notable workflow patterns from the wider ecosystem:

- named presets and saved configurations
- frame-by-frame preview
- deterministic replay
- structured session recording
- TUI-based live tweaking

Practical implication for YAM:

- recipe presets are worth keeping
- seeded replay and golden snapshots are good next tooling targets
- if something cannot be reproduced, it is too hard to tune

## 16.9 Organic visuals usually mix rules and randomness

Useful external pattern:

- deterministic structure gives coherence
- stochastic variation gives life

This shows up in:

- growth systems
- noise-driven placement
- procedural branching
- cellular and rule-based evolution

Practical implication for YAM:

- support/routing logic should stay structured
- ornament and branch variation can stay probabilistic
- the system should not collapse into pure randomness or pure geometry

## 16.10 Actionable takeaways for YAM

The research supports the following priorities:

1. field-driven rendering
2. soft masks and distance fields
3. glyph-density mapping
4. reproducible presets and replay
5. debug/introspection overlays
6. palette and density tuning as explicit controls

These are better investments than trying to over-optimize the ANSI emission step itself.

---

# 17. CODEX-INGESTIBLE IMPLEMENTATION REPORT

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
