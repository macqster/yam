# YAM-RUST Architecture Contract

## Assertions

- Ownership and data flow belong here, not in scene-model, rendering, TODO, or archive docs.
- `RenderState` is built once per frame and then treated as read-only.

## Change Impact

- If you change this, also review `docs/scene-model.md`, `docs/rendering.md`, and the `RenderState` tests.

## Core Rules

- `core/` - data only, no UI, no terminal, no rendering
- `core/guide.rs` - world-space guide primitives and query helpers; guides are semantic data, not raster masks
- `core/spatial` - the first canonical spatial relation layer; it currently owns the shared transform/projection/attachment helpers and will absorb more relation logic over time
- `systems/` - mutate `WorldState` only, no rendering
- `render/` - terminal render primitives, chafa/hero conversion, grid composition, masks, and final text conversion
- `scene/` - layer ordering, camera/viewport types, coordinate helpers, and scene-level grid composition
- `ui/` - runtime UI state, persisted offsets/settings, screen-space widgets, and temporary scene adapter
- `runtime.rs` - event loop, input, tick, and render orchestration only
- Rust should remain the runtime and simulation core because it already owns the deterministic frame loop, scene layers, world state, and render contracts; other tools should be justified by whether they improve data authoring, offline analysis, or botanical research rather than by fashion
- if a non-Rust tool is useful, it should usually live outside the hot path: Python or notebooks can help with offline botanical research or exploratory data analysis, and structured data files or a small database can help with species registries and journals without moving the runtime core out of Rust
- the external-tool rule of thumb is simple: keep real-time simulation, rendering, and UI in Rust; allow sidecar tools only for research, registry authoring, batch processing, or one-off inspection that would be awkward in the live terminal app
- Lua is a plausible optional extension layer if we want compact species scripts or debug/dev plugins, but it should remain bounded and Rust-hosted: scripts can define species presets, authoring helpers, or overlay behavior, while Rust keeps the canonical state, determinism, and render ownership
- if Lua is introduced, it should be treated as a plugin/script layer for data-driven flora authoring and tooling, not as the owner of world mutation, render orchestration, or the primary simulation loop

## Forbidden Coupling

- `core -> ui`
- `core -> render`
- `systems -> ui`
- `systems -> render`
- `render -> world mutation`
- `scene layers -> persistent state mutation`
- `ui widgets -> world mutation`
- `metamechanics -> world mutation outside the explicit UI state seam`

## Rendering Pipeline

- runtime draw closure calls `render_scene`
- `Scene` builds ordered layers through `ui::scene::build_ui_layers`
- `Scene` computes a read-only `RenderState` once per frame and passes it to every layer
- `RenderState` is split into:
  - `world`: hero and clock attachment facts that stay world-pinned
  - `hud`: viewport and camera facts that stay screen/terminal-attached
- `UiState` owns the runtime attachment offsets that feed the hero/clock attachment object
- `WorldState` owns `GuideState` so world-attached guide primitives stay in simulation data, not UI state
- hero and clock attachment facts are computed through explicit `scene::entity::HeroClockAttachment` produced by `scene::entity::hero_and_clock_poses(...)`
- each layer emits a full-frame `LayerOutput`
- `Scene` merges layer grids with `render::compositor::merge_grid`
- `Scene` converts the final grid into ratatui `Line`s
- ratatui receives one final `Paragraph` for the frame
- scene rendering uses the full terminal area for viewport and viewport-rect values
- the scene model contract lives in [`scene-model.md`](scene-model.md) and defines the deterministic layer/space/masking rules above ratatui
- the layer order is a hard precedence ladder: world base first, world props next, world-tied debug/dev assets after that, HUD after world, and modal overlays last
- overwrite priority follows numeric `z_index` inside that ladder, so world-tied diagnostic assets can intentionally overwrite world props without becoming HUD content
- masks only gate explicit compositor writes; they do not create a second ordering system
- the presentation stack is conceptualized as world -> HUD -> overlay, with overlays reserved for modal or top-z-index panels
- metamechanics is a subordinate control/observation seam inside `ui/`; it may toggle overlays or presentation flags, but it does not own world state, projection, or render order
- `dev_mode` is the umbrella metamechanics toggle: it enables the layout/editing surface and the debug overlay, while `debug` remains the actual diagnostic presentation
- day-to-day debug visibility is distinct from dev-mode control surfaces: everyday debug overlays may remain available during normal use, while deeper mutation/editing controls stay gated behind dev mode
- keyboard interaction should follow stable muscle-memory conventions from ASCII TUIs and roguelike workflows, with explicit modes, discoverable hints, and predictable key reuse
- the interface should prefer semantic structure over decorative novelty so future plant stats, morphology data, and lifecycle variables remain readable at high density
- a command palette or similar action hub is the preferred discoverability fallback for rare actions, entity jumps, and overlay toggles
- the interface mode map should remain stable and explicit:
  - `normal`: day-to-day scene use, lightweight HUD, and everyday debug visibility
  - `inspect`: focus, entity detail, and drill-down reading
  - `debug`: diagnostic overlays that are safe to leave available in normal use
  - `dev`: gated mutation/editing controls and simulation tooling
  - `command palette`: search/fallback discovery for rare actions and jumps
- turning `dev_mode` off closes any open hotkeys, move, or settings modal state so the modal family cannot stay latched outside the dev surface
- `settings` is the modal metamechanics popup: it shows tabbed, dev-mode controls for positions, widgets, gif, and theme values without owning world state or projection
- modal move/settings/hotkeys popups now share one centered modal shell: the shell paints an opaque BTAS-style backdrop before text and border are drawn, so their controls stay readable over world content and stay architecturally unified
- compositor cells with a background color and a space glyph are treated as opaque backdrop writes, so modal panels clear the GIF beneath them instead of tinting it through
- the clock is not a UI entity: it is a world-attached hero companion, and the debug/info panels only observe its projected screen position
- guides are world-attached semantic primitives owned by `WorldState`; for now they are linework-only primitives (points, lines, polylines, and outline shapes) that the debug overlay visualizes and vines may query, but they are not raster masks or solid fills; each guide carries its own label, may belong to an optional named group, and the state also carries named `GuideSet` collections so larger guide groups can be addressed explicitly
- the guide / line generator is project-wide, not vines-only: it supports guide drawing now and should remain suitable for future mask edges, rulers, and other world-space annotations with the same world-space contract, and it must remain capable of generating any line in any direction across the full YAM world size
- the broader spatial system should be able to describe absolute world datum guidance and relative attachment guidance for plantlife, flowers, vines, and other organisms without forcing those behaviors through render-only helpers
- YAM should provision for two main simulation spaces: the main scene visualiser and the greenhouse/lab space, with the greenhouse expected to host multiple distinct flora species rather than a single vine-like organism type
- plant simulation must support procedurally growing organisms with different anatomies and morphologies, so vines are one family in a larger flora system rather than an accidental singularity
- the spatial system should lean on Cartesian and Euclidean logic where possible, because signed axes, centered datum math, and direct distance reasoning make world authoring and placement easier to reason about
- the soft-line renderer and pointer probe should be treated as one authoring workflow: pointer records exact coordinates for guides, points, masks, and other spatial relations, and the line engine makes those relations visible in world space; the term `nodes` is currently reserved for plant morphology/anatomy systems and should be treated as provisional until the spatial terminology is researched further
- the practical guide workflow is point-first: capture exact coordinates with the pointer, compose them into lines/polylines/outlines, then validate the result with the soft-line renderer before treating it as stable world structure
- the current capture UI is deliberately narrow: pointer probe for exact world coordinates, debug/info for readout, hotkeys for discoverability, and move mode for explicit world-attached repositioning
- the UI should also expose read-only preview lists for guide sets and subsets so grouped geometry, such as a polyline outline used for a mask shape, can be inspected without mutation
- the flora architecture should follow a hybrid graph-plus-segment model: plant state is a growth graph of organs, while geometry is emitted as Euclidean segments and outlines under deterministic rules
- the project's plant-structure thinking has historical inspiration from `cbonsai`, and that lineage should remain visible as a useful constraint even as YAM grows beyond a single bonsai-style plant into a broader greenhouse and multi-species model
- inspiration lineage for the current plant/UI thinking can stay short and explicit:
  - `cbonsai` for compact plant growth, branching, and readable terminal botany
  - `Dwarf Fortress` for ASCII-era muscle memory, dense inspectable state, and strong world-model thinking
  - `Cataclysm: DDA` for keyboard-first survival UI, modal discipline, and practical terminal ergonomics
- the terminology should stay consistent with the flora model: `metamer` for repeating structural units, `internode` for segments, `meristem` for growth points, `axis` for branch systems, `insertion` for organ attachment, and `node` reserved as a provisional plant-term pending further research
- the first implementation targets should be small and inspectable: the Y-shaped tree-stump scaffold, a guide-following vine family, and a monstera-like plant with large leaf organs
- a minimal Rust-shaped plant model can stay small and explicit: `Plant` owns species and lifecycle state, `Axis` owns a growth branch and ordered metamers, `Metamer` carries one internode and organ attachments, `Meristem` represents an active growth point, and `Organ` covers leaf/flower/fruit/branch outputs; the geometry layer should be derived from this state rather than owning it
- a minimal lifecycle update loop can stay equally small: `Seed -> Growth -> Mature -> Senescent -> Decay`, with active meristems driving growth steps, species rules choosing new metamers or organs, and geometry being regenerated from state each tick
- organ state should remain explicit and inspectable: buds, leaves, flowers, and fruit can each progress through `bud -> growing -> mature -> aging -> dead` without requiring the whole plant to collapse into a single global state
- the greenhouse/lab space should be the place where lifecycle tuning becomes visible, while the main scene can keep the current prototypes comparatively static and readable
- each plant organism should be treated as an independent life-form with its own life state, stats, and variables; a species registry or database layer should hold species definitions, morphology traits, growth rules, and other reusable data that drive in-YAM generation and emulation
- the species registry should be read-heavy and simulation-friendly: it can store canonical species metadata, but per-plant runtime state stays with the living organism instance in `WorldState` or its flora subsystem
- each life-form should also have a dedicated log-journal so lifecycle events, growth changes, and debugging notes can be tracked per organism without flattening everything into a single global log
- a life-form journal should stay compact and event-oriented: lifecycle transitions, growth steps, organ births/removals, environment influences, damage/pruning, and debug annotations are the highest-value entries
- the journal should be human-readable first and machine-friendly second, so greenhouse inspection can scan it quickly without losing deterministic simulation detail
- the species registry payload should stay compact and reusable: species id/name, morphology defaults, branching pattern, internode length, leaf distribution, growth rate, tropism rules, lifecycle tuning, allowed organs, and debug labels are the highest-value fields
- the registry should not store per-instance life history; that belongs in the individual plant journal and runtime state
- the state/stat/journal/registry layer may deserve its own dedicated render mode in the future, distinct from the main scene and greenhouse, if the inspection burden grows enough to justify a specialized view
- an alternative UI strategy is a per-life-form popup window that shows the organism’s relevant data and allows limited tweaks such as growth rate or lifecycle length, while keeping the underlying state registry-backed and the journal per-instance
- comparison rule of thumb: a dedicated mode fits large-scale registry browsing and greenhouse administration; per-life-form popups fit quick inspection or light tuning; lightweight debug overlays fit routine day-to-day checks when the organism count is still manageable
- the registry taxonomy can be kept explicit by grouping fields into anatomy defaults, growth rules, visual phenotype ranges, lifecycle tuning, and debug labels; this makes the same registry useful for generation, emulation, and inspection without overloading one field bucket
- botanical basics should remain a reference point for species design: if a morphology term, growth rule, or anatomy field is unclear, the registry should be informed by real plant structure before the term is committed to YAM
- terminology authority should be explicit:
  - `strict` terms are botanically grounded and should match standard plant meaning as closely as practical
  - `inferred` terms are YAM-specific design interpretations built from real plant structure and terminal constraints
  - `provisional` terms are placeholders that may change once the plant model or spatial terminology is researched further
- plant-language references live in [`docs/glossary.md`](glossary.md); this contract only needs the YAM-specific rule that `node` stays reserved for plant morphology/anatomy and the spatial side should continue to use points, anchors, guides, lines, and polylines
- a concrete species-entry checklist can help keep the registry consistent: `species_id`, `display_name`, `habit/form`, `anatomy defaults`, `morphology notes`, `branching pattern`, `internode length`, `leaf distribution`, `growth rate`, `tropism rules`, `lifecycle tuning`, `allowed organs`, `debug labels`, and `journal hints`
- the current prototype targets should be treated as concrete morphological briefs: the tree-stump scaffold is a 3-5 cell-thick bark-textured support with a fork just below the hero, the vine family is a border-aware sprawling growth form with a thick main stem and smaller branches, and the monstera-like plant is a multi-stem growth area with big fenestrated leaves and species-specific lifecycle behavior
- example species entry: the Y-shaped tree-stump scaffold can be modeled as a woody support habit with a short trunk, one early fork below the hero, thick bark-textured segments, dead or minimal meristems, and a structural role rather than active canopy growth; this is an inference inspired by woody stem/meristem basics, not a claim that the scaffold is a real species
- the scaffold should be treated as a pre-rendered hero-support remnant in the main scene: a dead or senescent trunk-like form with one stable fork under the hero GIF, minimal lifecycle behavior, and only very light moss growth over the bark while vines can later attach and partially overgrow it
- the scaffold should not behave like a normal active plant: its lifecycle can be fixed or nearly fixed, with the primary requirement being structural stability, bark texture, and readable Y-shaped support geometry
- example species entry: the vine family can be modeled as a climbing or sprawling habit with an active apical meristem, long internodes, optional lateral meristems, border-awareness, and the ability to produce leaves, flowers, and possibly fruit along the main stem; this is an inference inspired by climbing plant and node/internode basics
- example species entry: the monstera-like plant can be modeled as an aroid-style climbing or self-supporting form with multiple stems from one growth area, large fenestrated leaves, aerial/climbing tendencies, and a lifecycle that can produce flowers and fruit later in development; this is an inference inspired by Monstera morphology rather than a direct species copy
- a species-entry template can stay consistent by filling these fields for each prototype: `species_id`, `display_name`, `habit/form`, `support strategy`, `stem/axis plan`, `growth-tip behavior`, `branching pattern`, `internode range`, `leaf shape/distribution`, `organ outputs`, `life-state defaults`, `registry tags`, `journal hints`, and `inspection notes`
- a botanical species-template should keep a few fields separate from general registry data: `taxonomic inspiration`, `support habit`, `growth mode`, `leaf architecture`, `reproductive strategy`, `life cycle notes`, and `ecology cues`; this helps keep the registry grounded while still remaining a YAM-specific abstraction
- compact species template examples:
  - tree-stump scaffold: `species_id = yam.scaffold.stump_v1`, `display_name = bifurcated stump scaffold`, `habit/form = woody support`, `support strategy = fixed hero anchor`, `stem/axis plan = short trunk with one fork`, `growth-tip behavior = minimal/dead`, `branching pattern = one early Y-fork`, `leaf shape/distribution = none or negligible`, `organ outputs = structural only`, `life-state defaults = dead-or-senescent`, `registry tags = scaffold, support, hero-anchor, bark`, `inspection notes = pre-rendered, stable fork, moss-only lifecycle, vine attachment zones`
  - vine family: `species_id = yam.vine.border_v1`, `display_name = border-aware vine family`, `habit/form = climbing/sprawling`, `support strategy = border-aware guide-following`, `stem/axis plan = long main stem plus optional laterals`, `growth-tip behavior = active apical meristem`, `branching pattern = opportunistic side branches`, `leaf shape/distribution = repeat along stem`, `organ outputs = leaf, flower, fruit optional`, `life-state defaults = dynamic`, `registry tags = vine, climbing, border-aware, guided`, `inspection notes = gravity response, wall avoidance, mask interaction, thick main stem`
  - monstera-like plant: `species_id = yam.floral.monstera_like_v1`, `display_name = fenestrated multi-stem flora`, `habit/form = aroid-like climber or self-supporting multi-stem`, `support strategy = multiple stems from one growth area`, `stem/axis plan = several axes from a base`, `growth-tip behavior = active and species-specific`, `branching pattern = sparse to moderate`, `leaf shape/distribution = large fenestrated leaves`, `organ outputs = leaf, flower, fruit possible`, `life-state defaults = active growth`, `registry tags = monstera-like, fenestrated, multi-stem, aroid-inspired`, `inspection notes = leaf size, perforation, stem count, lifecycle pacing`
- example species entry: the Y-shaped tree-stump scaffold can be modeled as a woody support habit with a short trunk, one early fork below the hero, thick bark-textured segments, dead or minimal meristems, and a structural role rather than active canopy growth; this is an inference inspired by woody stem/meristem basics, not a claim that the scaffold is a real species
- example species entry: the vine family can be modeled as a climbing or sprawling habit with an active apical meristem, long internodes, optional lateral meristems, border-awareness, and the ability to produce leaves, flowers, and possibly fruit along the main stem; this is an inference inspired by climbing plant and node/internode basics
- example species entry: the monstera-like plant can be modeled as an aroid-style climbing or self-supporting form with multiple stems from one growth area, large fenestrated leaves, aerial/climbing tendencies, and a lifecycle that can produce flowers and fruit later in development; this is an inference inspired by Monstera morphology rather than a direct species copy
- the main scene is the live visualiser/screensaver composition: hero GIF, tree-stump hero scaffolding with a Y-shaped fork under the hero, clock widget, weather widget, and procedurally generated vines that frame the composition organically
- the greenhouse is the early conceptual multi-room simulation space: rooms, labs, pots, bowls, and controlled-environment biome themes for developing and simulating plant lifecycles
- current flora prototypes include tree-stump scaffolding pre-generated at boot, tropical framing vines, and a monstera-like plant with large aesthetic growing leaves
- linework guides are rendered through a shared Bresenham-style geometry layer plus a glyph-appearance layer, following [`docs/soft-line-atlas.md`](soft-line-atlas.md) for shallow/stroke transitions and longer world-spanning lines, not with filled blocks or raster masks; the engine target is universal line coverage across the full YAM world size, using the grammar key `LineFamily -> LengthBucket -> Direction -> PhaseRole -> LocalStep`
- the hero frame pipeline currently uses Chafa with `--color-space=rgb`, `--color-extractor=average`, and `--dither=none` so dark reds are preserved by the conversion step before any pixel-side correction, and `hero_layer` preserves the styled spans when it copies the frame into the scene grid so the hero does not collapse to monochrome text. The ditherit-style braille/source-color trial is documented only as a historical experiment because it improved red retention but introduced unacceptable blocking and edge smearing in the face area
- the debug overlay may also show passive scrollbar indicators for camera/world position, anchored to the outermost terminal row/column, rendered as a minimal dark-blue gauge using `┄`/`═` horizontally and `┊`/`║` vertically, and derived from `RenderState` camera origins normalized across their full world range
- the debug overlay may also expose a dev-only blinking pointer probe that moves with arrow keys while enabled and reports its absolute world position in the debug info panel, so future masking and offset debugging can read a precise world-space point
- the debug overlay may also temporarily draw a faint soft-line probe for linework testing, using the atlas in [`docs/soft-line-atlas.md`](soft-line-atlas.md) rather than raster masks, so the guide grammar can be evaluated against real world coordinates before vines or other world annotations consume it; the same atlas also defines the longer slope families used to cover full-world line spans, and the live debug surface now renders visible `GuideState` linework through the same helper
- the debug/info surface stays compact and reports only the live control facts that matter during resize and entity-edit checks: FPS, frame, play state, camera mode, move mode/target, pointer probe state/absolute position, camera position, hero world/screen position, hero visibility, clock world/screen position, and clock visibility
- the dev-mode footer stays compact and uses `[h]otkeys` to open the modal hotkeys popup, where camera centering, the pointer probe, and other developer controls are described
- `[C]` stores the current camera position as the dev-mode camera home, and `[c]` recalls that stored home without switching into follow-hero mode
- `[p]` toggles the dev-only pointer probe, and its arrow-key motion is a probe/debug aid rather than a world or camera mode
- the hotkeys popup is a modal overlay at `z_index = 390`, between passive debug and move/settings, and it uses the shared modal shell to list the current developer controls without adding footer clutter
- the move popup is a modal overlay at `z_index = 395`, between hotkeys and settings, and it uses the shared modal shell to make entity movement explicit with `1/2/3` selection and `hjkl` movement
- the settings popup is a modal overlay at `z_index = 400`, and it uses the same shared modal shell with tabbed sections for positions, widgets, gif, and theme values
- the runtime input loop already enforces the current modal gating in code: `dev_mode` is the master switch, `hotkeys`/`move`/`settings` are mutually exclusive modal surfaces, pointer probe motion is only active in dev mode, and camera-home/pointer actions are blocked unless their dev state is open
- the dev-mode footer also uses `[m]ove` to open the modal move popup, where `1/2/3` select the active entity target and `hjkl` move that target while the popup is open

## UI / Metamechanics Working Set

- current state: `dev_mode` is the umbrella toggle, `h` opens hotkeys, `m` opens move, and `s` opens settings
- current move grammar: `1/2/3` select the active target, `hjkl` move that target, and the popup itself stays modal
- current settings grammar: tabbed positions/widgets/gif/theme controls stay presentation-only inside the modal popup
- current modal surface: move/settings panels use an opaque BTAS backdrop so the hero GIF does not bleed through
- current camera split: the screenshot-aligned manual boot seed `(-63, -17)` is distinct from the centered `follow-hero` runtime path, and the dev-mode camera-home controls now store and recall a user-chosen manual position
- current guide model: `WorldState` owns `GuideState`, which stores linework-only world-space annotations for future vines, masks, rulers, and debug visualization; sprites and solid masks are explicitly future work; guides have labels and optional groups, and `GuideState` now also stores named `GuideSet` collections so larger sets can be queried or edited as collections; new sets should be created with `GuideSet::new(...)` and registered through `GuideState::add_set(...)`
- current guide line grammar: soft linework is rendered through [`docs/soft-line-atlas.md`](soft-line-atlas.md) with a Bresenham-style geometry layer plus a glyph-appearance layer, plus slope-aware stroke selection and `|` / `:`-style vertical emphasis for guide axes and rulers; curves and turns are constructed as connected segments rather than filled shapes
- current pointer probe: `p` toggles a dev-only blinking world-space pointer that can be moved with arrow keys and is surfaced as an absolute position in the debug info panel
- resume point later: the next UI work should either add editable settings values or refine the camera-home flow, but not add a new authority layer
- if this block changes, update the matching rendering note and the UI-related backlog items together

## Out of Scope

- This document does not define conceptual scene categories; see [`scene-model.md`](scene-model.md).
- This document does not define numeric layer meaning in detail; see [`rendering.md`](rendering.md).

## Active Layers

- field/background: `z_index = 0`
- hero/entity: `z_index = 10`
- clock/world entity: `z_index = 100`
- debug overlay: `z_index = 300`
- hotkeys popup: `z_index = 390`
- move popup: `z_index = 395`
- settings popup: `z_index = 400`
- status/footer: `z_index = 1000`

## Presentation Contract

- world systems are the scene and world-attached content
- HUD systems are screen-attached footers, indicators, and passive debug
- overlay systems are modal or top-z-index panels such as settings or active debug UI
- terminal layout should be treated as a framebuffer, not a panel dashboard

## Coordinate Contract

The intended model is:

- datum/origin: `(0, 0)`
- world space is Euclidean and centered on the datum
- the world quadrants are sign-defined around that datum:
  - top-left: `(-x, -y)`
  - top-right: `(x, -y)`
  - bottom-left: `(-x, y)`
  - bottom-right: `(x, y)`
- world coordinates use Cartesian orientation: `x` increases to the right, `y` increases upward
- the world datum is the shared absolute reference point for guides, anchors, masks, and organism behavior
- terminal/screen coordinates use the usual terminal orientation: `x` increases to the right, `y` increases downward
- world space: simulation/object positions
- camera: world-space origin of the visible crop, not the viewport itself
- viewport: terminal-sized crop rectangle, not the camera itself
- static full-screen world size: `212x56`
- terminal full-screen frame: `212x57`, with the bottom row reserved for the footer
- default startup camera state is manual pan with the stored seed `(-63, -17)` so the boot view starts with the screenshot-aligned hero/clock composition from boot; this manual seed is distinct from the centered `follow-hero` runtime path
- default follow-hero camera crops are centered on the world datum across resizes once follow-hero mode is enabled; manual pan mode is clamped to one cell of overscan beyond the world border/frame
- the centered `124x32` follow-hero crop starts at camera `(-62, -16)`
- anchor space: offsets relative to another rendered object
- screen space: fixed terminal overlay positions
- world border and HUD border each keep a 1-cell inset where needed for symmetry and future UI placement, and HUD layout may still consult the shared world-spacing model for alignment rules even though it remains screen-attached at render time
- world-ui elements stay tied to world entities and follow the world contract
- hud-ui elements stay tied to viewport/camera/terminal position and follow the screen contract
- clock is treated as a world entity: it stays tied to the hero in world space and carries its own hero-relative offset
- footer/status is treated as hud-ui: it lives in screen space alongside hotkeys and version/build labels
- the repo now exposes explicit helpers for both sides of that split:
- `resolve_world_ui(...)` resolves anchor + offset in world space and stays world-pinned
- `resolve_hud_ui(...)` keeps hud values screen-attached and camera-independent, even when their spacing/alignment logic is derived from the shared world model
- the long-term goal is a single spatial relation resolver that can serve world datum guides, relative anchors, masks, and lifecycle-driven movement without each feature inventing its own attachment math
- the smallest useful canonical spatial relation layer now owns four things first: datum/world transforms, attachment resolution, guide/guide-set lookup, and screen projection helpers; higher-level mask and organism relations can be layered on later without forcing the first cut to solve every spatial question at once
- the lowest-risk extraction plan is likely:
  - `scene/coords.rs` keeps world/screen coordinate primitives and the basic transform helpers until the new layer is stable
  - `scene/entity.rs` keeps attachment composition helpers while entity-specific pose math is still simple
  - `core/guide.rs` keeps guide data and guide-set lookup as the canonical guide store
  - `render/guide.rs` stays render-only and consumes the guide store through projection helpers instead of redefining relations
  - the new canonical spatial relation layer should eventually absorb only the shared resolver logic, not the render helpers or the raw data structs on day one
- the first canonical spatial API surface is now present and should stay small and explicit:
  - `SpatialPoint` for world-space coordinates
  - `SpatialAnchor` for attachment origins
  - `SpatialAttachment` for anchor-plus-offset resolution
  - `SpatialProjection` for world-to-screen and screen-to-world helpers
  - `SpatialGuideIndex` for guide and guide-set lookup
  - `SpatialResolver` for the shared relation glue that ties those pieces together
- the likely mapping from today’s modules to that surface is:
  - `scene/coords.rs` -> `SpatialPoint`, `SpatialAnchor`, `SpatialAttachment`, and `SpatialProjection`
  - `scene/entity.rs` -> attachment composition helpers that can later collapse into `SpatialAttachment`
  - `core/guide.rs` -> `SpatialGuideIndex` and the raw guide/guide-set data model
  - `render/guide.rs` -> render-only consumers of `SpatialGuideIndex` and `SpatialProjection`
  - the future `core/spatial` module -> `SpatialResolver` and any shared relation logic that should not live in render code
- the safest migration order is likely:
  1. introduce `core/spatial` with the new shared types and resolver helpers, while keeping the old modules as compatibility shims
  2. move shared projection and attachment math into the new layer without changing the visible contracts
  3. switch guide lookup consumers to the new `SpatialGuideIndex` surface while preserving `GuideState` storage
  4. redirect render guide drawing through the new projection helpers
  5. only then retire the old helper paths once the tests and docs are all pointing at the new canonical layer
- each migration step should be guarded by the existing projection, resize, guide-set, and render-determinism tests before the old helper is removed
- the footer row is intentionally the bottom terminal row of the full terminal frame, while the world itself occupies the `212x56` playfield above it; `footer_row(height)` encodes that contract
- projection is defined in `docs/scene-model.md` and applied by the renderer

## Hero Geometry Contract

- source hero GIF: square `820x820` pixels
- terminal render target: fixed `96x48` cells
- decoded GIF subimage frames are expanded and flattened to an opaque full `820x820` logical canvas before terminal conversion
- the target is a layout/scaling result, not a raw pixel-to-cell division
- hero world anchor: `(0, 0)` when centered in world space
- hero visual center should cross the datum, while the rendered cell footprint remains `96x48`
- the world retains a 1-cell inset boundary for world-ui border work, and the active HUD/border layout also preserves a 1-cell inset for overlay/UI work
- world-ui should not be repositioned by camera semantics after it is anchored in world space
- hud-ui should not inherit world coordinates directly; it should use viewport/screen positioning while still borrowing the shared spacing model for consistent offsets, insets, and alignment
- the debug border probe is a datum-centered world-border indicator that is rendered in world space and therefore moves with camera panning

The remaining architectural gap is that `coords::Space` is not yet the authoritative resolver for all placement paths. Camera semantics are intentionally treated as a viewport crop helper on the active path; new features should not invent a second meaning for camera or viewport.

## Known Architectural Debt

- Historical `Layer::render(...)` references are archive-only; the active layer API is `Layer::render_to_grid(...)`.
- Hero and clock layers read from the per-frame `RenderState`; on the active path they are world-pinned and do not move with camera projection.
- `coords::Space` exists but is not yet the authoritative position resolver.
- Masks are present but are still a probe, not a complete scene-wide occlusion policy.
