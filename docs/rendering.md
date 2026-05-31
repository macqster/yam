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
- L390 - help popup
- L395 - move strip
- L400 - settings popup
- L405 - quit-confirm popup
- L1000 - HUD/footer/status

## Screen Zones

The full terminal frame is divided into stable presentation zones:

- `main scene` - the `212x56` world playfield that carries the visualiser/screensaver composition, hero, flora, guides, and world-tied diagnostics
- `hud/footer` - the reserved bottom row of the `212x57` terminal frame, used for compact status, hints, and runtime mode reminders
- `debug/inspect` - world-tied diagnostic readouts and passive overlays that stay readable during normal use without becoming the footer
- `modal overlay` - centered, top-most panels such as help, move, settings, and quit-confirm that temporarily sit above both world and HUD

Rules:

- screen zones are stable; they should not move around between frames unless the terminal size changes
- the main scene owns the visual composition and all world-tied assets
- the sandbox world reuses the same render stack and projection helpers as the main scene, but keeps intentionally sparse content so drawing-engine trials can be judged without main-scene composition noise
- `yam-sandbox` is the direct launch path for that sparse world, so dry-trial drawing and pointer work do not depend on entering the main scene first
- sandbox is not a separate app shell: it is an internal YAM space that should be reachable by world switching, and the main-scene hero/clock composition should stay out of it so the sandbox can read like a clean room rather than an emptied scene with leftover props
- switchable runtime worlds are selected and described through `WorldKind::SELECTABLE` and `WorldKind::profile()`; render layers may branch by world profile/composition/capabilities, but they should not own the world-switching order, transition labels, grid, camera, guide, or population policy
- the hud/footer owns screen-attached status, hints, and mode reminders
- debug and inspect outputs may be visible during normal use, but they must not displace the footer contract
- modal overlays may cover any zone while active, but they remain explicit and bounded

## Visible Content Map

- main scene: hero GIF, tree-stump scaffolding with a Y-shaped fork under the hero, vines, flora, guides, weather/clock composition, and world-tied diagnostic geometry
- weather visuals should remain YAM-owned Ratatui rendering fed by normalized weather state; provider contracts and sprite-atlas rules live in [`weather-widget.md`](weather-widget.md)
- sandbox: sparse world-space drawing, guide, and pointer-authoring trials should render here when comparative spatial review is needed, without reintroducing dedicated palette or weather inspection packets into world-space
- hud/footer: compact mode hint, version stamp, and one-line runtime reminders only
- debug/inspect: coordinate readouts, camera/world position, probe state, entity detail, and other readable diagnostics; it may show numbers and labels, but not the main command vocabulary
- modal overlay: help, move, settings, palette inspection, weather atlas inspection, quit-confirm, and other temporary control surfaces that are opened intentionally

## Modal Vocabulary

The currently implemented modal vocabulary is intentionally small and grouped:

- `help` - the discoverability sheet for the current dev controls
- `move` - target selection and movement for world-attached entities
- `settings` - tabbed presentation/state inspection for positions, ui overlays, features, gif, and theme values
- `palette` - curated plus extracted BTAS/TNBA swatch inspection in a dedicated dev modal
- `weather` - comparative weather atlas inspection in a dedicated dev modal
- `pointer` - dev-only probe state, shown through the debug surface rather than as a standalone modal
- `camera home` - stored and recalled through the runtime keys, not through a separate overlay

Rules:

- help should describe the currently implemented dev controls, not a hypothetical full command catalog
- move should stay focused on target choice and explicit motion
- settings should stay tabbed and presentation-oriented
- palette and weather inspection should stay comparative and read-only, rather than turning those modals into editing surfaces
- the pointer probe and camera-home actions are dev-only helpers, not always-on HUD content

Rules:

- the footer stays short and always readable
- debug/inspect surfaces may be informative, but they should not become a second footer
- the main scene carries visual density; the HUD carries reminders; overlays carry actions
- any new visible element should be assigned to one of these regions before implementation
- the footer may show the current mode and the minimal runtime hint only
- debug/inspect may show state facts and labels, but not the full action menu
- in `dev_mode`, the active world should also be announced by a small centered top-row label such as `MAIN SCENE` or `SANDBOX`, so alternate spaces can be identified at a glance without depending on the debug panel text
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
- shared projection helpers on `RenderState` are the source of truth for telemetry values that must match visible layer placement, and companion screen helpers now project through `core::spatial::SpatialResolver` and return signed core spatial screen values rather than world-position aliases
- the clock is a world entity: debug/info panels report its projected screen position, but they do not define it
- guide primitives live in `WorldState` and may be projected or visualized by debug layers, but for now they are linework-only world-space annotations rather than raster masks or solid fills; sprites and solid masks stay future work
- the guide / line generator is project-wide, not vines-only: it is now used for guide drawing and should remain suitable for future mask edges, rulers, and other world annotations that need deterministic world-space coverage, and it must remain capable of generating any line in any direction across the full YAM world size
- the drawing engine sits one layer above raw line grammar: `render/drawing.rs` owns reusable path-stroke, glyph-stamp, checked signed-to-grid conversion, and occupancy-mask primitives so flora, mask edges, guide authoring, and lightweight UI accents can share one deterministic cell-writing contract
- linework rendering follows [`docs/soft-line-atlas.md`](soft-line-atlas.md), with a Bresenham-style geometry layer and a glyph-appearance layer, using a small slope-aware glyph grammar with `|` / `:` for vertical emphasis so rulers, vectors, and curves read as directional strokes instead of block fills; the engine target is universal line coverage across the full YAM world size, using the grammar key `LineFamily -> LengthBucket -> Direction -> PhaseRole -> CellBand -> LocalStep`
- the soft-line renderer is intended to cover every possible line in world space; ad hoc block-fills or special-case line escapes are not the target architecture
- the pointer probe is a practical guide-authoring tool: it can be used to record exact coordinates for points, guides, and masks, and the line renderer should make those recorded relations legible in world space; the term `nodes` is currently reserved for plant morphology/anatomy systems and should be treated as provisional until the spatial terminology is researched further
- the pointer probe remains a dev-only world-space capture instrument across both current worlds: it supports point-to-point drawing, guide authoring, and future mask outlines without becoming a permanent always-on main-scene overlay feature
- the renderer should prefer Cartesian and Euclidean reasoning for world-space line work because signed axes and direct distance logic make precise authoring easier to validate

## Flora Render Contract

Flora rendering is a visualization client of world/flora state, not a lifecycle
or species owner.

Rules:

- render layers may visualize derived flora geometry, projected guides, labels,
  and read-only inspection facts
- render layers must not own organism identity, lifecycle mutation,
  species-registry defaults, greenhouse room selection, or guide-following state
- flora geometry should be projected through the shared spatial resolver and
  drawn through reusable drawing primitives instead of family-specific cell
  write loops
- species-specific morphology should remain inspectable, but the detailed
  registry templates and candidate species prose belong in the owning docs
- vine-specific readiness lives in [`vines.md`](vines.md); greenhouse rooms,
  candidate organisms, fixtures, environments, labels, and creative prompts
  live in [`greenhouse-roadmap.md`](greenhouse-roadmap.md); terminology lives in
  [`glossary.md`](glossary.md)

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
- debug info panel: inspect the live pointer position, camera position, and projected entity facts through a small tabbed readout rather than one long mixed-purpose list
- help popup: remind the user that pointer, camera-home, and move/settings are the current dev controls, while staying context-aware enough to be opened from the main scene before `dev_mode` is enabled
- move strip: step selected world-attached entities when authoring placement relationships

Rules:

- capture should happen in world space first, not in HUD space
- the debug info panel should be the factual readout, not the authoring editor
- the debug info panel now groups facts into `runtime`, `hero`, `companions`, and `vines` tabs, with the tab row rendered directly beneath the top scrollbar band and `Tab` / `Shift+Tab` cycling those groups when the settings modal is not active; the intended split is `runtime` for session/control plus camera/pointer facts, `hero` for hero animation/placement facts, `companions` for clock/weather/date projection facts, and `vines` for guide/vine inspection plus the soft-band spatial probe readout
- the help popup should remain discoverability, not a second editor surface
- `?` may be opened from the plain main scene as well as dev mode, but the popup should separate always-available actions from dev-only controls instead of implying that all listed tools are live immediately
- move mode is for explicit world-attached positioning, not for hidden geometry mutation; its live grammar is `Tab` / `Shift+Tab` to cycle the visible target set and arrow keys to move the active target
- the still-reserved `calendar` seam may remain editable in settings, but it should stay out of the lightweight move/help surfaces until it has a live rendered role
- guide-set previews should be read-only by default and belong in the debug/inspect surface rather than the HUD footer

## Pipeline

- `runtime` receives input and ticks state
- all scene rendering passes through `render_scene`
- the common runtime path keeps one long-lived `Scene`; the plain `render_scene(...)` helper still exists as a compatibility seam for direct callers and tests
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
- the default startup camera state is manual pan with the stored seed `(-60, -15)` so the boot view starts with the current frozen screenshot-aligned hero/clock/date/weather composition; this manual seed is distinct from the centered `follow-hero` runtime path
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
- the first canonical spatial API surface stays narrow here too: `SpatialPoint`, signed `SpatialScreenPoint`, `SpatialAnchor`, `SpatialAttachment`, `SpatialProjection`, `SpatialGuideIndex`, `SpatialAnchorLookup`, and `SpatialResolver` should be enough for rendering to consume the shared relation layer without taking ownership of the raw spatial data model; the compatibility layer re-exports that signed screen type as `ScreenPos` for module-internal compatibility projections that may be off-screen, and the active companion, hero, debug, guide, and vine render paths now consume core spatial types directly
- the compatibility layer in `scene/coords.rs` now resolves `Space::Anchor(EntityId)` through `core::spatial::SpatialAnchorLookup` when an entity is present, so render-side anchor use can rely on entity-backed lookup without making scene projection code own anchor identity
- that anchor lookup is still a compatibility path, not the final canonical resolver; the long-term goal remains to move entity-backed relation logic fully into `core/spatial`
- the likely module mapping from render’s point of view is:
  - `scene/coords.rs` supplies compatibility projection helpers and transitional type names
  - `core/guide.rs` supplies the queryable guide index and guide sets
  - `render/guide.rs` stays render-only and consumes those primitives through `core::spatial`
  - `core/spatial` already owns the shared resolver first cut so render does not need to know relation details
- the safest migration order from the renderer’s point of view is:
  1. keep the current render output unchanged while the shared spatial layer appears
  2. move only relation math into the new resolver, not the grid composition logic
  3. keep guide drawing on the new guide index/projection API
  4. preserve the existing render-determinism and layer-order tests at each step
  5. retire the old helper calls only when the renderer no longer needs to know where the relation math lives
- fullscreen is a special case of the camera contract: when the viewport matches or exceeds the world extent, the visible crop should be static and centered on the world datum `(0, 0)`, even if debug controls still mutate the stored camera position
- fullscreen lock is now exercised in `build_render_state(...)`: the stored camera can still move, but the frame uses a datum-centered crop whenever the terminal fully covers the world extent
- `RenderState::clock_screen()` is the shared core-spatial signed projected clock position used by both the clock layer and the debug overlay
- `resolve_world_ui(...)` is the helper for world-attached elements that stay pinned in world space
- `resolve_hud_ui(...)` is the helper for screen-attached overlays
- `resolve_hud_ui(...)` is the helper for screen-attached overlays, including those whose layout rules are derived from world-spacing conventions
- `GuideState` in `core/guide.rs` is the queryable world-space guide store that future vines can use for linework primitives such as points, lines, polylines, and outline shapes; each guide is individually labeled and may also participate in an optional named group, and `GuideState` also carries named `GuideSet` collections so larger guide groups can be queried or edited as collections; guide sets are constructed with `GuideSet::new(...)` and registered through `GuideState::add_set(...)`
- footer placement is intentionally the bottom row of the full terminal frame via `footer_row(height)`, while the world playfield occupies the `212x56` area above it
- the footer is plain text on the bottom row, with no green background highlight, rendered in BTAS-grey, and the version stamp stays right-aligned
- the footer now uses a split compact layout: the main scene keeps `[q]uit • [d]ev` on the left and a visually quieter right-aligned version stamp, while alternate worlds may still pair `[?] help` with the status stamp instead of trying to carry more dev-only vocabulary inline
- the interaction contract is mode-driven and keyboard-first:
  - `normal` uses familiar navigation and toggle keys for day-to-day scene use
  - `inspect` focuses on selection, entity reading, and drill-down navigation
  - `debug` names the diagnostic surface family, while the currently implemented diagnostic overlays are shown through `dev` mode rather than as always-on normal-use chrome
  - `dev` gates editing, mutation, and simulation tooling behind explicit controls
  - `command palette` is the fallback for rare actions, search, and entity jumps
- `dev` mode currently exposes the concrete runtime hotkeys already implemented in code: `[?]` help, `[m]ove`, `[s]ettings`, `[p]ointer`, `[P]alette`, `[W]eather`, `[C]` store camera home, `[c]` recall camera home, and `F5` for font cycling when dev controls are enabled
- the mode-specific layout contract should stay stable so the footer and overlays do not become the primary discoverability surface; rare actions belong in the command palette or the modal hotkey shell, not in the always-on footer
- the debug overlay can include passive camera/world scrollbar indicators anchored to the outermost terminal row/column; they are read-only, derived from `RenderState`, rendered as a minimal dark-blue gauge using `┄`/`═` horizontally and `┊`/`║` vertically, and sized/positioned from camera origins normalized across the world range so they report camera/world placement rather than acting like a scrollable panel
- the debug overlay may also expose a dev-only blinking pointer probe that moves with arrow keys while enabled and reports its absolute world position in the debug info panel, so future masking and offset debugging can read a precise world-space point
- the debug overlay may also temporarily render a faint soft-line probe for linework testing, using [`docs/soft-line-atlas.md`](soft-line-atlas.md) rather than raster masks, so the guide grammar can be exercised against real world coordinates; that atlas also covers longer slope families for full-world lines and future guide/mask edge drawing, and the live debug surface now renders visible `GuideState` linework through the same helper; the current calibration pair is the mirrored `64x10` long-shallow target in both directions, and its visible cadence should read closer to `--''` at the lead-in and `__. -` near the exit than to a mostly-underscore ramp; punctuation in that family should also lean with the stroke direction inside the cell, so comma-like and apostrophe-like marks are used intentionally instead of generic filler punctuation; the current classifier also adds a coarse `CellBand` so glyph choice can reflect top/middle/bottom placement inside the cell, and that band is now derived from the stroke’s sub-cell position relative to the ideal segment; the debug info panel now exposes a soft-band readout for the canonical probe so the band classifier can be checked while tuning, but the latest screenshots still show several unresolved issues: the rendered line does not closely resemble the manual `64x10` reference, the shape still reads as segmented rather than smooth, the band readout does not yet map cleanly to the visible lean, and mirrored long-shallow probes still share too much of the same body rhythm; the long-shallow renderer therefore remains a calibration target rather than a final proven grammar
- the pointer probe is the preferred absolute coordinate reference for guide authoring and future vines placement work
- the debug info panel should stay compact and biased toward the live control facts needed for resize and entity-edit checks: the default `runtime` tab covers live session/control state plus camera/pointer facts, while `hero`, `companions`, and `vines` carry the denser entity-placement and spatial-inspection facts without forcing them all into one always-visible list
- the footer stays compact and role-separated instead of becoming a second command sheet: on the main scene it keeps quit/dev on the left and only the version stamp on the right so the frame weight stays calm, while the modal help popup carries the denser developer control list such as move, camera-home, pointer probe, palette, and weather tools
- `?` is a global modal/help shortcut across the app surface family: it may promote the help popup above peer dev surfaces rather than requiring the user to back out first, and it may also be opened from the plain main scene as the discoverability entry into `dev_mode`
- `Esc` is the global back/close key across dev surfaces: it should first cancel the top-most dev/modal interaction, including the quit-confirm surface when present
- layout-affecting dev edits remain live but unsaved until explicitly persisted: camera home, camera pan, companion offsets, selected UI/features toggles, and similar persisted controls mark the runtime state dirty instead of writing immediately
- `q` still quits immediately from a clean state, but if persisted state is dirty it first opens a quit-confirm modal with explicit save/discard/cancel paths so the accepted frozen values are never redefined silently
- leaving `dev_mode` closes any open hotkeys, move, or settings modal state so the modal stack cannot reappear latched when the dev surface is restored
- `[C]` stores the current camera position as the dev-mode camera home, and `[c]` recalls that stored home without switching into follow-hero mode
- `[p]` toggles the dev-only pointer probe, and its arrow-key motion is a probe/debug aid rather than a world or camera mode
- dev mode and settings-style presentation flags are metamechanics inputs; they are consumed by the scene layers, not rendered outside the pipeline
- the runtime input loop already enforces the current modal gating in code: `dev_mode` is the master switch, help/move/settings/palette/weather/quit-confirm are dev-facing modal surfaces with shared close behavior, pointer probe motion is only active in dev mode, and camera-home/pointer actions are blocked unless their dev state is open
- the settings popup is a modal overlay rendered in the overlay layer; it uses the shared modal shell with tabbed sections for positions, ui, features, gif, and theme values
- modal help/move/settings/quit-confirm overlays all share one centered shell that paints an opaque BTAS-style backdrop before text is written, so their controls stay readable over the scene and the popup family stays visually consistent
- compositor cells with a background color and a space glyph are treated as opaque backdrop writes, so modal overlays clear the GIF beneath them instead of tinting it through
- the help popup is a modal overlay rendered between debug and move/settings; it uses the shared modal shell to list the current developer controls without adding footer clutter, and when it is already open it should not spend body rows repeating the trivial “open help” affordance
- the move surface is a modal overlay rendered between hotkeys and settings; it now takes the form of a compact lower-band strip so the moved scene elements stay visible, and it uses `Tab` / `Shift+Tab` to cycle targets with arrow keys for movement
- the quit-confirm popup is a modal overlay rendered above settings and below loading; it uses the shared modal shell to make dirty persisted-state exits explicit with a centered decision footer: `[s]ave and quit • [d]iscard and quit • ⎋ cancel`
- the move strip is opened with `[m]`; while it is open, `Tab` / `Shift+Tab` select the active entity target and arrow keys move that target, but that control now lives in the help popup instead of the always-on footer
- the move strip shows the active target and keeps entity movement explicit instead of spreading more hotkeys into the footer
- the help popup now also lists the pointer probe, palette popup, and weather sprite popup so those dev-only tools stay discoverable without turning the footer into a full command legend

## UI / Metamechanics Working Set

- current state: the modal UI stack is `help` at `390`, `move` at `395`, `settings` at `400`, and `quit-confirm` at `405`
- current move grammar: `Tab` / `Shift+Tab` cycle the active target and arrow keys move it while the lower move strip stays open
- current settings grammar: positions/ui/features/gif/theme tabs stay presentation-oriented and do not own world state
- current modal surface: move/settings panels paint an opaque BTAS backdrop before the border and text are drawn, and opaque space+background cells clear the GIF underneath
- current camera split: the screenshot-aligned manual boot seed `(-60, -15)` is distinct from the centered `follow-hero` runtime path, and the dev-mode camera-home controls now store and recall a user-chosen manual position
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
- The first runtime-efficiency slice should make that direction concrete without changing visible hero output: treat the live `chafa` path as an offline compiler/refresh step, and aim for runtime startup to load a previously prepared `HeroFrameSet` cache instead of decoding the GIF, writing temp frame PNGs, and spawning `chafa` per frame inside `Hero::new()`.
- The safest migration order for that cache path is:
  1. define the serialized `HeroFrameSet` / `CellGrid` runtime shape
  2. add a cache loader that can hydrate hero frames without shelling out to `chafa`
  3. keep the current live `chafa` path as the cache builder / fallback while the cache format stabilizes
  4. only then flip normal runtime startup to prefer cached hero frames by default
- The acceptance bar for that first hero-cache migration should stay conservative:
  - startup should avoid the current GIF decode + temp-file + per-frame process-spawn cost on the common path
  - visible hero geometry, frame count, and color stability should match the current Chafa baseline closely enough that the existing hero/render tests still describe the expected output
  - the cache should stay runtime-local and deterministic rather than introducing a second live rendering authority
- The missing tooling layer is a structured per-cell `CellGrid`, not a raw ANSI editor: ANSI snapshots can be imported and exported, but manual and scripted corrections should operate on cells containing glyph, foreground color, optional background color, and mask/style metadata.
- The future compiler should evaluate hero rendering from two deliberately separate directions: a monochrome `2x4` braille shape pass that controls thresholding, dot packing, and optional error diffusion; and an independent color pass that controls source sampling, palette quantization, red-family protection, and frame-to-frame color stability.
- Pre-generated hero art should be treated as asset authoring, not as runtime terminal capture: cached frames may include semi-manual correction overlays, region-specific lifts, and per-frame/cell stabilization where that improves face readability, red retention, silhouette stability, or animation consistency.
- The preferred eventual workflow is `Chafa/custom backend -> ANSI or direct cells -> CellGrid -> scripted/manual patches -> HeroFrameSet -> scene grid`, with Chafa and custom braille experiments acting as compiler backends rather than live scene dependencies.
- The first render-loop allocation cleanup slices are now live: runtime keeps one long-lived `Scene`, reuses the boxed layer stack across frames, skips obviously closed modal/help/quit layers before asking them to allocate grids at all, reuses the final composed `Grid` across frames in the live runtime loop, and now has a reusable per-layer scratch-grid seam adopted by the simple always-active layers, the lightweight companion projection layers (`clock`, `weather`, `date`), the always-active hero layer, the debug overlay, and the vine layer. The next conservative step on that path is no longer “take another obvious layer,” but deciding whether any of those remaining draw paths should move to even cheaper specialized helpers without destabilizing the current layer contract.
- There is now also a narrow fast ASCII-only compositor write path for plain UI chrome. It preserves the existing “spaces style but do not overwrite symbols” contract, falls back to the general grapheme-aware writer for non-ASCII text, and is currently used only by obviously ASCII-bound always-on chrome such as the footer, world label, and debug-panel/tab labels.
- The first concrete runtime cache shape and migration note now live in [`docs/hero-cache.md`](hero-cache.md); that note should be kept narrow and runtime-facing, while broader renderer strategy still belongs here.
- The common hero startup path now prefers a prepared `HeroFrameSet` cache in the user runtime cache directory (`$XDG_CACHE_HOME/yam/` when available, otherwise `~/.cache/yam/`), and only falls back to the live GIF-decode + temp-frame + `chafa` compilation path when that cache is missing, stale, or invalid. That keeps the current Chafa baseline as the visual authority while finally moving ordinary startup onto the cheaper load-first path without dirtying the repo checkout.
- That fallback path is now intentionally non-fatal when `chafa` is unavailable: if no valid cache exists and the compiler backend cannot be spawned, the renderer returns an explicit placeholder frame instead of panicking, which keeps YAM alive on a fresh machine while still making the missing backend visible.
- Third-party ANSI editors are useful references, but current tools tend to split between CP437/limited-color manual editing, destructive image conversion, and non-editable terminal replay; none should be treated as the primary YAM editing surface unless it proves Unicode braille, truecolor, animation, and lossless cell round-tripping.
- REXPaint is viable through CrossOver as an optional manual editing node, but `.xp` should stay an interchange/export target rather than the YAM source of truth: REXPaint is CP437/font-atlas oriented, so braille glyphs require a controlled tile/font mapping and round-trip validation before edited frames can feed `HeroFrameSet`.
- The REXPaint experiment path is `decoded frame -> custom braille renderer or Chafa import -> CellGrid -> .xp export -> REXPaint edits -> .xp import -> CellGrid patches -> HeroFrameSet`; existing PNG-to-XP converters are useful references but should not define final braille or color fidelity.
- The Ansizalizer/ansipx experiment is also a generator reference, not a Chafa-equivalent path: `ansipx.Custom` maps brightness or variance into a character ramp, so even the full braille range produces density texture rather than Chafa-style `2x4` dot-mask geometry. Its useful lesson is that small glyph vocabularies and 256-color-safe ANSI may help editor compatibility, but they do not preserve the hero face/silhouette quality required for the active path.
- Any future custom braille backend must encode the actual `2x4` dot occupancy per terminal cell before assigning color; density-sorted braille ramps, including the tested Ansizalizer/ansipx full-braille ramp, should be treated as rejected for Chafa-like hero fidelity.
- The recent hero-rendering test run that produced the `hero-ansipx` preview artifacts was unsuccessful and remains a cautionary experiment only; it does not change the active Chafa-backed baseline.
- `write_string` remains the fully general grapheme-aware text path for mixed-width or non-ASCII strings, while `write_ascii_string` exists as the cheaper path for known ASCII-only chrome.
- `grid_to_lines` groups adjacent cells by style.
- Clock and weather attachment on the active path are world-pinned sibling companions: both follow the hero in world space and keep their own hero-relative offsets. They do not inherit camera, viewport, or terminal motion directly.
- Debug info that prints companion positions is reporting projected world-entity positions, not screen-attached UI placement.
- Debug world borders are rendered as a stable ASCII 2x2 datum-centered indicator in world space, so they move with camera panning and remain a debug view of the real world bounds. It keeps one top padding row and one side padding cell for symmetry, and those margins are intentional and reserved for future UI placement. The bottom one-row padding is currently occupied by the footer.
- The world itself keeps a 1-cell inset boundary, and the HUD/viewport overlay layer also keeps a 1-cell inset boundary where needed for future UI elements.

## Current Risks

- Legacy helper functions remain in `src/render/hero.rs`, but the active scene path uses layer grids and `LayerOutput`.
- Fullscreen lock should remain a structural invariant: the code should treat fullscreen as an immovable, datum-centered crop, not just a larger windowed viewport.

## Known Gaps

- Masking is limited to the probe behavior described above.
- There is no generalized scene-wide mask pipeline yet.
