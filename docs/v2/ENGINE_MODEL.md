# YAM v2 Engine Model

This document records the current deterministic engine contract.

## State

- `Environment`
- `Lifecycle`
- `Species`
- `Balance`
- `Ecosystem`
- `Organism`

## Update Rule

- the runtime hands `TickMsg` into the ecosystem step
- the ecosystem updates environment and organisms deterministically
- organisms in decay are removed from the active set
- hero organisms remain anchored

## Constraints

- no rendering logic in engine code
- no direct terminal access
- no hidden mutation outside the step contract
