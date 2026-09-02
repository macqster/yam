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
- L10 - world props and world-attached composition pieces, including the `WorldKind::Greenhouse`-only room-bounds/fixture-marker render
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
- sandbox is not a separate app shell: it is an internal YAM space that should be reachable by world switching, and the main-scene hero/clock composition should stay out of it by default so the sandbox can read like a clean room rather than an emptied scene with leftover props
- scaffold and composition prototyping should now default to the sandbox first: the sandbox may selectively host the hero, companion widgets, and scaffold through UI-owned prototype visibility toggles while still reusing the same world/render/layer pipeline as the main scene
- switchable runtime worlds are selected and described through `WorldKind::SELECTABLE` and `WorldKind::profile()`; render layers may branch by world profile/composition/capabilities, but they should not own the world-switching order, transition labels, grid, camera, guide, or population policy
- the hud/footer owns screen-attached status, hints, and mode reminders
- debug and inspect outputs may be visible during normal use, but they must not displace the footer contract
- modal overlays may cover any zone while active, but they remain explicit and bounded

## Visible Content Map

- main scene: hero GIF, world-owned scaffold cradle, vines, flora, guides, weather/clock composition, and world-tied diagnostic geometry
- the live scaffold pass is intentionally narrow: a static rear support cradle/back-brace/leg-brace set plus a small foreground nesting edge, all rendered from world-owned scaffold data through read-only layers around the hero; future enrichment should follow [`main-scene-scaffold.md`](main-scene-scaffold.md), preserve the seated support read first, and keep any foreground lip composition-owned rather than decorative wallpaper
- the current scaffold visibility contract is scene-conscious: the main scene owns its own scaffold visibility policy, while sandbox prototype toggles may expose hero, companions, and scaffold there for comparison without creating a second scaffold owner or a render-owned truth
- weather visuals should remain YAM-owned Ratatui rendering fed by normalized weather state; provider contracts and sprite-atlas rules live in [`weather-widget.md`](weather-widget.md)
- sandbox: sparse world-space drawing, guide, and pointer-authoring trials should render here when comparative spatial review is needed, without reintroducing dedicated palette or weather inspection packets into world-space
- greenhouse: a minimal read-only room render (bounds outline plus fixture-anchor markers only, `scene/layers/greenhouse_layer.rs`), gated on `WorldKind::Greenhouse`; seedling simulation state and deterministic growth dispatch are live, while hero, companions, scaffold, and flora geometry/detail rendering remain absent from this world
- hud/footer: compact mode hint, version stamp, and one-line runtime reminders only
- debug/inspect: coordinate readouts, camera/world position, probe state, entity detail, and other readable diagnostics; it may show numbers and labels, but not the main command vocabulary
- modal overlay: help, move, settings, palette inspection, weather atlas inspection, quit-confirm, and other temporary control surfaces that are opened intentionally

## Modal Vocabulary

The currently implemented modal vocabulary is intentionally small and grouped:

- `help` - the discoverability sheet for the current dev controls
- `move` - target selection and movement for world-attached entities
- `settings` - tabbed presentation/state inspection for positions, ui overlays, runtime, features, gif, and theme values
- `palette` - curated plus extracted BTAS/TNBA swatch inspection in a dedicated dev modal
- `weather` - comparative weather atlas inspection in a dedicated dev modal
- `pointer` - dev-only probe state, shown through the debug surface rather than as a standalone modal
- `camera home` - stored and recalled through the runtime keys, not through a separate overlay

Rules:

- help should describe the currently implemented dev controls, not a hypothetical full command catalog
- move should stay focused on target choice and explicit motion
- settings should stay tabbed and presentation-oriented
- the runtime settings tab exposes one persisted render-FPS ceiling through the same selected-row control used by the other settings tabs; its supported values are `15`, `30`, `60`, and `120`, with `120` as the default
- changing the runtime value with Left/Right or Enter updates the live loop period without changing hero animation timing, world simulation cadence, input behavior, resize handling, DPMS state, or thermal policy
- palette and weather inspection should stay comparative and read-only, rather than turning those modals into editing surfaces
- the pointer probe and camera-home actions are dev-only helpers, not always-on HUD content

Rules:

- the footer stays short and always readable
- debug/inspect surfaces may be informative, but they should not become a second footer
- the main scene carries visual density; the HUD carries reminders; overlays carry actions
- any new visible element should be assigned to one of these regions before implementation
- if a future scaffold or occlusion experiment seems to need masking, prefer proving the shape first with ordinary world geometry and layer ordering; the current foreground nesting edge follows that rule, and masks should still be introduced only when a specific occlusion or nesting read cannot be expressed cleanly without them
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
- the dev settings `runtime` tab owns a persisted render-FPS ceiling with `15`, `30`, `60`, and `120` FPS presets; it controls the loop's sleep period without changing hero animation cadence, world tick cadence, input handling, resize handling, or DPMS state
- the runtime FPS control is a cadence ceiling, not a promise that every frame will be materially different: the immediate-mode scene is still recomputed and emitted at the selected cadence
- `120` preserves the historical animation target; `60` is the normal interactive compromise; `30` and `15` are lower-power options for deployments where animation smoothness is less important than reducing redraw frequency
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
- render-side anchor use consumes `core::spatial::SpatialAnchorLookup` directly for entity-backed lookup; the `scene::coords` compatibility layer that used to sit in front of it has been retired (2026-07-21) after a repo-wide audit found zero call sites outside its own tests
- the module mapping from render's point of view is:
  - `core/guide.rs` supplies the queryable guide index and guide sets
  - `render/guide.rs` stays render-only and consumes those primitives through `core::spatial`
  - `core/spatial` owns the shared resolver so render does not need to know relation details
- fullscreen is a special case of the camera contract: when the viewport matches or exceeds the world extent, the visible crop should be static and centered on the world datum `(0, 0)`, even if debug controls still mutate the stored camera position
- fullscreen lock is now exercised in `build_render_state(...)`: the stored camera can still move, but the frame uses a datum-centered crop whenever the terminal fully covers the world extent
- `RenderState::clock_screen()` is the shared core-spatial signed projected clock position used by both the clock layer and the debug overlay
- world-attached elements stay pinned in world space by composing anchor+offset through `core::spatial::SpatialResolver::resolve_anchor` (via `scene::entity`); screen-attached overlays are computed directly in screen space and never pass through world-to-screen projection, including those whose layout rules are derived from world-spacing conventions
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
- `Ctrl+C` is the unconditional interrupt and is checked before any mode dispatch, so it exits from the loading screen, any modal, and settings edit alike. It does not save and does not play the quit dissolve: an escape hatch that opens a confirmation modal is not an escape hatch, and raw mode means nothing else in the process will ever act on the terminal interrupt. `q` remains the graceful path; `Ctrl+C` is the one that must work when a surface is wedged
- leaving `dev_mode` closes any open hotkeys, move, or settings modal state so the modal stack cannot reappear latched when the dev surface is restored
- `[C]` stores the current camera position as the dev-mode camera home, and `[c]` recalls that stored home without switching into follow-hero mode
- `[p]` toggles the dev-only pointer probe, and its arrow-key motion is a probe/debug aid rather than a world or camera mode
- the dev-mode character chords are literal, with no shift qualifier: `{`/`}` cycle the clock font and `-`/`+` step the hero FPS. Shift-qualified variants of `=` and `-` were removed on 2026-09-02 because no keyboard enhancement flags are pushed, so crossterm reports the shifted character (`+`, `_`) rather than the base key with a SHIFT modifier, and those arms could never match
- dev mode and settings-style presentation flags are metamechanics inputs; they are consumed by the scene layers, not rendered outside the pipeline
- the runtime input loop already enforces the current modal gating in code: `dev_mode` is the master switch, help/move/settings/palette/weather/quit-confirm are dev-facing modal surfaces with shared close behavior, pointer probe motion is only active in dev mode, and camera-home/pointer actions are blocked unless their dev state is open
- the settings popup is a modal overlay rendered in the overlay layer; it uses the shared modal shell with tabbed sections for positions, ui, runtime, features, gif, and theme values
- on the `runtime` tab, Left and Right mean off and on for the boolean boot-phase rows rather than either key flipping the value, so a repeated keypress settles on a state instead of oscillating; row 0 keeps its stepping behavior for the render-FPS presets
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

- Hero fidelity direction, the acceptance bar for source or preset changes, and the offline package roadmap live in [hero-revision.md](hero-revision.md); the compiled package contract and its offline compiler in [hero-package.md](hero-package.md).
- Hero source assets are declared in `src/render/hero_source.rs`, not spread across constants, cache-name literals, and test literals. A `HeroSource` owns the asset path, its logical canvas size, its expected frame count, its requested cell footprint, its cache identity/revision, and its minimum frame-0 coverage. Adding or swapping hero art is a descriptor change; `hero_source::ALL` is the registry every hero source must be listed in.
- Two sources are registered:
  - `IVY_VECTOR` (`assets/hero_gif_2.gif`, `1080x1080`, 48 frames, requested at `96x48` cells) — the same character and pose cycle redrawn as flat vector art in Moho, added 2026-08-19 and promoted to `hero_source::DEFAULT` in 0.4.1.
  - `IVY` (`assets/hero_gif_1.gif`, `820x820`, 64 frames, requested at `96x48` cells) — the original cel-derived art, the default until 0.4.1. Still registered and still gated, so it stays reachable rather than becoming unreferenced art.
- Both entries are live art, which is what makes the swap reversible: an ordinary launch renders `IVY_VECTOR`, and `YAM_HERO_SOURCE=hero_gif_1` returns to the original without a rebuild.
- `YAM_HERO_SOURCE=<stem>` selects any registered source by stem for one launch (`hero_source::resolve_from_env`, read once in `Hero::new`). This is deliberately the smallest form of the selection surface — enough to look at candidate art in the running app, and to step back to previous art — not a settings contract. An unset or unknown stem resolves back to `DEFAULT`, because a typo in an env var should cost the reader their experiment rather than their hero. A world- or settings-owned selector, and any persistence of that choice, remain the next slice.
- The requested cell footprint is an upper bound, not a guarantee: chafa preserves source aspect, so the hero's actual footprint is measured from the rendered frames in `render::hero`, then hard-locked to frame 0's measurement so animation cannot shift hero geometry mid-loop.
- Startup prefers a compiled `HeroPackage` when one is present and provably built from the same source and preset, then the frame cache, then the live chafa path. Any validation failure falls through silently rather than erroring, because a package is an optional acceleration and the live path can always rebuild. See [hero-package.md](hero-package.md).
- Frame caches are per-source and revision-keyed (`<stem>.r<revision>.<w>x<h>.frame_cache.json`), so two sources cannot share a cache and renderer/serialized-contract changes cannot silently reuse older output. Freshness stays mtime-based against that source's own GIF; if the compile-time source path becomes unreachable after the build tree moves, an existing revision-matched cache remains usable instead of degrading to a placeholder.
- Four tests gate any asset swap, and all four iterate `hero_source::ALL` rather than only the default: `absent_color_is_actually_absent_from_every_source` fails when a source carries half a rendered cell's worth of pixels or more within 128 (Euclidean RGB) of its own `absent_color`, that being the distance below which chafa starts discarding art as already-painted. A source that overlaps its palette on purpose is listed in that test's `ACCEPTED_OVERLAP` with the pixel count the choice puts inside the radius, and has that number pinned instead of being required to separate — so the art stays guarded, because any drift in palette, radius, or chosen colour moves the count and fails. Both constants are derived rather than picked: 128 clears the ~111 that the measured per-channel drop threshold implies, and the half-cell significance floor exists because a colour thinner than half a cell is discarded by cell averaging whatever `absent_color` is. Without that floor the gate measures the GIF exporter's anti-aliasing fringe instead of the art — on `hero_gif_1` that fringe is 108 of 249 distinct colours, and it was what set the reported clearance before 0.4.3; `every_hero_source_matches_its_declared_geometry` fails when a descriptor's frame count or canvas size disagrees with the actual file, `hero_frames_keep_a_transparent_canvas_rather_than_a_matte` fails if the alpha contract is lost again, and `rendered_hero_frames_contain_real_content_not_placeholders` fails when a live render produces placeholder frames or collapses below that source's declared frame-0 coverage floor.
- The coverage floor is per-source (`HeroSource::min_frame0_coverage_percent`) because cell density is a property of the art, not of the pipeline: flat vector fills light fewer braille dots than textured cel shading at the same requested size. One shared floor would either wave a real collapse through on the dense asset or fail the sparse one merely for being drawn differently.
- Measured 2026-08-19 against chafa 1.18.2, through the test's own `covered_cells` helper: `hero_gif_1` frame 0 covers 1918/4608 cells (41.6%, floor 20%), `hero_gif_2` covers 923/4608 (20.0%, floor 10%). `hero_gif_2` is lower by choice, not by fault — see its `absent_color` above. Before the 0.4.2 `absent_color` fix the same measurement gave 932 (20.2%) and 706 (15.3%), when the pipeline was discarding a little over half of each hero without anyone choosing that.
- `hero_gif_2` is authored as ten flat colours (`ffffff`, `000000`, `bba381`, `9a7b59`, `b8170c`, `7c0307`, `7f6e87`, `5e4762`, `332a29`, `395f0b`). The exported GIF carries 212 distinct opaque colours: 93.8% of opaque pixels are those ten, and the remaining 6.16% is anti-aliasing fringe between the flat regions. Two of the ten never render, for reasons that are not the drop rule: `ffffff` has zero opaque pixels because white is the transparency index, and `395f0b` (the iris) peaks at 177 pixels in a frame against a 253-pixel cell, so cell averaging absorbs it. Nothing in the renderer recovers a sub-cell feature; it has to be larger in the art.
- That also reconciles the 41.5% figure this section used to cite for `hero_gif_1` and that a 2026-08-19 re-measurement could not reproduce: the post-fix number lands within 0.2 points of it. The most likely reading is that 41.5% was a real measurement of a pipeline without this defect, and the `--bg` value drifted afterwards rather than the figure being wrong. Treat that as inference, not a proven history — but the floor's original ~2x headroom is restored either way.
- That content test skips itself when `chafa` is not on `PATH`, because every live render there yields placeholders by design. CI installs `chafa` before verification, so the test is an enforced pipeline gate as well as a local gate on configured maintainer machines.
- GIF subimage frames are expanded onto that source's full logical canvas before chafa rendering so partial frames, including frames 15 and 30, cannot stretch vertically. The canvas is transparent (not an opaque matte fill): the source GIF carries real per-pixel alpha, and the pipeline preserves it end to end into the temp PNG chafa reads, instead of compositing every frame onto a flatten color. `assets/hero_gif_1.gif` was replaced 2026-07-22 with the alpha-carrying original that has been tracked in this repo all along as `tools/legacy-python/hero/assets/hero_go.gif`; the file it replaced was a flattened, alpha-stripped copy of the same art introduced by the first Rust hero commit, which is why the Rust renderer lost dark regions from its first day. See `docs/audit.md` for the full provenance trace.
- The hero frame pipeline uses Chafa with `--color-space=rgb`, `--color-extractor=average`, `--dither=none`, and `--fg-only`; `--bg` is derived from that source's `HeroSource::absent_color`. **`--bg` is not a background fill and is never painted under `--fg-only`** — verified: chafa emits zero background codes at every `--bg` value tested. It is the colour chafa treats as *already on screen*, so any art resembling it is judged redundant and dropped rather than drawn. [`chafa-drop-rule.md`](chafa-drop-rule.md) documents the mechanism, the measurement harness, and the procedure for choosing this value when new art is registered. How much of the palette it discards is therefore a per-asset decision, which is why it is a descriptor field rather than a global constant. The two registered sources use it for opposite purposes:
  - `IVY` uses `#00e000`, a colour absent from its palette, so nothing is discarded. Distance there is a trade rather than a maximum: too close and art drops, but on partially transparent cells the value bleeds into chafa's foreground pick and that bleed grows with distance. Measured: `#ffffff` (clearance 5) yields 0 off-palette cells but drops near-white highlights outright, `#00e000` (141) yields 8, `#00ff00` (170) yields 15, `#00f0b0` (176) yields 50. `#00e000` is the least clearance that still clears the drop radius.
  - `IVY_VECTOR` uses `#336699`, which deliberately overlaps its own palette, because for this asset the cull is wanted. Its ten flat colours render as fully-lit braille (`⣿`), so a faithful render is a solid mass; discarding the darkest tiers is what keeps the hero open. `#7c0307` and `#332a29` have identical RGB sums (134 each), so no neutral value separates them — a chromatically opposite one does, keeping the dark red while still dropping the leggings and line art. Nearest art colour is (104, 90, 110) at 69.
- `hero_layer` preserves the styled spans when it copies the frame into the scene grid so the hero does not collapse to monochrome text. Investigation (2026-07-22) found the prior `average` extractor, combined with the then-flattened opaque canvas, was dropping roughly 80% of the frame grid as "no coverage" rather than merely desaturating it: dark, low-contrast regions (not just dark reds) were being judged close enough to the assumed background to skip entirely. Preserving real source alpha plus switching to `median` fixed both problems together — `median` alone (kept against the old flattened canvas) recovered coverage by painting the true background too, which is not correct either. The extractor returned to `average` in 0.4.6, once the flattened canvas it had originally failed against was long gone: measured at the shipped backgrounds it renders exactly the same cells, frame for frame, at lower reconstruction error (`hero_gif_2` 122 to 112). The 2026-07-22 note is history, not current configuration. The ditherit-style braille/source-color trial remains documented in the log as a rejected alternative because it improved red retention but introduced unacceptable blocking and edge smearing in the face area.
- The decode path applies no per-pixel color correction. An undocumented `tone_lift_dark_reds` HSV step (lift value on hue `<=20`/`>=340`, saturation `>=0.45`, value `<=0.42`) ran on every pixel of every frame until 2026-07-29, when it was removed ahead of new hero art: it was tuned to one asset's palette, and measurement showed it moved only 3.27% of frame-0 pixels for a net effect of +1 covered cell and +14 unique foreground colors out of ~740 — inside the noise of the alpha/`median` fix that actually recovered dark regions. Reintroducing per-asset color correction should go through the `HeroSource` descriptor, not back into the shared decode loop.
- `LachlanArthur/Braille-ASCII-Art` is a future renderer reference, not an active dependency: its browser implementation is useful for studying explicit `2x4` braille dot packing, threshold controls, and Floyd-Steinberg/Stucki/Atkinson-style error diffusion when comparing possible non-Chafa hero experiments.
- Chafa conversion writes each decoded image frame into a run-isolated temporary PNG batch directory, and that directory is removed when the batch finishes.
- Hero frames must remain fixed width and fixed height before render.
- Hero rendering must not use ratatui wrapping.
- Hero startup now prefers a validated `HeroPackage`, then the disposable
  per-source `HeroFrameSet` cache, then live Chafa compilation. Package and
  cache ownership remain separate: the package is intentional,
  provenance-bound compiled output, while the cache is runtime acceleration.
- The package/cache startup migration is landed. Future work should focus on
  source and preset visual acceptance, supported-terminal review, and any
  explicitly scoped `CellGrid` or custom-backend experiment; it should not
  reopen the basic startup order without new evidence.
- A valid package or cache is an acceleration artifact, not visual approval.
  Schema, provenance, geometry, and placeholder checks remain necessary but do
  not replace real-terminal review of visible output.
- The remaining tooling layer is a `CellGrid`-based correction/editor workflow,
  not a raw ANSI editor: the structured per-cell representation already exists,
  and ANSI snapshots can be imported and exported, but manual and scripted
  corrections should operate on cells containing glyph, foreground color,
  optional background color, and mask/style metadata.
- Any future custom compiler backend should evaluate hero rendering from two
  deliberately separate directions: a monochrome `2x4` braille shape pass that
  controls thresholding, dot packing, and optional error diffusion; and an
  independent color pass that controls source sampling, palette quantization,
  red-family protection, and frame-to-frame color stability.
- Pre-generated hero art should be treated as asset authoring, not as runtime terminal capture: cached frames may include semi-manual correction overlays, region-specific lifts, and per-frame/cell stabilization where that improves face readability, red retention, silhouette stability, or animation consistency.
- The preferred eventual workflow is `Chafa/custom backend -> ANSI or direct cells -> CellGrid -> scripted/manual patches -> HeroFrameSet -> scene grid`, with Chafa and custom braille experiments acting as compiler backends rather than live scene dependencies.
- The first render-loop allocation cleanup slices are now live: runtime keeps one long-lived `Scene`, reuses the boxed layer stack across frames, skips obviously closed modal/help/quit layers before asking them to allocate grids at all, reuses the final composed `Grid` across frames in the live runtime loop, and now has a reusable per-layer scratch-grid seam adopted by the simple always-active layers, the lightweight companion projection layers (`clock`, `weather`, `date`), the always-active hero layer, the debug overlay, and the vine layer. The next conservative step on that path is no longer “take another obvious layer,” but deciding whether any of those remaining draw paths should move to even cheaper specialized helpers without destabilizing the current layer contract.
- There is now also a narrow fast ASCII-only compositor write path for plain UI chrome. It preserves the existing “spaces style but do not overwrite symbols” contract, falls back to the general grapheme-aware writer for non-ASCII text, and is currently used only by obviously ASCII-bound always-on chrome such as the footer, world label, and debug-panel/tab labels.
- The concrete disposable-cache contract lives in [`docs/hero-cache.md`](hero-cache.md), and the validated package contract lives in [`docs/hero-package.md`](hero-package.md); broader renderer strategy remains here.
- Prepared artifacts live in the user runtime cache directory (`$XDG_CACHE_HOME/yam/` when available, otherwise `~/.cache/yam/`). A matching package is checked first; if it is absent or invalid, startup checks the revision-keyed frame cache before falling back to GIF decode, temporary frames, and live Chafa.
- The final fallback is intentionally non-fatal when `chafa` is unavailable: if neither prepared artifact is usable and the compiler backend cannot be spawned, the renderer returns an explicit placeholder frame instead of panicking.
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
