# Repo Audit

Date: 2026-04-27
Last reviewed: 2026-05-28

## Unresolved Risks

- Highest-priority weak seam: the spatial relation layer is still the most structurally fragile area because anchor identity, projection typing, and guide relation ownership are still only partly unified.
- The reserved `calendar` companion seam still crosses offsets, render-state, and dev UI surfaces; keep it clearly labeled as reserved until a future widget rework gives it a live rendered surface.
- `UiState` remains the operational hub for runtime UI, modal state, weather refresh, camera inputs, settings editing, and persistence; future cleanup should prefer small vocabulary/helper extractions rather than a broad ownership rewrite.
- The dev-mode surface family is structurally coherent, but the current debug panel still carries too many mixed-purpose facts by default and should be tightened before any broader UI work resumes.
- The pre-expansion architecture batch is active: main-scene enrichment and greenhouse ecosystem work should remain conceptual or infrastructural until spatial, flora identity, world profile/population/storage, and docs/tooling readiness are prepared deliberately.

## Weakest Areas

1. Spatial relation layer: still the most fragile seam because the canonical resolver exists, but compatibility bridging and type boundaries remain only partly consolidated.
2. Hero-rendering pipeline: Chafa is stable, but the offline compiler / `CellGrid` path remains experimental and the hero pipeline still has more than one proving ground.
3. Flora runtime: the first vine prototype is live through deterministic growth and leaf hosting, and `core::organism` now provides the first shared identity/species-profile vocabulary, but the broader registry/journal machinery is still mostly ahead of implementation.
4. Theme/surface consistency: the BTAS contract is now reusable, but a few surfaces still rely on legacy semantic aliases and need gradual convergence rather than sudden rewrites.
5. Docs/runtime synchronization: most current contracts are aligned, but visual changes still need runtime identity checks and source verification to avoid stale-binary confusion.

## Current Work Priority

1. Prioritize overall stability and efficiency before adding new features.
2. Keep hero GIF aesthetics held steady and defer large flora/world expansions until the system is prepared for them; conceptual prep is fine when it tightens the contracts.
3. Prepare flora runtime development systematically around organism identity, species registry payloads, per-instance journals, and shared spatial guidance instead of adding another ad hoc plant family.
4. Improve coherence and consistency across UI, theming, and docs.
5. Keep `cargo fmt && bash scripts/check.sh` and the full `cargo test` suite green together now that the broader stabilization pass is restored.

## Active Readiness Gates

- The ownership contract lives in [`docs/vines.md`](vines.md) and should remain current before additional vine phases or new plant families begin.
- Do not start broader flora feature work until the signed projection, anchor identity, and screen-attached invariance tests stay green together.
- Keep vines as world-attached organisms that query guide/spatial state; render layers should visualize resolved vine geometry rather than own vine state.
- Keep vines independent of raster masks, filled sprites, or empty-cell masking until the mask contract is explicitly promoted.
- Keep the current hero GIF aesthetics and footer contract stable while testing vine placement around them.
- Clean terminology drift before implementation: spatial capture uses points, anchors, guides, lines, and polylines; `node` remains reserved for plant morphology/anatomy.
- Readiness validation on 2026-05-05: targeted Phase 0 checks are green for spatial projection, guide-set lookup, anchor identity, footer/HUD invariance, and resize round-trip behavior; the remaining risk is architectural consolidation, not an active regression.
- The active backlog now treats vine phases 1 through 7 as landed and keeps only branching/organs, border awareness, and broader flora/greenhouse preparation as future execution work.

## Active Risk Notes

- `low` The runtime loop is now less likely to strand the user's terminal in raw-mode / alternate-screen state if a later size, input, or draw step returns early: `runtime.rs` now keeps terminal teardown in a small drop guard instead of relying only on the happy-path exit tail.
  - evidence: `src/runtime.rs`
- `low` The live hero compiler path is less likely to turn asset/cache failure into a hard runtime crash or poisoned cache: missing or corrupt GIF decode, temp-directory creation, temp image writes, non-UTF-8 temp paths, and missing `chafa` now return placeholder hero frames instead of panicking, and placeholder frames are not saved as trusted hero caches.
  - evidence: `src/render/chafa.rs`
- `low` Docs hygiene now covers the first-level active docs surface instead of only the oldest core subset, so current contracts such as vines, hero cache, weather widget, theme, resource map, and soft-line atlas participate in the same markdown/spell gate as the front-door docs.
  - evidence: `scripts/check-docs.sh`, `docs/hygiene.md`
- `low` The direct terminal dependency is now aligned with the version already pulled through Ratatui, removing the previous duplicate `crossterm`/`mio` stack from this crate's dependency graph.
  - evidence: `Cargo.toml`, `Cargo.lock`
- `low` Flora prep now has a small shared identity vocabulary before a second plant family exists: organism id, species id, journal id, lifecycle state, generic stats, organism identity, organism family, and the first border-vine species profile live in `core::organism` / `core::flora` without changing visible runtime behavior.
  - evidence: `src/core/organism.rs`, `src/core/flora.rs`
- `low` World switching is no longer only a binary UI-local toggle: selectable worlds, transition labels, titles, and coarse composition profiles now live on `WorldKind`, `Boot` is explicitly non-selectable, and UI persisted snapshots convert through the core world-selection/profile contract.
  - evidence: `src/core/world.rs`, `src/ui/state.rs`
- `low` The repo front door is less likely to drift into broken preview/media references again: the missing `docs/assets/...` README placeholders were removed, and `scripts/check-docs.sh` now fails if `README.md` references a local asset path that does not exist.
  - evidence: `README.md`, `scripts/check-docs.sh`, `docs/hygiene.md`
- `low` The common direct-binary startup path is no longer an obvious performance problem after the hero-cache work: a small local audit on 2026-05-14 showed `./target/debug/yam-rust --version` effectively instant for a single run and about `1.21s` total over 200 repeated launches (roughly `6ms` per launch), while the much slower `cargo run -- --version` path was dominated by Cargo wrapper overhead rather than YAM runtime initialization.
  - evidence: local timing audit on 2026-05-14; `src/render/chafa.rs`, `src/render/hero.rs`
- `low` Recent `yam-install && yam` wall-clock variance is currently better explained by Cargo/install-path work than by YAM runtime startup: the pasted terminal history ranged from about `1.64s` to `21.78s` for the install step, including one near-no-op reinstall with no visible compile work. That output shape looked like an older direct Cargo install path rather than the newer offline-first wrapper; the repo now ships an explicit `bin/yam-install` wrapper that routes through `scripts/update.sh` so future timing reads are easier to interpret. Treat those numbers as build/install variance unless a timed direct-binary launch sample says otherwise.
  - evidence: pasted terminal history on 2026-05-14; `bin/yam-install`, `scripts/update.sh`, `src/main.rs`
- `low` The first opt-in diagnostics smoke test on 2026-05-14 also exposed brief Cargo package-cache lock waits during both install and full-verify flows, which is a much better match for the recent install-time wobble than any runtime startup regression. The new local NDJSON diagnostics path now gives the repo one concrete way to separate Cargo lock/build variance from launcher and runtime boot timing before any deeper optimization pass is justified.
  - evidence: local `YAM_DIAGNOSTICS=1` smoke test on 2026-05-14; `scripts/update.sh`, `src/diagnostics.rs`, `src/runtime.rs`
- `low` The launch wrappers were also still biasing measurements toward Cargo work because `bin/yam` and `bin/yam-sandbox` would unconditionally use `cargo run --release` whenever the repo checkout existed. They now prefer the installed runtime and only trigger the install/update path when the installed binary is missing or older than repo runtime inputs, with an explicit `YAM_USE_REPO_RUN=1` escape hatch for deliberate development runs.
  - evidence: `bin/yam`, `bin/yam-sandbox`, `bin/yam-install`, `scripts/update.sh`
- `low` A first interactive wall-clock timing sample is not a good proxy for steady-state runtime cost yet: launching the built binary into the real TUI and quitting under automation took about `33.89s`, but that figure is dominated by the intentional boot/loading sequence and terminal-session orchestration rather than ordinary per-frame render cost. Future profiling should sample inside the live loop or through a boot-bypass harness instead of reading wall-clock boot time as a render benchmark.
  - evidence: local interactive timing audit on 2026-05-14; `src/runtime.rs`, `src/scene/mod.rs`
- `medium` The first render-loop reuse, hidden-layer skip, final-buffer reuse, and scratch-grid adoption slices are now in place: runtime keeps one long-lived `Scene`, no longer asks obviously closed modal/help/quit layers to allocate empty grids, reuses the final composed `Grid` across frames, and can now reuse scratch grids for the simple active layers, the lightweight companion projection layers, the hero layer, the debug overlay, and the vine layer. The remaining hot-path seam is no longer another obvious layer conversion, but deciding whether any of the still-general draw paths deserve cheaper specialized helpers.
  - evidence: `src/scene/mod.rs`, `src/ui/scene.rs`, `src/render/compositor.rs`
- `medium` The renderer now has a narrow fast ASCII-only text-write helper for the hottest plain-ASCII chrome, and it is adopted by the always-on footer plus the debug/world-label chrome. The remaining question is whether more UI surfaces actually need it, not whether the seam should exist.
  - evidence: `src/render/compositor.rs`, `src/scene/layers/modal.rs`, `src/scene/layers/status_layer.rs`
- `medium` The current dev-mode cleanup seam is mostly about role tightening rather than missing features: the debug panel tab split is cleaner again after separating runtime control facts from hero-specific animation/placement facts, and the main-scene footer is quieter again now that the right side is back to the version stamp alone instead of a help/version catling, but `calendar (reserved)` still remains too visible across some move/settings/help-adjacent surfaces.
  - evidence: `src/scene/layers/debug_layer.rs`, `src/scene/layers/hotkeys_layer.rs`, `src/scene/layers/move_layer.rs`, `src/scene/layers/settings_layer.rs`, `src/scene/layers/status_layer.rs`
- `low` The last meaningful dev-UI vocabulary drift was tightened on 2026-05-14: the live settings tab and docs now agree on `ui`, the clean-boot manual camera seed references now agree on `(-60, -15)`, and the active contracts now describe the current dev-gated debug posture instead of implying a broader always-visible debug surface.
  - evidence: `src/ui/state.rs`, `src/scene/layers/settings_layer.rs`, `docs/rendering.md`, `docs/architecture.md`, `docs/scene-model.md`, `TODO.md`
- `medium` `scene_config.json` is active for tooling and should stay aligned with the tooling defaults if they change.
  - evidence: `scene_config.json`, `docs/config.md`, `tools/experiments/config.py`
- `medium` The spatial model is still split across `scene/coords.rs`, `scene/entity.rs`, `core/guide.rs`, and `render/guide.rs`; we still need a single canonical relation layer for datum, anchors, guides, masks, and organism guidance.
  - evidence: `src/scene/coords.rs`, `src/scene/entity.rs`, `src/core/guide.rs`, `src/render/guide.rs`
- `medium` Flora state remains vine-shaped in storage and growth dispatch even though the shared identity vocabulary now exists; the next implementation step should decide whether multi-family storage is an organism registry, an enum-backed family store, or a different small structure before another plant family or greenhouse population lands.
  - evidence: `src/core/organism.rs`, `src/core/flora.rs`, `src/systems/growth.rs`, `docs/scene-model.md`, `docs/vines.md`
- `low` World selection now has an explicit core contract for the current selectable worlds plus their coarse composition profiles, and main-scene-only render/UI branches use that profile instead of matching every current world by hand. Greenhouse should still wait for a fuller world-population/storage contract covering initialization, overlays, camera defaults, flora population, guide sets, and debug surfaces.
  - evidence: `src/core/world.rs`, `src/ui/state.rs`, `src/scene/layers/hero_layer.rs`, `src/scene/layers/weather_layer.rs`
- `low` Active render projections now use signed `ScreenPos` values through `project_world_to_screen(...)`, including `RenderState` companion helpers, hero rendering, guide drawing, vine rendering, and the debug projection bundle. The old scene-level `world_to_screen(...) -> WorldPos` compatibility wrapper has been removed; the remaining migration debt is the legacy `resolve_position(...) -> WorldPos` bridge for older compatibility call sites.
  - evidence: `src/scene/coords.rs`, `src/render/render_state.rs`, `src/scene/layers/debug_layer.rs`, `src/scene/layers/hero_layer.rs`, `src/render/guide.rs`, `src/scene/layers/vine_layer.rs`
- `low` The hero-rendering pipeline is still experiment-heavy outside the active Chafa path: the `hero-ansipx` preview artifacts were not a replacement baseline, so the offline compiler / `CellGrid` direction remains documented but unproven.
  - evidence: `src/render/chafa.rs`, `docs/rendering.md`, `docs/architecture.md`, `docs/LOG.md`
- `low` `Space::Anchor(EntityId)` now has a world-aware resolution path in `scene/coords.rs`, but the broader spatial layer is still on compatibility shims and the rest of the callers have not been migrated to the entity-backed helper yet.
  - evidence: `src/scene/coords.rs`, `src/core/world.rs`, `src/scene/entity.rs`, `src/ui/state.rs`
- `low` A recent footer visual check exposed a stale-binary risk pattern: `yam-install` can complete while `yam-rust --version` still reports an older build stamp, so screenshot comparisons should verify the installed runtime identity before treating the output as the current source of truth.
  - evidence: `yam-rust --version`, `docs/LOG.md`, `docs/config.md`, `README.md`

## Priority Order

1. Spatial relation layer consolidation
2. Hero-rendering pipeline hardening
3. Broader flora runtime implementation

## Rule

- Keep this file focused on current risk status, not history or backlog text.
- Keep resolved detail in `docs/LOG.md` and archived reports rather than re-accumulating it here.
