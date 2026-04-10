# Yam Visualizer

Dedicated Kitty visualizer for macOS with three coordinated layers:

- Chafa-rendered hero animation from a GIF
- slow procedural ivy ornament
- compact time/date panel

This is separate from the repo's Fastfetch startup path. It is meant to be launched intentionally in its own Kitty window, tab, or overlay.

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

After `./install.sh`, you can launch it from any Kitty shell with:

```bash
yam
```

`yam` prefers the repo copy at `~/yam/visualizer` when that repo exists locally. This is intentional so iteration does not depend on reinstalling the runtime bundle after every config change.

## Configuration

Main settings live in [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json).

The easiest things to tweak are:

- Chafa frame count, size, symbol mode, palette, and alpha handling
- renderer cadence for hero and ivy motion
- info panel dimensions and placement
- ivy experiment settings and glyph/color vocabulary

## Assets and caching

- Preferred input: `/Users/maciejkuster/Downloads/chafa_studies/ives_window_keyed_opt.gif`
- If that GIF is missing, the app generates a subtle fallback GIF from `assets/ives_yam.png`
- extracted PNG frames are cached in `visualizer/assets/frames_raw/`
- rendered ANSI frames are cached in `visualizer/assets/frames_chafa/`

Delete the cache directories if you want to force a fresh render after changing source assets or Chafa settings.

## Architecture

- `src/main.py` owns the loop and screen lifecycle
- `src/chafa_pipeline.py` creates or loads fixed-size cached Chafa frames
- `src/layout.py` defines hero/info regions and ivy no-go zones
- `src/ivy_engine.py` maintains slow edge-biased ornamental growth
- `src/info_panel.py` renders the quiet time/date card
- `src/renderer.py` composes the terminal scene in one process
- `src/terminal.py` handles alt-screen and cursor cleanup

For the current project status, known caveats, and future ivy integration boundary, see [STATUS.md](/Users/maciejkuster/yam/visualizer/STATUS.md).

## Limitations

- Weather is intentionally not implemented yet
- The renderer assumes the hero art and info panel do not overlap ivy because composition stays terminal-text-native and avoids raster stacking
- Layout is tuned for a roomy Kitty window, not tiny terminals
- The fallback GIF is a generated stand-in, not bespoke animation art
- The current ivy engine is experimental and not yet considered the final model for reliable full-scene sprawl
