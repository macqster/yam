# Repo Audit

Date: 2026-04-27

## Unresolved Risks

- Highest-priority weak seam: the spatial relation layer is still the most structurally fragile area because anchor identity, projection typing, and guide relation ownership are still only partly unified.

## Weakest Areas

1. Spatial relation layer: still the most fragile seam because the canonical resolver exists, but compatibility bridging and type boundaries remain only partly consolidated.
2. Hero-rendering pipeline: Chafa is stable, but the offline compiler / `CellGrid` path remains experimental and the hero pipeline still has more than one proving ground.
3. Flora runtime: species, journals, and morphology are coherent in contract form, but the actual multi-species simulation machinery is still mostly ahead of implementation.
4. Theme/surface consistency: the BTAS contract is now reusable, but a few surfaces still rely on legacy semantic aliases and need gradual convergence rather than sudden rewrites.
5. Docs/runtime synchronization: most current contracts are aligned, but visual changes still need runtime identity checks and source verification to avoid stale-binary confusion.

## Current Work Priority

1. Prioritize overall stability and efficiency before adding new features.
2. Keep hero GIF aesthetics held steady and flora deferred until the system is prepared for it; focus only on code-side stability and efficiency improvements.
3. Defer flora runtime development until the species/morphology stack is prepared systematically.
4. Improve coherence and consistency across UI, theming, and docs.

## Vines Readiness Gate

- The ownership contract lives in [`docs/vines.md`](vines.md) and should remain current before runtime vine work begins.
- Do not start vine feature work until the signed projection, anchor identity, and screen-attached invariance tests stay green together.
- Keep vines as world-attached organisms that query guide/spatial state; render layers should visualize resolved vine geometry rather than own vine state.
- Keep vines independent of raster masks, filled sprites, or empty-cell masking until the mask contract is explicitly promoted.
- Keep the current hero GIF aesthetics and footer contract stable while testing vine placement around them.
- Clean terminology drift before implementation: spatial capture uses points, anchors, guides, lines, and polylines; `node` remains reserved for plant morphology/anatomy.
- Readiness validation on 2026-05-05: targeted Phase 0 checks are green for spatial projection, guide-set lookup, anchor identity, footer/HUD invariance, and resize round-trip behavior; the remaining risk is architectural consolidation, not an active regression.

## Coordination Contract

- `TODO.md` should stay execution-focused.
- `docs/audit.md` should stay risk-focused.
- `docs/LOG.md` should stay append-only and historical.
- Use the audit for current risk status and the log for dated decision history, not as substitutes for the backlog.

- [medium] `scene_config.json` is active for tooling and should stay aligned with the tooling defaults if they change.
  - evidence: `scene_config.json`, `docs/config.md`, `tools/experiments/config.py`
- [medium] Projection and camera semantics now have shared projection, resize round-trip, and viewport-origin helper tests.
  - evidence: `docs/architecture.md`, `docs/rendering.md`, `src/render/render_state.rs`, `src/scene/mod.rs`
- [medium] The spatial model is still split across `scene/coords.rs`, `scene/entity.rs`, `core/guide.rs`, and `render/guide.rs`; we still need a single canonical relation layer for datum, anchors, guides, masks, and organism guidance.
  - evidence: `src/scene/coords.rs`, `src/scene/entity.rs`, `src/core/guide.rs`, `src/render/guide.rs`
- [low] `resolve_position(...)` is still the legacy world-shaped bridge even though `ScreenPos` is now active again in the compatibility helpers; that keeps the signed projection semantics correct, but the final migration away from world-shaped screen results is not finished yet.
  - evidence: `src/scene/coords.rs`, `src/core/spatial.rs`
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
- [resolved] The footer now routes through the BTAS theme helper path instead of a raw palette constant, which makes the status bar consistent with the rest of the theme contract.
  - evidence: `src/scene/layers/status_layer.rs`, `src/theme/btas.rs`, `src/theme/style.rs`, `docs/theme.md`
- [resolved] The remaining obvious debug/UI raw colors were folded into BTAS helper vocabulary as well: the small debug panel now uses BTAS debug text, and guide traces use a shared BTAS trace style instead of ad hoc green/dark-gray literals.
  - evidence: `src/ui/debug.rs`, `src/render/guide.rs`, `src/scene/layers/debug_layer.rs`, `src/theme/btas.rs`, `src/theme/style.rs`, `docs/theme.md`
- [resolved] The remaining raw color literals are now explicitly treated as intentional low-level render/test exceptions, not as theme drift: hero-art calibration and compositor-level tests may still use precise color literals when the point is to verify rendering behavior rather than define a reusable UI surface.
  - evidence: `src/render/hero.rs`, `src/render/compositor.rs`, `docs/theme.md`
- [resolved] The spatial compatibility layer now preserves signed off-screen screen coordinates correctly again: projected world and anchor paths use screen-space helpers without collapsing negative positions into unsigned values, while screen-attached values stay camera-invariant.
  - evidence: `src/scene/coords.rs`, `src/core/spatial.rs`

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
