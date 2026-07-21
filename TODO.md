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

- `verify` keep the repo in soft feature freeze mode while this batch is active: polish, stability, bug fixes, contract repairs, and architecture preparation are in scope; large new entities, mechanics, and worlds are not.
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
- `verify` treat `WorldKind::profile()`, `WorldKind::SELECTABLE`, and `WorldKind::loading_label()` as the current world-selection/profile contract; profile-owned grid, camera, guide plan, population plan, and capabilities now route future greenhouse/lab worlds through this seam while `Boot` stays non-selectable.

## 1. Spatial Relation Layer

`core::spatial` is now the canonical resolver everywhere with no remaining compatibility facade: `scene::coords` was retired 2026-07-21 after confirming zero call sites outside its own tests (see `docs/LOG.md`, `docs/architecture.md`). Standing invariants to keep verifying as the layer grows:

- `verify` world-space, screen-space, and anchor-space remain distinct in type names, helper names, tests, and docs.
- `verify` projection remains singular and uses the same signed world-to-screen transform for hero, companions, guides, vines, and future plant geometry.
- `verify` vine rendering consumes `core::spatial::SpatialResolver` directly and keeps shared drawing writes on checked signed-to-grid conversion.
- `verify` resize, camera movement, anchor resolution, and rounding/jitter coverage stays tight before adding new world-attached renderables.
- `verify` guides remain semantic world-space linework and are not reinterpreted from rendered pixels.
- `next` `scene::entity::EntityPose`/`AttachedEntityPose` still duplicate the shape of `core::spatial::SpatialAnchor`/`SpatialAttachment` under domain-specific names; low-risk, not scheduled, but worth revisiting if a third attachment-shaped consumer appears.

## 2. Flora Runtime And Organism Model

- `refactor` split vine-specific types and helpers out of the broad `core::flora` surface once a second organism family is close enough to need shared vocabulary.
- `verify` preserve the shared organism identity set before new plant families land: organism id, species id, journal id, life state, stats, and the first species-profile shape.
- `inspect` decide where species profiles should be loaded from once there is more than the in-memory border-vine `SpeciesRegistry` profile: static Rust fixtures, structured data files, or a small registry loader.
- `verify` preserve `FloraState` family-count and organism-identity adapters; do not add another top-level ad hoc vector beside `FloraState::vines` without first deciding whether `FloraState` needs an organism registry or a small enum-backed family store, with enum-backed family store as the current first-pass bias.
- `refactor` move the current border-vine growth rule away from one hard-coded seed branch before adding another plant growth rule.
- `verify` render layers stay read-only and visualize geometry derived from world/flora/spatial state.
- `verify` `OrganismJournal` remains a per-instance event log; `SpeciesRegistry` must not store runtime life history.
- `inspect` keep botanical terminology grounded through `docs/glossary.md`; reserve `node` for plant morphology/anatomy unless the spatial terminology contract changes.

## 3. World Modes And Greenhouse Prep

- `docs` treat `docs/greenhouse-roadmap.md` as the single owning greenhouse roadmap and operation plan; keep greenhouse strategy, phase tasks, candidate organisms, stop conditions, and creative-input briefs there instead of in this backlog.
- `verify` preserve the explicit world-selection/profile contract before adding `Greenhouse` or lab worlds: selectable worlds live in `WorldKind::SELECTABLE`, titles, transition labels, coarse composition, grid, camera defaults, guide plan, population plan, and capabilities live in `WorldKind::profile()`, and UI persistence snapshots convert through that core contract.
- `verify` `WorldKind::{Boot, MainScene, Sandbox}` interaction with camera, loading, input, settings, persistence, and composition-gated render surfaces stays defined while the world-switching contract remains generalized.
- `verify` preserve the roadmap's 0.4 gate checklist and locked first-pass greenhouse decisions before implementation: docs aligned, verification green, spatial ownership stable, enum-backed flora store bias, functional-space contract first, and hero/render fallback hardened.
- `inspect` keep the first greenhouse pass functional-space-first: room,
  access paths, zones, fixtures, planting sites, symbolic environment, and
  read-only inspection before new plant families, lifecycle systems, or
  persistence.
- `verify` keep the landed pure `core::greenhouse` slice inert: `GreenhouseState`, room, access path, zone, fixture, planting site, symbolic environment, and read-only inspection records stay data-only until a later batch deliberately attaches them to `WorldState`.
- `verify` greenhouse remains a separate simulation world or world-internal room model, not panel chrome layered on top of the main scene; room selection stays internal to world state.
- `inspect` preserve the newly ingested greenhouse identity hierarchy before UI work: greenhouse/lab, planting site or bay, support, organism, warning, and journal-link identities must stay distinct.
- `inspect` if greenhouse grows into a frame-plus-labs structure later, promote it in order: single inert nursery room first, then room-to-lab naming, then greenhouse frame/navigation chrome only after room ownership is proven.
- `inspect` decide whether greenhouse inspection starts as lightweight per-organism popups or a dedicated registry/journal mode before building either surface.
- `inspect` keep the first external creative-input request bounded by the roadmap brief so ideas enter as room profiles, species profiles, fixtures, environment presets, or inspection text instead of hidden UI or render-owned behavior.
- `verify` greenhouse progression remains curation-oriented rather than game-like: no chores, currency, unlock grind, daily obligations, or automatic main-scene mutation from transfer status.

## 4. Main Scene Enrichment Prep

- `verify` main-scene enrichment must preserve hero aesthetics, footer placement, modal geometry, and companion projection semantics.
- `verify` the main scene remains the live visualiser/screensaver composition; enrichment should add world-attached content, not a dashboard layout.
- `verify` keep the landed scaffold passes honest: `core::scaffold` owns the rear support cradle plus the small foreground nesting edge for the main scene, scaffold render layers stay read-only, and decorative branching still remains deferred until the seated silhouette is proven.
- `verify` use sandbox as the first scaffold-prototyping room: hero, companions, and scaffold may be toggled there for composition trials, but the toggles remain UI-owned presentation switches over the shared world/render pipeline rather than a second runtime owner.
- `inspect` treat scaffold masks as a later seam, not the next default: prefer world geometry plus layer order first, then introduce an explicit mask primitive only if a concrete occlusion or nesting read proves it is necessary.
- `inspect` stage vine Phase 8 in slices only after the shared flora vocabulary is ready: lateral axes, small leaves, larger decorative leaves after clipping/z-order tests, then flowers, fruit, or particles.
- `inspect` stage vine Phase 9 border behavior through world bounds, boundary guides, or exclusion outlines before any mask-like behavior.
- `verify` any new scaffold, leaf, particle, or plant surface gets a world/HUD/overlay assignment before implementation.
- `verify` add negative tests for any enrichment that could blur world/HUD boundaries, mutate world state from render code, or disturb the footer row.

## 5. Rendering And Hero Pipeline

- `verify` hero rendering stays renderer-owned and cache-first on the common path.
- `verify` the live Chafa compiler path degrades to placeholder frames instead of panicking when the GIF, temp directory, temp image write, or `chafa` command fails.
- `inspect` define the future `HeroFrameSet` offline compiler contract before replacing or bypassing the active Chafa-backed path.
- `inspect` define the intermediate `CellGrid` correction format with glyph, foreground color, optional background color, and mask/style metadata before building manual editor tooling.
- `inspect` prototype `.xp` export/import only after `CellGrid` exists, with explicit braille font/tile mapping and round-trip validation for glyph/color fidelity.
- `verify` cached frames, Chafa output, and any future manual corrections preserve fixed hero frame geometry across resize and scene stabilization checks.
- `inspect` continue render-loop optimization only when live-loop profiling points at a real remaining hot path; do not restart broad startup optimization from wall-clock boot timing alone.

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
