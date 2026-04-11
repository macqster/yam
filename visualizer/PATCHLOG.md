# Visualizer Patch Log

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

- Replaced hero collision priority from rectangle-trim-first to mask-first in [layout.py](/Users/maciejkuster/yam/visualizer/src/layout.py)
- Added silhouette-mask loading from:
  - [hero_mask.png](/Users/maciejkuster/yam/visualizer/assets/hero_mask.png)
- Hero mask behavior:
  - white pixels block ivy
  - black pixels remain passable
  - `hero_safe_pad_*` expands blocked mask cells
- Rectangular trim logic remains as fallback only when the mask asset is unavailable
- Added an explicit distinction between:
  - hero blocked collision geometry
  - hero visible guidance geometry used by growth logic
- This prevents the growth engine from treating the padded mask bounding box as if it were the visible hero shape

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

1. Continue tuning main-trunk route behavior
2. Keep the second upper-right organism deferred
3. After trunk behavior is stable, move into large-leaf and flower lifecycle work

### Debug Overlay Refresh

- Re-enabled debug mode in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json)
- Updated [renderer.py](/Users/maciejkuster/yam/visualizer/src/renderer.py) debug overlays to show:
  - visible hero and info block frames in light blue
  - collision-mask frames in light grey
- Switched debug frame drawing from rendered-line dimensions to explicit layout rectangles
- This makes visible-vs-collision mismatch easier to inspect while continuing vine route tuning

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
