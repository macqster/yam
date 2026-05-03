# Repo Audit

Date: 2026-04-27

## Unresolved Risks

- [medium] `scene_config.json` is active for tooling and should stay aligned with the tooling defaults if they change.
  - evidence: `scene_config.json`, `docs/config.md`, `tools/experiments/config.py`
- [medium] Projection and camera semantics now have shared projection, resize round-trip, and viewport-origin helper tests.
  - evidence: `docs/architecture.md`, `docs/rendering.md`, `src/render/render_state.rs`, `src/scene/mod.rs`
- [medium] The spatial model is still split across `scene/coords.rs`, `scene/entity.rs`, `core/guide.rs`, and `render/guide.rs`; we still need a single canonical relation layer for datum, anchors, guides, masks, and organism guidance.
  - evidence: `src/scene/coords.rs`, `src/scene/entity.rs`, `src/core/guide.rs`, `src/render/guide.rs`
- [medium] `Space::Anchor(EntityId)` currently resolves exactly like `Space::World`, so anchor identity is not actually part of resolution yet; the type suggests a richer relation model than the implementation provides.
  - evidence: `src/scene/coords.rs`
- [low] `WorldPos` is currently used as the result type for both world and screen projection paths, while `ScreenPos` exists but is not used in the active resolution API; that weakens type-level separation between simulation space and screen space.
  - evidence: `src/scene/coords.rs`, `src/render/render_state.rs`, `src/scene/camera.rs`

## Recently Resolved

- [resolved] The current runtime behavior matches the active scene/render contracts for the world/footer split, dev-mode gating, and guide-set handling; the remaining work is consolidation of the spatial relation layer rather than drift correction.
  - evidence: `src/scene/mod.rs`, `src/runtime.rs`, `src/core/guide.rs`, `src/render/guide.rs`, `docs/architecture.md`, `docs/scene-model.md`, `docs/rendering.md`
- [resolved] The spatial split has a documented minimal API, module mapping, and migration order, so the remaining work is now a planned refactor into `core/spatial` rather than an open architectural ambiguity.
  - evidence: `docs/architecture.md`, `docs/scene-model.md`, `docs/rendering.md`
- [resolved] `scene-model.md` is the canonical conceptual scene contract filename.
- [resolved] `docs/architecture.md` now points at `scene-model.md`.
- [resolved] `TODO.md` is restored as the active backlog at the root.
- [resolved] The root docs have been separated into a front door, a real backlog, and a docs map.
- [resolved] The debug border bounds are now directly tested and kept in sync with the footer row contract.
- [resolved] The hero path is now explicitly documented as chafa-backed, with cached-frame ownership reserved for future migration if needed.
- [resolved] `RenderState` now has resize, round-trip, and projection-helper coverage.
- [resolved] The debug border contract now has a direct bounds test, and the footer/debug split remains documented in the active contracts.

## Deferred

- [deferred] Legacy `Layer::render(...)` history remains in archived notes.
- [deferred] Historical migration and version-map notes are archived instead of active.
- [deferred] Older audit and issue reports remain in `docs/archive/` for context only.

## Rule

- Keep this file focused on current risk status, not history or backlog text.
- Move resolved items out when they are closed, and record the closure in `docs/LOG.md`.
