# Workflow and Replay Takeaways

This extract captures the process and reproducibility lessons from the research note.

## Authoring and replay are important

Notable workflow patterns from the wider ecosystem:

- named presets and saved configurations
- frame-by-frame preview
- deterministic replay
- structured session recording
- TUI-based live tweaking

Practical implication for YAM:

- recipe presets are worth keeping
- seeded replay and golden snapshots are good tooling targets
- if something cannot be reproduced, it is too hard to tune

## Structure vs randomness

Organic visuals usually mix:

- deterministic structure for coherence
- stochastic variation for life

This shows up in:

- growth systems
- noise-driven placement
- procedural branching
- cellular and rule-based evolution

Practical implication for YAM:

- support/routing logic should stay structured
- ornament and branch variation can stay probabilistic
- the system should not collapse into pure randomness or pure geometry

## Good workflow primitives

Useful additions for this repo:

- named recipes for common states
- seeded replay for repeatable growth
- golden frame snapshots for regression detection
- debug overlays that reflect the actual runtime state
- concise repro bundles with config, seed, and terminal size

## Debugging order

When the scene looks wrong, this order usually saves time:

1. layout and masks
2. fields and guides
3. growth behavior
4. glyph selection
5. final render polish

That matches the broader lesson from the research: if the underlying scene is wrong, no amount of output tweaking will fully fix it.
