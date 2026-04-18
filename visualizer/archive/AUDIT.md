

# Vines Engine Audit

Date: 2026-04-16
Scope: `visualizer/src/vines_engine.py`
Status: analytical audit only — no behavior changes proposed in this document

## Executive Summary

`vines_engine.py` is no longer a narrow growth routine. It currently acts as a combined:

1. simulation stepper
2. branch / trunk lifecycle manager
3. foliage-host generator
4. spatial post-processor
5. scene-specific art-direction controller

This explains both its current strength and its current fragility.

The engine is producing increasingly rich results because it now combines structural growth with several canopy-shaping passes. However, those responsibilities are concentrated inside `tick()`, which makes behavior ordering more important, tuning less transparent, and future extension riskier.

The current engine should be understood as:

> a hybrid between structural growth simulation and art-direction-aware canopy shaping

That is not a flaw by itself, but it is now the defining maintenance constraint.

---

## What the Engine Currently Does Well

### 1. Core tip lifecycle remains legible

The main simulation backbone is still conceptually clean:

- tip life decays
- expired tips are removed
- surviving tips attempt movement
- stems are committed into state
- branch opportunities are evaluated
- special hanger behavior may be injected
- active tips are capped

This is still a good engine core. The lifecycle has not dissolved into pure post-processing.

### 2. Trunk route phase model is strong

The route state system is one of the healthiest parts of the file.

Current trunk phases:

- `approach`
- `hero_top`
- `post_top`

This gives the trunk a readable high-level intent and replaces weaker purely local drift behavior. It is also easy to explain, debug, and extend.

### 3. Layout is treated as world geometry

The engine is clearly operating against scene constraints rather than painting blindly.

Observed world references include:

- `layout.allowed_cells`
- `layout.hero_guide`
- `layout.info_guide`

That is the correct direction: the scene is behaving like a constrained terminal world rather than a free ornament surface.

### 4. Mature-stem foliage hosting is a real upgrade

Foliage is no longer restricted to immediate tip positions.

The engine now derives foliage hosts from mature structural cells, which means:

- growth history influences later ornament placement
- trunk and branch maturity matter
- the scene gains a stronger sense of persistence

This is one of the most important structural upgrades in the current system.

### 5. Directional foliage bias is conceptually valuable

The current logic that relates stem orientation to leaf placement direction is simple, but important.

It is the first clear sign that foliage is beginning to respond to structural orientation rather than mere occupancy.

---

## Main Architectural Diagnosis

### The engine has become a behavior landfill

`vines_engine.py` now contains multiple kinds of logic that are all valid individually, but not all of the same conceptual type.

Examples currently coexisting in one place:

- core simulation
- trunk route choreography
- mature-host discovery
- directional foliage shaping
- upper-left enrichment bias
- lower-right readability thinning
- top-run breakup
- global horizontal continuity suppression
- special-case hanging stems / prototype behavior

This concentration creates three practical problems.

#### A. Behavior ordering matters too much

Several passes operate on the outputs of earlier passes. For example:

- enrichment adds potential foliage hosts
- directional logic reorients or reshapes them
- jitter perturbs them
- thinning removes some
- breakup removes more
- continuity suppression removes more again

The final look is therefore heavily dependent on pass order, but that order is currently implicit rather than formalized.

#### B. Local fixes can silently fight each other

Because many probabilistic and spatial passes are stacked, a later pass can partially undo an earlier one without obvious traceability.

This raises the risk of “mysterious regressions,” especially when tuning for one region causes losses elsewhere.

#### C. Tuning pressure is drifting toward superstition

Once multiple post-passes stack inside one method, it becomes harder to reason from cause to effect.

The maintenance risk is not only complexity. It is opacity.

---

## Biggest Pressure Point

### `tick()` is doing too much

The current `tick()` method appears to carry responsibility for:

- structural simulation
- tip spawning and branching
- special-case branch injection
- maturity analysis
- foliage host derivation
- host enrichment
- directional remapping
- jitter / sag shaping
- readability thinning
- top-band breakup
- continuity suppression
- ornament rebuild / stamping
- bookkeeping and debug-oriented state accumulation

This is the clearest maintainability problem in the file.

The issue is not style. The issue is that the conceptual phases of the engine are present but hidden.

---

## Hidden Internal Phase Structure

The current file already contains a natural internal phase model.

### Phase 1 — Structural Growth

Responsibilities:

- decay active tips
- move tips
- commit stems
- spawn branches
- spawn special hanger behavior
- limit active tip count

### Phase 2 — Foliage Host Discovery

Responsibilities:

- detect mature structural cells
- classify usable host positions
- infer local structure direction
- merge terminal and mature-host candidates

### Phase 3 — Spatial Shaping / Art Direction

Responsibilities:

- enrich sparse zones
- bias canopy toward selected regions
- introduce directional spread / sag
- thin readability-critical zones
- break unnatural horizontal continuity
- suppress over-uniform canopy bands

### Phase 4 — Ornament Reconstruction

Responsibilities:

- rebuild leaf stamps
- rebuild flower / accent stamps if present
- thicken wood where needed
- finalize renderable ornament state

The engine already behaves as if these phases exist. They are simply not expressed as explicit boundaries.

---

## Specific Drift Points

### 1. Direction inference appears split across multiple mini-systems

There seems to be more than one conceptual direction model active:

- mature-host direction inference
- later foliage-orientation logic

These may be compatible in practice, but they increase the chance of drift because they answer slightly different questions using partially overlapping heuristics.

Recommended conceptual split:

- one function should infer **stem orientation**
- one function should infer **leaf emission direction**

Those are related, but not identical.

### 2. Enrichment is effectively a field system in disguise

Upper-left biasing and other local enrichment behavior are not merely random boosts. They function like:

- field sampling
- region-weighted density modulation
- deterministic noise over spatial zones

This is a major concept that is currently encoded as inline arithmetic rather than named system behavior.

That is fine for experimentation, but it should be recognized as foundational.

### 3. Jitter / sag shaping is useful but under-defined

The current jitter logic appears to aim for several goals at once:

- break symmetry
- introduce droop / sag
- preserve cluster feel
- avoid over-rigid canopy geometry

That is valid. The problem is that the behavior currently reads more like threshold choreography than named system policy.

### 4. Top-run breakup is a custom morphology repair pass

The logic that detects and breaks upper horizontal continuity is clearly valuable. It reduces the “hard line across the top” look and introduces more organic droop.

But this is best understood as a dedicated canopy morphology correction pass, not generic trunk simulation.

It deserves a clear named identity in the future.

### 5. Lower-right thinning is composition policy, not neutral growth law

Preserving woody readability in the lower-right region is a legitimate visual goal.

However, it is best classified as scene composition policy:

- keep trunk readable
- reduce foliage clutter near the base
- preserve perceived structure

That matters because composition policy should ideally be easier to find, reason about, and disable than core simulation behavior.

### 6. Top-left hanging stems are prototype logic embedded in production flow

The hanging-stem / monstera prototype behavior is clearly experimental in character:

- cadence-driven
- spatially local
- direct tip injection into the active system
- art-direction motivated

This is not inherently wrong, but it is a clear marker that prototype behavior is already entering the main engine path.

---

## Durable Foundations Worth Preserving

These ideas feel like long-term keepers:

- `GrowthTip` lifecycle model
- phase-based trunk routing
- layout-driven world constraints
- mature structural cells as foliage hosts
- deterministic spatial variation
- rebuild-at-end ornament pipeline

These are the strongest conceptual foundations in the current engine.

---

## Likely Future Failure Modes

### 1. Inter-pass conflict

As more shaping passes accumulate, one pass may partially erase or distort the intent of another.

Typical symptoms could include:

- unexpected dead zones
- canopy collapse in a tuned region
- over-thinned edges
- persistent dominance of one side of the scene

### 2. Parameter opacity

A future maintainer may not know whether a visual issue should be solved by changing:

- growth
- branching
- enrichment
- jitter
- thinning
- breakup
- ornament reconstruction

This is a discoverability problem as much as a code problem.

### 3. Prototype creep

Without clearer boundaries, more experimental behaviors may continue entering the main engine path, increasing the cost of reasoning about the baseline system.

---

## Refactor Direction (Conceptual, Not Yet Prescriptive)

The next smart move is not necessarily new behavior.

The higher-value step is to make the engine’s internal phases explicit without changing its output.

A future helper-boundary model could look like this:

### Structural Growth Helpers

- `_advance_tips()`
- `_spawn_branch_if_needed()`
- `_spawn_info_hanger_if_needed()`

### Foliage Host Helpers

- `_collect_mature_hosts()`
- `_infer_host_directions()`

### Spatial Shaping Helpers

- `_apply_host_enrichment()`
- `_apply_directional_bias()`
- `_apply_canopy_jitter()`
- `_thin_base_readability_zone()`
- `_break_upper_horizontal_runs()`
- `_suppress_global_horizontal_continuity()`

### Ornament Rebuild Helpers

- `_rebuild_ornaments()`

Even if the behavior remained unchanged, a refactor like this would make the engine substantially easier to document, test, and extend.

---

## Documentation-Ready One-Sentence Summary

`vines_engine.py` currently orchestrates structural vine growth, stateful trunk routing, foliage-host derivation, and multiple spatial art-direction passes that shape canopy density and readability before ornament reconstruction.

---

## Practical Conclusion

### Healthy

- core lifecycle is still recognizable
- phase-based trunk routing is strong
- layout-aware world interaction is correct
- mature-host foliage logic is a major structural gain

### Drifting

- too many post-passes live inline
- scene-specific behavior is mixed with general growth logic
- direction logic appears partially duplicated conceptually
- art-direction policy is embedded in engine core

### Priority

Do not treat the next step as “add more features.”

Treat the next step as:

> make the internal phases explicit while preserving behavior

That creates the foundation needed before moving further into directional foliage or entity-based leaf logic.

---

## Stage 2 Planning Stub

### Objective

Stage 2 focuses on **conceptual cleanup**, not feature expansion.

The goal is to reduce internal drift and clarify core systems that are currently overlapping or duplicated.

---

### 1. Direction System Unification

Current state:
- direction is inferred in multiple places
- host discovery and foliage bias both derive direction independently

Problem:
- duplicated logic
- potential inconsistencies
- harder tuning and reasoning

Target:
- single, explicit function for **stem orientation inference**
- separate, explicit function for **foliage emission direction**
- clear data flow between them

---

### 2. Spatial Field Formalization

Current state:
- enrichment, thinning, and bias use inline deterministic arithmetic

Problem:
- behavior is correct but implicit
- difficult to reason about globally

Target:
- define spatial shaping as explicit “field” functions
- make region bias, density modulation, and suppression readable as system logic

---

### 3. State Flow Cleanup

Current observation:
- some transient outputs (e.g. structural-phase leaf hints) are not clearly consumed downstream

Problem:
- weak signal about what data actually matters between phases

Target:
- ensure each phase output is either:
  - consumed by the next phase
  - or removed
- reinforce STATE_MODEL ownership rules in code

---

### 4. Prototype Behavior Isolation

Current state:
- special-case logic (e.g. hanging stems) exists but is now helperized

Next step:
- optionally group prototype behaviors under a dedicated toggle or subsystem
- ensure they can be disabled without affecting core engine behavior

---

### 5. Phase Surface Clarity

Even after Stage 1 extraction, ensure:
- each helper clearly belongs to a phase
- no cross-phase leakage occurs
- comments remain aligned with ENGINE_SPEC contracts

---

### Non-Goals (Stage 2)

- no major visual redesign
- no parameter retuning as primary goal
- no new feature systems (e.g. full leaf entities yet)

---

### Exit Criteria

Stage 2 is complete when:

- direction logic is unified and traceable
- spatial shaping reads as an explicit system (not scattered heuristics)
- state flow between phases is unambiguous
- prototype behaviors are clearly isolated