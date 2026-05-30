---
name: yam-architecture-review
description: Use when reviewing YAM architecture, especially before greenhouse ecosystem work, main-scene enrichment, flora expansion, world-mode changes, spatial relation changes, render-layer changes, or broad internal coherence passes.
---

# YAM Architecture Review

## Read First

- Read [../../AGENTS.md](../../AGENTS.md) for repo operating rules.
- Read [../../docs/architecture.md](../../docs/architecture.md) for ownership and coupling rules.
- Read [../../docs/scene-model.md](../../docs/scene-model.md) for scene/world behavior.
- Read [../../docs/rendering.md](../../docs/rendering.md) for render and layer contracts.
- Read [../../docs/audit.md](../../docs/audit.md) for current risks.

## Review Shape

- Findings first, ordered by severity.
- Ground findings in concrete file and line references.
- Separate active bugs from architecture debt and future-readiness gaps.
- Treat greenhouse work as future simulation infrastructure, not as dashboard chrome.
- Prefer recommendations that preserve current runtime behavior until contracts are ready.

## What To Check

- World profile/spec ownership for selection, loading labels, grid, camera defaults, guide plans, population plans, capabilities, rooms, and inspection surfaces.
- Flora storage and growth dispatch for multi-family readiness.
- `core::organism` registry and per-organism journal readiness.
- `core::spatial` ownership of projection, anchors, guides, and future organism guidance.
- Render layers as read-only visualization of world/flora/spatial state.
- UI state as presentation/metamechanics, not simulation ownership.

## Handoff

- If the user asked for a review, do not implement unless asked.
- If the user asked to proceed, implement only the smallest infrastructure slice that resolves the reviewed gap.
- Update [../../TODO.md](../../TODO.md), [../../docs/audit.md](../../docs/audit.md), and [../../docs/LOG.md](../../docs/LOG.md) when the review changes the active work order or risk snapshot.
