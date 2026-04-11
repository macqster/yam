# Visualizer Status

This document is the maintenance snapshot for the current `visualizer/` work.

It describes what exists now, what is stable, what is provisional, and where future integration work should attach.

## Scope

The visualizer is a standalone Kitty-facing terminal scene inside `visualizer/`.

It is separate from the repo's Fastfetch startup path.

Current layers:

- Chafa-rendered hero animation from a GIF
- procedural ivy layer
- compact time/date info panel

## Stable Boundaries

These parts are considered stable enough to build on:

- `src/main.py`
  - owns lifecycle, timing loop, resize handling, and scene composition calls
- `src/chafa_pipeline.py`
  - extracts and caches fixed-size Chafa frames
- `src/info_panel.py`
  - renders the time/date card
- `src/renderer.py`
  - composes hero art, ivy cells, and panel into one terminal scene
- `src/terminal.py`
  - alt-screen and cursor control

These parts are the correct integration seam for future ivy work:

- `src/layout.py`
- `src/ivy_engine.py`

## Current Launcher Behavior

There are two ways to run the visualizer:

1. Repo-local:

```bash
./visualizer/run_visualizer.sh
```

2. Installed command:

```bash
yam
```

Important detail:

- `yam` now prefers `~/yam/visualizer` when that repo exists.
- If the repo copy is unavailable, it falls back to `~/.local/share/yam-visualizer`.

This was done intentionally so visualizer iteration uses the repo source of truth immediately without needing reinstall for every tuning pass.

## Runtime Install Model

`./install.sh` currently installs:

- Fastfetch startup assets under `~/.local/share/fastfetch-chafa/`
- the visualizer runtime bundle under `~/.local/share/yam-visualizer/`
- launchers in `~/.local/bin/`

But for active development, the preferred editable source of truth is still:

- `~/yam`

## Current Config Surface

Primary config file:

- `config/visualizer.json`

Detailed manual:

- `CONFIG.md`

Current major sections:

- `chafa`
  - source GIF, render size, Chafa symbol/color options
- `timing`
  - render cadence, hero animation cadence, ivy tick cadence, info refresh cadence
- `layout`
  - hero placement, panel placement, panel gap, exclusion padding
- `ivy`
  - current experimental ivy parameters
- `panel`
  - title and display settings

## What Is Working

- the visualizer launches correctly from Kitty with `yam`
- Chafa GIF frames are generated and cached
- the hero art is rendered at fixed dimensions
- the info panel is rendered and placeable
- the screen is owned by one renderer process
- the startup Fastfetch path and the visualizer path are separated cleanly

## What Is Not Resolved

The current ivy system is not considered solved.

Specifically:

- left/right and edge fill works
- hero-adjacent vertical coverage remains unreliable
- repeated parameter tuning did not fix the central coverage problem

Current conclusion:

- the engine is now beyond the original `VineHead` prototype and is split into state/growth/ornament modules
- it still remains heuristic and experimental rather than a principled coverage planner
- future work should favor stronger route/state logic over more scalar penalty stacking

## Current Ivy Engine Status

`src/ivy_engine.py` remains provisional, but the current ivy stack is now:

- `src/ivy_engine.py`
  - orchestration and public API
- `src/ivy_growth.py`
  - trunk/branch guidance and route heuristics
- `src/ivy_ornament.py`
  - leaves, wood thickening, death clusters, flowers, merge logic
- `src/ivy_state.py`
  - mutable runtime state
- `src/ivy_types.py`
  - shared types and palette constants

This is maintainable enough to keep iterating on, but it is not yet the final behavior model.

## Recommended Future Integration Direction

When ingesting external Python ivy-sprawl code, the target architecture should be:

- `layout.py`
  - provide explicit geometry and allowed-space information
- new ivy engine
  - own scaffold growth
  - separate stems from ornaments
  - track coverage by region
  - target underfilled legal areas
- `renderer.py`
  - remain unchanged except for consuming the final merged glyph map

The best future engine contract is:

```python
reset(size, layout)
tick(layout)
get_segments() -> dict[tuple[int, int], str]
```

That keeps `main.py` and `renderer.py` stable while allowing a full engine replacement.

## Known Technical Debt

- `visualizer/README.md` originally described the ivy layer more optimistically than current results justify
- `layout.py` now provides allowed-cell and region masks, but hero collision only recently pivoted to silhouette-mask-first behavior
- `layout.py` now distinguishes between:
  - blocked hero collision geometry derived from the silhouette mask
  - visible hero guidance geometry used by growth/routing logic
- flowers exist in ornament code but are not yet the finished visual system
- the current cleaned hero mask is good enough to use as the temporary sprawl boundary, but it should still be treated as provisional art/source data rather than a final canonical silhouette
- there is no debug instrumentation view yet for:
  - allowed mask
  - stem-only coverage
  - region deficits
  - spawn origin tracing
  - failed move heatmaps

## Maintenance Rules Going Forward

- treat `main.py`, `renderer.py`, `terminal.py`, `info_panel.py`, and `chafa_pipeline.py` as stable unless there is a clear bug
- avoid further ad hoc tuning of the current `VineHead` model unless the goal is strictly temporary experimentation
- document launcher/runtime behavior whenever install behavior changes
- keep repo-local iteration behavior explicit in docs
- prefer replacing the ivy engine over mutating it further without a new global coverage model

## Practical Next Step

Before ingesting more ivy code:

1. keep current visualizer launch/install behavior as-is
2. preserve the existing engine interface
3. keep using `hero_mask.png` as the preferred hero collision source and reduce reliance on rectangular trim heuristics
4. continue refining trunk route states before moving on to large leaf/flower lifecycle work

Current practical baseline:

- hero frame is `48x24`
- hero mask placement is accepted as temporary truth
- trim tweaking should now be secondary to route behavior refinement

## Unspecified Future Objectives

These are intentionally deferred, but should remain visible in project scope:

- multi-glyph leaves with richer shape language
- flower rendering as a distinct ornament system
- full lifecycle handling for leaves and flowers:
  - emergence
  - maturity
  - aging
  - decay
- a secondary independent plant/organism for the upper-right negative space, if that corner continues to feel compositionally empty
