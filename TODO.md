# Active Backlog

<!-- cspell:ignore twimc -->

This file is the repo-wide work order and stabilization checklist.

Coordination contract:

- keep this file execution-focused
- keep risk and status notes in `docs/audit.md`
- keep the historical record in `docs/LOG.md`
- keep active unresolved issue notes in `known_issues.md`
- if a TODO item becomes mainly a risk note, move it to the audit
- if a TODO item is blocked on or clarified by an active issue, link the matching `known_issues.md` entry
- if a TODO item becomes historical, record the closure in the log and remove it here

Issue link rule:

- use explicit `known_issues.md` ids such as `KI-###` when an execution item is directly tied to an active unresolved issue
- do not create a `known_issues.md` entry for broad architecture debt; use this file and `docs/audit.md` until there is a small active user-visible or developer-visible issue

## Current Pre-Expansion Batch

- `next` **the hero GIF and its rendering infrastructure are the main goal of the 0.4 development phase** (set 2026-07-25): source art, GIF decode, the `chafa` compile path, the frame cache, and the offline compiler / `CellGrid` direction. Both the infrastructure and the rendered art are in scope for deliberate change — the previous repo-wide hero art freeze is lifted for this track only. Execution items live in section 5 below; the scope, standing constraints, and what the lifted freeze does *not* change are in `docs/audit.md`'s Hero Track section.
- `verify` keep the repo in soft feature freeze mode while this batch is active: polish, stability, bug fixes, contract repairs, and architecture preparation are in scope; large new entities, mechanics, and worlds are not. The hero track above is explicitly in scope for this batch — it is infrastructure and art on an existing surface, not a new entity, mechanic, or world.
- `docs` keep `TODO.md`, `docs/audit.md`, `docs/greenhouse-roadmap.md`, `docs/vines.md`, `docs/scene-model.md`, `docs/architecture.md`, `docs/rendering.md`, and `docs/hygiene.md` aligned before any main-scene enrichment or greenhouse work starts.
- `docs` prepare the external eval report ingestion path before the first report lands: keep the raw report as reference input, route active findings into `TODO.md` or `docs/audit.md`, record the ingestion batch in `docs/LOG.md`, and avoid creating a second competing authority surface.
- `verify` keep `known_issues.md` empty unless a concrete active issue appears; broad pre-expansion risks belong in the audit and this backlog.
- `verify` keep `scripts/verify.sh` green for every maintenance handoff.
- `verify` treat the 0.4 pre-expansion gates as active before broad feature work:
  docs aligned, verification green, spatial ownership stable, flora storage
  decision made, greenhouse/world contract decided, and hero/render failure
  modes hardened.
- `inspect` use `cargo tree -d` after dependency changes and prefer version convergence when it reduces duplicated terminal/runtime infrastructure without forcing design churn.
- `verify` treat the current live vine runtime honestly: storage, deterministic seed, guide lookup, static axis derivation, read-only render layer, debug facts, deterministic growth, local tip lifespan, segment aging, and leaf-organ hosting are already implemented; branching, richer organs, and border behavior remain future work.
- `verify` treat `core::organism` as the first shared flora identity vocabulary plus the first in-memory `SpeciesRegistry` / `OrganismJournal` skeleton; keep it small until another plant family or greenhouse population proves the next field is needed.
- `verify` treat `WorldKind::profile()`, `WorldKind::SELECTABLE`, and `WorldKind::loading_label()` as the current world-selection/profile contract; profile-owned grid, camera, guide plan, population plan, and capabilities already route `Greenhouse` (landed 2026-07-21) and would route any future lab world through this same seam, while `Boot` stays non-selectable.

## 1. Spatial Relation Layer

`core::spatial` is now the canonical resolver everywhere with no remaining compatibility facade: `scene::coords` was retired 2026-07-21 after confirming zero call sites outside its own tests (see `docs/LOG.md`, `docs/architecture.md`). Standing invariants to keep verifying as the layer grows:

- `verify` world-space, screen-space, and anchor-space remain distinct in type names, helper names, tests, and docs.
- `verify` projection remains singular and uses the same signed world-to-screen transform for hero, companions, guides, vines, and future plant geometry.
- `verify` vine rendering consumes `core::spatial::SpatialResolver` directly and keeps shared drawing writes on checked signed-to-grid conversion.
- `verify` resize, camera movement, anchor resolution, and rounding/jitter coverage stays tight before adding new world-attached renderables.
- `verify` guides remain semantic world-space linework and are not reinterpreted from rendered pixels.
- `next` `scene::entity::EntityPose`/`AttachedEntityPose` still duplicate the shape of `core::spatial::SpatialAnchor`/`SpatialAttachment` under domain-specific names; low-risk, not scheduled, but worth revisiting if a third attachment-shaped consumer appears.
- `next` retire `src/ui/anchor.rs`: an entirely unused screen-anchor module duplicating what `core::spatial` owns, and the same leftover shape as the retired `scene::coords` — confirm zero call sites the same way, then remove.

## 2. Flora Runtime And Organism Model

- `refactor` split vine-specific types and helpers out of the broad `core::flora` surface once a second organism family is close enough to need shared vocabulary.
- `verify` preserve the shared organism identity set before new plant families land: organism id, species id, journal id, life state, stats, and the first species-profile shape.
- `verify` preserve `FloraState` family-count and organism-identity adapters and the locked enum-backed `FloraInstance` family store (`FloraState::organisms: Vec<FloraInstance>`, `vines()`/`vines_mut()`/`push_vine()` accessors); a second family gets a new `FloraInstance` variant, not another top-level ad hoc vector.
- `verify` `systems::growth::run_growth` iterates every vine in `world.flora` (2026-07-21) rather than finding one hard-coded seed id, matching `systems::aging::run_aging`'s existing all-vines shape; keep this true as families beyond `Vine` are added.
- `verify` render layers stay read-only and visualize geometry derived from world/flora/spatial state.
- `verify` `OrganismJournal` remains a per-instance event log; `SpeciesRegistry` must not store runtime life history.
- `inspect` keep botanical terminology grounded through `docs/glossary.md`; reserve `node` for plant morphology/anatomy unless the spatial terminology contract changes.

## 3. World Modes And Greenhouse Prep

- `docs` treat `docs/greenhouse-roadmap.md` as the single owning greenhouse roadmap and operation plan; keep greenhouse strategy, phase tasks, candidate organisms, stop conditions, and creative-input briefs there instead of in this backlog.
- `verify` preserve the explicit world-selection/profile contract now that `Greenhouse` is a real selectable world (2026-07-21): selectable worlds live in `WorldKind::SELECTABLE`, titles, transition labels, coarse composition, grid, camera defaults, guide plan, population plan, and capabilities live in `WorldKind::profile()`, and UI persistence snapshots (`WorldKindSnapshot`) convert through that core contract.
- `verify` `WorldKind::{Boot, MainScene, Sandbox, Greenhouse}` interaction with camera, loading, input, settings, persistence, and composition-gated render surfaces stays defined while the world-switching contract remains generalized.
- `verify` preserve the roadmap's 0.4 gate checklist and locked first-pass greenhouse decisions: docs aligned, verification green, spatial ownership stable, enum-backed flora store (locked), species-profile format (locked), `WorldState.greenhouse` attachment (`Some(GreenhouseState::nursery())` for `Greenhouse` only, `None` elsewhere), functional-space contract, and hero/render fallback hardened.
- `inspect` keep the first greenhouse pass functional-space-first: room,
  access paths, zones, fixtures, planting sites, symbolic environment, and
  read-only inspection before new plant families, lifecycle systems, or
  persistence.
- `verify` growth dispatch and inspection have landed (2026-07-22): a first `OrganismFamily::Seedling` occupies `left_tray` (soft `PlantingSite::occupant` reference) and advances `Dormant -> Growing -> Mature` on its own tick cadence via `run_greenhouse_growth`; a read-only `GreenhouseInspectLayer` (`i` hotkey, dev-mode + Greenhouse-world gated) surfaces the room's `inspection_refs`. Remaining: per-fixture/per-organism live detail and any curation/transfer write-path.
- `verify` greenhouse remains a separate simulation world (`WorldKind::Greenhouse`), not panel chrome layered on top of the main scene; room selection stays internal to world state.
- `inspect` preserve the newly ingested greenhouse identity hierarchy before UI work: greenhouse/lab, planting site or bay, support, organism, warning, and journal-link identities must stay distinct.
- `inspect` if greenhouse grows into a frame-plus-labs structure later, promote it in order: single inert nursery room first, then room-to-lab naming, then greenhouse frame/navigation chrome only after room ownership is proven.
- `inspect` keep the first external creative-input request bounded by the roadmap brief so ideas enter as room profiles, species profiles, fixtures, environment presets, or inspection text instead of hidden UI or render-owned behavior.
- `verify` greenhouse progression remains curation-oriented rather than game-like: no chores, currency, unlock grind, daily obligations, or automatic main-scene mutation from transfer status.

## 4. Main Scene Enrichment Prep

- `verify` main-scene enrichment must preserve hero aesthetics, footer placement, modal geometry, and companion projection semantics. As with the vine rule, this is scoped: enrichment work must not change the hero as a side effect, but the hero track itself may change it deliberately.
- `verify` the main scene remains the live visualiser/screensaver composition; enrichment should add world-attached content, not a dashboard layout.
- `verify` keep the landed scaffold passes honest: `core::scaffold` owns the rear support cradle plus the small foreground nesting edge for the main scene, scaffold render layers stay read-only, and decorative branching still remains deferred until the seated silhouette is proven.
- `verify` use sandbox as the first scaffold-prototyping room: hero, companions, and scaffold may be toggled there for composition trials, but the toggles remain UI-owned presentation switches over the shared world/render pipeline rather than a second runtime owner.
- `inspect` treat scaffold masks as a later seam, not the next default: prefer world geometry plus layer order first, then introduce an explicit mask primitive only if a concrete occlusion or nesting read proves it is necessary.
- `inspect` stage vine Phase 8 in slices only after the shared flora vocabulary is ready: lateral axes, small leaves, larger decorative leaves after clipping/z-order tests, then flowers, fruit, or particles.
- `inspect` stage vine Phase 9 border behavior through world bounds, boundary guides, or exclusion outlines before any mask-like behavior.
- `verify` any new scaffold, leaf, particle, or plant surface gets a world/HUD/overlay assignment before implementation.
- `verify` add negative tests for any enrichment that could blur world/HUD boundaries, mutate world state from render code, or disturb the footer row.

## 5. Rendering And Hero Pipeline

**This section is the 0.4 main goal** (set 2026-07-25) — see the Current
Pre-Expansion Batch above and `docs/audit.md`'s Hero Track section for scope and
standing constraints. Items here take precedence over sections 1-4 and 6-9
unless a concrete regression says otherwise.

`docs/hero-track.md` is the traced baseline for this work (measured 2026-07-25:
stage-by-stage trace, cold/warm costs, ranked opportunities, open questions).
Start there rather than re-deriving the pipeline. Its ranked list is deliberately
ordered by measured value, and the top item is *not* hero-specific: the
unconditional 120 FPS redraw loop outweighs every hero-local optimization.

**Prep for this track is complete (2026-07-25); the rework phase is the next
action.** Nothing further is planned before rework starts. The prep landed on
`claude/full-audit-1e44b1`, pushed with no pull request opened, so it has run no
CI — the gate was green locally at every commit, but see `docs/hygiene.md` on
what a pushed topic branch does and does not verify.

Six decisions are recorded as `HQ-1` through `HQ-6` in that document's Resolution
Points section, to be resolved *during* the rework phase rather than up front.
Each carries its own resolution criteria and names the doc that owns its answer;
cite the id rather than restating the question. All are currently `open` and none
is pre-judged. Resolve `HQ-3` before investing in either conversion branch, since
it sequences the others; `HQ-5` changes the baseline the rest are measured
against, so it is worth settling early.

- `next` `HQ-1` decide whether `tone_lift_dark_reds` stays, via an A/B of rendered output against the current alpha + median path.
- `next` `HQ-2` establish whether the unconditional 120 FPS redraw loop is intentional, and if it changes, what should drive redraws.
- `next` `HQ-3` decide whether `chafa` remains a subprocess or conversion moves in-process; this sequences the stdin, parallelism, and offline-compiler work.
- `next` `HQ-4` decide whether the fixed 96x48 hero geometry survives or becomes terminal-responsive, and what that implies for cache keying.
- `next` `HQ-5` decide whether the 64-frame / 820x820 source art shape is itself revised, now that art is in scope.
- `next` `HQ-6` close the `ANSI_PARSE_ERROR` cacheability gap (`src/render/chafa.rs:44` vs `is_placeholder_frame`), where a parse failure is currently judged cacheable and would be persisted as trusted art; the open part is whether to extend the detection list or replace the in-band magic string with a typed error.

- `verify` hero rendering stays renderer-owned and cache-first on the common path.
- `verify` the live Chafa compiler path degrades to placeholder frames instead of panicking when the GIF, temp directory, temp image write, or `chafa` command fails.
- `inspect` define the future `HeroFrameSet` offline compiler contract before replacing or bypassing the active Chafa-backed path.
- `inspect` define the intermediate `CellGrid` correction format with glyph, foreground color, optional background color, and mask/style metadata before building manual editor tooling.
- `inspect` prototype `.xp` export/import only after `CellGrid` exists, with explicit braille font/tile mapping and round-trip validation for glyph/color fidelity.
- `verify` cached frames, Chafa output, and any future manual corrections preserve fixed hero frame geometry across resize and scene stabilization checks.
- `inspect` continue render-loop optimization only when live-loop profiling points at a real remaining hot path; do not restart broad startup optimization from wall-clock boot timing alone.
- `next` decide whether to close the `HERO_GIF_PATH` build-tree binding properly (`include_bytes!` at ~4.3MB binary cost, or binary-relative resolution behind an install layout contract); the 2026-07-25 cache-retention change is a mitigation only — such a binary still cannot rebuild its cache. See `docs/audit.md` and `docs/hero-cache.md`'s Source Reachability section.

## 6. UI, Dev Surfaces, And Workflow

- `refactor` extract small helper/state seams from `UiState` only where a concrete workflow gets simpler: world switching, companion offsets, weather refresh, settings editing, or dev overlay toggles.
- `verify` `calendar (reserved)` stays demoted outside lightweight move/help surfaces until a live calendar surface exists.
- `verify` hotkeys, move strip, footer, settings, help, palette, weather inspection, pointer probe, and quit-confirm keep separate roles and one shared modal-shell vocabulary where applicable.
- `inspect` introduce a dedicated FIGlet/font subsystem for YAM text-art surfaces instead of continuing one-off ASCII literals; use `docs/reference-sigye.md` as the starter reference.
- `inspect` add a reusable styled hotkey-hint formatter so overlays can present compact cues like `up/down nav` and `left/right change` with explicit token/description contrast.
- `verify` help, move, settings, and quit-confirm continue to share one centered modal shell so popup styling and geometry do not drift apart.
- `verify` settings remain modal, tabbed, dev-gated, and presentation-oriented; UI widgets must not mutate world simulation state.

## 7. Docs, Tooling, And Release Hygiene

- `verify` `scripts/check-docs.sh` covers the live root/front-door docs and first-level `docs/*.md` contract surface, not only the oldest core contract subset.
- `verify` keep `AGENTS.md` short, procedural, and pointer-heavy; do not let it duplicate architecture contracts or active backlog content.
- `verify` keep repo-local `skills/*/SKILL.md` files short and procedural with matching `agents/openai.yaml` metadata; their names, frontmatter descriptions, and interface metadata must pass `scripts/check-docs.sh`, and they should promote only repeatable work modes that point back to canonical docs.
- `verify` active markdown docs stay clean under repo-configured `markdownlint`, `markdownlint-cli2`, and `cspell`.
- `verify` README local asset references must point to committed files.
- `verify` `README.md` current release stays synchronized with `Cargo.toml`.
- `inspect` keep future README polish bounded and factual: preserve the restored intro voice, but revisit small front-door nits from the external eval only when touching the README anyway, especially heading hierarchy around `twimc`, terse unlabeled command blocks, and repo-shape claim precision.
- `next` teach `scripts/check-docs.sh` to validate markdown link targets, not only doc existence and README `src="..."` assets; exclude `docs/archive/`, whose export-era `sediment://` URIs are dead by nature. The one broken active link this gap allowed was fixed 2026-07-25, but nothing stops the next one.
- `next` audit the 136 `allow(dead_code)` suppressions (two module-wide) hiding 75 bin-target findings; remove the plain leftovers (`merge_grid_legacy`, the two unused `scene::render_scene*` entry points, the unused `render/hero.rs` and `render/clock.rs` helpers, both `ui` debug helpers) and keep only the deliberate forward vocabulary, so the `-D warnings` gate stops being green by suppression.
- `next` fix `scene_config.json`'s `gif_path`, which resolves to nothing from the repo root and degrades to a silently empty hero in the only consumer that honors it.
- `verify` `TODO.md` issue references must point to active `known_issues.md` ids.
- `verify` append each completed maintenance batch to `docs/LOG.md` using the local system time noted in the log's current logging rule.
- `verify` keep `docs/audit.md` risk-focused and `docs/LOG.md` historical; avoid re-accumulating completed work in the active backlog.

## 8. Contract Pointers

- `verify` projection details stay in `docs/scene-model.md` and `docs/rendering.md`.
- `verify` ownership and coupling rules stay in `docs/architecture.md`.
- `verify` vine-specific status and design notes stay in `docs/vines.md`.
- `verify` weather-widget provider/model/render ownership stays in `docs/weather-widget.md`.
- `verify` theme/palette contracts stay in `docs/theme.md` and the palette reference docs.
- `verify` release and branch policy stay in `docs/release-model.md`.
- `verify` research/resource scouting stays in `docs/resource-map.md`, not in this backlog.
- `verify` render-time validation goals stay here only as execution checks, not duplicated contract prose.

## 9. Maintenance Rules

- `verify` add new backlog items as execution steps, inspections, or regression checks; keep contract wording in the owning docs.
- `verify` prefer one narrowly scoped item per line so TODO stays easy to prune.
- `verify` if a TODO item survives multiple passes without changing shape, either promote it to the owning contract doc or remove it.
- `verify` any behavior change keeps its test, log entry, and owning doc update in the same change.
- `verify` new work that introduces a concept already named in an active doc must reference the canonical doc instead of restating the rule.
- `verify` all scene rendering continues through `render_scene` and `Scene::render`; no side path should write to the terminal buffer.
- `verify` `src/core` remains independent from `scene` modules, and `src/systems` remains independent from scene/render/UI/terminal modules; `scripts/check.sh` must keep guarding those boundaries.
- `verify` new world-attached renderables reuse shared spatial/entity pose helpers instead of adding bespoke anchor math.
- `verify` boundary changes in render, layer, attachment, or mask code prefer an explicit negative test when practical.
- `verify` metamechanics remains a subordinate control/observation seam inside `ui/`; `dev_mode` may toggle presentation flags, but it does not own world state, projection, or render order.
- `verify` follow-hero camera mode stays centered on the world datum across terminal resizes, while manual pan mode remains clamped to world overscan.
- `verify` the screenshot-aligned manual boot seed `(-60, -15)` remains distinct from the centered `follow-hero` runtime path, so boot composition and resize behavior stay separately owned.
