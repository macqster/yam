# YAM Architecture Constraints

This file is the most important upload when ChatGPT is asked to propose new
YAM structures, workflows, or feature phases.

## Ownership Rules

- `core/` owns pure data and simulation vocabulary
- `systems/` may mutate state but must not render
- `scene/` and `render/` visualize existing state
- `ui/` owns modal, settings, and presentation state

When ChatGPT proposes a new concept, it should also imply an owner:

- room/environment/state concepts should point toward `core/`
- mutation/transition logic should point toward `systems/`
- visible composition should point toward `scene/` and `render/`
- temporary controls, settings, and prompts should point toward `ui/`

Render code must not become the hidden owner of:

- greenhouse state
- growth truth
- room selection
- inspection truth
- environment mutation

## Spatial Rules

- `core::spatial` is the canonical shared spatial layer
- `scene::coords` is compatibility-only
- world-space, screen-space, anchor-space, and guide-space should stay distinct
- new proposals should not invent a second projection model
- new renderables should reuse shared spatial/entity pose helpers

Important vocabulary:

- use `points`, `anchors`, `guides`, `lines`, and `polylines` for spatial work
- reserve `node` for plant morphology/anatomy, not generic coordinate capture

## World Rules

- world switching belongs to `WorldKind::SELECTABLE` and `WorldKind::profile()`
- `Boot` remains non-selectable
- `MainScene` and `Sandbox` are the current selectable worlds
- future greenhouse or lab work should route through the same world/profile
  contract instead of becoming a UI-local mode toggle

Useful rule of thumb:

- if a proposal sounds like "open a panel" or "add a tab beside the scene",
  it is probably drifting away from YAM's world-space model
- if a proposal sounds like "enter a room", "switch to a world", or "inspect a
  place", it is much more likely to fit

## Flora Rules

- `core::organism` is the first shared plant identity vocabulary
- `FloraState` is still vine-shaped in storage today
- the current first-pass bias for multi-family storage is an enum-backed family
  store
- do not propose another ad hoc top-level family vector beside vines
- species defaults belong in a registry; per-instance history belongs in
  journals

## Greenhouse Rules

- greenhouse remains a place, not panel chrome
- room selection belongs to greenhouse/world state, not tabs pasted over the
  main scene
- first greenhouse work is functional-space-first:
  `room -> access paths -> zones -> fixtures -> planting sites -> environment profile -> inspection surface`
- the first code-bearing slice should be pure data plus invariant tests
- visible greenhouse rendering should come only after data ownership is proven

The first runtime greenhouse slice should therefore optimize for:

- stable identifiers
- tiny inspectable data structures
- invariant tests
- sparse room vocabulary
- future attachment points

It should not optimize for:

- spectacle
- broad UI
- "fully featured" first impressions
- large species catalogs

## Tone And Product Rules

YAM should stay:

- terminal-native
- calm
- inspectable
- botanical
- art-directed
- world-space-first

YAM should avoid drifting into:

- dashboard shells
- productivity app interaction logic
- harvest/economy gameplay
- decorative greenhouse clutter without ownership
- realism-heavy simulation that the current architecture cannot support
