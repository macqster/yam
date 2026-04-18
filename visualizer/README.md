# Yam Visualizer

Dedicated terminal visualizer for macOS, currently tuned for Ghostty, with three coordinated layers:

- Chafa-rendered hero animation from a GIF
- state-driven vines growth system with a procedural ornament layer
- compact time/date panel

This is separate from the repo's Fastfetch startup path. It is meant to be launched intentionally in its own terminal window, tab, or split.

Start here:

- [SOURCE_INDEX.md](SOURCE_INDEX.md)
- [EXTERNAL_YAM_SHOULD_COPY.md](reference/EXTERNAL_YAM_SHOULD_COPY.md)
- [TUNING_CHECKLIST.md](TUNING_CHECKLIST.md)

Current maintenance snapshot:

- [STATUS.md](STATUS.md)
- [VOCABULARY.md](VOCABULARY.md)
- [CONFIG.md](CONFIG.md)
- [WORKFLOW.md](WORKFLOW.md)
- [MASKS_AND_GUIDES.md](MASKS_AND_GUIDES.md)
- [DEV_TOOLS.md](DEV_TOOLS.md)
- [PROJECT_PROCESS.md](PROJECT_PROCESS.md)

## Requirements

- macOS
- Ghostty
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

For a named preset:

```bash
./visualizer/run_recipe.sh debug
```

Quit with `Ctrl+C`.

After `./install.sh`, you can launch it from any shell with:

```bash
yam
```

Use `yam --list-recipes` to print the available presets, or `yam --recipe debug` / `yam --recipe presentation` to start one.

`yam` prefers the repo copy at `~/_git/yam/visualizer` when that repo exists locally. This is intentional so iteration does not depend on reinstalling the runtime bundle after every config change.
The launcher resolves that repo copy through `YAM_REPO` when available, then falls back to `~/.local/share/yam-visualizer`.

## Configuration

Main settings live in [visualizer.json](config/visualizer.json).
The running app watches that file for changes and reloads it automatically, so scaffold, layout, and vines tweaks should be visible without restarting.

There are now two documentation layers for config:

- practical overview here in this README
- full field-by-field manual in [CONFIG.md](CONFIG.md)

Named config overlays live in [recipes/README.md](recipes/README.md).
Lint the base config or recipe overlays with `./visualizer/lint_config.py`.

The easiest things to tweak first are:

- Chafa frame count, size, symbol mode, palette, and alpha handling
- renderer cadence for hero and vines motion
- info panel dimensions and placement
- vines experiment settings and glyph/color vocabulary

### Practical Config Guide

Start with these sections in order:

1. `chafa`
   Use this to control the hero art footprint and quality.
   The single most important composition knob here is `height`.

2. `layout`
   Use this to place the hero and panel and define how close vines is allowed to get.
   Hero collision now prefers the silhouette mask asset and only falls back to trim logic if the mask is unavailable.
   The current working assumption is:
   - the cleaned hero mask is the real temporary sprawl boundary
   - future mask/GIF asset swaps are expected
   Internally, scaffold and woody vine layers now accumulate through a shared
   density field before the hero and panel are stamped on top, which keeps
   overlapping structure more continuous.
   The support-mask config surface still uses `trunk_mask_*` keys for compatibility.
   The runtime also accepts `support_mask_*` aliases for the same settings.
   The most important collision controls are:
   - `hero_mask_path`
   - `hero_mask_threshold`
   - `hero_mask_scale_x`
   - `hero_mask_scale_y`
   - `hero_mask_alignment_margin`
   - `trunk_mask_path`
   - `trunk_mask_scale_x`
   - `trunk_mask_scale_y`
   - `trunk_mask_offset_x`
   - `trunk_mask_offset_y`
   - `hero_collision_trim_*`
   - `hero_safe_pad_*`
   - `info_collision_trim_*`
   - `info_safe_pad_*`
   The static scaffold is configured separately under `scaffold` and is rendered before vines so vines can overgrow it.

3. `timing`
   Use this to switch between development/debug cadence and calmer presentation cadence.
   Typical pattern:
   - development: higher `render_fps`, faster `vines_tick_seconds`
   - presentation: slower hero and vines cadence

4. `vines`
   Use this last.
   This is the most experimental section and the easiest place to create aggressive or awkward growth if multiple values are pushed at once.

### Recommended Tuning Order

When the scene looks wrong, change things in this order:

1. hero size and position
2. panel position
3. collision trims and safe padding
4. timing cadence
5. vines growth pressure
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
- `layout.trunk_mask_scale_x`
  Controls the support-mask width used for debug and growth scoring.
- `layout.trunk_mask_offset_y`
  Controls where the support mask sits in the scene.
- `layout.trunk_field`
  Not a config key, but the runtime distance field derived from the support mask.
  It now shapes scaffold selection and vines scoring with smooth proximity instead
  of binary in/out behavior.
- `layout.hero_safe_pad_x`
  Controls how tightly vines can frame the hero.
- `layout.info_safe_pad_x`
  Controls how tightly vines can approach the panel text.
- `scaffold.fork_y`
  Controls where the scaffold splits below the hero.
- `scaffold.base_x`
  Shifts the finished scaffold horizontally relative to the hero center.
- `scaffold.base_y`
  Shifts the finished scaffold vertically relative to the hero bottom band.
- `scaffold.trunk_height`
  Changes how far the scaffold is allowed to climb before splitting and how much vertical room the base selection expects.
- `scaffold.upper_lift`
  Raises or lowers the branch tips relative to the fork.
The scaffold is now selected from a support field and rendered through a
shared density field, so the knobs steer placement through a soft below-hero
preference rather than snapping to a fragmented binary mask. The visible
scaffold still avoids hero pixels.
- `vines.forward_bonus`
  Too high makes scaffold-like rails.
- `vines.support_wrap_bonus`
  Too high makes the vine orbit obstacle edges.
- `vines.hero_contour_attraction`
  Too high makes the vine feel pushy around hero/panel boundaries.
- `vines.leaf_stamp_chance`
  Controls how quickly the plant becomes visually busy.
- `timing.vines_tick_seconds`
  Strongly affects perceived aggressiveness.

### Debug vs Presentation

Two common operating modes:

- Debug mode
  Use faster redraw and faster vines stepping so growth behavior is easy to inspect.
- Presentation mode
  Use calmer cadence so the scene feels atmospheric rather than mechanical.

Current values can move between those modes during development. The config should be treated as a live tuning surface, not a fixed finished spec.

Note: debug overlays are part of the core development workflow and are essential for validating layout, mask alignment, and growth behavior.

## Assets and caching

- Preferred input: `visualizer/assets/ives_window_procreate_edit_22.gif`
- If that GIF is missing, the app generates a subtle fallback GIF from `visualizer/assets/ives_yam.png`
- extracted PNG frames are cached in `visualizer/assets/frames_raw/`
- rendered ANSI frames are cached in `visualizer/assets/frames_chafa/`
- those cache locations are configured in `visualizer.json` and resolved relative to the visualizer repo root unless given as absolute paths

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

The broader spatial model is documented in [MASKS_AND_GUIDES.md](MASKS_AND_GUIDES.md).

- `src/main.py` owns the loop and screen lifecycle
- `src/chafa_pipeline.py` creates or loads fixed-size cached Chafa frames
- `src/layout.py` defines hero/info regions and vines no-go zones
- `src/vines_engine.py` is the public vines engine adapter
- `src/vines_growth.py` holds movement, guidance, support, and contour-follow logic
- `src/vines_ornament.py` holds leaf stamps, death clusters, thickening, and segment merge logic
- `src/vines_state.py` holds mutable vines state
- `src/vines_types.py` holds shared types and palette constants
- `src/info_panel.py` renders the quiet time/date card
- `src/renderer.py` composes the terminal scene in one process
- `src/terminal.py` handles alt-screen and cursor cleanup

## Vocabulary

The canonical naming dictionary lives in [VOCABULARY.md](VOCABULARY.md).
Use it for new config keys, code symbols, and documentation terms.

For the current project status, known caveats, and future vines integration boundary, see [STATUS.md](STATUS.md).

## Limitations

- Weather is intentionally not implemented yet
- The renderer assumes the hero art and info panel do not overlap vines because composition stays terminal-text-native and avoids raster stacking
- Hero collision uses `visualizer/assets/hero_mask.png` as the primary collision geometry (source of truth), with trim-based collision kept only as fallback behavior
- The current hero baseline is `72x36`, and mask tuning should be understood relative to that footprint
- Layout is tuned for a roomy Ghostty window, not tiny terminals
- The fallback GIF is a generated stand-in, not bespoke animation art
- The current vines engine is still experimental and not yet considered the final model for reliable full-scene sprawl

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

- [archive/LEAF_STUDY.md](archive/LEAF_STUDY.md)

Future architecture reference:

- [reference/RESEARCH.md](reference/RESEARCH.md)
  - repository-wide rendering and architecture learnings
  - useful as a design reference, not a live runtime spec
