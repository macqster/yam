

# Vines Engine Specification

Date: 2026-04-16
Scope: `visualizer/src/vines_engine.py`
Purpose: define **phase contracts** and **allowed responsibilities** for the engine without changing current behavior

---

## Core Principle

The engine is a **phase-structured system**.

Each phase:
- has clearly defined inputs
- produces clearly defined outputs
- must not silently mutate unrelated state

Rendering quality must emerge from **engine behavior**, not ornament hacks.

---

## Phase Overview

The engine operates in four conceptual phases per tick:

1. Structural Growth
2. Foliage Host Discovery
3. Spatial Shaping (Art Direction)
4. Ornament Reconstruction

These phases are currently implicit and must be preserved during refactor.

---

# 1. Structural Growth Phase

## Responsibility
Advance the simulation of the vine structure.

## Reads
- active tips
- layout.allowed_cells
- layout guides (hero/info)
- config (growth + branching)

## Writes
- updated tips
- stem state (occupied cells)
- trunk route state

## Allowed Actions
- decay tip life
- remove expired tips
- move tips
- commit new stems
- spawn branches
- inject special-case tips (e.g. hanger) IF explicitly scoped
- enforce max tip count

## Must NOT
- place leaves
- apply spatial density shaping
- thin or enrich regions
- modify ornament output

---

# 2. Foliage Host Discovery Phase

## Responsibility
Identify where foliage *can* exist based on structure.

## Reads
- committed stem state
- local neighborhood connectivity

## Writes
- set of foliage host positions
- optional orientation hints per host

## Allowed Actions
- detect mature structural cells
- include terminal tip positions
- infer local stem orientation

## Must NOT
- bias density by region
- randomly enrich or remove hosts
- apply art-direction rules

---

# 3. Spatial Shaping Phase (Art Direction Layer)

## Responsibility
Shape canopy density and distribution for readability and aesthetics.

## Reads
- foliage host positions
- layout (hero position, scene zones)
- implicit spatial fields (bias, thinning, etc.)

## Writes
- modified host set (add/remove/reposition)

## Allowed Actions
- enrich sparse regions
- thin dense regions
- apply directional bias (e.g. sag, spread)
- break unnatural patterns (e.g. horizontal lines)
- apply composition rules (e.g. trunk readability zones)

## Must NOT
- modify trunk structure
- alter tip lifecycle
- directly render glyphs

## Notes
This phase contains most “art-direction” logic.
It is expected to evolve, but should remain **explicit and named**, not hidden inline.

---

# 4. Ornament Reconstruction Phase

## Responsibility
Convert structural + host data into renderable glyph output.

## Reads
- final host positions
- stem state
- config (ornament styling)

## Writes
- leaf stamps
- flower stamps (if present)
- wood/thickness representation

## Allowed Actions
- assign glyphs
- apply visual variation
- rebuild full ornament buffer

## Must NOT
- change density decisions
- create new hosts
- compensate for weak growth logic

---

# Cross-Phase Rules

## 1. Phase Ordering Is Fixed

The phases must execute in this order:

Structural Growth → Host Discovery → Spatial Shaping → Ornament Reconstruction

Changing this order changes system behavior.

---

## 2. Engine vs Ornament Separation

- Engine decides **where things exist**
- Ornament decides **how things look**

Violation of this rule leads to brittle visuals.

---

## 3. Mask Is Canonical Collision

- layout mask defines blocked cells
- growth must respect mask at all times
- no phase may override mask-based collision

---

## 4. Deterministic Noise Preferred

Where randomness is used:
- prefer deterministic pseudo-random logic
- avoid frame-to-frame flicker

---

## 5. Debug Is First-Class

- debug overlays must remain accurate
- phase changes must not desync debug from reality

---

# Known Non-Core Behaviors (Current State)

These exist but are not fundamental engine laws:

- upper-left enrichment bias
- lower-right thinning for trunk readability
- top-run breakup logic
- global horizontal suppression
- top-left hanging stem prototype

These should remain identifiable and separable.

---

# Refactor Goal (Non-Behavioral)

The immediate goal is:

> make phases explicit WITHOUT changing output

This includes:
- extracting helper methods per phase
- adding detailed `#` comments in code
- preserving current pass order and logic

No behavioral change should occur during this step.

---

# One-Sentence Definition

The vines engine is a phase-structured system that grows vine structure, derives foliage hosts, shapes canopy distribution through spatial rules, and reconstructs terminal glyph output.
