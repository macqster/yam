# YAM v2 Runtime Loop

This document records the current minimal runtime behavior.

## Flow

```text
build model
build ecosystem
optional input message
tick messages
render frame
print frame
```

## Current Messages

- `TickMsg`
- `ResizeMsg`
- `KeyMsg`

## Current Behavior

- `TickMsg` advances the runtime tick and nudges non-hero organisms
- `ResizeMsg` updates the model dimensions
- `KeyMsg(key="spawn")` appends a deterministic seed organism
- the Bubble Tea runtime shell is the default launch path
- the Python loop remains available as a compatibility fallback

## Constraints

- keep the loop deterministic
- do not touch the legacy visualizer runtime
- keep the runtime dispatcher small and explicit
