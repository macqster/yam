# Repo Audit

<!-- cspell:ignore twimc -->

Date: 2026-04-27
Last reviewed: 2026-08-07 (full repository and branch reconciliation; hero cache/runtime identity gaps repaired and current validation green)

## Unresolved Risks

- Highest-priority weak seam: the spatial relation layer's compatibility-shim phase is now closed (`scene::coords` was retired 2026-07-21 after a repo-wide audit found zero callers outside its own tests; `core::spatial` is the sole resolver everywhere), but higher-level guide/mask/organism-guidance unification is still only partly built out, so this remains the area to watch as greenhouse/flora work approaches.
- The reserved `calendar` companion seam still crosses offsets, render-state, and dev UI surfaces; keep it clearly labeled as reserved until a future widget rework gives it a live rendered surface.
- `UiState` remains the operational hub for runtime UI, modal state, weather refresh, camera inputs, settings editing, and persistence; future cleanup should prefer small vocabulary/helper extractions rather than a broad ownership rewrite.
- The dev-mode surface family is structurally coherent, but the current debug panel still carries too many mixed-purpose facts by default and should be tightened before any broader UI work resumes.
- YAM still has some natural dashboard gravity because it already has modal/dev
  surfaces and companion chrome; future UI and greenhouse work should keep
  studying dashboard TUIs like Glint as infrastructure references only, not as
  product-shape precedents.
- The pre-expansion architecture batch is active: main-scene enrichment and greenhouse ecosystem work should remain conceptual or infrastructural until spatial, flora storage/growth dispatch, world rooms/environments, inspection modes, and docs/tooling readiness are prepared deliberately.
- Greenhouse planning now has a single dedicated roadmap; `WorldState` attachment, multi-family flora storage, species-profile format, and the world/room model itself are all now closed (2026-07-21: `WorldKind::Greenhouse` is selectable with a minimal read-only render, `docs/greenhouse-roadmap.md` has the full account). Growth dispatch and inspection surfaces (popups, per-fixture detail, an actual organism in a planting site) are the remaining open gaps before greenhouse content is more than an empty visitable room.
- The greenhouse brainstorming sources have been distilled into the roadmap as candidate material only; the useful current bias is functional-space-first nursery/propagation-room work, symbolic per-room environment, read-only inspection, tiny planting-site capacity, and curation-style progression rather than gameplay.
- The front-door README has now been hand-reworked into a broadly good-enough creative state for YAM, so future README intervention should stay minimal and factual. The remaining seams are low-stakes polish or hygiene details, chiefly the exact canonical release-line convention expected by `scripts/check-docs.sh` and a few presentational quirks such as the `twimc` heading / centering wrappers.

## Weakest Areas

1. Spatial relation layer: still the most fragile seam because the canonical resolver and anchor lookup trait exist, but compatibility bridging and higher-level relation ownership remain only partly consolidated.
2. Hero-rendering pipeline: Chafa is stable and the long-standing dark-region/dark-red coverage loss is now fixed (2026-07-22, see Active Risk Notes), but the offline compiler / `CellGrid` path remains experimental and the hero pipeline still has more than one proving ground.
3. Flora runtime: the first vine prototype is live through deterministic growth and leaf hosting, `core::organism` provides the first shared identity/species-registry/journal vocabulary, multi-family storage is now locked as an enum-backed `FloraInstance` family store (2026-07-21), and growth/aging dispatch now iterates every vine instance rather than one hard-coded id (2026-07-21) — but the growth *rule* itself is still vine-specific code, since no second family exists yet to generalize it against, and that remains ahead of implementation.
4. Theme/surface consistency: the BTAS contract is now reusable, but a few surfaces still rely on legacy semantic aliases and need gradual convergence rather than sudden rewrites.
5. Greenhouse world modeling: the roadmap has a functional-space contract, an inert room/environment state, a real `WorldState` attachment, and a selectable `WorldKind::Greenhouse` with a minimal read-only render (2026-07-21) — but no growth dispatch, no inspection UI, and no organism actually occupying a planting site yet.
6. Docs/runtime synchronization: most current contracts are aligned, but visual changes still need runtime identity checks and source verification to avoid stale-binary confusion.

## Current Work Priority

1. Prioritize overall stability and efficiency before adding new features.
2. Keep hero GIF aesthetics held steady and defer large flora/world expansions until the system is prepared for them; conceptual prep is fine when it tightens the contracts.
3. Prepare flora runtime development systematically around organism identity, species registry payloads, per-instance journals, family-aware storage, and shared spatial guidance instead of adding another ad hoc plant family.
4. Improve coherence and consistency across UI, theming, and docs.
5. Keep `cargo fmt && bash scripts/check.sh` and the full `cargo test` suite green together now that the broader stabilization pass is restored.

## Active Readiness Gates

- 0.4 readiness check on 2026-05-31: full verification passed, `known_issues.md`
  stayed empty, `cargo tree -d` reported no duplicated dependency graph, and
  targeted world-profile, flora-adapter, and Chafa fallback tests passed. No
  new guard/test was added because the inspected seams already have executable
  coverage or an explicit open decision.
- The ownership contract lives in [`docs/vines.md`](vines.md) and should remain current before additional vine phases or new plant families begin.
- Do not start broader flora feature work until the signed projection, core-backed anchor identity, and screen-attached invariance tests stay green together.
- Keep vines as world-attached organisms that query guide/spatial state; render layers should visualize resolved vine geometry rather than own vine state.
- Keep vines independent of raster masks, filled sprites, or empty-cell masking until the mask contract is explicitly promoted.
- Keep the current hero GIF aesthetics and footer contract stable while testing vine placement around them.
- Clean terminology drift before implementation: spatial capture uses points, anchors, guides, lines, and polylines; `node` remains reserved for plant morphology/anatomy.
- Readiness validation on 2026-05-05: targeted Phase 0 checks are green for spatial projection, guide-set lookup, anchor identity, footer/HUD invariance, and resize round-trip behavior; the remaining risk is architectural consolidation, not an active regression.
- The active backlog now treats vine phases 1 through 7 as landed and keeps only branching/organs, border awareness, and broader flora/greenhouse preparation as future execution work.
- Greenhouse implementation should start with the functional room contract:
  room, access paths, zones, fixtures, planting sites, symbolic environment, and
  read-only inspection before new plant families, lifecycle systems, or
  persistence.
- The roadmap now locks the first-pass greenhouse defaults: keep the contract in
  the roadmap for now, start future implementation as pure `core::greenhouse`
  data plus tests, use `greenhouse_nursery` as the first room identity, keep
  the first artifact as a docs/plain-text sketch, and delay visible
  `WorldKind::Greenhouse` until data ownership is proven. **Fulfilled
  2026-07-21**: data ownership was proven (tested `WorldState` attachment),
  then `WorldKind::Greenhouse` landed with a minimal read-only render — see
  `docs/greenhouse-roadmap.md`'s Gate Checklist for current status.

## Active Risk Notes

- `medium` The current dependency graph contains two known duplicate families: `hashbrown` `0.16.1`/`0.17.1` inside Ratatui's tree and `syn` 2/3 across transitive procedural-macro dependencies. `Cargo.lock` resolves Serde `1.0.229`. A fresh local `cargo audit` on 2026-08-07 loaded 1,190 RustSec advisories and reported no findings; CI enforces the same audit. The older dependency paragraph below is retained only as the 2026-07-22 remediation record and is not current lockfile guidance.
  - evidence: `Cargo.lock`, `Cargo.toml`, `.github/workflows/verify.yml`

- `low` Terminal teardown runs through a drop guard in `runtime.rs`, so an early return from a size/input/draw step is less likely to strand raw-mode / alternate-screen state.
  - evidence: `src/runtime.rs`
- `low` The live hero compiler path degrades to placeholder frames (not a panic) on GIF decode failure, temp-dir/write failure, non-UTF-8 temp paths, or missing `chafa`; placeholder frames are never saved as trusted hero caches.
  - evidence: `src/render/chafa.rs`
- `low` `scripts/check-docs.sh` covers the first-level active docs surface (vines, hero cache, weather widget, theme, resource map, soft-line atlas), not only the front-door docs.
  - evidence: `scripts/check-docs.sh`, `docs/hygiene.md`
- `low` The direct `crossterm`/`mio` dependency stays aligned with the version Ratatui already pulls in, avoiding a duplicate stack.
  - evidence: `Cargo.toml`, `Cargo.lock`
- `low` Flora prep has a shared identity/registry/journal vocabulary (organism id, species id, journal id, lifecycle state, stats, family, `SpeciesRegistry`, `OrganismJournal`) ahead of a second plant family, without changing visible runtime behavior.
  - evidence: `src/core/organism.rs`, `src/core/flora.rs`
- `low` The repo front door no longer references missing `docs/assets/...` paths; `scripts/check-docs.sh` fails the build if `README.md` points at a local asset that doesn't exist.
  - evidence: `README.md`, `scripts/check-docs.sh`, `docs/hygiene.md`
- `low` Direct-binary startup (`yam-rust --version`) is not a performance concern (~6ms/launch measured 2026-05-14); `cargo run` overhead is Cargo wrapper cost, not YAM init cost. Install-time wall-clock variance (2026-05-14) is better explained by Cargo package-cache lock waits than by runtime startup; `bin/yam`/`bin/yam-sandbox` now prefer the installed binary over `cargo run --release` unless `YAM_USE_REPO_RUN=1`. Treat future timing reports as build/install variance unless a direct-binary sample says otherwise.
  - evidence: `src/render/chafa.rs`, `src/render/hero.rs`, `bin/yam-install`, `scripts/update.sh`, `src/diagnostics.rs`, `src/runtime.rs`, `bin/yam`, `bin/yam-sandbox`
- `medium` Render-loop reuse (long-lived `Scene`, no empty-grid allocation for closed modal/help/quit layers, reused final `Grid`, scratch-grid reuse for simple/companion/hero/debug/vine layers) is in place. Open question is whether remaining general draw paths need cheaper specialized helpers, not another obvious layer conversion.
  - evidence: `src/scene/mod.rs`, `src/ui/scene.rs`, `src/render/compositor.rs`
- `medium` A narrow fast ASCII-only text-write helper exists and is adopted by the footer and debug/world-label chrome; open question is whether more surfaces need it.
  - evidence: `src/render/compositor.rs`, `src/scene/layers/modal.rs`, `src/scene/layers/status_layer.rs`
- `medium` Dev-mode cleanup is mostly role-tightening at this point (debug panel tab split, quieter footer), but `calendar (reserved)` still shows up too visibly across some move/settings/help-adjacent surfaces.
  - evidence: `src/scene/layers/debug_layer.rs`, `src/scene/layers/hotkeys_layer.rs`, `src/scene/layers/move_layer.rs`, `src/scene/layers/settings_layer.rs`, `src/scene/layers/status_layer.rs`
- `medium` `scene_config.json` is active for tooling and should stay aligned with the tooling defaults if they change.
  - evidence: `scene_config.json`, `docs/config.md`, `tools/experiments/config.py`
- `low` `core::spatial` is the sole spatial resolver; `scene::coords` (its `ScreenPos` alias, `project_world_to_screen`, `resolve_element_screen_position`, and the `Space`/`EntityId`/`Element` compatibility dispatch) was retired 2026-07-21 after confirming zero call sites outside its own tests, and the matching `scripts/check.sh` isolation guard was removed with it. `scene/entity.rs` still names its attachment structs domain-specifically (`EntityPose`/`AttachedEntityPose`) rather than constructing `SpatialAnchor`/`SpatialAttachment` directly, but its math already delegates to `SpatialResolver` — a naming duplication, not a routing split. Entity-backed anchor lookup is exclusively `core::spatial::SpatialAnchorLookup`, implemented on `WorldState`. See `docs/LOG.md` for the retirement's full account.
  - evidence: `src/core/spatial.rs`, `src/scene/entity.rs`, `src/core/guide.rs`, `src/render/guide.rs`, `src/render/render_state.rs`, `scripts/check.sh`
- `low` Flora storage is closed: `FloraState` stores an enum-backed `FloraInstance` family store (`organisms: Vec<FloraInstance>`, `Vine(VineInstance)` the sole variant so far) instead of a bespoke `vines` field, with every call site migrated. Growth dispatch (`systems::growth::run_growth`) now iterates every vine instance rather than one hard-coded id, matching `run_aging`; the growth *rule* itself remains vine-specific until a second family exists. See `docs/LOG.md` for the fix history.
  - evidence: `src/core/flora.rs`, `src/systems/growth.rs`, `src/systems/aging.rs`, `src/scene/layers/vine_layer.rs`, `src/scene/layers/debug_layer.rs`, `src/core/world.rs`
- `low` `WorldKind::Greenhouse` is a real selectable world (2026-07-21): `WorldState.greenhouse` is `Some(GreenhouseState::nursery())` for that world only, rendered by a minimal read-only `GreenhouseLayer` (bounds outline + fixture markers), verified end-to-end in the running app. No growth dispatch, mutation, or inspection UI yet — see "Weakest Areas" #5.
  - evidence: `src/core/world.rs`, `src/core/greenhouse.rs`, `src/scene/layers/greenhouse_layer.rs`
- `low` `docs/greenhouse-roadmap.md` is the single owning surface for greenhouse strategy, ingested brainstorming/reference material, phase tasks, gates, and stop conditions; `TODO.md` and `docs/audit.md` carry only pointers, not a second copy of the contract.
  - evidence: `docs/greenhouse-roadmap.md`, `TODO.md`, `docs/README.md`
- `low` The Glint study is a useful external contrast case: a strong Rust/Ratatui reference for widget registries and setup flows, but also a reminder of what YAM should not become — a pane-grid, widget-first dashboard shell. Future borrowing should stay infrastructure-only.
  - evidence: `docs/resource-map.md`
- `low` The hero-rendering pipeline is still experiment-heavy outside the active Chafa path: the offline compiler / `CellGrid` direction remains documented but unproven.
  - evidence: `src/render/chafa.rs`, `docs/rendering.md`, `docs/architecture.md`
- `low` Fixed the dark-region/dark-red coverage loss that has affected hero rendering since the Rust baseline (2026-07-22): a direct sanity-check investigation found the prior `--color-extractor=average` Chafa flag, combined with flattening every frame onto an opaque matte canvas, was dropping roughly 80% of the frame grid as "no coverage" — not desaturating it, omitting it — because low-contrast dark regions (any dark content, not only reds) read as close enough to the assumed background to skip. Two changes together fix it: the source asset (`assets/hero_gif_1.gif`) now carries real per-pixel alpha that the pipeline preserves end to end instead of discarding at flatten time, and the extractor switched to `--color-extractor=median`. The asset's provenance is worth recording precisely, because it explains the "since day 0" part: the alpha-carrying original has been tracked in this repo the whole time at `tools/legacy-python/hero/assets/hero_go.gif` (imported in `768b193`, used by the legacy Python hero pipeline), and the very first Rust hero commit (`0606be7`, 2026-04-23) added `assets/hero_gif_1.gif` as a *separate, flattened, alpha-stripped copy* of that same art rather than pointing at the original. The Rust renderer has therefore been reading alpha-free frames since its first day, which is exactly the window over which dark reds were reported missing. The new `assets/hero_gif_1.gif` is byte-identical (md5 `8afff117…`) to that in-repo original. Neither alone is correct: `median` against the old flattened canvas recovered coverage only by also painting the true background; real alpha alone (`average` extractor) barely moved the numbers. Verified live in the running app (`scripts/tmux-smoke.sh`), not just synthetic frame tests: color codes in the rendered output now include genuine dark-red values that were previously absent. The renamed `HERO_DISPLAY_BG` constant (formerly `HERO_FRAME_BG`) now reflects its actual role, "chafa `--bg` display hint only," not "opaque flatten fill," and the corner-transparency assumption in `decoded_hero_frames_keep_full_canvas_geometry` flipped from asserting opacity to asserting transparency.
  - evidence: `src/render/chafa.rs`, `assets/hero_gif_1.gif`, `docs/rendering.md`
- `low` Hero source identity is consolidated behind `render::hero_source::HeroSource`: asset path, logical canvas size, expected frame count, requested cell footprint, and cache revision live in one descriptor. Per-source revision-keyed cache names prevent source collisions and invalidate output after compiler/serialized-contract changes. Existing revision-matched caches remain usable when a relocated build makes the compile-time source path unreachable. `hero_source::ALL` remains a one-entry registry until an actual selection surface exists.
  - evidence: `src/render/hero_source.rs`, `src/render/chafa.rs`, `src/render/hero.rs`, `docs/rendering.md`, `docs/hero-cache.md`
- `low` The docs gate no longer reports success while silently skipping half of itself (2026-08-01, `experimental` branch). Two separate defects were found: `scripts/check-docs.sh` printed an unqualified "All docs checks passed" even when both markdown/spelling linters were absent, and its `markdownlint` invocation had gone dead, because that binary ships in `markdownlint-cli` while CI only ever installed `markdownlint-cli2` and the current workstation has neither. (Correction to this note's first version, which claimed the line could never have run anywhere: `docs/LOG.md` 2026-05-19 records `markdownlint-cli` being installed on the previous workstation, so it did run there before the CI and machine changes stranded it.) The dead invocation is removed, skips now print an explicit `SKIPPED` summary naming the missing linters, and `YAM_DOCS_STRICT=1` (set in CI) turns a missing linter into a failure. Running the linters for the first time on this machine immediately found two real spelling misses in same-week edits (`mtimes`, `sixel`), both added to `.cspell.json`; markdown was clean across all 38 active docs. Both CLIs are now actually installed here (`npm install -g --prefix ~/.local`, which avoids the sandbox-denied `~/.npmrc` and `~/.npm` paths), so the gate runs in full rather than skipping, and both were exercised against deliberately injected faults to confirm they fail it — a checker that has only ever been seen to pass is not yet known to work. Remaining gap: linter versions are unpinned in both CI and local installs, so local and CI can drift apart.
  - evidence: `scripts/check-docs.sh`, `.github/workflows/verify.yml`, `.cspell.json`, `docs/hygiene.md`
- `low` The content-level hero gap is closed locally and in CI: `rendered_hero_frames_contain_real_content_not_placeholders` asserts that every live-rendered frame is real output rather than a placeholder and that frame-0 coverage has not collapsed below 20% of the grid (measured 41.5%). It skips honestly when `chafa` is absent locally; CI installs `chafa` before the full gate, so the assertion executes there. `scripts/tmux-smoke.sh` remains the required live-scene check after renderer changes.
  - evidence: `src/render/chafa.rs`, `scripts/tmux-smoke.sh`, `.github/workflows/verify.yml`
- `low` The hero decode path no longer applies per-pixel color correction (2026-07-29, `experimental` branch): `tone_lift_dark_reds` and its HSV helpers were removed along with their five tests. The step was undocumented in every contract doc while running on every pixel of every frame, and it was tuned to one asset's palette. Measured before removal on frame 0: it modified 3.27% of pixels (21990 of 672400) and changed the rendered result by +1 covered cell and +14 unique foreground colors against ~740 — inside the noise of the alpha-plus-`median` fix that actually recovered dark regions in 2026-07-22. Treat any future per-asset color correction as descriptor-owned, not a shared decode-loop mutation. Not yet re-verified live via `scripts/tmux-smoke.sh` at the time of writing.
  - evidence: `src/render/chafa.rs`, `docs/rendering.md`, `docs/architecture.md`
- `low` `assets/hero_gif_1.gif` and `tools/legacy-python/hero/assets/hero_go.gif` are now byte-identical (~4.1MB each, ~8.2MB total) after the 2026-07-22 asset swap. This duplication is known and currently deliberate rather than accidental: the Rust runtime reads the former by a compile-time path, while `tools/legacy-python/runtime/system.py` and `tools/experiments/` still resolve the latter, and the legacy Python tree is frozen reference material that is not worth repointing for ~4MB. Revisit only if the legacy tree is retired or the art changes again — if it changes, both copies must move together or the two pipelines will silently diverge on source art.
  - evidence: `assets/hero_gif_1.gif`, `tools/legacy-python/hero/assets/hero_go.gif`, `tools/legacy-python/runtime/system.py`
- `low` The hero frame cache is written with compact JSON (`serde_json::to_string`, not `to_string_pretty`), which cut the generated file from 81MB to 27MB with no behavior change; the file is machine-only, so indentation was pure overhead. Warm-load cost measured at ~150ms against ~4300ms for the live `chafa` compile path (2026-07-25), so the cache-first startup path in `docs/hero-cache.md` is comfortably meeting its acceptance bar.
  - evidence: `src/render/hero_cache.rs`, `docs/hero-cache.md`
- `low` The main-scene scaffold has a real world-owned runtime slice (`core::scaffold`) with read-only render layers (rear support cradle, foreground nesting edge); open question is visual sufficiency, not missing ownership. Sandbox hosts the same surfaces behind UI-owned visibility toggles for prototyping without changing world ownership.
  - evidence: `src/core/scaffold.rs`, `src/core/world.rs`, `src/scene/layers/scaffold_layer.rs`, `docs/main-scene-scaffold.md`, `src/ui/state.rs`
- `low` Runtime identity now watches Git HEAD, its symbolic branch ref, and packed refs from `build.rs`; Cargo therefore rebuilds the embedded commit identity after a commit instead of retaining the hash from the last source-code rebuild. Installed-runtime identity should still be checked before trusting screenshot comparisons because installation itself can remain stale.
  - evidence: `build.rs`, `yam-rust --identity`, `docs/config.md`
- `low` Dependency remediation history (2026-07-22): narrowing `image` to the required GIF/PNG features removed the vulnerable AVIF encoder stack, and a scoped `anyhow` update cleared the remaining advisory. Subsequent dependency updates moved Serde to `1.0.229` and introduced the currently documented `syn` 2/3 duplicate alongside Ratatui's `hashbrown` 0.16/0.17 pair; use the current medium-risk note above and live `cargo tree -d` / `cargo audit` output for present state.
  - evidence: `Cargo.lock`, `Cargo.toml`
- `low` The 19 non-test `.expect()`/`unreachable!()` call sites (`src/runtime.rs:50`, `src/render/hero.rs:92`, `src/render/fonts.rs:80`, 16 in `src/weather/atlas.rs`) are each traced to a structural invariant enforced by an exhaustive match, a constructor guarantee, or compile-time data — not a soft assumption. Considered closed unless a future change (e.g. exposing `Hero::frames` to external mutation) reopens the question.
  - evidence: `src/runtime.rs`, `src/render/hero.rs`, `src/render/fonts.rs`, `src/weather/atlas.rs`
- `low` Weather refresh tests inject deterministic results through an injectable-fetch seam while still traversing the same spawned worker/channel path as production, so tests can't drift from the production refresh contract; see `docs/weather-widget.md`'s rule to that effect.
  - evidence: `src/ui/state.rs`, `docs/weather-widget.md`
- `low` `systems::fields::update_fields()` skips out-of-bounds entities and repairs all three field buffers to the exact grid area before indexing/clearing, closing the same invariant-drift shape as the earlier `GreenhouseState::active_room()` fix (construction-only invariant, mutable public fields). A repo-wide sweep for the same shape (`.unwrap()`/`panic!()` in production code, find-by-id resolvers, direct grid/array indexing) came back clean elsewhere.
  - evidence: `src/systems/fields.rs`
- `low` `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, and `cargo check --all-targets` are clean; the ownership boundary guard in `scripts/check.sh` (`core` must not import `scene`; `systems` must not import scene/render/UI/terminal) is unchanged.
  - evidence: `scripts/check.sh`, `src/render/chafa.rs`, `src/render/hero.rs`, `src/scene/layers/hero_layer.rs`, `src/runtime.rs`

## Priority Order

1. Spatial relation layer consolidation
2. Hero-rendering pipeline hardening
3. Broader flora runtime implementation

## Rule

- Keep this file focused on current risk status, not history or backlog text.
- Keep resolved detail in `docs/LOG.md` and archived reports rather than re-accumulating it here.
