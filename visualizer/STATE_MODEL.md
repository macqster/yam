

# Ivy Engine State Model

Date: 2026-04-16
Scope: `visualizer/src/ivy_engine.py`
Purpose: document the engine’s main runtime data structures, ownership, and update lifecycle

---

## Core Principle

The engine state is not one flat blob.

It consists of several distinct kinds of runtime data:

1. structural simulation state
2. route / choreography state
3. foliage-host derivation state
4. ornament/render output state
5. debug / observability state

Each state type should have a clear owner and update phase.

---

## State Categories Overview

| Category | Purpose | Primary Owner | Updated In |
|---|---|---|---|
| Active Tip State | drives live structural growth | Structural Growth phase | Phase 1 |
| Stem / Occupancy State | records committed vine structure | Structural Growth phase | Phase 1 |
| Route State | controls high-level trunk intent | Structural Growth phase | Phase 1 |
| Foliage Host State | identifies where foliage may exist | Host Discovery + Spatial Shaping | Phases 2–3 |
| Ornament State | stores renderable leaf/wood stamps | Ornament Reconstruction | Phase 4 |
| Debug State | tracks diagnostics and validation signals | any relevant phase | cross-phase |

---

# 1. Active Tip State

## Purpose
Represents currently living growth fronts.

This is the primary dynamic simulation state.

## Canonical Structure
Expected anchor object:
- `GrowthTip`

Expected fields include:
- position (`x`, `y`)
- direction (`dx`, `dy`)
- life / remaining growth potential
- max life reference
- structural identity flag(s), such as trunk vs branch

## Owner
Structural Growth phase

## Lifetime
Short-to-medium lived.
Tips are created, advanced, branched, and deleted continuously.

## Reads
- movement heuristics
- collision rules
- branching rules
- route state for trunk-specific decisions

## Writes
- updated tip list / collection
- newly committed stem positions
- newly spawned child tips

## Must NOT Be Used For
- direct ornament rendering
- persistent canopy density decisions after the tip has already matured into structure

---

# 2. Stem / Occupancy State

## Purpose
Represents committed vine structure in world space.

This is the persistent structural memory of the engine.

## Typical Contents
May include structures such as:
- occupied stem positions
- trunk-specific positions
- branch positions
- age / maturity hints tied to occupied cells

## Owner
Structural Growth phase

## Lifetime
Long-lived within the run.
Once a stem cell is committed, later phases treat it as part of the structural world.

## Reads
- host discovery
- local connectivity analysis
- ornament reconstruction
- debugging / region accounting

## Writes
- new committed positions during tip advancement
- possible age / maturity updates over time

## Must NOT Be Used For
- ad hoc visual patching
- phase-skipping logic that bypasses host discovery

---

# 3. Route / Choreography State

## Purpose
Stores high-level intent for special structural actors, especially the main trunk.

This state exists so the trunk can behave like a guided organism rather than pure local drift.

## Typical Contents
May include:
- trunk phase markers (`approach`, `hero_top`, `post_top`)
- one-off trigger flags
- state used to avoid repeated special spawns

## Owner
Structural Growth phase

## Lifetime
Medium-to-long lived.
Persists as long as the associated guided behavior is relevant.

## Reads
- trunk movement selection
- special-case branch injection logic

## Writes
- route phase updates
- trigger consumption / one-shot flags

## Must NOT Be Used For
- leaf shaping
- ornament styling
- generic density control

---

# 4. Foliage Host State

## Purpose
Represents positions where foliage is allowed or encouraged to exist.

This is not yet final render output.
It is an intermediate state between structure and ornament.

## Typical Contents
May include:
- host position set
- orientation hints per host
- host subsets derived from mature structure vs tips
- spatially shaped / enriched / thinned host variants

## Owners
- Host Discovery phase (initial derivation)
- Spatial Shaping phase (modification and redistribution)

## Lifetime
Short-lived per tick, but conceptually central.
It is typically rebuilt or heavily refreshed from structure.

## Reads
- committed stem structure
- neighborhood connectivity
- spatial field rules
- scene composition zones

## Writes
- final host set passed into ornament reconstruction

## Must NOT Be Used For
- direct trunk simulation
- collision overrides
- glyph-specific styling decisions

---

# 5. Ornament State

## Purpose
Stores renderable output derived from structure + foliage hosts.

This is the state the renderer ultimately consumes.

## Typical Contents
May include:
- leaf stamp positions
- flower / accent stamp positions
- thickened wood positions
- glyph-ready structural overlays

## Owner
Ornament Reconstruction phase

## Lifetime
Short-lived to medium-lived, depending on whether rebuilt every tick or cached between ticks.

## Reads
- final foliage hosts
- committed stem structure
- ornament style rules

## Writes
- render-ready stamp collections
- any final ornament buffers exposed to renderer

## Must NOT Be Used For
- solving structural density problems
- inventing missing hosts
- compensating for weak growth logic with visual hacks

---

# 6. Debug / Observability State

## Purpose
Tracks diagnostics needed to understand and validate engine behavior.

This state is not decorative. It is part of the development workflow.

## Typical Contents
May include:
- failed move counts
- spawn origin counts
- region coverage tracking
- counters for special behavior activation
- snapshots or aggregates useful for overlay/debug readout

## Owner
Cross-phase; whichever phase produces the signal should update it

## Lifetime
Variable.
Some values may reset per tick; others may accumulate for session-level diagnosis.

## Reads
- debug overlay generation
- tuning and regression analysis

## Writes
- any relevant phase during execution

## Must NOT Be Used For
- hidden gameplay/render logic dependencies
- phase control that only works when debug is enabled

---

# Ownership Rules

## Rule 1 — Structure Comes First

Only the Structural Growth phase may create or commit vine structure.

## Rule 2 — Hosts Are Intermediate

Foliage hosts are derived from structure and may be shaped spatially, but they are not render output yet.

## Rule 3 — Ornament Is Downstream

Ornament state must always be downstream of structure and hosts.
It must never become an alternate source of truth.

## Rule 4 — Debug Must Reflect Reality

Debug state must describe what the engine is actually doing.
It must not become a parallel logic path.

---

# Data Flow Summary

Per tick, the intended state flow is:

1. Active Tip State advances
2. Stem / Occupancy State is updated
3. Route State is updated as needed
4. Foliage Host State is derived from structure
5. Foliage Host State is spatially shaped
6. Ornament State is rebuilt
7. Debug State records diagnostics across relevant phases

This flow should remain readable even if helper extraction changes file layout.

---

# Known State Risks

## 1. State Blurring

Risk:
- using ornament state to solve structure problems
- using debug counters as hidden behavior inputs

## 2. Ownership Drift

Risk:
- multiple phases mutating the same state without explicit contract

## 3. Lifetime Confusion

Risk:
- treating per-tick derived host data as if it were long-lived structural truth

## 4. Scene-Specific Leakage

Risk:
- art-direction flags becoming embedded in generic structural state

---

# Stage 1 Refactor Guidance

During Stage 1 refactor:

- preserve current state behavior exactly
- clarify ownership in helper boundaries
- add detailed `#` comments in `.py` code explaining what state each helper reads/writes
- do not merge conceptually different state categories just because they are nearby in code

---

# One-Sentence Definition

The engine state model separates live growth fronts, committed structure, guided trunk intent, foliage-host intermediates, renderable ornament output, and debug diagnostics into distinct conceptual layers.
