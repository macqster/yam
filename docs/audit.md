# Repo Audit

Date: 2026-04-27

## Unresolved Risks

- Highest-priority weak seam: the spatial relation layer is still the most structurally fragile area because anchor identity, projection typing, and guide relation ownership are still only partly unified.

- [medium] `scene_config.json` is active for tooling and should stay aligned with the tooling defaults if they change.
  - evidence: `scene_config.json`, `docs/config.md`, `tools/experiments/config.py`
- [medium] Projection and camera semantics now have shared projection, resize round-trip, and viewport-origin helper tests.
  - evidence: `docs/architecture.md`, `docs/rendering.md`, `src/render/render_state.rs`, `src/scene/mod.rs`
- [medium] The spatial model is still split across `scene/coords.rs`, `scene/entity.rs`, `core/guide.rs`, and `render/guide.rs`; we still need a single canonical relation layer for datum, anchors, guides, masks, and organism guidance.
  - evidence: `src/scene/coords.rs`, `src/scene/entity.rs`, `src/core/guide.rs`, `src/render/guide.rs`
- [low] `WorldPos` is currently used as the result type for both world and screen projection paths, while `ScreenPos` exists but is not used in the active resolution API; that weakens type-level separation between simulation space and screen space.
  - evidence: `src/scene/coords.rs`, `src/render/render_state.rs`, `src/scene/camera.rs`
- [low] The hero-rendering pipeline is still experiment-heavy outside the active Chafa path: the `hero-ansipx` preview artifacts were not a replacement baseline, so the offline compiler / `CellGrid` direction remains documented but unproven.
  - evidence: `src/render/chafa.rs`, `docs/rendering.md`, `docs/architecture.md`, `docs/LOG.md`
- [low] `Space::Anchor(EntityId)` now has a world-aware resolution path in `scene/coords.rs`, but the broader spatial layer is still on compatibility shims and the rest of the callers have not been migrated to the entity-backed helper yet.
  - evidence: `src/scene/coords.rs`, `src/core/world.rs`, `src/scene/entity.rs`, `src/ui/state.rs`
- [low] A recent footer visual check exposed a stale-binary risk pattern: `yam-install` can complete while `yam-rust --version` still reports an older build stamp, so screenshot comparisons should verify the installed runtime identity before treating the output as the current source of truth.
  - evidence: `yam-rust --version`, `docs/LOG.md`, `docs/config.md`, `README.md`
- [resolved] The footer spacing source bug was real as well: `status_layer.rs` still contained the old triple-spacing form, so the screenshot mismatch was partly a code issue and not only a stale-binary issue.
  - evidence: `src/scene/layers/status_layer.rs`
- [resolved] The footer color diagnostic proved the runtime/terminal styling pipeline is healthy; the final working solution is compact spacing, no background tint, no dim modifier, and a subdued grey foreground that reads cleanly on the current Ghostty theme.
  - evidence: `src/scene/layers/status_layer.rs`, `src/theme/palette.rs`, `docs/LOG.md`

## Priority Order

1. Spatial relation layer consolidation
2. Hero-rendering pipeline hardening
3. Broader flora runtime implementation

## Recently Resolved

- [resolved] The current runtime behavior matches the active scene/render contracts for the world/footer split, dev-mode gating, and guide-set handling; the remaining work is consolidation of the spatial relation layer rather than drift correction.
  - evidence: `src/scene/mod.rs`, `src/runtime.rs`, `src/core/guide.rs`, `src/render/guide.rs`, `docs/architecture.md`, `docs/scene-model.md`, `docs/rendering.md`
- [resolved] The spatial split now has a first canonical `core/spatial` cut in code, with a minimal API surface and a documented migration order; the remaining work is incremental expansion, not a fresh design.
  - evidence: `src/core/spatial.rs`, `src/scene/coords.rs`, `src/scene/entity.rs`, `docs/architecture.md`, `docs/scene-model.md`, `docs/rendering.md`
- [resolved] `scene/coords.rs` now has a world-aware anchor resolution helper that consults `WorldState` entities when an `EntityId` is present, so anchor identity is no longer purely declarative in the compatibility layer.
  - evidence: `src/scene/coords.rs`, `src/core/world.rs`
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
