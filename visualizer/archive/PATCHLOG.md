
# Visualizer Patch Log

Terminology note:

- historical entries may still use `trunk_mask`
- in current docs, that concept is normalized as the support mask / support field used for scaffold guidance

## System Phase Summary (Current)

The visualizer has evolved through three broad phases:

1. Procedural Prototype
   - single-pass growth
   - heuristic-heavy behavior
   - limited separation between growth and rendering concerns

2. Structured Growth System
   - mask-based collision
   - phase-based trunk routing
   - stronger separation between engine, state, and ornament responsibilities

3. Behavior-Driven Canopy System (current)
   - cluster foliage
   - spatial density fields
   - early directional behavior
   - layered architecture (input / layout / growth / state / rendering / debug)

Current focus:
- stabilizing behavior layers
- reducing ad-hoc tuning
- preparing transition toward directional and entity-based foliage

## 2026-04-18

### Vocabulary Normalization

- Renamed the visualizer sprawl vocabulary to `vines` across code, config, and docs
- Renamed the runtime modules to the `vines_*` series so code, docs, and config now use one consistent vocabulary
- Added [VOCABULARY.md](/Users/maciejkuster/yam/visualizer/VOCABULARY.md) as the canonical terminology dictionary
- Updated the top-level and visualizer docs to link the glossary and use the new names consistently

## 2026-04-17

### Render Field and Soft Trunk Geometry

- Added [render_field.py](/Users/maciejkuster/yam/visualizer/src/render_field.py) as a lightweight intermediate density/priority layer
- Scaffold and woody vine glyphs now accumulate through a shared render field before hero and panel overlays are stamped
- Added `density_to_braille()` and density-aware woody glyph selection so scaffold output can read as continuous density instead of only symbolic strokes
- Added `trunk_field` to [layout.py](/Users/maciejkuster/yam/visualizer/src/layout.py) as a soft distance map derived from trunk mask cells
- Updated vines scoring to use the trunk distance field instead of only binary trunk-mask membership
- This keeps the trunk mask authoritative while giving scaffold and growth a smoother gradient to follow

### Visualizer Config Hot Reload

- Added mtime-based reload in [main.py](/Users/maciejkuster/yam/visualizer/src/main.py) so edits to [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json) are applied while the app is running
- Reloading now refreshes the Chafa pipeline, hero frames, vines engine, info panel text, and layout on config change
- Updated [README.md](/Users/maciejkuster/yam/visualizer/README.md) and [CONFIG.md](/Users/maciejkuster/yam/visualizer/CONFIG.md) to document the live-reload contract

### Scaffold Config Activation

- Reworked [tree_scaffold.py](/Users/maciejkuster/yam/visualizer/src/tree_scaffold.py) so scaffold config values now move runtime geometry instead of collapsing into the same mask-snapped output
- `base_x` and `base_y` now offset the base point from the trunk-mask center instead of behaving like soft edge hints
- `trunk_height` now constrains which trunk-mask rows are eligible for the scaffold base and fork path
- `fork_y` still controls the fork floor and remains active
- `left_reach` / `right_reach` still shape branch spread
- `upper_lift` now affects branch tip height instead of being flattened by top-row fallback
- Scaffold remains mask-guided and clipped against visible hero pixels, but its JSON knobs are now observable again
- Final render clipping now keeps scaffold cells inside the trunk mask itself, so the visible scaffold obeys the allowed area rather than only borrowing its shape during selection

### Trunk/Scaffold Config Repair

- Restored [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json) from a corrupted pasted stub back into valid JSON
- Added the explicit trunk mask config surface:
  - `trunk_mask_path`
  - `trunk_mask_threshold`
  - `trunk_mask_scale_x`
  - `trunk_mask_scale_y`
  - `trunk_mask_offset_x`
  - `trunk_mask_offset_y`
- Added the explicit scaffold config section so `build_tree_scaffold(layout, config)` can be tuned from the JSON baseline
- Tightened scaffold placement so the fork sits below the hero instead of running through the figure
- Updated docs to match the actual runtime config surface again

### Revert Dark-Red Recovery Pass

- Reverted the warm-shadow / dark-red recovery stage in [chafa_pipeline.py](/Users/maciejkuster/yam/visualizer/src/chafa_pipeline.py)
- Kept [ives_window_procreate_edit_22.gif](/Users/maciejkuster/yam/visualizer/assets/ives_window_procreate_edit_22.gif) as the render source
- Restored the milder preprocessing baseline because the stronger pass over-tinted lighter colors

### Targeted Dark-Red Recovery Pass

- Added a warm-shadow recovery stage to [chafa_pipeline.py](/Users/maciejkuster/yam/visualizer/src/chafa_pipeline.py)
- The preprocessor now gives deep reds / burgundies a small early lift before Chafa quantization
- Tuned the existing warm-red separation pass to be slightly more aggressive in the dark-to-mid shadow range
- Reduced blur and sharpened the final prepass slightly so deep reds are less likely to smear into brown/grey during symbol conversion
- Cache signatures already include the preprocess signature, so the updated pass naturally invalidates stale rendered frames

### Legacy Kitty Removal

- Removed the old Kitty baseline files from the repo:
  - `kitty/kitty.conf`
  - `kitty/current-theme.conf`
- The repo is now Ghostty-first without keeping parallel Kitty config around as active source material

### Chafa Pipeline Contract Restoration

- Restored [chafa_pipeline.py](/Users/maciejkuster/yam/visualizer/src/chafa_pipeline.py) to a config-driven path for the current visualizer baseline
- The pipeline now again respects the configured cache directories:
  - `cache_dir_raw`
  - `cache_dir_chafa`
- Restored fallback-image handling:
  - if the primary source GIF is missing, the configured fallback image is used instead of aborting immediately
- Re-enabled config-sensitive Chafa command assembly:
  - symbol set
  - fill mode
  - color mode
  - color space
  - color extractor
  - dither settings
  - preprocess setting
  - transparency threshold
  - optimization level
  - foreground/background choice
- Added a small cache manifest so repeated launches can reuse previously rendered frames when the source/config signature has not changed
- This keeps the runtime contract aligned with [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json) instead of leaving those keys as dead documentation-only fields

### Runtime Helper Tracking

- The particle helper used by [main.py](/Users/maciejkuster/yam/visualizer/src/main.py) is part of the actual runtime path:
  - [vines_particles.py](/Users/maciejkuster/yam/visualizer/src/vines_particles.py)
- It should be treated as source-of-truth runtime code, not a scratch experiment

## 2026-04-16

### Baseline Normalization

- Promoted [ives_window_procreate_edit_22.gif](/Users/maciejkuster/yam/visualizer/assets/ives_window_procreate_edit_22.gif) to the canonical hero source in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json)
- Switched fallback image back to the tracked still asset:
  - `assets/ives_yam.png`
- Retired older exploratory Procreate GIF variants from the active visualizer baseline
- Updated docs to reflect the current runtime footprint:
  - hero baseline is now `72x36`
  - visualizer is tuned for Ghostty-first use
- Tightened repo hygiene by ignoring scratch calibration and dither outputs:
  - `visualizer/assets/frames_test/`
  - `visualizer/assets/test_dithered.png`
  - `visualizer/assets/test_dithered_frames/`
  - `visualizer/assets/calibration_palette.png`
  - `visualizer/src/omp.cache`

## 2026-04-11

### Audit and Source-of-Truth Alignment

- Ingested external notes from `/Users/maciejkuster/Desktop/yam_patchlotes.md`
- Audited current `yam` visualizer implementation against:
  - runtime behavior
  - module boundaries
  - config surface
  - documentation drift
- Confirmed that the visualizer architecture is maintainable but still heuristic-heavy

### Hero Collision Pivot

- Established the hero mask as the canonical collision geometry (source of truth) in [layout.py]
- Mask loading from:
  - [hero_mask.png]
- Mask semantics:
  - white pixels = blocked (solid)
  - black pixels = passable
  - `hero_safe_pad_*` expands blocked cells for safety margin
- Rectangular trim is now strictly a fallback path when no valid mask is available
- Introduced a hard separation between:
  - collision geometry (mask-driven, authoritative)
  - guidance geometry (visible hero frame used for high-level routing)
- Growth logic must never infer shape from padded bounds; only the mask defines true boundaries

### Runtime Guardrail

- Added a startup guard in [main.py](/Users/maciejkuster/yam/visualizer/src/main.py)
- If Chafa returns no frames, the renderer now falls back to a blank hero block instead of indexing an empty frame list

### Documentation Updates

- Updated [README.md](/Users/maciejkuster/yam/visualizer/README.md)
  - layout collision section now explains mask-first hero collision
- Updated [CONFIG.md](/Users/maciejkuster/yam/visualizer/CONFIG.md)
  - added `hero_mask_path`
  - added `hero_mask_threshold`
  - clarified that `hero_collision_trim_*` is fallback/secondary when mask collision is active
- Updated [STATUS.md](/Users/maciejkuster/yam/visualizer/STATUS.md)
  - now reflects the split vines engine structure
  - no longer describes the current engine as the original single-file wandering prototype

### Config Surface

- Added to [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json):
  - `layout.hero_mask_path`
  - `layout.hero_mask_threshold`

### Current Priority After This Checkpoint

1. Continue trunk-route refinement within the phase-based system (`approach` → `hero_top` → `post_top`)
2. Consolidate growth heuristics into stable, composable behavior layers (reduce ad-hoc tuning)
3. Prepare transition from cluster foliage to directional / entity-based leaf system (no implementation yet)

### Debug Overlay Refresh

- Re-enabled debug mode in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json)
- Updated [renderer.py](/Users/maciejkuster/yam/visualizer/src/renderer.py) debug overlays to show:
  - visible hero and info block frames in light blue
  - collision-mask frames in light grey
- Switched debug frame drawing from rendered-line dimensions to explicit layout rectangles
- This makes visible-vs-collision mismatch easier to inspect while continuing vine route tuning
- Note: debug overlays are part of the core development workflow and should be kept in sync with layout/mask logic changes

### Debug Mask Correction

- Corrected the hero-mask debug overlay in [renderer.py](/Users/maciejkuster/yam/visualizer/src/renderer.py)
- Previous debug rendering showed only the bounding rectangle of the blocked hero mask cells
- Current debug rendering now outlines the actual blocked silhouette cells
- This avoids the false impression that the mask was not being applied when collision logic was already mask-driven

### Mask Rasterization and Fallback

- Reworked hero-mask rasterization in [layout.py](/Users/maciejkuster/yam/visualizer/src/layout.py)
- The primary hero mask is now:
  - thresholded to binary first
  - aspect-compensated before terminal-cell downsampling
  - sampled onto the hero grid using box resampling instead of direct bright-pixel LANCZOS resize
- Added config controls:
  - `layout.hero_mask_scale_x`
  - `layout.hero_mask_scale_y`
- Updated the debug overlay in [renderer.py](/Users/maciejkuster/yam/visualizer/src/renderer.py) again:
  - hero mask is now rendered as light-grey dots on the actual blocked cells instead of an outline approximation
- Tightened the default terminal-grid compensation in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json):
  - `hero_mask_scale_x: 0.45`
  - `hero_mask_scale_y: 0.95`
- Reason:
  - the current hero mask asset is still proportionally broader than the visible Chafa figure
  - this is a pragmatic rasterization correction, not a claim that the source mask is final
- The old hero-mask fallback branch was retired after the fallback asset was removed; the current pipeline is now primary-mask-only again.

### Mask Alignment Margin

- Added `layout.hero_mask_alignment_margin` in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json)
- Applied in [layout.py](/Users/maciejkuster/yam/visualizer/src/layout.py) after rasterization
- Purpose:
  - allow approximately one terminal-cell of leeway between the visible hero frame and the raw mask
  - avoid treating tiny edge-touching differences as collision truth

### Hero Mask as Real Sprawl Boundary

- Updated [vines_growth.py](/Users/maciejkuster/yam/visualizer/src/vines_growth.py)
- Hero contour-follow and hero proximity pressure now use the actual hero mask boundary instead of the rectangular hero guide
- Deliberate split:
  - large-scale route heuristics still use the visible hero frame
  - local sprawl boundary behavior now follows the mask silhouette

### Maintenance and Baseline Reset

- Performed a repo-wide maintenance and audit pass over the active visualizer delta
- Removed redundant config helper file:
  - `visualizer/config/visualizer_readme.md`
- Cleaned generated source cache:
  - `visualizer/src/__pycache__/`
- Updated docs to reflect the actual current baseline:
  - hero frame is now treated as `48x24`
  - the cleaned hero mask is the temporary source of truth for sprawl boundaries
  - mask scaling is still allowed, but only as modest calibration rather than broad compensation
- Kept the following work explicitly deferred:
  - large-leaf polish
  - flower visual polish and lifecycle expansion
  - any secondary organism in upper-right negative space

### Monstera Leaf Study Ingest

- Ingested external reference:
  - `/Users/maciejkuster/Desktop/study_leaf_monstera_report.md`
- Added in-repo deferred design reference:
  - [archive/LEAF_STUDY.md](/Users/maciejkuster/_git/yam/visualizer/archive/LEAF_STUDY.md)
- Recorded the future design baseline for large-leaf work:
  - silhouette first
  - bottom-origin upward growth
  - broad lower-middle mass
  - edge-attached cuts only
  - lifecycle logic separate from morphology/rendering
- Confirmed that current work priority does not change:
  - trunk route refinement first
  - large leaves later

### Trunk Route Refinement

- Updated [vines_growth.py](/Users/maciejkuster/yam/visualizer/src/vines_growth.py) so the trunk no longer treats the right-side staging / panel corridor as a neutral holding area
- Added stronger pre-contact approach behavior:
  - leftward reward before hero contact
  - up-left reward before hero contact
  - panel-corridor loitering penalty
- Updated [vines_engine.py](/Users/maciejkuster/yam/visualizer/src/vines_engine.py) to suppress branch spawning while the trunk is still far to the right of the hero
- Added config controls in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json):
  - `hero_contact_margin`
  - `hero_approach_left_bonus`
  - `hero_approach_diagonal_bonus`
  - `right_staging_margin`
  - `right_staging_left_penalty`
  - `panel_corridor_penalty`
  - `branch_suppression_margin`
  - `pre_contact_branch_factor`
- Validation snapshot after 180 ticks across seeds `7`, `11`, and `19`:
  - top-band occupancy remained present: `78`, `53`, `72`
  - right-side staging mass reduced relative to the previous failure mode: `33`, `33`, `28`

### Trunk Route Phase

- Replaced the weak top-band commit flag with an explicit trunk route phase in [vines_state.py](/Users/maciejkuster/yam/visualizer/src/vines_state.py):
  - `approach`
  - `hero_top`
  - `post_top`
- Updated [vines_engine.py](/Users/maciejkuster/yam/visualizer/src/vines_engine.py) to advance that state from trunk movement rather than from loose proximity checks
- Updated [vines_growth.py](/Users/maciejkuster/yam/visualizer/src/vines_growth.py) so `hero_top` applies a stronger leftward persistence bias and stronger anti-drop behavior
- Suppressed trunk branching during `hero_top` using:
  - `vines.hero_top_branch_factor`
- Validation snapshot after 180 ticks across seeds `7`, `11`, and `19`:
  - all runs entered `hero_top` and then progressed to `post_top`
  - phase dwell times:
    - seed `7`: `hero_top 43`
    - seed `11`: `hero_top 38`
    - seed `19`: `hero_top 36`

### Trunk Thickening Pass

- Fixed stem aging so revisiting an existing stem cell no longer resets its birth frame in [vines_engine.py](/Users/maciejkuster/yam/visualizer/src/vines_engine.py)
- Added explicit trunk-lineage tracking in [vines_state.py](/Users/maciejkuster/yam/visualizer/src/vines_state.py):
  - `trunk_cells`
  - `trunk_birth`
- Updated [vines_ornament.py](/Users/maciejkuster/yam/visualizer/src/vines_ornament.py) so thickened wood now distinguishes:
  - ordinary aged stem thickening
  - stronger, earlier thickening on mature main-trunk cells
- Main-trunk thickening is now biased toward the older route from the origin up to roughly the info-panel corridor, instead of fattening the whole plant uniformly
- Added config controls in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json):
  - `trunk_thickening_min_age`
  - `trunk_thickening_bonus`
  - `trunk_thickening_info_margin`
  - `trunk_thickening_core_bias`

### Hero Exit Route Phase

- Extended the trunk route state machine from:
  - `approach` → `hero_top` → `post_top`
  to:
  - `approach` → `hero_top` → `hero_exit` → `post_top`
- Updated [vines_engine.py](/Users/maciejkuster/yam/visualizer/src/vines_engine.py) so the trunk now enters a short explicit `hero_exit` phase after leaving the upper hero band
- During `hero_exit`:
  - branching stays suppressed
  - the trunk is biased left and down-left
  - rightward and upward drift is penalized
- Updated [vines_growth.py](/Users/maciejkuster/yam/visualizer/src/vines_growth.py) with dedicated route-phase scoring for `hero_exit`
- Added config controls in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json):
  - `hero_exit_margin`
  - `hero_exit_left_bonus`
  - `hero_exit_downleft_bonus`
  - `hero_exit_right_penalty`
  - `hero_exit_up_penalty`
  - `hero_exit_branch_factor`

Intent:
- stop the trunk from peeling into the panel corridor immediately after the top-band crawl
- make the post-top transition read as an intentional continuation of the main vine rather than a loose release


## 2026-04-13

### Tropical Canopy Pass — Multi-Cell Leaf System + Density Control

This pass marks the first emergence of a structured canopy system rather than a purely visual density tweak. It shifts the visual balance from woody stems to a dense, tropical, multi-layer leaf canopy while introducing early forms of spatial logic and cluster behavior. The goal was to achieve "controlled chaos" with clearly readable clusters instead of sparse single-cell dots.

#### Multi-Cell Leaf Entities (Engine-Level)

- Introduced clustered leaf spawning instead of single-glyph placement
- Leaves now grow as small multi-cell groups (2–5 cells typical)
- Cluster shapes vary slightly per spawn to avoid repetition
- Clusters are anchored to nearby stem cells but expand outward with slight jitter
- Result:
  - visually coherent "leaf patches"
  - reduced noise compared to dot-based leaves

#### Leaf Density Rebalance

- Increased global leaf-to-stem ratio
- Reduced reliance on brown stem glyph emission in ornament pass
- Boosted probability of leaf cluster spawning on:
  - older stem cells
  - trunk-adjacent regions
- Result:
  - greener, more jungle-like composition
  - stems now act as scaffolding, not dominant visual mass

#### Spatial Density Field (Implicit Zoning)

Implemented early spatial field logic (engine-side), forming the basis of a future density/behavior field system:

- Upper band (top frame):
  - added stochastic jitter to break linear uniformity
  - randomized gaps and cluster sizes
- Left column (origin side):
  - encouraged dense early growth for "rooted" feel
- Right vertical trunk (primary cascade):
  - maintained high density but with cluster variance
- Bottom-right quarter (base of trunk):
  - applied density attenuation
  - reduced cluster spawn probability
  - prevents overgrowth blob at trunk base

#### Anti-Uniformity Corrections

- Broke continuous straight-line growth patterns along the top edge
- Introduced micro-branch jitter during trunk traversal
- Reduced long uninterrupted horizontal runs of identical glyphs
- Result:
  - more organic canopy silhouette
  - avoids "ASCII fence" artifacts

#### Dot Suppression Pass

- Reduced single-cell dot emissions (`.`-like leaf proxies)
- Favored cluster-first placement over fallback single glyphs
- Cleaned up visual noise introduced by prior high-density attempt
- Result:
  - clearer visual grouping
  - less "static" noise

#### Engine vs Ornament Responsibility Split (Reinforced)

- Reinforced strict separation between behavior and rendering layers:
  - `vines_engine.py`:
    - owns spatial decisions
    - controls cluster spawning
    - defines density distribution
  - `vines_ornament.py`:
    - renders clusters
    - applies glyph/style variation

- This pass confirms the architectural rule:
  - visual density must emerge from engine behavior
  - ornament must never compensate for weak growth logic

#### Outcome Snapshot

- Achieved first stable canopy phase:
  - dense but readable vine system
  - cluster-based foliage with spatial variation
  - trunk remains legible under increased foliage load
- Marks transition point from:
  - visual tuning → behavior-driven canopy formation
- Current state described by user as:
  - "somewhere between — controlled chaos"


#### Mixed Glyph Leaf Rendering (Interim Visual Grammar)

- Introduced a hybrid rendering approach combining:
  - symbolic glyphs (`*`, `✦`, `✧`, etc.)
  - dot-based glyphs (`•`, `●`, small round forms)
- Goal:
  - avoid overly abstract "flower-like" symbolic noise
  - avoid overly soft "blob-like" dot-only canopy
- Implemented in [vines_ornament.py]:
  - cluster renderer now selects from mixed glyph pools
  - glyph choice varies per cluster and per cell within cluster
- Visual effect:
  - improved texture richness
  - better readability at distance
  - maintains terminal-native aesthetic

- Important distinction:
  - this is a **rendering-layer improvement**, not a structural leaf model
  - actual leaf logic is still cluster-based, not morphology-based

- Result:
  - best current visual balance between:
    - structure (symbolic glyphs)
    - softness (dot glyphs)
  - adopted as the **interim baseline leaf language**

#### Future Direction — True Leaf Entities (Deferred)

- Current system still operates on:
  - cluster heuristics
  - glyph substitution

- Planned upgrade:
  - replace clusters with **leaf entities**:
    - anchored to stem
    - directional (facing vector)
    - size classes (young / mature / aging)
    - silhouette templates (monstera-style, etc.)

- Responsibility split (target architecture):
  - `vines_engine.py`:
    - decides *where*, *when*, and *orientation*
  - `vines_ornament.py`:
    - renders full multi-cell leaf silhouettes

- This will enable:
  - lifecycle-driven visuals
  - more realistic foliage structures
  - reduced reliance on randomness for visual richness

## 2026-04-16

### Rendering Pipeline Introduction (Chafa → Dither → Frames → Renderer)

- Introduced an implicit multi-stage rendering pipeline:
  - Chafa symbol/color conversion
  - Optional dithering prepass (`dither_prepass.py`)
  - Frame extraction / frame-set handling (`--frames` mode)
  - Renderer consumption of frame sequence
- This replaces the earlier implicit assumption of direct Chafa → renderer flow
- The visualizer must now be understood as consuming a processed frame stream rather than raw Chafa output

### Transparency Handling (Frame-Level Issue)

- Identified inconsistency in alpha handling across extracted frames:
  - First frame rendered with black background instead of transparency
  - Subsequent frames behave correctly
- Indicates pipeline-level inconsistency:
  - either initialization state
  - or alpha channel loss during first-frame processing
- Marked as an active system concern rather than a one-off rendering bug

### Growth Engine Evolution (From Clusters → Behavioral System)

- The growth system has evolved beyond simple cluster-based foliage:
  - trunk lineage tracking
  - age-based thickening
  - spatial density fields
  - directional biasing
  - phase-based routing (`approach`, `hero_top`, `post_top`)
- Reframed internally as:
  - a proto-ecosystem / behavior-driven growth system
- Patchlog previously described this as “visual improvement”; this is now recognized as a structural engine evolution

### Directional Clustering (Emerging Behavior)

- Began transition from isotropic cluster spawning to directional clustering:
  - clusters influenced by growth vector and trunk flow
  - reduced “blob-like” expansion
- This is a precursor to full leaf-entity orientation logic

### Glyph System Progression (Toward Role-Based Rendering)

- Current mixed glyph system (`symbolic + dot`) now acts as a bridge toward:
  - role-based glyph selection
- Glyphs increasingly encode:
  - structural role (stem vs leaf vs cluster mass)
  - visual weight
- Marks early stage of semantic rendering layer

### Debug Layer Reclassification

- Debug overlays (renderer + layout) are no longer auxiliary:
  - hero mask visualization
  - collision vs visible geometry comparison
  - layout frame inspection
- Reclassified as:
  - core development tooling required for system tuning
- Essential for diagnosing:
  - mask alignment
  - growth boundary behavior
  - spatial bias issues

### System Architecture (Updated Mental Model)

Current visualizer structure should be understood as layered:

1. Input Layer
   - source GIF
   - Chafa processing
   - optional dithering

2. Frame Pipeline
   - frame extraction
   - frame normalization
   - transparency handling

3. Layout Layer
   - terminal grid
   - hero placement
   - collision mask

4. Growth Engine
   - trunk routing
   - branching logic
   - spatial heuristics

5. State System
   - trunk lineage
   - growth phases
   - aging

6. Rendering Layer
   - leaf clusters
   - glyph selection
   - styling

7. Debug Layer
   - overlays
   - mask visualization
   - layout inspection

- This replaces the earlier simpler mental model of:
  - “procedural vine + renderer”

### Stage 1 Engine Refactor Checkpoint (Behavior-Preserving)

- Refactored `visualizer/src/vines_engine.py` to make the engine’s conceptual phase structure explicit without intentionally changing behavior
- `tick()` now reads as a top-level phase pipeline:
  1. Structural Growth
  2. Foliage Host Discovery
  3. Spatial Shaping
  4. Ornament Reconstruction
- Extracted Phase 1 helpers:
  - `_run_structural_growth_phase(...)`
  - `_process_growth_tip(...)`
- Extracted Phase 2 helpers:
  - `_run_foliage_host_discovery_phase(...)`
  - `_collect_mature_foliage_hosts(...)`
- Extracted Phase 3 helpers:
  - `_apply_host_enrichment(...)`
  - `_apply_directional_foliage_bias(...)`
  - `_apply_canopy_jitter(...)`
  - `_filter_canopy_for_readability(...)`
  - `_spawn_top_left_hanging_stems(...)`
- Extracted Phase 4 helper:
  - `_run_ornament_reconstruction_phase(...)`
- Added detailed `#` comments in new helpers to preserve phase ownership and intent clarity
- Verified checkpoint outcome:
  - engine still runs
  - current visual output remains acceptable after helper extraction
- This marks completion of Stage 1:
  - structure extraction
  - phase labeling
  - behavior preservation
- Deferred for later work:
  - Stage 2 cleanup / consolidation
  - direction-system unification
  - explicit spatial-field formalization

### Stage 2 Conceptual Cleanup Checkpoint (In Progress)

- Began Stage 2 work in `visualizer/src/vines_engine.py` with focus on:
  - direction-system unification
  - state-flow cleanup
  - spatial-field formalization
  - readability-policy split

#### Direction-System Unification

- Clarified Phase 2 terminology:
  - `mature_leaf_dirs` → `stem_orientations`
- Phase 2 comments now explicitly describe direction data as:
  - structural orientation hints
  - not final foliage-emission policy
- Extracted structural direction inference into:
  - `_infer_stem_orientation(...)`
- Extracted foliage-emission policy into:
  - `_infer_foliage_emission_direction(...)`
- Architectural result:
  - Phase 2 answers: *where is the stem going?*
  - Phase 3 answers: *where should foliage fan out?*

#### State-Flow Cleanup

- Removed unused transient structural-phase output:
  - `active_leaf_positions`
- Updated `_run_structural_growth_phase(...)` to return only:
  - next active tips
  - structural direction hints actually consumed downstream
- Updated `_process_growth_tip(...)` comments to reflect the cleaned state flow
- Result:
  - each Phase 1 output now has clear downstream purpose

#### Spatial-Field Formalization

- Converted previously implicit region-bias arithmetic into named field helpers:
  - `_sample_host_enrichment_field(...)`
  - `_should_thin_for_base_readability(...)`
  - `_sample_top_run_breakup_ceiling(...)`
- This preserves current behavior while making shaping policy legible as explicit field logic

#### Readability Policy Split

- Split upper-canopy readability logic into dedicated helpers:
  - `_apply_top_run_breakup_policy(...)`
  - `_apply_global_horizontal_suppression(...)`
- `_filter_canopy_for_readability(...)` now acts more clearly as a policy orchestrator rather than a dense mixed-logic block

#### Current Status

- Engine still runs after Stage 2 cleanup passes
- Current output remains acceptable for now
- Stage 2 so far has focused on conceptual clarity rather than visual redesign or retuning

#### Remaining Stage 2 Follow-Up

- helper spacing / formatting cleanup in `vines_engine.py`
- optional comment tightening so Phase 3 helper groups read as a cleaner policy stack
- possible future consolidation of additional shaping heuristics into named policy helpers if needed


## 2026-04-17

### Chafa Hero Rendering Investigation — Constraint Mapping / Failure Boundary

This session focused on the hero-rendering pipeline rather than the vines engine. The goal was to determine whether the current hero GIF could be rendered in terminal with all of the following constraints active at once:
- braille-only symbol rendering
- no visible background painting by Chafa
- no fill / no background-carried density cheats
- stable, aesthetically clean integration with the visualizer scene

#### Objective Clarification

The desired visual target was:
- high apparent resolution (braille density)
- dark reds / browns / purples preserved
- no black or grey image panel painted behind the hero
- no blocky fill artifacts
- no fallback to coarse ASCII-like symbol output

The investigation established that this was not merely a parameter-tuning problem; it exposed hard constraints in how Chafa behaves under foreground-only symbol rendering.

#### Pipeline Experiments Performed

Multiple Chafa command variants were tested and iterated through `visualizer/src/chafa_pipeline.py`, including combinations of:
- `--format=symbols`
- `--symbols=braille`
- `--fg-only`
- explicit `--bg=...`
- attempted `--bg-only`
- different `--color-space` values:
  - `din99d`
  - `rgb`
- different `--color-extractor` values:
  - `median`
  - `average`
- different dither strategies:
  - `none`
  - `diffusion`
  - `ordered`
- different dither intensities / grain sizes

In parallel, the image preprocessing stage in `_preprocess_frame(...)` was iteratively tuned to try to compensate for Chafa’s quantization behavior.

#### Preprocess Experiments Performed

The frame prepass was adjusted repeatedly to improve foreground survival under braille + fg-only rendering:
- dark-red / warm-red preservation pass
- midtone / dark-region lift so more pixels survive symbol quantization
- contrast / brightness / gamma shaping
- blur + unsharp preservation path
- temporary spatial expansion experiment (later removed)

Intent of the preprocessing experiments:
- make dark hair reds survive quantization
- prevent purple shirt collapse into brown / grey
- increase perceived body density without violating the “no fill / no background” rule

#### Failed Rendering Branches

The following branches were explored and explicitly rejected.

##### 1. Background-Carried Recovery Branch

Tried to preserve darker reds by allowing Chafa to use background color, then stripping or cleaning background escape sequences after rendering.

Observed failure modes:
- horizontal bars
- blocky background slabs
- black or brown panel artifacts
- unstable / ugly results despite partial dark-red recovery

Conclusion:
- letting background participate does recover some color information
- but visually violates the project’s hard rule that background should never be painted
- this branch was abandoned

##### 2. `--bg-only` Branch

Tried to treat Chafa as a background-only color field renderer.

Observed failure modes:
- invalid command combinations
- exit status `2` from Chafa on several invocations
- broken assumptions when combined with symbol-mode output
- conceptual mismatch with the project goal

Conclusion:
- `--bg-only` is not a viable stable solution for this visualizer’s hero layer under the current architecture
- this branch was abandoned

##### 3. Space-Symbol / Hidden-Glyph Branch

Tried to force “invisible” symbol rendering using space-like symbol settings so only color fields remained.

Observed failure modes:
- invalid symbol invocations in some combinations
- background-style fill behavior reappearing under a different form
- contradicted the requirement that rendering stay braille-based and foreground-driven

Conclusion:
- not compatible with the intended hero-rendering grammar
- abandoned

##### 4. Coarse ASCII / Non-Braille Branch

Temporarily tested lower-density symbol choices while debugging crashes and fill behavior.

Observed failure modes:
- visible coarse glyphs
- loss of intended high-resolution look
- immediate aesthetic regression relative to braille baseline

Conclusion:
- project rule reaffirmed: hero rendering should remain braille-first

#### Spatial Expansion Experiment (Removed)

A temporary preprocess pass attempted to increase visual mass by spatially expanding qualifying pixels / tones.

Observed failure modes:
- slab artifacts
- banding / false mass fields
- rectangular or horizontally smeared regions that Chafa then quantized aggressively

Conclusion:
- spatial expansion was not a safe way to increase fg-only density
- the pass was removed from the pipeline

#### Stable Baseline Recovered

After abandoning invalid / contradictory branches, the session converged back to the clean, stable baseline:
- symbol mode only
- braille symbols only
- foreground-only rendering (`--fg-only`)
- no background painting
- no fill
- simpler, safer Chafa invocation

This baseline is the one reflected by the final screenshots of the session.

#### Key Constraint Discovered

The most important result of the session was not a new visual win but a clarified limitation:

Under the simultaneous constraints of:
- braille-only output
- foreground-only rendering
- no fill
- no Chafa-painted background

Chafa can only represent the hero as a sparse foreground dot field.

That means:
- empty terminal cells are unavoidable
- “background visibility” in the final result is not necessarily a bug; it is often simply the absence of rendered foreground signal
- preprocessing can improve color survival and local readability, but it cannot fully replace lost spatial coverage when background participation is forbidden

In other words:
- the remaining weakness is no longer primarily a command-line bug
- it is a representation-limit issue under the chosen visual constraints

#### Working Outcome at End of Session

By the end of the session, the pipeline was back in a stable and honest state:
- no Chafa crashes in the final active branch
- no illegal symbol / bg-only combinations
- no fill-based background panel
- braille rendering restored
- terminal background remains untouched

Visual state at end:
- silhouette readable
- red hair survives partially
- purple shirt survives partially
- figure remains thinner / sparser than desired
- no clean solution was found that simultaneously achieves:
  - braille-only
  - no fill
  - no background painting
  - high apparent density / solidity

#### Practical Conclusion / New Boundary

This session effectively mapped the current Chafa design boundary for the hero layer.

What was established:
- many prior failures were real command / mode incompatibilities and have now been ruled out
- the current remaining shortfall is not just “more tuning needed”
- within the current architecture, Chafa braille + fg-only behaves as a sparse point-field renderer
- achieving a denser, more image-like hero while preserving the hard no-background / no-fill rule will likely require one of the following future directions:
  - accept the sparse braille aesthetic as the true baseline
  - slightly relax the no-background / no-fill rule
  - use a denser mixed symbol vocabulary instead of pure braille
  - build a custom renderer / compositor beyond stock Chafa behavior

Status after this checkpoint:
- investigation complete for the current constraint set
- hero pipeline stabilized
- limitation documented rather than left ambiguous
- future work should treat this as a known boundary, not an unresolved mystery

## 2026-04-17 Scaffold Plumbing Fix
- Fixed a live runtime wiring bug in `main.py`: `renderer.compose_scene(...)` now receives the active `config` object.
- This makes `config["scaffold"]` visible to the renderer again, so scaffold edits in `visualizer.json` no longer collapse to defaults.
- Also rebuilt `layout` and reset vines on config reload, so layout-bound settings like mask scale/offset now propagate without requiring a terminal resize.

## 2026-04-17 Scaffold Hero-Relative Placement
- Reworked scaffold base selection so `scaffold.base_x/base_y` now anchor relative to the hero footprint instead of the trunk-mask center.
- `fork_y` now offsets from the hero bottom band, which makes the scaffold noticeably easier to place underneath the hero GIF.
- The scaffold now follows `layout.trunk_field` as a soft distance surface and prefers the below-hero corridor instead of hard-snapping to a single mask cell.
- Fixed the clamp bug that was still pushing `base_x` into the same rightmost trunk-mask cell; scaffold offsets now affect the actual chosen base point.
- Added sane minimum scaffold geometry so zero-ish reach/height values still render a visible structure instead of collapsing to nothing.
- Relaxed the final render clip so the scaffold can actually move under the hero while still avoiding visible hero pixels.
- Decoupled `fork_y` from the base height so the fork row now moves independently instead of flattening into the same bottom clamp.

## 2026-04-18 Vines Reset Regression
- Fixed a runtime bug in `main.py` where Vines was being reset every frame because terminal sizes were compared by identity instead of value.
- The visualizer now compares `TerminalSize` objects by value, so the vine can continue growing across frames.
- This restores the expected behavior in Ghostty and in the installed runtime bundle.

## 2026-04-18 Woody Glyph Preservation
- The render-field layer was converting explicit woody glyph hints into braille too aggressively.
- Woody stroke glyphs are now preserved, and density glyphs are only used when no real glyph hint is present.
- Scaffold row rendering now uses the same glyph-preserving path, which should remove the broken dotted trunk look.

## 2026-04-18 Scaffold Visibility Pruning Removed
- The scaffold builder was still clipping the final path with a trunk-distance visibility threshold and pruning small components.
- That last-stage cull was breaking continuity and leaving the scaffold fragmented even though the path solver itself was continuous.
- The final render now keeps the generated scaffold path as long as it stays inside bounds and off visible hero pixels.

## 2026-04-18 Scaffold Connectivity Anchor
- The scaffold was still fragmenting because the retained component was anchored to the fork-side fragment instead of the main trunk path.
- Connected-component retention now seeds from the first visible trunk-path point, with 8-neighbor adjacency so diagonal woody contact stays contiguous.
- If no trunk-path seed survives, the builder falls back to the largest connected component instead of an arbitrary fragment.

## 2026-04-18 Research Reference Recorded
- Added [reference/RESEARCH.md](/Users/maciejkuster/_git/yam/visualizer/reference/RESEARCH.md) as a future architecture reference.
- The document is kept as design context for field-first rendering and soft-mask ideas, but it is not treated as a live runtime spec.
