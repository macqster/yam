# YAM Version Map

This repo now treats the codebase as a two-track system.

## v1

- current baseline implementation
- legacy visualizer stack
- shell integration and startup rice
- documented by [`v1/README.md`](../v1/README.md)

## v2

- native rebuild source tree
- explicit layer separation
- documented by `docs/v2/`
- documented by [`v2/README.md`](../v2/README.md)

## Working Rule

- keep v1 stable while v2 is reconstructed
- add new work to v2 unless it is explicitly compatibility-related
- do not blur the two tracks in the same module tree

## Launcher Rule

- default `yam` to v2
- use `yam --version v1` for the legacy visualizer
- preserve the v1 recipe path as the compatibility fallback
