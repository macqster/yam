# Yam Visualizer

Dedicated terminal visualizer for macOS, currently tuned for Ghostty, with three coordinated layers:

- Chafa-rendered hero animation from a GIF
- state-driven ivy growth system with a procedural ornament layer
- compact time/date panel

This is separate from the repo's Fastfetch startup path. It is meant to be launched intentionally in its own terminal window, tab, or split.

Current maintenance snapshot:

- [STATUS.md](/Users/maciejkuster/yam/visualizer/STATUS.md)

## Requirements

- macOS
- Kitty
- `python3`
- `chafa`
- Python package from `visualizer/requirements.txt`

Install the Python dependency:

```bash
python3 -m venv visualizer/.venv
source visualizer/.venv/bin/activate
python3 -m pip install -r visualizer/requirements.txt
```

## Run

From the repo root:

```bash
./visualizer/run_visualizer.sh
```

Quit with `Ctrl+C`.

After `./install.sh`, you can launch it from any shell with:

```bash
yam
```

`yam` prefers the repo copy at `~/yam/visualizer` when that repo exists locally. This is intentional so iteration does not depend on reinstalling the runtime bundle after every config change.

## Configuration

Main settings live in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json).

There are now two documentation layers for config:

- practical overview here in this README
- full field-by-field manual in [CONFIG.md](/Users/maciejkuster/yam/visualizer/CONFIG.md)

The easiest things to tweak first are:

- Chafa frame count, size, symbol mode, palette, and alpha handling
- renderer cadence for hero and ivy motion
- info panel dimensions and placement
- ivy experiment settings and glyph/color vocabulary

### Practical Config Guide

Start with these sections in order:

1. `chafa`
   Use this to control the hero art footprint and quality.
   The single most important composition knob here is `height`.

2. `layout`
   Use this to place the hero and panel and define how close ivy is allowed to get.
   Hero collision now prefers the silhouette mask asset and only falls back to trim logic if the mask is unavailable.
   The current working assumption is:
   - the cleaned hero mask is the real temporary sprawl boundary
   - future mask/GIF asset swaps are expected
   The most important collision controls are:
   - `hero_mask_path`
   - `hero_mask_threshold`
   - `hero_mask_scale_x`
   - `hero_mask_scale_y`
   - `hero_mask_alignment_margin`
   - `hero_collision_trim_*`
   - `hero_safe_pad_*`
   - `info_collision_trim_*`
   - `info_safe_pad_*`

3. `timing`
   Use this to switch between development/debug cadence and calmer presentation cadence.
   Typical pattern:
   - development: higher `render_fps`, faster `ivy_tick_seconds`
   - presentation: slower hero and ivy cadence

4. `ivy`
   Use this last.
   This is the most experimental section and the easiest place to create aggressive or awkward growth if multiple values are pushed at once.

### Recommended Tuning Order

When the scene looks wrong, change things in this order:

1. hero size and position
2. panel position
3. collision trims and safe padding
4. timing cadence
5. ivy growth pressure
6. ornament density

This avoids using growth tuning to solve what is really a layout problem.

### High-Impact Knobs

These settings have the biggest visible effect:

- `chafa.height`
  Changes how much of the scene the hero occupies.
- `layout.hero_offset_y`
  Changes the vertical composition and available crawl space.
- `layout.hero_mask_threshold`
  Controls how aggressively the hero silhouette mask blocks cells.
- `layout.hero_safe_pad_x`
  Controls how tightly ivy can frame the hero.
- `layout.info_safe_pad_x`
  Controls how tightly ivy can approach the panel text.
- `ivy.forward_bonus`
  Too high makes scaffold-like rails.
- `ivy.support_wrap_bonus`
  Too high makes the vine orbit obstacle edges.
- `ivy.hero_contour_attraction`
  Too high makes the vine feel pushy around hero/panel boundaries.
- `ivy.leaf_stamp_chance`
  Controls how quickly the plant becomes visually busy.
- `timing.ivy_tick_seconds`
  Strongly affects perceived aggressiveness.

### Debug vs Presentation

Two common operating modes:

- Debug mode
  Use faster redraw and faster ivy stepping so growth behavior is easy to inspect.
- Presentation mode
  Use calmer cadence so the scene feels atmospheric rather than mechanical.

Current values can move between those modes during development. The config should be treated as a live tuning surface, not a fixed finished spec.

Note: debug overlays are part of the core development workflow and are essential for validating layout, mask alignment, and growth behavior.

## Assets and caching

- Preferred input: `visualizer/assets/ives_window_procreate_edit_22.gif`
- If that GIF is missing, the app generates a subtle fallback GIF from `visualizer/assets/ives_yam.png`
- extracted PNG frames are cached in `visualizer/assets/frames_raw/`
- rendered ANSI frames are cached in `visualizer/assets/frames_chafa/`

Delete the cache directories if you want to force a fresh render after changing source assets or Chafa settings.

## Architecture

The system is layered conceptually as follows:

1. Input Layer
   - source GIF
   - Chafa conversion

2. Frame Pipeline
   - optional dithering prepass
   - frame extraction and caching
   - transparency handling

3. Layout Layer
   - terminal grid
   - hero placement
   - collision mask (authoritative)

4. Growth Engine
   - trunk routing
   - branching and spatial heuristics

5. State System
   - growth state
   - lineage and aging

6. Rendering Layer
   - ornament (glyphs, clusters)
   - scene composition

7. Debug Layer
   - overlays for layout, mask, and collision inspection

- `src/main.py` owns the loop and screen lifecycle
- `src/chafa_pipeline.py` creates or loads fixed-size cached Chafa frames
- `src/layout.py` defines hero/info regions and ivy no-go zones
- `src/ivy_engine.py` is the public ivy engine adapter
- `src/ivy_growth.py` holds movement, guidance, support, and contour-follow logic
- `src/ivy_ornament.py` holds leaf stamps, death clusters, thickening, and segment merge logic
- `src/ivy_state.py` holds mutable ivy state
- `src/ivy_types.py` holds shared types and palette constants
- `src/info_panel.py` renders the quiet time/date card
- `src/renderer.py` composes the terminal scene in one process
- `src/terminal.py` handles alt-screen and cursor cleanup

For the current project status, known caveats, and future ivy integration boundary, see [STATUS.md](/Users/maciejkuster/yam/visualizer/STATUS.md).

## Limitations

- Weather is intentionally not implemented yet
- The renderer assumes the hero art and info panel do not overlap ivy because composition stays terminal-text-native and avoids raster stacking
- Hero collision uses `visualizer/assets/hero_mask.png` as the primary collision geometry (source of truth), with trim-based collision kept only as fallback behavior
- The current hero baseline is `72x36`, and mask tuning should be understood relative to that footprint
- Layout is tuned for a roomy Kitty window, not tiny terminals
- The fallback GIF is a generated stand-in, not bespoke animation art
- The current ivy engine is still experimental and not yet considered the final model for reliable full-scene sprawl

## Future Objectives

Unspecified-future objectives worth preserving explicitly:

- multi-glyph leaves with fuller shape language instead of only small single-cell accents and stamps
- flowers as first-class ornament elements, not just glyph substitutions
- full lifecycle treatment for foliage and flowers:
  - emergence
  - maturity
  - aging
  - decay
- a possible second independent plant/organism in the upper-right corner if that area remains compositionally underused

Deferred design reference for future large-leaf work:

- [LEAF_STUDY.md](/Users/maciejkuster/yam/visualizer/LEAF_STUDY.md)
