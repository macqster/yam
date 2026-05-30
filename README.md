# YAM

> A Rust/Ratatui terminal scene engine: animated hero rendering, world-space
> projection, compact companion widgets, BTAS-inspired UI styling, and a
> maintenance-first path toward richer procedural scene life.

Current release: `0.3.9`

> Preview note: the front door currently ships without committed screenshot/demo
> assets. Add preview media only when the files are checked into the repo and the
> README paths are live.

---

## Status

YAM is in active development on the Rust/Ratatui runtime path.

Current posture:

- runtime-first
- maintenance-first
- visual polish after contract stability
- no claim of being a general-purpose dashboard framework

## Quick Start

```bash
git clone https://github.com/macqster/yam.git
cd yam
cargo run --release
```

Sandbox:

```bash
cargo run --release -- --sandbox
```

Runtime identity check:

```bash
cargo run --release -- --identity
```

Run the full maintenance gate:

```bash
bash scripts/verify.sh
```

## What This Repo Is

This repository contains the active Rust runtime for YAM.

YAM is not a generic dashboard framework. It is a curated terminal scene engine
with a small set of deliberate runtime surfaces, strict world/viewport semantics,
and a documentation model that treats architecture, rendering, config, and
surface contracts as first-class project artifacts.

## What YAM Does Today

YAM currently provides:

- a boot/loading world with staged visual transitions
- a main scene world
- a sparse sandbox world
- a Chafa-backed animated hero layer
- a tree-stump scaffold and procedural vine framing layer
- world-attached clock, weather, and one-line Polish date companions
- a compact weather widget with plain-text weather sprites and localized facts
- dev surfaces for help, move mode, settings, palette inspection, weather review,
  and quit confirmation
- a local diagnostics path for install/runtime timing
- maintenance scripts for docs, formatting, linting, checking, and tests

Reserved or future-facing surfaces:

- `calendar` companion seam
- greenhouse/lab spaces
- flora expansion beyond the current scaffold and vine prototype
- deeper terminal-art asset compilation and inspection workflows

## Design Identity

YAM aims for a restrained dark-deco terminal aesthetic:

- graphite backgrounds
- muted blue and green accents
- warm companion text
- compact HUD/footer information
- composed scene layout over generic widget density

The project should feel like a coherent terminal diorama rather than a pile of
panels.

## Launcher Commands

| Command | Purpose |
| --- | --- |
| `yam` | Canonical launcher command after wrapper installation. |
| `yam-sandbox` | Launches the sparse sandbox world after wrapper installation. |
| `yam-rust` | Direct Rust runtime binary. Useful for debugging and manual execution. |
| `yam-install` | Rebuilds/reinstalls the runtime and launcher wrappers through the current update path. |
| `yam-diagnostics` | Summarizes recent local diagnostics sessions or tails raw NDJSON events. |

## Runtime Controls

| Key | Action |
| --- | --- |
| `q` | Exit the runtime. |

### Direct Development Run

```bash
cargo run --release
```

Sandbox run:

```bash
cargo run --release -- --sandbox
```

Runtime identity check:

```bash
cargo run --release -- --identity
```

### Installed Wrapper Path

The wrapper path is intended for the normal local workstation flow:

```bash
bash scripts/update.sh
```

After that, use:

```bash
yam
```

or:

```bash
yam-sandbox
```

Launcher behavior:

- `yam` and `yam-sandbox` prefer the installed `yam-rust` binary
- if the repo checkout exists and repo runtime inputs are newer than the
  installed binary, the launcher refreshes through `scripts/update.sh`
- set `YAM_USE_REPO_RUN=1` only when intentionally using the older direct
  `cargo run --release` development path
- set `YAM_DIAGNOSTICS=1` to write local NDJSON diagnostics to
  `~/.local/state/yam/diagnostics.ndjson` or
  `$XDG_STATE_HOME/yam/diagnostics.ndjson`
- use `yam-diagnostics` to read recent diagnostics sessions; add `--tail` to
  print raw events or `--session <id>` to focus on one run

## Architecture at a Glance

```text
runtime state / config
        ↓
scene model + entity placement
        ↓
world → viewport projection
        ↓
layer assembly
        ↓
Ratatui frame output
```

Core boundaries:

- world-space entities are not HUD elements
- viewport changes affect framing, not scene scale
- debug surfaces are development aids, not canonical scene content
- visual changes should remain synchronized with runtime identity and docs
  contracts

## Repository Shape

| Path | Role |
| --- | --- |
| `src/core/` | Core world, grid, spatial, guide, entity, and flora primitives. |
| `src/render/` | Rendering/compositor path, Chafa integration, hero cache, fonts, masks, and draw helpers. |
| `src/scene/` | Scene orchestration, camera/viewport logic, and render layers. |
| `src/scene/layers/` | Runtime layers for hero, loading, clock, date, weather, vines, debug, modal, status, and related surfaces. |
| `src/ui/` | UI state, anchors, scene-layer assembly, and reusable widgets. |
| `src/weather/` | Weather model, provider, wttr normalization, layout, text, render path, and sprite atlas. |
| `src/theme/` | BTAS palette, glyphs, style, and semantic render helpers. |
| `src/systems/` | Early simulation/system seams for growth, aging, density, fields, constraints, and ticks. |
| `assets/` | Runtime visual/font assets. |
| `bin/` | Local launcher wrappers. |
| `scripts/` | Maintenance, check, verify, and update scripts. |
| `tools/` | Experiments and archived legacy Python prototype material. |
| `docs/` | Active contracts, design notes, release model, palette references, and archive entry points. |

## Documentation Map

### Start Here

| Document | Role |
| --- | --- |
| [AGENTS.md](AGENTS.md) | Agent-facing operating guidance. |
| [skills/yam-maintenance/SKILL.md](skills/yam-maintenance/SKILL.md) | Repo-local maintenance workflow skill. |
| [skills/yam-architecture-review/SKILL.md](skills/yam-architecture-review/SKILL.md) | Repo-local architecture review workflow skill. |
| [docs/README.md](docs/README.md) | Documentation index and routing map. |
| [TODO.md](TODO.md) | Active execution backlog. |
| [known_issues.md](known_issues.md) | Active unresolved issues only. |
| [docs/audit.md](docs/audit.md) | Current risk and drift snapshot. |
| [docs/LOG.md](docs/LOG.md) | Append-only project history. |

### Core Contracts

| Document | Role |
| --- | --- |
| [docs/glossary.md](docs/glossary.md) | Shared terminology source of truth. |
| [docs/architecture.md](docs/architecture.md) | Ownership and implementation architecture. |
| [docs/scene-model.md](docs/scene-model.md) | Deterministic scene/world model. |
| [docs/rendering.md](docs/rendering.md) | Render order, layering, and UI/render contracts. |
| [docs/config.md](docs/config.md) | Configuration authority and runtime scope. |
| [docs/hygiene.md](docs/hygiene.md) | Repo hygiene and drift-prevention rules. |

### Active Surface Contracts

| Document | Role |
| --- | --- |
| [docs/loading-screen.md](docs/loading-screen.md) | Boot/loading-screen contract. |
| [docs/weather-widget.md](docs/weather-widget.md) | Weather-widget contract. |
| [docs/vines.md](docs/vines.md) | Vine ownership/readiness contract. |
| [docs/hero-cache.md](docs/hero-cache.md) | Hero-frame cache design and runtime path. |
| [docs/theme.md](docs/theme.md) | Reusable BTAS theme contract. |
| [docs/release-model.md](docs/release-model.md) | Branch and release policy. |
| [docs/resource-map.md](docs/resource-map.md) | Research/reference map. |

## Maintenance Gates

| Gate | Purpose |
| --- | --- |
| `bash scripts/check-docs.sh` | Active-doc existence, version sync, issue-link hygiene, and optional markdown/spell checks. |
| `bash scripts/check.sh` | Rust formatting, clippy, and cargo check. |
| `cargo test --quiet` | Test suite. |
| `bash scripts/verify.sh` | Full maintenance/release gate. |

The repo is pinned to stable Rust through [`rust-toolchain.toml`](rust-toolchain.toml).

## Current Priorities

Current project priority is:

1. stability
2. efficiency
3. UI/docs contract cleanup
4. broader new surface work

Near-term pressure points tracked in the repo include:

- spatial relation cleanup
- hero startup/cache efficiency
- continued theme/surface convergence
- keeping visual changes synchronized with runtime identity checks
- keeping flora expansion behind stable scene and render contracts

Hero aesthetics are intentionally held steady while runtime and contract seams are
cleaned up.

## Non-goals

YAM is not currently trying to be:

- a general-purpose terminal dashboard framework
- a generic widget library
- a full simulation engine
- a configurable end-user product
- a replacement for the project’s deeper contract docs

## Versioning and History

Current release: `0.3.9`

- release policy: [docs/release-model.md](docs/release-model.md)
- project history: [docs/LOG.md](docs/LOG.md)
- active issues: [known_issues.md](known_issues.md)

## Working Rules

- keep terminology aligned with [docs/glossary.md](docs/glossary.md)
- keep active behavior contracts in the relevant docs under `docs/`
- keep `TODO.md` execution-focused
- keep `known_issues.md` issue-focused
- keep `docs/audit.md` risk-focused
- keep `docs/LOG.md` append-only
- keep build output and runtime cache artifacts out of the repo
- update README claims only when the repo structure or runtime behavior supports
  them

## Environment Assumptions

- UTF-8 braille support is required for hero rendering
- full-color terminal output is recommended
- the app is tested primarily in Kitty-family and Ghostty-like terminals
- the current local Ghostty baseline uses a hidden titlebar with a `120x31`
  window config, opening to about `124x32` usable cells on the current macOS
  setup
