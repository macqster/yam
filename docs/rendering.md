# YAM-RUST Render Contract

## Assertions

- Numeric layer ordering implements the conceptual layer model defined in [`scene-model.md`](scene-model.md).
- Debug border geometry is world-space data projected through the active camera.
- Hero frames must be fixed size before render.

## Change Impact

- If you change this, also review `docs/scene-model.md`, `docs/architecture.md`, and the frame-rendering tests.

Ratatui is an immediate-mode renderer.
Every frame is rebuilt from state and emitted as a complete terminal grid.

The active renderer treats ratatui as the final output adapter. Scene layers write into engine-owned `Grid` values first, and only the final composed grid is converted back into ratatui text.

## Layer Order

- L0 - world base/background
- L10 - world props and world-attached composition pieces
- L100 - world-tied companions and anchored world widgets
- L300 - world-tied debug/dev probes and diagnostic overlays
- L390 - hotkeys popup
- L395 - move popup
- L400 - settings popup
- L1000 - HUD/footer/status

## Screen Zones

The full terminal frame is divided into stable presentation zones:

- `main scene` - the `212x56` world playfield that carries the visualiser/screensaver composition, hero, flora, guides, and world-tied diagnostics
- `hud/footer` - the reserved bottom row of the `212x57` terminal frame, used for compact status, hints, and runtime mode reminders
- `debug/inspect` - world-tied diagnostic readouts and passive overlays that stay readable during normal use without becoming the footer
- `modal overlay` - centered, top-most panels such as hotkeys, move, and settings that temporarily sit above both world and HUD

Rules:

- screen zones are stable; they should not move around between frames unless the terminal size changes
- the main scene owns the visual composition and all world-tied assets
- the hud/footer owns screen-attached status, hints, and mode reminders
- debug and inspect outputs may be visible during normal use, but they must not displace the footer contract
- modal overlays may cover any zone while active, but they remain explicit and bounded

## Visible Content Map

- main scene: hero GIF, tree-stump scaffolding with a Y-shaped fork under the hero, vines, flora, guides, weather/clock composition, and world-tied diagnostic geometry
- hud/footer: compact mode hint, version stamp, and one-line runtime reminders only
- debug/inspect: coordinate readouts, camera/world position, probe state, entity detail, and other readable diagnostics; it may show numbers and labels, but not the main command vocabulary
- modal overlay: hotkeys, move, settings, command help, and temporary control surfaces that are opened intentionally

## Modal Vocabulary

The currently implemented modal vocabulary is intentionally small and grouped:

- `hotkeys` - the discoverability sheet for the current dev controls
- `move` - target selection and movement for world-attached entities
- `settings` - tabbed presentation/state inspection for positions, widgets, gif, and theme values
- `pointer` - dev-only probe state, shown through the debug surface rather than as a standalone modal
- `camera home` - stored and recalled through the runtime keys, not through a separate overlay

Rules:

- hotkeys should describe the currently implemented dev controls, not a hypothetical full command catalog
- move should stay focused on target choice and explicit motion
- settings should stay tabbed and presentation-oriented
- the pointer probe and camera-home actions are dev-only helpers, not always-on HUD content

Rules:

- the footer stays short and always readable
- debug/inspect surfaces may be informative, but they should not become a second footer
- the main scene carries visual density; the HUD carries reminders; overlays carry actions
- any new visible element should be assigned to one of these regions before implementation
- the footer may show the current mode and the minimal runtime hint only
- debug/inspect may show state facts and labels, but not the full action menu
- modal overlays may show the denser key vocabulary, but only while active

## Rules

- layers must emit `LayerOutput`
- `LayerOutput.grid` is the layer proposal for the full frame
- `LayerOutput.mask` is optional compositor data
- `Scene` sorts layers by `z_index`
- `merge_grid` is the only active cell merge path
- overwrite priority is strictly numeric: higher `z_index` layers may overwrite lower ones within the same composed frame
- domain precedence is fixed: world layers compose before HUD, and HUD composes before modal overlays
- world-tied debug/dev assets belong to the world stack, not the HUD stack, even when they present diagnostic UI
- HUD layers are screen-attached and may overwrite world layers, but they must not be treated as world state
- modal overlays are always top-most among visible layers and may overwrite both world and HUD output
- masks only gate writes where the compositor explicitly consumes them; they do not create a second ordering rule
- final output is rendered once through `Paragraph::new(grid_to_lines(&final_grid))`
- no layer should rely on ratatui layout wrapping for hero/image content
- viewport selection is now a full-frame pass; the old centered tiered viewport box no longer drives layer placement
- `RenderState` is split into `world` and `hud` sections to keep world-pinned attachments separate from screen-attached overlays
- shared projection helpers on `RenderState` are the source of truth for telemetry values that must match visible layer placement
- the clock is a world entity: debug/info panels report its projected screen position, but they do not define it
- guide primitives live in `WorldState` and may be projected or visualized by debug layers, but for now they are linework-only world-space annotations rather than raster masks or solid fills; sprites and solid masks stay future work
- the guide / line generator is project-wide, not vines-only: it is now used for guide drawing and should remain suitable for future mask edges, rulers, and other world annotations that need deterministic world-space coverage, and it must remain capable of generating any line in any direction across the full YAM world size
- linework rendering follows [`docs/soft-line-atlas.md`](soft-line-atlas.md), with a Bresenham-style geometry layer and a glyph-appearance layer, using a small slope-aware glyph grammar with `|` / `:` for vertical emphasis so rulers, vectors, and curves read as directional strokes instead of block fills; the engine target is universal line coverage across the full YAM world size, using the grammar key `LineFamily -> LengthBucket -> Direction -> PhaseRole -> CellBand -> LocalStep`
- the soft-line renderer is intended to cover every possible line in world space; ad hoc block-fills or special-case line escapes are not the target architecture
- the pointer probe is a practical guide-authoring tool: it can be used to record exact coordinates for points, guides, and masks, and the line renderer should make those recorded relations legible in world space; the term `nodes` is currently reserved for plant morphology/anatomy systems and should be treated as provisional until the spatial terminology is researched further
- the renderer should prefer Cartesian and Euclidean reasoning for world-space line work because signed axes and direct distance logic make precise authoring easier to validate

## Flora Model Contract

The first flora implementation should stay small, inspectable, and graph-based:

- tree-stump scaffold: a simple Y-shaped support structure beneath the hero, with a small number of thick structural segments and minimal or dead growth tips
- vine family: a guide-following growth program with one or more axes, expected to use captured world-space guides as growth paths
- monstera-like plant: a thicker axis with large leaf organs and sparse branching

Rules:

- flora are growth programs that emit geometry
- each species may vary in branching pattern, internode length, leaf distribution, growth rate, and tropism rules
- the initial model should favor metamers, meristems, and axes over one-off geometry hacks
- the project's plant-structure thinking has historical inspiration from `cbonsai`, and that lineage should remain visible as a useful constraint even as YAM grows beyond a single bonsai-style plant into a broader greenhouse and multi-species model
- inspiration lineage for the current plant/UI thinking can stay short and explicit:
  - `cbonsai` for compact plant growth, branching, and readable terminal botany
  - `Dwarf Fortress` for ASCII-era muscle memory, dense inspectable state, and strong world-model thinking
  - `Cataclysm: DDA` for keyboard-first survival UI, modal discipline, and practical terminal ergonomics
- species-specific morphology should remain inspectable in debug/inspect surfaces
- a minimal Rust-shaped plant model can stay small and explicit: `Plant` owns species and lifecycle state, `Axis` owns a growth branch and ordered metamers, `Metamer` carries one internode and organ attachments, `Meristem` represents an active growth point, and `Organ` covers leaf/flower/fruit/branch outputs; geometry should be derived from this state rather than owning it
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
- Lua scripting could be used as a bounded optional layer for species authoring or debug/dev plugins, but it should stay Rust-hosted and should not own the canonical world state, lifecycle authority, or render path
- the registry taxonomy can be kept explicit by grouping fields into anatomy defaults, growth rules, visual phenotype ranges, lifecycle tuning, and debug labels; this makes the same registry useful for generation, emulation, and inspection without overloading one field bucket
- botanical basics should remain a reference point for species design: if a morphology term, growth rule, or anatomy field is unclear, the registry should be informed by real plant structure before the term is committed to YAM
- terminology authority should be explicit:
  - `strict` terms are botanically grounded and should match standard plant meaning as closely as practical
  - `inferred` terms are YAM-specific design interpretations built from real plant structure and terminal constraints
  - `provisional` terms are placeholders that may change once the plant model or spatial terminology is researched further
- plant-language references live in [`docs/glossary.md`](glossary.md); this contract only needs the YAM-specific rule that `node` stays reserved for plant morphology/anatomy and the spatial side should continue to use points, anchors, guides, lines, and polylines
- a concrete species-entry checklist can help keep the registry consistent: `species_id`, `display_name`, `habit/form`, `anatomy defaults`, `morphology notes`, `branching pattern`, `internode length`, `leaf distribution`, `growth rate`, `tropism rules`, `lifecycle tuning`, `allowed organs`, `debug labels`, and `journal hints`
- example species entry: the Y-shaped tree-stump scaffold can be modeled as a woody support habit with a short trunk, one early fork below the hero, thick bark-textured segments, dead or minimal meristems, and a structural role rather than active canopy growth; this is an inference inspired by woody stem/meristem basics, not a claim that the scaffold is a real species
- the scaffold should be treated as a pre-rendered hero-support remnant in the main scene: a dead or senescent trunk-like form with one stable fork under the hero GIF, minimal lifecycle behavior, and only very light moss growth over the bark while vines can later attach and partially overgrow it
- the scaffold should not behave like a normal active plant: its lifecycle can be fixed or nearly fixed, with the primary requirement being structural stability, bark texture, and readable Y-shaped support geometry
- example species entry: the vine family can be modeled as a climbing or sprawling habit with an active apical meristem, long internodes, optional lateral meristems, border-awareness, and the ability to produce leaves, flowers, and possibly fruit along the main stem; this is an inference inspired by climbing plant and node/internode basics
- example species entry: the monstera-like plant can be modeled as an aroid-style climbing or self-supporting form with multiple stems from one growth area, large fenestrated leaves, aerial/climbing tendencies, and a lifecycle that can produce flowers and fruit later in development; this is an inference inspired by Monstera morphology rather than a direct species copy
- the current prototype targets should be treated as concrete morphological briefs: the tree-stump scaffold is a 3-5 cell-thick bark-textured support with a fork just below the hero, the vine family is a border-aware sprawling growth form with a thick main stem and smaller branches, and the monstera-like plant is a multi-stem growth area with big fenestrated leaves and species-specific lifecycle behavior
- a species-entry template can stay consistent by filling these fields for each prototype: `species_id`, `display_name`, `habit/form`, `support strategy`, `stem/axis plan`, `growth-tip behavior`, `branching pattern`, `internode range`, `leaf shape/distribution`, `organ outputs`, `life-state defaults`, `registry tags`, `journal hints`, and `inspection notes`
- a botanical species-template should keep a few fields separate from general registry data: `taxonomic inspiration`, `support habit`, `growth mode`, `leaf architecture`, `reproductive strategy`, `life cycle notes`, and `ecology cues`; this helps keep the registry grounded while still remaining a YAM-specific abstraction
- compact species template examples:
  - tree-stump scaffold: `species_id = yam.scaffold.stump_v1`, `display_name = bifurcated stump scaffold`, `habit/form = woody support`, `support strategy = fixed hero anchor`, `stem/axis plan = short trunk with one fork`, `growth-tip behavior = minimal/dead`, `branching pattern = one early Y-fork`, `leaf shape/distribution = none or negligible`, `organ outputs = structural only`, `life-state defaults = dead-or-senescent`, `registry tags = scaffold, support, hero-anchor, bark`, `inspection notes = pre-rendered, stable fork, moss-only lifecycle, vine attachment zones`
  - vine family: `species_id = yam.vine.border_v1`, `display_name = border-aware vine family`, `habit/form = climbing/sprawling`, `support strategy = border-aware guide-following`, `stem/axis plan = long main stem plus optional laterals`, `growth-tip behavior = active apical meristem`, `branching pattern = opportunistic side branches`, `leaf shape/distribution = repeat along stem`, `organ outputs = leaf, flower, fruit optional`, `life-state defaults = dynamic`, `registry tags = vine, climbing, border-aware, guided`, `inspection notes = gravity response, wall avoidance, mask interaction, thick main stem`
  - monstera-like plant: `species_id = yam.floral.monstera_like_v1`, `display_name = fenestrated multi-stem flora`, `habit/form = aroid-like climber or self-supporting multi-stem`, `support strategy = multiple stems from one growth area`, `stem/axis plan = several axes from a base`, `growth-tip behavior = active and species-specific`, `branching pattern = sparse to moderate`, `leaf shape/distribution = large fenestrated leaves`, `organ outputs = leaf, flower, fruit possible`, `life-state defaults = active growth`, `registry tags = monstera-like, fenestrated, multi-stem, aroid-inspired`, `inspection notes = leaf size, perforation, stem count, lifecycle pacing`

## Guide Capture Workflow

When the pointer probe is used for authoring:

1. capture exact world coordinates with the pointer
2. convert the captured coordinates into guide points or anchors, and reserve `nodes` for plant morphology/anatomy systems unless the terminology is formally expanded later
3. connect the captured coordinates into a line or polyline for movement and growth paths
4. derive masks from explicit captured outlines when a bounded region is required
5. render the captured relation with the soft-line engine so the line grammar can be verified visually

Rules:

- the pointer probe records coordinates
- the soft-line renderer validates the resulting geometry
- masks should come from captured outlines, not from ad hoc raster shapes
- any authoring UI should preserve the world-space datum contract while the capture is happening
- the UI should support at least a read-only preview list of guide sets and subsets so grouped geometry, such as a polyline outline for a mask shape, can be inspected without editing it

## Capture UI Contract

The current debug/dev surface for guide capture is:

- pointer probe: capture exact coordinates in world space
- debug info panel: inspect the live pointer position, camera position, and projected entity facts
- hotkeys popup: remind the user that pointer, camera-home, and move/settings are the current dev controls
- move popup: step selected world-attached entities when authoring placement relationships

Rules:

- capture should happen in world space first, not in HUD space
- the debug info panel should be the factual readout, not the authoring editor
- the hotkeys popup should remain discoverability, not a second editor surface
- move mode is for explicit world-attached positioning, not for hidden geometry mutation
- guide-set previews should be read-only by default and belong in the debug/inspect surface rather than the HUD footer

## Pipeline

- `runtime` receives input and ticks state
- all scene rendering passes through `render_scene`
- `render_scene` builds a temporary `Scene`
- `Scene::render` uses the full terminal area for viewport and viewport rect values
- `Scene::render` also computes a single read-only `RenderState` for hero/clock/debug values
- `Scene::render` builds `RenderState` through `build_render_state(...)`, which is covered by a resize-invariance test
- each layer writes to a full-frame `Grid`
- scene captures the hero mask, currently applying it only to field output
- scene merges all grids into `final_grid`
- scene clears the frame and draws final lines

## Current Camera Contract

The active implementation treats camera as a viewport crop helper:

- world positions are defined around the `(0, 0)` datum
- world space is Euclidean and centered on the datum
- the world quadrants are sign-defined around that datum
- world coordinates use Cartesian orientation (`y` increases upward)
- terminal/screen coordinates use terminal orientation (`y` increases downward)
- camera is the world-space origin of the visible crop
- viewport is the terminal-sized crop rectangle that follows camera
- the static full-screen world playfield is `212x56`
- the terminal full-screen frame is `212x57`, with the bottom row reserved for the footer
- `Viewport::from_camera` copies camera coordinates directly as the visible crop origin
- the default startup camera state is manual pan with the stored seed `(-63, -17)` so the boot view starts with the screenshot-aligned hero/clock composition from boot; this manual seed is distinct from the centered `follow-hero` runtime path
- follow-hero camera mode keeps the visible crop centered on the world datum across resizes once it is enabled; manual pan mode is still clamped in runtime state and in `build_render_state(...)` so the visible crop can overscan the world border/frame by at most one cell on any edge
- the centered `124x32` follow-hero crop uses camera `(-62, -16)`
- debug border sampling is a datum-centered world-space probe that is projected through the active camera; it is not HUD chrome
- world-ui layers attach to world entities and resolve before screen-space overlay work
- hud-ui layers attach to the viewport/camera/terminal frame and do not inherit world motion directly
- hud-ui layout may still consult the shared world-spacing model for consistent offsets, insets, and alignment, but the rendered result remains screen-attached
- the clock is a world entity: it follows the hero in world space and keeps its own relative offset
- the footer/status bar is hud-ui: it is screen-attached and does not inherit world motion
- world-ui features move only with world attachment/projection, while hud-ui features stay terminal-fixed
- the world datum is the shared absolute reference for rulers, guides, masks, and organism guidance; screen space remains a separate terminal projection layer
- the smallest canonical spatial layer should stay narrow at first: datum/world transforms, attachment resolution, guide-set lookup, and screen projection helpers are the minimum shared contract before masks and organism guidance become first-class relation types
- the first canonical spatial API surface stays narrow here too: `SpatialPoint`, `SpatialScreenPoint`, `SpatialAnchor`, `SpatialAttachment`, `SpatialProjection`, `SpatialGuideIndex`, and `SpatialResolver` should be enough for rendering to consume the shared relation layer without taking ownership of the raw spatial data model, and the debug guide renderer already consumes `SpatialGuideIndex` directly
- the compatibility layer in `scene/coords.rs` now resolves `Space::Anchor(EntityId)` through `WorldState` when an entity is present, so render-side anchor use can rely on entity-backed lookup even while the broader spatial layer stays on shims
- that anchor lookup is still a compatibility path, not the final canonical resolver; the long-term goal remains to move entity-backed relation logic fully into `core/spatial`
- the likely module mapping from render’s point of view is:
  - `scene/coords.rs` supplies the coordinate/projection primitives
  - `core/guide.rs` supplies the queryable guide index and guide sets
  - `render/guide.rs` stays render-only and consumes those primitives
  - `core/spatial` already owns the shared resolver first cut so render does not need to know relation details
- the safest migration order from the renderer’s point of view is:
  1. keep the current render output unchanged while the shared spatial layer appears
  2. move only relation math into the new resolver, not the grid composition logic
  3. switch guide drawing to consume the new guide index/projection API
  4. preserve the existing render-determinism and layer-order tests at each step
  5. retire the old helper calls only when the renderer no longer needs to know where the relation math lives
- fullscreen is a special case of the camera contract: when the viewport matches or exceeds the world extent, the visible crop should be static and centered on the world datum `(0, 0)`, even if debug controls still mutate the stored camera position
- fullscreen lock is now exercised in `build_render_state(...)`: the stored camera can still move, but the frame uses a datum-centered crop whenever the terminal fully covers the world extent
- `RenderState::clock_screen()` is the shared projected clock position used by both the clock layer and the debug overlay
- `resolve_world_ui(...)` is the helper for world-attached elements that stay pinned in world space
- `resolve_hud_ui(...)` is the helper for screen-attached overlays
- `resolve_hud_ui(...)` is the helper for screen-attached overlays, including those whose layout rules are derived from world-spacing conventions
- `GuideState` in `core/guide.rs` is the queryable world-space guide store that future vines can use for linework primitives such as points, lines, polylines, and outline shapes; each guide is individually labeled and may also participate in an optional named group, and `GuideState` also carries named `GuideSet` collections so larger guide groups can be queried or edited as collections; guide sets are constructed with `GuideSet::new(...)` and registered through `GuideState::add_set(...)`
- footer placement is intentionally the bottom row of the full terminal frame via `footer_row(height)`, while the world playfield occupies the `212x56` area above it
- the footer is plain text on the bottom row, with no green background highlight, rendered in BTAS-grey, and the version stamp stays right-aligned
- the default footer help is a compact `[q]uit • [d]ev` hint, and the dev-mode footer keeps the same compact punctuation style for the runtime controls
- the interaction contract is mode-driven and keyboard-first:
  - `normal` uses familiar navigation and toggle keys for day-to-day scene use
  - `inspect` focuses on selection, entity reading, and drill-down navigation
  - `debug` keeps safe diagnostic overlays available during normal use
  - `dev` gates editing, mutation, and simulation tooling behind explicit controls
  - `command palette` is the fallback for rare actions, search, and entity jumps
- `dev` mode currently exposes the concrete runtime hotkeys already implemented in code: `[h]otkeys`, `[m]ove`, `[s]ettings`, `[p]ointer`, `[C]` store camera home, `[c]` recall camera home, and `F5` for the debug info surface when dev controls are enabled
- the mode-specific layout contract should stay stable so the footer and overlays do not become the primary discoverability surface; rare actions belong in the command palette or the modal hotkey shell, not in the always-on footer
- the debug overlay can include passive camera/world scrollbar indicators anchored to the outermost terminal row/column; they are read-only, derived from `RenderState`, rendered as a minimal dark-blue gauge using `┄`/`═` horizontally and `┊`/`║` vertically, and sized/positioned from camera origins normalized across the world range so they report camera/world placement rather than acting like a scrollable panel
- the debug overlay may also expose a dev-only blinking pointer probe that moves with arrow keys while enabled and reports its absolute world position in the debug info panel, so future masking and offset debugging can read a precise world-space point
- the debug overlay may also temporarily render a faint soft-line probe for linework testing, using [`docs/soft-line-atlas.md`](soft-line-atlas.md) rather than raster masks, so the guide grammar can be exercised against real world coordinates; that atlas also covers longer slope families for full-world lines and future guide/mask edge drawing, and the live debug surface now renders visible `GuideState` linework through the same helper; the current calibration pair is the mirrored `64x10` long-shallow target in both directions, and its visible cadence should read closer to `--''` at the lead-in and `__. -` near the exit than to a mostly-underscore ramp; punctuation in that family should also lean with the stroke direction inside the cell, so comma-like and apostrophe-like marks are used intentionally instead of generic filler punctuation; the current classifier also adds a coarse `CellBand` so glyph choice can reflect top/middle/bottom placement inside the cell, and that band is now derived from the stroke’s sub-cell position relative to the ideal segment; the debug info panel now exposes a soft-band readout for the canonical probe so the band classifier can be checked while tuning, but the latest screenshots still show several unresolved issues: the rendered line does not closely resemble the manual `64x10` reference, the shape still reads as segmented rather than smooth, the band readout does not yet map cleanly to the visible lean, and mirrored long-shallow probes still share too much of the same body rhythm; the long-shallow renderer therefore remains a calibration target rather than a final proven grammar
- the pointer probe is the preferred absolute coordinate reference for guide authoring and future vines placement work
- the debug info panel stays compact and reports only the live control facts needed for resize and entity-edit checks: FPS, frame, play state, camera mode, move mode/target, pointer probe state/absolute position, camera position, hero world/screen position, hero visibility, clock world/screen position, and clock visibility
- the dev-mode footer stays compact and uses `[h]otkeys` to open the modal hotkeys popup, where camera centering, the pointer probe, and other developer controls are described
- leaving `dev_mode` closes any open hotkeys, move, or settings modal state so the modal stack cannot reappear latched when the dev surface is restored
- `[C]` stores the current camera position as the dev-mode camera home, and `[c]` recalls that stored home without switching into follow-hero mode
- `[p]` toggles the dev-only pointer probe, and its arrow-key motion is a probe/debug aid rather than a world or camera mode
- dev mode and settings-style presentation flags are metamechanics inputs; they are consumed by the scene layers, not rendered outside the pipeline
- the runtime input loop already enforces the current modal gating in code: `dev_mode` is the master switch, `hotkeys`/`move`/`settings` are mutually exclusive modal surfaces, pointer probe motion is only active in dev mode, and camera-home/pointer actions are blocked unless their dev state is open
- the settings popup is a modal overlay rendered in the overlay layer; it uses the shared modal shell with tabbed sections for positions, widgets, gif, and theme values
- modal hotkeys/move/settings overlays all share one centered shell that paints an opaque BTAS-style backdrop before text is written, so their controls stay readable over the scene and the popup family stays visually consistent
- compositor cells with a background color and a space glyph are treated as opaque backdrop writes, so modal overlays clear the GIF beneath them instead of tinting it through
- the hotkeys popup is a modal overlay rendered between debug and move/settings; it uses the shared modal shell to list the current developer controls without adding footer clutter
- the move popup is a modal overlay rendered between hotkeys and settings; it uses the shared modal shell to make entity movement explicit with `1/2/3` selection and `hjkl` movement
- the dev-mode footer also uses `[m]ove` to open the modal move popup; while it is open, `1/2/3` select the active entity target and `hjkl` move that target
- the move popup shows the active target and keeps entity movement explicit instead of spreading more hotkeys into the footer
- the hotkeys popup now also lists the pointer probe so the dev-only probe is discoverable without widening the footer again

## UI / Metamechanics Working Set

- current state: the modal UI stack is `hotkeys` at `390`, `move` at `395`, and `settings` at `400`
- current move grammar: `1/2/3` select the target and `hjkl` move it while move mode is open
- current settings grammar: positions/widgets/gif/theme tabs are presentation-only and do not own world state
- current modal surface: move/settings panels paint an opaque BTAS backdrop before the border and text are drawn, and opaque space+background cells clear the GIF underneath
- current camera split: the screenshot-aligned manual boot seed `(-63, -17)` is distinct from the centered `follow-hero` runtime path, and the dev-mode camera-home controls now store and recall a user-chosen manual position
- current pointer probe: `p` toggles a dev-only blinking world-space pointer that can be moved with arrow keys and is surfaced as an absolute position in the debug info panel
- resume point later: if UI work resumes, start by editing values in the settings popup or by refining the camera-home store/recall flow from the dev-mode controls
- this block should stay aligned with `docs/architecture.md` and the UI-related backlog entries

This is the contract the current code follows. It is intentionally narrower than the older projection notes in the research bundle, which discuss center-based camera framing.

## Masks

Mask values use compositor semantics:

- `true` means a top-layer write is allowed
- `false` means a top-layer write is blocked

Current mask behavior is intentionally limited. The hero layer can emit a silhouette mask, and the scene applies that mask only to the field layer as a verification probe. HUD and debug layers do not consume the hero mask.

## Text And Geometry Caveats

- Hero source GIF is `820x820` pixels and is rendered into a fixed `96x48` cell footprint.
- That `96x48` target is the current layout result used to preserve the GIF's proportions in terminal cell space.
- GIF subimage frames are expanded and flattened onto an opaque full `820x820` logical canvas before chafa rendering so partial frames, including frames 15 and 30, cannot stretch vertically.
- The hero frame pipeline currently uses Chafa with `--color-space=rgb`, `--color-extractor=average`, and `--dither=none` so dark reds are preserved by the conversion step before any pixel-side correction, and `hero_layer` preserves the styled spans when it copies the frame into the scene grid so the hero does not collapse to monochrome text. The ditherit-style braille/source-color trial is retained only as a documented experiment in the log because it improved red retention but introduced unacceptable blocking and edge smearing in the face area.
- `LachlanArthur/Braille-ASCII-Art` is a future renderer reference, not an active dependency: its browser implementation is useful for studying explicit `2x4` braille dot packing, threshold controls, and Floyd-Steinberg/Stucki/Atkinson-style error diffusion when comparing possible non-Chafa hero experiments.
- Chafa conversion writes each decoded image frame into a run-isolated temporary PNG batch directory, and that directory is removed when the batch finishes.
- Hero frames must remain fixed width and fixed height before render.
- Hero rendering must not use ratatui wrapping.
- Hero rendering uses the chafa-backed frame conversion path; cached-frame ownership remains a future migration option if measurable instability returns.
- Desired hero-rendering direction: keep the active Chafa path as the production baseline while moving future experiments toward an offline hero-frame compiler that emits a stable internal frame cache for runtime use.
- The missing tooling layer is a structured per-cell `CellGrid`, not a raw ANSI editor: ANSI snapshots can be imported and exported, but manual and scripted corrections should operate on cells containing glyph, foreground color, optional background color, and mask/style metadata.
- The future compiler should evaluate hero rendering from two deliberately separate directions: a monochrome `2x4` braille shape pass that controls thresholding, dot packing, and optional error diffusion; and an independent color pass that controls source sampling, palette quantization, red-family protection, and frame-to-frame color stability.
- Pre-generated hero art should be treated as asset authoring, not as runtime terminal capture: cached frames may include semi-manual correction overlays, region-specific lifts, and per-frame/cell stabilization where that improves face readability, red retention, silhouette stability, or animation consistency.
- The preferred eventual workflow is `Chafa/custom backend -> ANSI or direct cells -> CellGrid -> scripted/manual patches -> HeroFrameSet -> scene grid`, with Chafa and custom braille experiments acting as compiler backends rather than live scene dependencies.
- Third-party ANSI editors are useful references, but current tools tend to split between CP437/limited-color manual editing, destructive image conversion, and non-editable terminal replay; none should be treated as the primary YAM editing surface unless it proves Unicode braille, truecolor, animation, and lossless cell round-tripping.
- REXPaint is viable through CrossOver as an optional manual editing node, but `.xp` should stay an interchange/export target rather than the YAM source of truth: REXPaint is CP437/font-atlas oriented, so braille glyphs require a controlled tile/font mapping and round-trip validation before edited frames can feed `HeroFrameSet`.
- The REXPaint experiment path is `decoded frame -> custom braille renderer or Chafa import -> CellGrid -> .xp export -> REXPaint edits -> .xp import -> CellGrid patches -> HeroFrameSet`; existing PNG-to-XP converters are useful references but should not define final braille or color fidelity.
- The Ansizalizer/ansipx experiment is also a generator reference, not a Chafa-equivalent path: `ansipx.Custom` maps brightness or variance into a character ramp, so even the full braille range produces density texture rather than Chafa-style `2x4` dot-mask geometry. Its useful lesson is that small glyph vocabularies and 256-color-safe ANSI may help editor compatibility, but they do not preserve the hero face/silhouette quality required for the active path.
- Any future custom braille backend must encode the actual `2x4` dot occupancy per terminal cell before assigning color; density-sorted braille ramps, including the tested Ansizalizer/ansipx full-braille ramp, should be treated as rejected for Chafa-like hero fidelity.
- The recent hero-rendering test run that produced the `hero-ansipx` preview artifacts was unsuccessful and remains a cautionary experiment only; it does not change the active Chafa-backed baseline.
- `write_string` currently iterates `char`s, not display-width-aware graphemes.
- `grid_to_lines` groups adjacent cells by style.
- Clock attachment on the active path is world-pinned: the clock follows the hero in world space and keeps its own hero-relative offset. It does not inherit camera, viewport, or terminal motion directly.
- Debug info that prints clock position is reporting the projected world-entity clock position, not a screen-attached UI placement.
- Debug world borders are rendered as a stable ASCII 2x2 datum-centered indicator in world space, so they move with camera panning and remain a debug view of the real world bounds. It keeps one top padding row and one side padding cell for symmetry, and those margins are intentional and reserved for future UI placement. The bottom one-row padding is currently occupied by the footer.
- The world itself keeps a 1-cell inset boundary, and the HUD/viewport overlay layer also keeps a 1-cell inset boundary where needed for future UI elements.

## Current Risks

- Legacy helper functions remain in `src/render/hero.rs`, but the active scene path uses layer grids and `LayerOutput`.
- Fullscreen lock should remain a structural invariant: the code should treat fullscreen as an immovable, datum-centered crop, not just a larger windowed viewport.

## Known Gaps

- Masking is limited to the probe behavior described above.
- There is no generalized scene-wide mask pipeline yet.
