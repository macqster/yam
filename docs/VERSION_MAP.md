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
- documented by [`docs/v2/README.md`](v2/README.md)

## Working Rule

- keep release history in branches/tags, not in source directory names
- add new work to the canonical root runtime tree
- do not blur the root runtime and the legacy visualizer in the same module tree

## Launcher Rule

- default `yam` to the root runtime
- use `yam --version v1` for the legacy visualizer
- preserve the visualizer recipe path as the compatibility fallback
