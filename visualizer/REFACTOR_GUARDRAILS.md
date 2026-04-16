

# Refactor Guardrails

Date: 2026-04-16
Scope: `visualizer/src/ivy_engine.py`
Purpose: constrain refactor work so behavior remains stable and changes are explainable

---

## Core Rule

> Do NOT change behavior during Stage 1 refactor.

All changes must be structural (extraction, naming, comments) only.

---

## Phase Integrity Rules

The engine phases are fixed and must be preserved:

1. Structural Growth
2. Foliage Host Discovery
3. Spatial Shaping (Art Direction)
4. Ornament Reconstruction

### Requirements
- Do NOT reorder phases
- Do NOT merge phases
- Do NOT move logic across phase boundaries unless explicitly instructed

---

## Engine vs Ornament Separation

- Engine decides **where things exist**
- Ornament decides **how things look**

### Prohibited
- Moving density logic into ornament
- Creating or removing hosts during ornament phase
- Compensating for weak growth logic with glyph tricks

---

## Collision & Layout Rules

- Mask is the **canonical collision geometry**
- All growth must respect `layout.allowed_cells`

### Prohibited
- Bypassing mask checks
- Reintroducing bounding-box-driven collision as primary logic

---

## Behavior Preservation Rules

- Preserve current pass order inside `tick()`
- Preserve all existing parameters and their meaning
- Preserve probabilistic behavior (including deterministic pseudo-random patterns)

### Allowed
- Extract helper methods
- Rename internal variables for clarity (without semantic change)
- Add comments

### Prohibited
- Changing thresholds or probabilities
- Changing spatial bias behavior
- Changing branching frequency or rules

---

## Debug & Observability

- Debug overlays and counters are **first-class**

### Requirements
- Do NOT remove debug hooks
- Do NOT desync debug output from actual behavior

---

## Commenting Standard (MANDATORY)

All new or refactored code must include detailed `#` comments.

### Each helper must explain:
- WHAT it does
- WHY it exists (intent, not just mechanics)
- WHICH phase it belongs to

### Example style
```python
# Phase 3: Spatial Shaping
# Applies upper canopy breakup to avoid rigid horizontal lines.
# This preserves a natural drooping silhouette and prevents banding.
```

---

## Refactor Scope (Stage 1)

### Allowed Work
- Extract phase helpers (e.g. `_advance_tips`, `_collect_hosts`, etc.)
- Group related logic into named functions
- Add or improve comments

### Not Allowed
- New features
- Parameter changes
- Behavioral tuning
- Performance rewrites that alter execution order

---

## Known Non-Core Behaviors (Do Not Remove Yet)

These are allowed to remain but must be isolated and clearly named:

- upper-left enrichment bias
- lower-right readability thinning
- top-run breakup logic
- global horizontal suppression
- top-left hanging stem prototype

---

## Acceptance Criteria for Stage 1

A refactor is valid only if:

- Visual output is indistinguishable (within randomness tolerance)
- Debug overlays still align with layout and mask
- Code is more readable and phase boundaries are clearer

---

## One-Line Rule for Codex

> Extract structure, preserve behavior, document intent.
