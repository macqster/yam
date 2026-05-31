# YAM Project Brief

## What YAM Is

YAM is a Rust terminal project built on Ratatui. It is not a dashboard app.
Its core identity is a calm, world-space visualizer with a strong authored
scene, lightweight companions, atmospheric rendering, and growing internal
simulation ambition.

The current application already has:

- a main scene world
- a sparse sandbox world
- a Chafa-backed hero render path
- scaffold and vine composition around the main scene
- clock, date, and weather companions
- modal and debug surfaces
- a first shared organism vocabulary in progress

Current selectable worlds:

- `MainScene`
- `Sandbox`

Important baseline note:

- `Boot` exists as a loading/transition world but is intentionally
  non-selectable

## What YAM Is Not

YAM is not:

- a generic productivity dashboard
- a management sim focused on chores or optimization loops
- a plant database UI with decorative art around it
- a feature pile where render code secretly owns simulation truth

## 0.4 Expansion Intent

Version `0.4` is a preparation-and-expansion phase, not a license for scope
sprawl.

The main expansion direction is toward a future greenhouse ecosystem and richer
organism/world infrastructure. The important part is to do this deliberately.

This means `0.4` should be read as:

- architecture consolidation
- greenhouse/world contract shaping
- flora/runtime generalization prep
- careful new planning inputs

It should not be read as:

- immediate feature flood
- broad new gameplay loops
- greenhouse visuals before greenhouse ownership
- ad hoc secondary projection or world models

The repo is intentionally protecting these priorities:

1. preserve architecture coherence
2. keep projection/spatial ownership singular
3. keep render layers read-only
4. grow flora/runtime vocabulary before multiplying organism types
5. introduce greenhouse space as world-attached simulation, not UI chrome

## Current Greenhouse Direction

The greenhouse is a future separate simulation world or world-internal room
model.

It should eventually support:

- rooms or labs
- supports and fixtures
- planting sites
- symbolic environment profiles
- inspection and journaling surfaces
- multiple flora families
- careful curation of organisms that may later be promoted toward the main
  scene

But the first pass must stay much smaller:

- one inert nursery or propagation room
- tiny capacity
- read-only inspection bias
- no game-like progression pressure
- no broad lifecycle/persistence system yet

## Current Expansion Posture

The current healthy external planning posture is:

- propose bounded options
- stay close to the current seams
- separate first-pass contract work from later richer simulation
- help build naming, vocabulary, and sequencing clarity
- treat the greenhouse as a place with rules, not as a decorative idea board

## What ChatGPT Is Best Used For Here

ChatGPT is useful for:

- bounded room concepts
- fixture and support catalogs
- naming passes
- symbolic environment vocabularies
- inspection/journal tone and small text surfaces
- staged option sets with tradeoffs
- coherence checking across notes

ChatGPT is less useful if asked to:

- invent architecture without constraints
- propose large implementation plans divorced from current seams
- act as if YAM were a normal GUI dashboard or farming sim
- flood the project with systems that the repo is not structurally ready for
