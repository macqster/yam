# yam-rust

Terminal scene engine in Rust.

## What This Repo Is

This repository contains the active Rust runtime tree for YAM-Rust.

## Quick Links

- [`docs/README.md`](docs/README.md) - docs map
- [`TODO.md`](TODO.md) - active backlog
- [`docs/LOG.md`](docs/LOG.md) - append-only repo log
- [`docs/scene-model.md`](docs/scene-model.md) - deterministic scene model
- [`docs/architecture.md`](docs/architecture.md) - architecture contract
- [`docs/rendering.md`](docs/rendering.md) - render order contract
- [`docs/audit.md`](docs/audit.md) - current repo audit
- [`docs/config.md`](docs/config.md) - scene config ownership note
- [`docs/archive/README.md`](docs/archive/README.md) - historical reports and reviews

## Runtime

- `yam-rust` is the installed command
- `yam-install` rebuilds and reinstalls the binary
- `q` exits

## Current Work

The active work order lives in [`TODO.md`](TODO.md).

## Working Rules

- keep changes logged in `docs/LOG.md`
- keep build output out of the repository
- avoid reintroducing old runtime artifacts unless explicitly needed
- reserve uppercase markdown filenames for the repo front door and other high-visibility entry points
- use `docs/README.md` when you need the docs map

## Maintenance

- run `scripts/check.sh` before committing
- `cargo clippy -- -D warnings` must pass without warnings
- keep active behavior contracts in the relevant docs under `docs/`
