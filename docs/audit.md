# Repo Audit

Date: 2026-04-27

## Current Unresolved Risks

- `scene_config.json` still looks like a small runtime config surface that needs an explicit ownership decision.
- The hero path still has a chafa-vs-cached-frame decision gate before vines work should advance.
- The projection and camera contract is conceptually aligned, but the implementation still needs stronger frame-level invariance coverage.
- `RenderState` is now the shared frame contract, but it still needs more explicit test coverage around resize and round-trip behavior.
- The debug border and footer contracts still need to stay in sync with the active render path.

## Recently Resolved Risks

- `scene-model.md` is now the canonical scene contract filename.
- `docs/architecture.md` now points at `scene-model.md`.
- `TODO.md` is restored as the active backlog at the root.
- `docs/ARCHITECTURE.md`, `docs/AUDIT.md`, and `docs/HYGIENE.md` have been normalized to lowercase filenames.
- The root docs have been separated into a front door, a real backlog, and a docs map.

## Deferred Risks

- Legacy `Layer::render(...)` history remains in archived notes.
- Historical migration and version-map notes are now archived instead of active.
- Older audit and issue reports remain in `docs/archive/` for context only.

## Notes

- This file is a current risk snapshot, not a backlog and not a change log.
- Resolve items here by moving them into the backlog or by closing them in code and documenting the resolution in `docs/LOG.md`.
