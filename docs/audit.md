# Repo Audit

Date: 2026-04-27
Last reviewed: 2026-05-11

## Unresolved Risks

- Highest-priority weak seam: the spatial relation layer is still the most structurally fragile area because anchor identity, projection typing, and guide relation ownership are still only partly unified.
- The reserved `calendar` companion seam still crosses offsets, render-state, and dev UI surfaces; keep it clearly labeled as reserved until a future widget rework gives it a live rendered surface.
- `UiState` remains the operational hub for runtime UI, modal state, weather refresh, camera inputs, settings editing, and persistence; future cleanup should prefer small vocabulary/helper extractions rather than a broad ownership rewrite.

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
5. Keep `cargo fmt && bash scripts/check.sh` and the full `cargo test` suite green together now that the broader stabilization pass is restored.

## Active Readiness Gates

- The ownership contract lives in [`docs/vines.md`](vines.md) and should remain current before runtime vine work begins.
- Do not start vine feature work until the signed projection, anchor identity, and screen-attached invariance tests stay green together.
- Keep vines as world-attached organisms that query guide/spatial state; render layers should visualize resolved vine geometry rather than own vine state.
- Keep vines independent of raster masks, filled sprites, or empty-cell masking until the mask contract is explicitly promoted.
- Keep the current hero GIF aesthetics and footer contract stable while testing vine placement around them.
- Clean terminology drift before implementation: spatial capture uses points, anchors, guides, lines, and polylines; `node` remains reserved for plant morphology/anatomy.
- Readiness validation on 2026-05-05: targeted Phase 0 checks are green for spatial projection, guide-set lookup, anchor identity, footer/HUD invariance, and resize round-trip behavior; the remaining risk is architectural consolidation, not an active regression.

## Active Risk Notes

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

## Priority Order

1. Spatial relation layer consolidation
2. Hero-rendering pipeline hardening
3. Broader flora runtime implementation

## Rule

- Keep this file focused on current risk status, not history or backlog text.
- Keep resolved detail in `docs/LOG.md` and archived reports rather than re-accumulating it here.
