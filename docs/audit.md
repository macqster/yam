# Repo Audit

Date: 2026-04-27

## Unresolved Risks

- [high] `scene_config.json` is active but still carries legacy-looking hero path and dimension values that should be reconciled or intentionally documented.
  - evidence: `scene_config.json`, `docs/config.md`
- [high] The hero path still needs a chafa-vs-cached-frame decision before vines work advances.
  - evidence: `TODO.md`, `docs/rendering.md`
- [medium] Projection and camera semantics still need stronger frame-level invariance coverage in code.
  - evidence: `docs/architecture.md`, `docs/rendering.md`
- [medium] `RenderState` still needs more explicit test coverage around resize and round-trip behavior.
  - evidence: `TODO.md`, `docs/architecture.md`
- [medium] The debug border and footer contracts still need to stay synchronized with the active render path.
  - evidence: `docs/rendering.md`, `docs/architecture.md`

## Recently Resolved

- [resolved] `scene-model.md` is the canonical conceptual scene contract filename.
- [resolved] `docs/architecture.md` now points at `scene-model.md`.
- [resolved] `TODO.md` is restored as the active backlog at the root.
- [resolved] The root docs have been separated into a front door, a real backlog, and a docs map.

## Deferred

- [deferred] Legacy `Layer::render(...)` history remains in archived notes.
- [deferred] Historical migration and version-map notes are archived instead of active.
- [deferred] Older audit and issue reports remain in `docs/archive/` for context only.

## Rule

- Keep this file focused on current risk status, not history or backlog text.
- Move resolved items out when they are closed, and record the closure in `docs/LOG.md`.
