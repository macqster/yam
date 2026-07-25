# Repo Audit

<!-- cspell:ignore twimc -->

Date: 2026-04-27
Last reviewed: 2026-07-25 (full audit; every claim below re-derived from the tree rather than carried forward. Independently re-verified: `scripts/verify.sh` green at 277 tests (275 at audit time, plus the two hero-cache freshness tests this pass added), `cargo audit` clean at 241 crates, `cargo tree -d` showing only the documented `hashbrown` pair, the 19 non-test panic sites at exactly their recorded locations, `scene::coords` at zero references, byte-identical hero assets, and the 2026-07-22 dark-red fix still holding under a cold-cache live `chafa` compile at 604 distinct foreground colors. Corrected: the greenhouse status claims below and in `README.md`, which had gone stale against `CHANGELOG.md` and the running app after growth dispatch and inspection landed)

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
- Greenhouse planning now has a single dedicated roadmap; `WorldState` attachment, multi-family flora storage, species-profile format, and the world/room model itself are all now closed (2026-07-21: `WorldKind::Greenhouse` is selectable with a minimal read-only render, `docs/greenhouse-roadmap.md` has the full account). Growth dispatch, a real organism in a planting site, and a read-only inspection popup all landed 2026-07-25, so the room is no longer empty or inert. The remaining open gaps are curation/transfer gates and richer per-fixture inspection detail — see "Weakest Areas" #5.
- The greenhouse brainstorming sources have been distilled into the roadmap as candidate material only; the useful current bias is functional-space-first nursery/propagation-room work, symbolic per-room environment, read-only inspection, tiny planting-site capacity, and curation-style progression rather than gameplay.
- The front-door README has now been hand-reworked into a broadly good-enough creative state for YAM, so future README intervention should stay minimal and factual. The remaining seams are low-stakes polish or hygiene details, chiefly the exact canonical release-line convention expected by `scripts/check-docs.sh` and a few presentational quirks such as the `twimc` heading / centering wrappers.

## Weakest Areas

1. Spatial relation layer: still the most fragile seam because the canonical resolver and anchor lookup trait exist, but compatibility bridging and higher-level relation ownership remain only partly consolidated.
2. Hero-rendering pipeline: Chafa is stable and the long-standing dark-region/dark-red coverage loss is now fixed (2026-07-22, see Active Risk Notes), but the offline compiler / `CellGrid` path remains experimental and the hero pipeline still has more than one proving ground.
3. Flora runtime: the first vine prototype is live through deterministic growth and leaf hosting, `core::organism` provides the first shared identity/species-registry/journal vocabulary, multi-family storage is now locked as an enum-backed `FloraInstance` family store (2026-07-21), and growth/aging dispatch now iterates every vine instance rather than one hard-coded id (2026-07-21) — but the growth *rule* itself is still vine-specific code, since no second family exists yet to generalize it against, and that remains ahead of implementation.
4. Theme/surface consistency: the BTAS contract is now reusable, but a few surfaces still rely on legacy semantic aliases and need gradual convergence rather than sudden rewrites.
5. Greenhouse world modeling: the first vertical slice is complete — functional-space contract, room/environment state, `WorldState` attachment, selectable `WorldKind::Greenhouse` with a read-only render (2026-07-21), plus growth dispatch, a seedling occupying the `left_tray` planting site, and a read-only inspection popup (2026-07-25). What remains is breadth, not a missing spine: one room, one fixture family, one organism, no curation/transfer gates, and no per-fixture detail beyond the `inspection_refs` strings.
6. Docs/runtime synchronization: most current contracts are aligned, but visual changes still need runtime identity checks and source verification to avoid stale-binary confusion.

## Current Work Priority

1. **The hero GIF and its rendering infrastructure are the main goal of the 0.4 development phase** (set 2026-07-25). This covers the whole chain — source art, GIF decode, the `chafa` compile path, the frame cache, and the offline compiler / `CellGrid` direction — and both the infrastructure *and* the rendered art are in scope for deliberate change. See "Hero Track (0.4 Main Goal)" below.
2. Prioritize overall stability and efficiency before adding new features.
3. Defer large flora/world expansions until the system is prepared for them; conceptual prep is fine when it tightens the contracts.
4. Prepare flora runtime development systematically around organism identity, species registry payloads, per-instance journals, family-aware storage, and shared spatial guidance instead of adding another ad hoc plant family.
5. Improve coherence and consistency across UI, theming, and docs.
6. Keep `cargo fmt && bash scripts/check.sh` and the full `cargo test` suite green together now that the broader stabilization pass is restored.

## Hero Track (0.4 Main Goal)

Set 2026-07-25. The hero GIF and its surrounding code infrastructure are the
primary development focus for 0.4, promoted from "hardening" to the leading
track.

Scope: the source art itself, GIF decode, the live `chafa` compile path, the
frame cache, the offline compiler / `CellGrid` direction, and the render/scene
seams the hero sits in.

**Traced baseline: [`docs/hero-track.md`](hero-track.md)** (2026-07-25) — the
full stage-by-stage trace, measured numbers, per-stage evaluation, ranked
opportunities, and the open questions the revision needs to answer. Read it
before starting revision work rather than re-deriving the pipeline.

What changed about the aesthetics rule: hero art was previously frozen
repo-wide. That global freeze is lifted for this track — reworked or new source
art is a legitimate 0.4 outcome, not a regression. The *scoped* rules survive
unchanged, because they exist to stop unrelated work from dragging the hero
along as a side effect rather than to protect the art from deliberate change:
vine work must still not require hero appearance changes
([`docs/vines.md`](vines.md)), and main-scene enrichment must still preserve
hero aesthetics ([`TODO.md`](../TODO.md)). Incidental drift from work that is
not on this track remains a regression.

Standing constraints for the track:

- Changes touching `src/render/chafa.rs` still require live `scripts/tmux-smoke.sh`
  verification, since CI has no `chafa` and every live-path test tolerates
  placeholder frames by design.
- The cache must stay a runtime-owned representation, not a second independent
  rendering authority ([`docs/hero-cache.md`](hero-cache.md)).
- Aesthetic changes should be deliberate and reviewed against the BTAS/TNBA
  color discipline, not accepted as incidental pipeline output.

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
- Keep the footer contract stable while testing vine placement around it, and keep hero aesthetics stable *as seen from vine work* — vine slices must not require hero appearance changes. This is no longer a repo-wide art freeze: deliberate hero art changes are in scope for the 0.4 hero track above.
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
- `low` `WorldKind::Greenhouse` is a real selectable world (2026-07-21): `WorldState.greenhouse` is `Some(GreenhouseState::nursery())` for that world only, rendered by a minimal read-only `GreenhouseLayer` (bounds outline + fixture markers). Growth dispatch and inspection landed on top of it (2026-07-25): `systems::growth::run_greenhouse_growth` advances every seedling `Dormant -> Growing -> Mature` on its own 6-tick cadence, a first `OrganismFamily::Seedling` occupies the nursery's `left_tray` site through a soft `PlantingSite::occupant` reference rather than ownership, and a dev-mode/Greenhouse-gated `GreenhouseInspectLayer` (`i`) surfaces the active room's `inspection_refs`. Re-verified end-to-end in the running app 2026-07-25 via `scripts/tmux-smoke.sh " " d w w i`, which renders the popup including the occupied "Left Tray Site" entry. See "Weakest Areas" #5 for what is still missing (breadth, not spine).
  - evidence: `src/core/world.rs`, `src/core/greenhouse.rs`, `src/scene/layers/greenhouse_layer.rs`, `src/scene/layers/greenhouse_inspect_layer.rs`, `src/systems/growth.rs`
- `low` `docs/greenhouse-roadmap.md` is the single owning surface for greenhouse strategy, ingested brainstorming/reference material, phase tasks, gates, and stop conditions; `TODO.md` and `docs/audit.md` carry only pointers, not a second copy of the contract.
  - evidence: `docs/greenhouse-roadmap.md`, `TODO.md`, `docs/README.md`
- `low` The Glint study is a useful external contrast case: a strong Rust/Ratatui reference for widget registries and setup flows, but also a reminder of what YAM should not become — a pane-grid, widget-first dashboard shell. Future borrowing should stay infrastructure-only.
  - evidence: `docs/resource-map.md`
- `low` The hero-rendering pipeline is still experiment-heavy outside the active Chafa path: the offline compiler / `CellGrid` direction remains documented but unproven.
  - evidence: `src/render/chafa.rs`, `docs/rendering.md`, `docs/architecture.md`
- `low` Fixed the dark-region/dark-red coverage loss that has affected hero rendering since the Rust baseline (2026-07-22): a direct sanity-check investigation found the prior `--color-extractor=average` Chafa flag, combined with flattening every frame onto an opaque matte canvas, was dropping roughly 80% of the frame grid as "no coverage" — not desaturating it, omitting it — because low-contrast dark regions (any dark content, not only reds) read as close enough to the assumed background to skip. Two changes together fix it: the source asset (`assets/hero_gif_1.gif`) now carries real per-pixel alpha that the pipeline preserves end to end instead of discarding at flatten time, and the extractor switched to `--color-extractor=median`. The asset's provenance is worth recording precisely, because it explains the "since day 0" part: the alpha-carrying original has been tracked in this repo the whole time at `tools/legacy-python/hero/assets/hero_go.gif` (imported in `768b193`, used by the legacy Python hero pipeline), and the very first Rust hero commit (`0606be7`, 2026-04-23) added `assets/hero_gif_1.gif` as a *separate, flattened, alpha-stripped copy* of that same art rather than pointing at the original. The Rust renderer has therefore been reading alpha-free frames since its first day, which is exactly the window over which dark reds were reported missing. The new `assets/hero_gif_1.gif` is byte-identical (md5 `8afff117…`) to that in-repo original. Neither alone is correct: `median` against the old flattened canvas recovered coverage only by also painting the true background; real alpha alone (`average` extractor) barely moved the numbers. Verified live in the running app (`scripts/tmux-smoke.sh`), not just synthetic frame tests: color codes in the rendered output now include genuine dark-red values that were previously absent. The renamed `HERO_DISPLAY_BG` constant (formerly `HERO_FRAME_BG`) now reflects its actual role, "chafa `--bg` display hint only," not "opaque flatten fill," and the corner-transparency assumption in `decoded_hero_frames_keep_full_canvas_geometry` flipped from asserting opacity to asserting transparency.
  - evidence: `src/render/chafa.rs`, `assets/hero_gif_1.gif`, `docs/rendering.md`
- `low` `assets/hero_gif_1.gif` and `tools/legacy-python/hero/assets/hero_go.gif` are now byte-identical (~4.1MB each, ~8.2MB total) after the 2026-07-22 asset swap. This duplication is known and currently deliberate rather than accidental: the Rust runtime reads the former by a compile-time path, while `tools/legacy-python/runtime/system.py` and `tools/experiments/` still resolve the latter, and the legacy Python tree is frozen reference material that is not worth repointing for ~4MB. Revisit only if the legacy tree is retired or the art changes again — if it changes, both copies must move together or the two pipelines will silently diverge on source art.
  - evidence: `assets/hero_gif_1.gif`, `tools/legacy-python/hero/assets/hero_go.gif`, `tools/legacy-python/runtime/system.py`
- `low` The hero frame cache is written with compact JSON (`serde_json::to_string`, not `to_string_pretty`), which cut the generated file from 81MB to 27MB with no behavior change; the file is machine-only, so indentation was pure overhead. Warm-load cost measured at ~150ms against ~4300ms for the live `chafa` compile path (2026-07-25), so the cache-first startup path in `docs/hero-cache.md` is comfortably meeting its acceptance bar.
  - evidence: `src/render/hero_cache.rs`, `docs/hero-cache.md`
- `medium` `ANSI_PARSE_ERROR` escapes the placeholder-detection net. When `ansi-to-tui` fails to parse `chafa`'s stdout, `render_frame_with_command` (`src/render/chafa.rs:44`) degrades to a literal `Text::raw("ANSI_PARSE_ERROR")` single-line frame — but `is_placeholder_frame` (`chafa.rs:136-151`) only matches five other failure strings, so this one is judged cacheable and would be persisted, then loaded back as trusted hero art on every later launch. Same silent-failure shape as the 2026-07-22 PNG-feature regression, and the cache makes it sticky rather than transient. Not observed in the wild (no known parse failure against current `chafa` output), so this is a latent gap rather than an active bug; found by the 2026-07-25 hero-track trace. Fix is either adding it to the detection list or, better, making the failure path return a typed error instead of an in-band magic string.
  - evidence: `src/render/chafa.rs`, `docs/hero-track.md`
- `medium` The idle render cost is dominated by the unconditional 120 FPS redraw loop, not by any one layer: ~23.5 of ~29 CPU points (one core) survive with the hero and companions hidden, measured 2026-07-25. The fastest-moving content in the scene is the hero at a 2 FPS default cadence, so the loop redraws roughly 60x more often than the content changes, and each redraw re-clones and re-normalizes the current hero frame (~1,109 `Span`s) even though `Hero::new` already normalized it. Recorded here because the hero-track trace found it while looking for hero cost, and it outweighs every hero-specific optimization available; whether 120 FPS is intentional is an open question in `docs/hero-track.md`.
  - evidence: `src/runtime.rs`, `src/scene/layers/hero_layer.rs`, `docs/hero-track.md`
- `medium` The hero GIF is read at runtime from `HERO_GIF_PATH`, an absolute `CARGO_MANIFEST_DIR` path baked at compile time, so a binary stays bound to the tree it was built from. Until 2026-07-25 this compounded badly with the cache freshness rule: `cache_is_fresh_against` returned `false` when the source's metadata was unreachable, so a binary whose build tree had moved or been deleted discarded its own valid cache and fell through to the live compile path — which needs that same unreachable GIF — leaving a placeholder frame. Demonstrated by moving `assets/hero_gif_1.gif` aside with a fresh 27MB cache in place: the app rendered `hero gif unavailable: ...`. The freshness rule is now "stale only when the source can be shown to be newer", so an unreachable source keeps the cache instead of invalidating it; re-running the same experiment now renders real cached hero art (593 distinct fg colors including dark reds, no placeholder). This is a mitigation, not a fix for the path binding itself: such a binary still cannot *rebuild* the cache, so a resolution change or cache wipe still yields placeholders. Closing it properly means embedding the asset (`include_bytes!`, ~4.3MB binary cost) or resolving relative to the installed binary (needs an install layout contract) — neither is scheduled. Building from a throwaway git worktree is a live instance of this, not a hypothetical.
  - evidence: `src/render/chafa.rs`, `docs/hero-cache.md`, `bin/yam-install`
- `medium` Dead code is broadly suppressed rather than removed: 136 `allow(dead_code)` sites, including two module-wide `#![allow(dead_code)]`, hide 75 dead-code findings on the bin target (measured 2026-07-25 by stripping the attributes and running `cargo check`). Much of that is deliberate forward vocabulary the audit already sanctions (greenhouse ids, organism registry, spatial helpers), but a distinct subset is plain leftover: `src/ui/anchor.rs` is an entirely unused 35-line module resolving screen anchors — the same concern `core::spatial` owns, and the same leftover shape as the retired `scene::coords`, which that retirement sweep missed. Also `render::compositor::merge_grid_legacy`, the two unused `scene::render_scene*` entry points (production calls only `render_scene_with_scene_and_grid`), six unused functions in `src/render/hero.rs`, three in `src/render/clock.rs`, and both `ui` debug helpers. The practical cost is that `cargo clippy -- -D warnings` stays green partly by suppression, so newly-dead code lands silently.
  - evidence: `src/ui/anchor.rs`, `src/render/compositor.rs`, `src/scene/mod.rs`, `src/render/hero.rs`, `src/render/clock.rs`, `src/core/greenhouse.rs`, `src/render/hero_cache.rs`
- `low` `scripts/check-docs.sh` never validates markdown link targets. It checks active-doc existence, skill frontmatter/metadata, `Cargo.toml`/`README.md` version sync, README `src="..."` assets, and `KI-###` id agreement — but not inline link destinations, which is why `docs/greenhouse-roadmap.md`'s link to the archived HighGrow analysis survived with one `../` too many (the file is under `docs/archive/`, not a sibling of `docs/`). That link was corrected on 2026-07-25; a repo-wide scan the same day found it to be the only broken relative link outside `docs/archive/` (which is historical and full of dead `sediment://` export URIs by nature). The checker gap itself is still open, so the next such link will fail just as silently.
  - evidence: `scripts/check-docs.sh`, `docs/greenhouse-roadmap.md`
- `low` `scene_config.json`'s `gif_path` value (`hero/assets/hero_go.gif`) does not resolve from the repo root; the real asset is at `tools/legacy-python/hero/assets/hero_go.gif`. The only consumer that honors the configured value is the legacy fallback branch in `tools/legacy-python/runtime/system.py`, where the failure is swallowed by a bare `except Exception: hero_block = ""` nested inside another `except`, so it degrades to a silently empty hero rather than an error. `tools/legacy-python` resolves its own copy absolutely and ignores the config value, so nothing user-facing breaks — but the earlier "keep `scene_config.json` aligned with tooling defaults" note understates this: the value is already wrong, not merely at risk of drifting.
  - evidence: `scene_config.json`, `tools/experiments/config.py`, `tools/legacy-python/runtime/system.py`
- `low` The main-scene scaffold has a real world-owned runtime slice (`core::scaffold`) with read-only render layers (rear support cradle, foreground nesting edge); open question is visual sufficiency, not missing ownership. Sandbox hosts the same surfaces behind UI-owned visibility toggles for prototyping without changing world ownership.
  - evidence: `src/core/scaffold.rs`, `src/core/world.rs`, `src/scene/layers/scaffold_layer.rs`, `docs/main-scene-scaffold.md`, `src/ui/state.rs`
- `low` Stale-binary risk: `yam-install` can complete while `yam-rust --version` still reports an older build stamp. Verify installed runtime identity before treating screenshot/output comparisons as current.
  - evidence: `yam-rust --version`, `docs/config.md`
- `low` Dependency graph is clean per `cargo tree -d` (trust this over manual `Cargo.lock` reading — a prior manual inference was wrong). Latest patch update applied 2026-07-21 (`ratatui`, `chrono`, `serde_json`, `unicode-segmentation`, `tachyonfx`); `serde` deliberately held at `1.0.228` since bumping it alone pulls in a new `syn` duplicate for no functional gain. One upstream duplicate remains: `hashbrown` `0.16.1`/`0.17.1` inside Ratatui's own tree (via `kasuari` vs. `lru`), not controllable from this crate's `Cargo.toml`. The `ratatui` patch bump itself (`0.30.0` → `0.30.2`) is larger than "patch" suggests at the `Cargo.lock` level: it pulled in 9 new transitive crates not present before (`approx`, `by_address`, `critical-section`, `fast-srgb8`, `libm`, `palette`, `palette_derive`, `ratatui-termina`, `termina` — color-math and an alternate terminal backend used internally by `ratatui-widgets`/`ratatui-core`), none of which are direct dependencies of this crate or introduce any *additional* duplicate-version conflict beyond the `hashbrown` pair above (re-confirmed via `cargo tree -d` and `cargo tree -i ratatui-termina`, which resolves to nothing built for this target — an unused alternate-backend feature, not shipped bloat). GitHub Dependabot security updates are now enabled (2026-07-22) on top of the manual freshness checks above, so a known-vulnerable dependency should surface as an automated PR rather than waiting for the next manual pass. `.github/dependabot.yml` (added 2026-07-22) also runs routine weekly freshness checks for both the `cargo` and `github-actions` ecosystems, capped at 5 open PRs each, so most of the manual `cargo tree -d` / docs.rs version-checking this file has done by hand should start arriving as PRs instead. Installing `cargo-audit` locally (2026-07-22) surfaced a real, previously-unknown finding: `RUSTSEC-2026-0204` in `crossbeam-epoch 0.9.18`, plus unmaintained (`paste`) and unsound (`anyhow`) warnings — all three traced to `image`'s **default features**, which pulled in the full `ravif`/`rav1e` AVIF encoder stack even though this crate only decodes GIFs (`src/render/chafa.rs`). Trimming `image` to `default-features = false, features = ["gif"]` dropped the dependency count from 300 to 239 and removed `crossbeam-epoch`/`paste` outright; a scoped `cargo update -p anyhow` (not a full `cargo update`, which would have also bumped `thiserror` and reintroduced a `syn` v2/v3 duplicate for no reason) cleared the remaining warning. `cargo audit` is now clean (0 findings) and wired into CI (`.github/workflows/verify.yml`) so this doesn't silently regress. That trim's stated premise ("the only format this crate decodes") was incomplete and caused a same-day silent regression: `src/render/chafa.rs` also *encodes* each temp hero frame to PNG before shelling out to `chafa`, and PNG encoding is a separate opt-in `image` feature, not implied by `gif`. Every hero frame render silently failed to a placeholder from the moment of that commit until it was caught (2026-07-22, during the dark-red coverage investigation, by clearing the pre-existing hero cache and forcing a live rebuild) — masked because the only test exercising the live path (`hero_frame_buffer_has_multiple_frames`) checks frame *count*, not content, and the maintainer's own local cache predated the regression and never went stale enough to hit the broken path. Fixed by adding `"png"` to the feature list (`features = ["gif", "png"]`); re-verified with `cargo tree -d` and `cargo audit` that this adds only the pure-codec `png`/`fdeflate` crates, not the `ravif`/`rav1e` stack the original trim removed. The gap this exposes is structural, not a missing assertion: CI (`.github/workflows/verify.yml`) never installs `chafa`, so every hero-rendering test that goes through the live compile path already tolerates placeholder frames by design (`hero_frame_buffer_has_multiple_frames` only checks frame count, which a placeholder batch also satisfies) — a content-level "no placeholders" test would fail in CI today, not catch regressions there. This class of regression is only catchable by running the real binary with `chafa` present, which is exactly what `scripts/tmux-smoke.sh` is for; treat that as the required check after any change touching `src/render/chafa.rs`, not an automated CI gate, unless `chafa` is added to the CI image first. Dependabot's first two PRs merged 2026-07-22: `actions/cache` 4→6 and `actions/checkout` 4→7, both low-risk Node-runtime bumps. Its third, `serde` 1.0.228→1.0.229, was closed (not merged) the same day: that release's own notes say "update to syn 3," which would reintroduce the exact duplicate this file already documents holding `serde` back for — Dependabot doesn't know about that constraint, so a future serde PR needs the same manual check before merging, not an automerge.
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

1. Hero GIF and rendering infrastructure — the 0.4 main goal (see "Hero Track")
2. Spatial relation layer consolidation
3. Broader flora runtime implementation

Greenhouse breadth (curation/transfer gates, richer per-fixture detail) now sits
behind the hero track for 0.4; [`docs/greenhouse-roadmap.md`](greenhouse-roadmap.md)
remains its owning surface.

## Rule

- Keep this file focused on current risk status, not history or backlog text.
- Keep resolved detail in `docs/LOG.md` and archived reports rather than re-accumulating it here.
