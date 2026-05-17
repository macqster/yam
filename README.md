## What This Repo Is

This repository contains the active Rust runtime tree for YAM.

## What YAM Is

YAM is a datum-first terminal scene engine built around:

- a main visualiser scene
- a sandbox world
- a future greenhouse/lab direction
- explicit world-space projection and debug-friendly spatial tooling

The current Ghostty baseline uses a hidden titlebar with a `120x31` window
config, which opens to about `124x32` usable cells on the current macOS setup.

Shared terminology lives in [docs/glossary.md](docs/glossary.md).

## Runtime Surface

### Implemented Now

- boot/loading world
- main scene world
- sandbox world
- hero GIF with tree-stump scaffold
- world-attached clock, weather, and one-line Polish date companions
- procedural vines framing the main-scene composition
- compact dev surface family for help, move, settings, palette, weather, and
  quit-confirm flows

### Reserved or Future-Facing

- `calendar` companion seam remains reserved
- greenhouse/lab spaces remain future work
- flora expansion beyond the current scaffold and vine prototype remains future
  work

## Runtime Commands

| Command | Purpose |
| --- | --- |
| `yam` | Canonical launcher command. |
| `yam-sandbox` | Launches the sparse sandbox world. |
| `yam-rust` | Current direct runtime binary for debugging or manual execution. |
| `yam-install` | Rebuilds and reinstalls the fallback binary and launcher wrappers through the current offline-first update path. |
| `yam-diagnostics` | Summarizes recent local diagnostics sessions or tails the raw NDJSON events. |
| `q` | Exits the runtime. |

### Launcher Behavior

- `yam` and `yam-sandbox` now prefer the installed runtime binary
- if the repo checkout exists and the installed binary is missing or older than
  repo runtime inputs, the launcher refreshes through the current `yam-install`
  path before launch
- set `YAM_USE_REPO_RUN=1` only when you intentionally want the older direct
  `cargo run --release` development path
- set `YAM_DIAGNOSTICS=1` to write a small local NDJSON diagnostics log for
  install/runtime timing under `~/.local/state/yam/diagnostics.ndjson` or
  `$XDG_STATE_HOME/yam/diagnostics.ndjson`
- use `yam-diagnostics` to read the most recent local diagnostics sessions; add
  `--tail` to print raw events or `--session <id>` to focus on one run

## Current Priorities

Current project priority is:

1. stability
2. efficiency
3. UI/docs contract cleanup
4. broader new surface work

Hero aesthetics are intentionally held steady while runtime and contract seams
are cleaned up.

## Documentation Map

### Document Roles

| Document | Role |
| --- | --- |
| [docs/README.md](docs/README.md) | Documentation map. |
| [TODO.md](TODO.md) | Active execution backlog. |
| [known_issues.md](known_issues.md) | Active unresolved issues only. |
| [docs/audit.md](docs/audit.md) | Current risk and drift snapshot. |
| [docs/LOG.md](docs/LOG.md) | Append-only history. |

### Key Links

| Document | Role |
| --- | --- |
| [docs/architecture.md](docs/architecture.md) | Architecture contract. |
| [docs/rendering.md](docs/rendering.md) | Render/layer contract. |
| [docs/scene-model.md](docs/scene-model.md) | Deterministic scene model. |
| [docs/weather-widget.md](docs/weather-widget.md) | Weather-widget contract. |
| [docs/vines.md](docs/vines.md) | Vine ownership contract. |
| [docs/resource-map.md](docs/resource-map.md) | Research/reference map. |
| [docs/archive/README.md](docs/archive/README.md) | Historical archive index. |

## Working Rules

- keep terminology aligned with [docs/glossary.md](docs/glossary.md)
- keep active behavior contracts in the relevant docs under `docs/`
- keep `TODO.md` execution-focused
- keep `known_issues.md` issue-focused
- keep `docs/audit.md` risk-focused
- keep `docs/LOG.md` append-only
- keep build output and runtime cache artifacts out of the repo

## Maintenance

| Gate | Purpose |
| --- | --- |
| `scripts/verify.sh` | Full maintenance/release gate. |
| `scripts/check.sh` | Smaller Rust-only gate. |
| `scripts/check-docs.sh` | Active-doc hygiene batches. |
| `cargo clippy -- -D warnings` | Must pass without warnings. |

Additional maintenance notes:

- the repo is pinned to stable Rust through `rust-toolchain.toml`
- use [docs/README.md](docs/README.md) when you need the docs map

## Environment Assumptions

- UTF-8 braille support is required for hero rendering
- full-color terminal output is recommended
- the app is tested primarily in Kitty-family and Ghostty-like terminals
