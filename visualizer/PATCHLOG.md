
# Visualizer Patch Log

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
  - now reflects the split ivy engine structure
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
  - `layout.hero_mask_scaled_fallback_path`
- Added a degeneracy check:
  - if the primary mask rasterizes into a near-full-frame footprint, the loader falls back to the pre-scaled mask
- Updated the debug overlay in [renderer.py](/Users/maciejkuster/yam/visualizer/src/renderer.py) again:
  - hero mask is now rendered as light-grey dots on the actual blocked cells instead of an outline approximation
- Tightened the default terminal-grid compensation in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json):
  - `hero_mask_scale_x: 0.45`
  - `hero_mask_scale_y: 0.95`
- Reason:
  - the current hero mask asset is still proportionally broader than the visible Chafa figure
  - this is a pragmatic rasterization correction, not a claim that the source mask is final

### Mask Alignment Margin

- Added `layout.hero_mask_alignment_margin` in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json)
- Applied in [layout.py](/Users/maciejkuster/yam/visualizer/src/layout.py) after rasterization
- Purpose:
  - allow approximately one terminal-cell of leeway between the visible hero frame and the raw mask
  - avoid treating tiny edge-touching differences as collision truth

### Hero Mask as Real Sprawl Boundary

- Updated [ivy_growth.py](/Users/maciejkuster/yam/visualizer/src/ivy_growth.py)
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
  - [LEAF_STUDY.md](/Users/maciejkuster/yam/visualizer/LEAF_STUDY.md)
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

- Updated [ivy_growth.py](/Users/maciejkuster/yam/visualizer/src/ivy_growth.py) so the trunk no longer treats the right-side staging / panel corridor as a neutral holding area
- Added stronger pre-contact approach behavior:
  - leftward reward before hero contact
  - up-left reward before hero contact
  - panel-corridor loitering penalty
- Updated [ivy_engine.py](/Users/maciejkuster/yam/visualizer/src/ivy_engine.py) to suppress branch spawning while the trunk is still far to the right of the hero
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

- Replaced the weak top-band commit flag with an explicit trunk route phase in [ivy_state.py](/Users/maciejkuster/yam/visualizer/src/ivy_state.py):
  - `approach`
  - `hero_top`
  - `post_top`
- Updated [ivy_engine.py](/Users/maciejkuster/yam/visualizer/src/ivy_engine.py) to advance that state from trunk movement rather than from loose proximity checks
- Updated [ivy_growth.py](/Users/maciejkuster/yam/visualizer/src/ivy_growth.py) so `hero_top` applies a stronger leftward persistence bias and stronger anti-drop behavior
- Suppressed trunk branching during `hero_top` using:
  - `ivy.hero_top_branch_factor`
- Validation snapshot after 180 ticks across seeds `7`, `11`, and `19`:
  - all runs entered `hero_top` and then progressed to `post_top`
  - phase dwell times:
    - seed `7`: `hero_top 43`
    - seed `11`: `hero_top 38`
    - seed `19`: `hero_top 36`

### Trunk Thickening Pass

- Fixed stem aging so revisiting an existing stem cell no longer resets its birth frame in [ivy_engine.py](/Users/maciejkuster/yam/visualizer/src/ivy_engine.py)
- Added explicit trunk-lineage tracking in [ivy_state.py](/Users/maciejkuster/yam/visualizer/src/ivy_state.py):
  - `trunk_cells`
  - `trunk_birth`
- Updated [ivy_ornament.py](/Users/maciejkuster/yam/visualizer/src/ivy_ornament.py) so thickened wood now distinguishes:
  - ordinary aged stem thickening
  - stronger, earlier thickening on mature main-trunk cells
- Main-trunk thickening is now biased toward the older route from the origin up to roughly the info-panel corridor, instead of fattening the whole plant uniformly
- Added config controls in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json):
  - `trunk_thickening_min_age`
  - `trunk_thickening_bonus`
  - `trunk_thickening_info_margin`
  - `trunk_thickening_core_bias`


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
  - `ivy_engine.py`:
    - owns spatial decisions
    - controls cluster spawning
    - defines density distribution
  - `ivy_ornament.py`:
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
- Implemented in [ivy_ornament.py]:
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
  - `ivy_engine.py`:
    - decides *where*, *when*, and *orientation*
  - `ivy_ornament.py`:
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

- Refactored `visualizer/src/ivy_engine.py` to make the engine’s conceptual phase structure explicit without intentionally changing behavior
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

- Began Stage 2 work in `visualizer/src/ivy_engine.py` with focus on:
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

- helper spacing / formatting cleanup in `ivy_engine.py`
- optional comment tightening so Phase 3 helper groups read as a cleaner policy stack
- possible future consolidation of additional shaping heuristics into named policy helpers if needed
