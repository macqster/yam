# Visualizer Project Process

Date: 2026-04-18
Scope: `visualizer/`
Purpose: define lightweight development and project-management practices that reduce iteration time and improve traceability.

This document is intentionally practical. It does not change runtime behavior.

## Primary Goals

- make visual tuning reproducible
- make regressions easier to isolate
- keep config changes understandable
- preserve context for future maintenance

## Recommended Working Conventions

### 1. Scene Recipes

Maintain a small set of named config presets for common work modes.

Suggested recipe types:

- `debug`
- `presentation`
- `tight_layout`
- `hero_heavy`
- `trunk_probe`

Why this helps:

- reduces manual JSON tweaking
- makes experiments easier to compare
- gives future contributors a stable starting point

### 2. One-Command Repro Bundle

Keep a single command or script that captures the minimum data needed to reproduce a visual problem.

Recommended bundle contents:

- config file snapshot
- terminal size
- RNG seed
- short runtime log
- optional screenshot or frame dump

Why this helps:

- turns “it looked wrong once” into something concrete
- makes bug reports actionable
- gives a stable handoff artifact for future debugging

### 3. Config Lint

Validate config before or during startup.

Useful checks:

- required keys are present
- numeric values are in range
- deprecated keys are flagged
- conflicting settings are warned about
- mixed placement models are called out clearly

Why this helps:

- catches subtle bad states early
- reduces runtime guesswork
- makes docs and runtime drift less likely

### 4. Golden Scene Snapshots

Keep a few known-good reference renders for comparison.

Best candidates:

- hero-only baseline
- scaffold-visible baseline
- dense vines baseline
- debug overlay baseline

Why this helps:

- catches layout regressions quickly
- makes “this felt better before” measurable
- creates a stable visual contract for future changes

### 5. Decision Log

Record major architectural or tuning decisions in short dated entries.

Good entries include:

- why a mask strategy changed
- why a field or guide approach was chosen
- why a config model was kept specialized instead of generalized
- what was intentionally not changed

Why this helps:

- prevents repeated debates over settled choices
- gives future work the rationale behind the current shape
- keeps the project easier to resume after a pause

### 6. Tuning Log

Track concrete tuning passes separately from higher-level decisions.

Each entry should capture:

- what changed
- why it changed
- what improved
- what got worse

Why this helps:

- visual systems often regress in one area while improving another
- a tuning log makes those tradeoffs explicit

### 7. Issue Template

Standardize bug reports and tuning requests.

Suggested fields:

- terminal size
- config preset
- RNG seed
- expected result
- actual result
- screenshot or frame reference
- notes about whether the issue is layout, mask, growth, render, or docs related

Why this helps:

- reduces back-and-forth
- speeds up triage
- makes reports comparable

### 8. Change Categories

Use consistent labels when tracking work.

Suggested categories:

- `layout`
- `mask`
- `growth`
- `render`
- `docs`
- `tooling`
- `debt`

Why this helps:

- makes project history searchable
- shows where time is being spent
- helps distinguish functional change from documentation or cleanup

## Recommended Priority

If only a few process improvements are adopted first, use this order:

1. scene recipes
2. one-command repro bundle
3. config lint
4. golden scene snapshots
5. decision log

That sequence gives quick practical value without requiring a large process overhaul.

## Minimal Operating Rules

- prefer named presets over ad hoc config edits when comparing behavior
- write down seeds and terminal sizes when a scene is being evaluated
- treat visual regressions as reproducible unless proven otherwise
- record major rationale in docs instead of relying on memory
- keep the process lightweight enough that it is actually used

## Relationship To Other Docs

This process note complements:

- [MASKS_AND_GUIDES.md](MASKS_AND_GUIDES.md)
- [DEV_TOOLS.md](DEV_TOOLS.md)
- [CONFIG.md](CONFIG.md)
- [STATUS.md](STATUS.md)

Together they describe the spatial model, the intended tooling, the live config surface, and the maintenance context.
