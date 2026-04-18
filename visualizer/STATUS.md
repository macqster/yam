# Visualizer Status

This document is the maintenance snapshot for the current `visualizer/` work.

It describes what exists now, what is stable, what is provisional, and where future integration work should attach.

Primary working docs:

- [SOURCE_INDEX.md](SOURCE_INDEX.md)
- [reference/EXTERNAL_YAM_SHOULD_COPY.md](reference/EXTERNAL_YAM_SHOULD_COPY.md)
- [TUNING_CHECKLIST.md](TUNING_CHECKLIST.md)

## Scope

The visualizer is a standalone terminal scene inside `visualizer/`, currently tuned for Ghostty.

It is separate from the repo's Fastfetch startup path.

Current layers:

- Chafa-rendered hero animation from a GIF
- procedural vines layer
- compact time/date info panel

Canonical dictionary:

- [VOCABULARY.md](VOCABULARY.md)

Tooling roadmap:

- [DEV_TOOLS.md](DEV_TOOLS.md)

Project process:

- [PROJECT_PROCESS.md](PROJECT_PROCESS.md)

Workflow:

- [WORKFLOW.md](WORKFLOW.md)

## Stable Boundaries

These parts are considered stable enough to build on:

- `src/main.py`
  - owns lifecycle, timing loop, resize handling, and scene composition calls
- `src/chafa_pipeline.py`
  - extracts and caches fixed-size Chafa frames
- `src/info_panel.py`
  - renders the time/date card
- `src/renderer.py`
  - composes hero art, vines cells, and panel into one terminal scene
- `src/terminal.py`
  - alt-screen and cursor control

These parts are the correct integration seam for future vines work:

- `src/layout.py`
- `src/vines_engine.py`

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

- `yam` now prefers `~/_git/yam/visualizer` when that repo exists.
- If the repo copy is unavailable, it falls back to `~/.local/share/yam-visualizer`.

This was done intentionally so visualizer iteration uses the repo source of truth immediately without needing reinstall for every tuning pass.

## Runtime Install Model

`./install.sh` currently installs:

- Fastfetch startup assets under `~/.local/share/fastfetch-chafa/`
- the visualizer runtime bundle under `~/.local/share/yam-visualizer/`
- launchers in `~/.local/bin/`

But for active development, the preferred editable source of truth is still:

- `~/_git/yam`

## Current Config Surface

Primary config file:

- `config/visualizer.json`

Detailed manual:

- `CONFIG.md`

Current major sections:

- `chafa`
  - source GIF, render size, Chafa symbol/color options
- `timing`
  - render cadence, hero animation cadence, vines tick cadence, info refresh cadence
- `layout`
  - hero placement, panel placement, panel gap, exclusion padding, hero mask, support mask
- `scaffold`
  - static woody support structure tuned from JSON
- `vines`
  - current experimental vines parameters
- `panel`
  - title and display settings

Named tuning presets are available through:

- `visualizer/recipes/`
- `visualizer/run_recipe.sh`

Config lint is available through:

- `visualizer/lint_config.py`

## What Is Working

- the visualizer launches correctly with `yam`
- Chafa GIF frames are generated and cached
- the hero art is rendered at fixed dimensions
- the info panel is rendered and placeable
- the screen is owned by one renderer process
- the startup Fastfetch path and the visualizer path are separated cleanly

## What Is Not Resolved

The current vines system is not considered solved.

Specifically:

- left/right and edge fill works
- hero-adjacent vertical coverage remains unreliable
- repeated parameter tuning did not fix the central coverage problem

Current conclusion:

- the engine is now beyond the original `VineHead` prototype and is split into state/growth/ornament modules

Reference material to preserve for future architecture work:

- [reference/RESEARCH.md](reference/RESEARCH.md)
  - useful summary of render-field, soft-mask, and layered-composition ideas
  - treat as a design reference, not a live spec
- it still remains heuristic and experimental rather than a principled coverage planner
- future work should favor stronger route/state logic over more scalar penalty stacking

## Current Vines Engine Status

`src/vines_engine.py` remains provisional, but the current vines stack is now:

- `src/vines_engine.py`
  - orchestration and public API
- `src/vines_growth.py`
  - trunk/branch guidance and route heuristics
- `src/vines_ornament.py`
  - leaves, wood thickening, death clusters, flowers, merge logic
- `src/vines_state.py`
  - mutable runtime state
- `src/vines_types.py`
  - shared types and palette constants

The scene compositor is now also expected to honor:

- `src/tree_scaffold.py`
  - static scaffold built from config, guided by the support field, rendered through the shared density field, and softly preferred into the below-hero corridor while still avoiding visible hero pixels
- `src/render_field.py`
  - intermediate density/priority field used to compose scaffold and woody vine glyphs before hero and panel overlays

This is maintainable enough to keep iterating on, but it is not yet the final behavior model.

## Recommended Future Integration Direction

When ingesting external Python vines-sprawl code, the target architecture should be:

- `layout.py`
  - provide explicit geometry and allowed-space information
- new vines engine
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

- `visualizer/README.md` originally described the vines layer more optimistically than current results justify
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
- prefer replacing the vines engine over mutating it further without a new global coverage model

## Practical Next Step

Before ingesting more vines code:

1. keep current visualizer launch/install behavior as-is
2. preserve the existing engine interface
3. keep using `hero_mask.png` as the preferred hero collision source and reduce reliance on rectangular trim heuristics
4. continue refining trunk route states before moving on to large leaf/flower lifecycle work

Current practical baseline:

- hero frame is `72x36`
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

Related deferred design reference:

- [archive/LEAF_STUDY.md](archive/LEAF_STUDY.md)
