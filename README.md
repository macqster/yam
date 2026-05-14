# yam

Rust terminal scene engine for YAM.

## What This Repo Is

This repository contains the active Rust runtime tree for YAM.

Current release: `0.3.8`

## What YAM Is

YAM is a datum-first terminal scene engine built around:

- a main visualiser scene
- a sandbox world
- a future greenhouse/lab direction
- explicit world-space projection and debug-friendly spatial tooling

The current Ghostty baseline uses a hidden titlebar with a `120x31` window config, which opens to about `124x32` usable cells on the current macOS setup.

Shared terminology lives in [docs/glossary.md](docs/glossary.md).

## Current Runtime Surface

Implemented now:

- boot/loading world
- main scene world
- sandbox world
- hero GIF with tree-stump scaffold
- world-attached clock, weather, and one-line Polish date companions
- procedural vines framing the main-scene composition
- compact dev surface family for help, move, settings, palette, weather, and quit-confirm flows

Reserved or future-facing:

- `calendar` companion seam remains reserved
- greenhouse/lab spaces remain future work
- flora expansion beyond the current scaffold and vine prototype remains future work

## Runtime Commands

- `yam` is the canonical launcher command
- `yam-sandbox` launches the sparse sandbox world
- `yam-rust` is the current direct runtime binary for debugging or manual execution
- `yam-install` rebuilds and reinstalls the fallback binary and launcher wrappers through the current offline-first update path
- `yam-diagnostics` summarizes the recent local diagnostics sessions or tails the raw NDJSON events
- `q` exits

Launcher behavior:

- `yam` and `yam-sandbox` now prefer the installed runtime binary
- if the repo checkout exists and the installed binary is missing or older than repo runtime inputs, the launcher refreshes through the current `yam-install` path before launch
- set `YAM_USE_REPO_RUN=1` only when you intentionally want the older direct `cargo run --release` development path
- set `YAM_DIAGNOSTICS=1` to write a small local NDJSON diagnostics log for install/runtime timing under `~/.local/state/yam/diagnostics.ndjson` (or `$XDG_STATE_HOME/yam/diagnostics.ndjson`)
- use `yam-diagnostics` to read the most recent local diagnostics sessions; add `--tail` to print raw events or `--session <id>` to focus on one run

## Current Priorities

Current project priority is:

1. stability
2. efficiency
3. UI/docs contract cleanup
4. only then broader new surface work

Hero aesthetics are intentionally held steady while runtime and contract seams are cleaned up.

## Document Roles

Use the docs by role:

- [docs/README.md](docs/README.md) - documentation map
- [TODO.md](TODO.md) - active execution backlog
- [known_issues.md](known_issues.md) - active unresolved issues only
- [docs/audit.md](docs/audit.md) - current risk and drift snapshot
- [docs/LOG.md](docs/LOG.md) - append-only history

## Key Links

- [docs/architecture.md](docs/architecture.md) - architecture contract
- [docs/rendering.md](docs/rendering.md) - render/layer contract
- [docs/scene-model.md](docs/scene-model.md) - deterministic scene model
- [docs/weather-widget.md](docs/weather-widget.md) - weather-widget contract
- [docs/vines.md](docs/vines.md) - vine ownership contract
- [docs/resource-map.md](docs/resource-map.md) - research/reference map
- [docs/archive/README.md](docs/archive/README.md) - historical archive index

## Working Rules

- keep terminology aligned with [docs/glossary.md](docs/glossary.md)
- keep active behavior contracts in the relevant docs under `docs/`
- keep `TODO.md` execution-focused
- keep `known_issues.md` issue-focused
- keep `docs/audit.md` risk-focused
- keep `docs/LOG.md` append-only
- keep build output and runtime cache artifacts out of the repo

## Maintenance

- the repo is pinned to stable Rust through `rust-toolchain.toml`
- run `scripts/verify.sh` for the full maintenance/release gate
- run `scripts/check.sh` for the smaller Rust-only gate
- run `scripts/check-docs.sh` for active-doc hygiene batches
- `cargo clippy -- -D warnings` must pass without warnings
- use `docs/README.md` when you need the docs map

## Environment Assumptions

- UTF-8 braille support is required for hero rendering
- full-color terminal output is recommended
- the app is tested primarily in Kitty-family and Ghostty-like terminals
