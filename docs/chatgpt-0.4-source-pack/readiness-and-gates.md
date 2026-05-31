# YAM 0.4 Readiness And Gates

## Current Status

As of `2026-05-31`, the repo is in a pre-expansion readiness posture rather
than broad feature-expansion mode.

The current 0.4 gate spine is:

- docs aligned
- verification green
- spatial ownership stable
- flora storage direction selected, exact shape still pending
- greenhouse/world contract decided
- hero/render failure modes hardened

Current practical reading:

- the baseline is healthy enough for deliberate prep work
- greenhouse planning is coherent enough to continue
- some implementation decisions are still intentionally deferred

## What Is Already Strong

- docs and planning ownership are disciplined
- verification is green
- world selection already has an explicit profile seam
- render layers are already treated as read-only visualizers
- greenhouse planning has a real first-pass contract instead of a vague wish
  list

## What Is Still Deliberately Open

- the exact shape of the first generalized flora store
- the exact timing of a dedicated greenhouse doc split from the roadmap
- when inspection should become a broader journal or registry surface
- when one nursery room should widen into a frame-plus-labs runtime structure

## Locked First-Pass Defaults

These defaults should be treated as active unless the repo later changes them
explicitly:

- `core::spatial` stays canonical
- `scene::coords` stays compatibility-only
- first greenhouse implementation starts as pure `core::greenhouse` data
- first room identity is `greenhouse_nursery`
- first pass is symbolic room-level environment, not rich simulation
- first inspection is read-only
- first room capacity is tiny, roughly one to three planting sites
- first species/profile work should prefer static Rust fixtures
- first multi-family flora generalization is biased toward enum-backed storage

## Important Non-Goals For The First Greenhouse Pass

Do not treat these as expected first moves:

- selectable visible greenhouse world immediately
- multiple live labs with tabbed runtime switching
- large species registries
- persistence architecture
- full journaling UI
- game-like chores, economy, or daily loops
- render-owned interaction logic
- broad greenhouse climate automation systems

Useful reframing:

- "not yet" is not rejection
- many richer greenhouse ideas are valid later, but invalid as the first owned
  slice

## Productive Near-Term Output From ChatGPT

Useful outputs right now:

- room profiles
- fixture/support vocabularies
- lab naming passes
- symbolic environment presets
- inspection/journal phrase catalogs
- staged implementation options with risks
- internal coherence reviews against the constraints

Less useful outputs right now:

- giant masterplans with runtime assumptions
- UI-heavy greenhouse dashboards
- speculative persistence schemas
- detailed crop science simulation
- anything that assumes the repo is already structurally greenhouse-ready

## Open Questions That Are Still Legitimate

- when to split out a dedicated `docs/greenhouse.md` from the roadmap
- whether the first visible greenhouse should stay one room for a while or
  quickly widen into a frame-plus-labs structure
- how soon greenhouse inspection should become a dedicated journal/registry
  surface instead of lightweight popups
- exactly when the flora store should generalize beyond the current vine-first
  shape
