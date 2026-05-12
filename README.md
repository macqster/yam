# yam-rust

Terminal scene engine in Rust.

## What This Repo Is

This repository contains the active Rust runtime tree for YAM-Rust.

## What YAM Is

YAM is a centered, datum-first Rust terminal simulation engine for a main visualiser scene and a greenhouse simulation space, driven by explicit world-space projection, spatial debugging, and a shared terminology glossary.

The current Ghostty baseline uses a hidden titlebar with a `120x31` window config, which opens to about `124x32` usable cells on the current macOS setup.

Terminology lives in [`docs/glossary.md`](docs/glossary.md); scene and render contracts point back to it rather than redefining terms inline.

## Implemented Runtime Surfaces

- boot/loading world
- main scene world
- sandbox world
- hero GIF with tree-stump-like scaffolding
- world-attached clock, weather, and one-line Polish date companions
- procedurally generated vines framing the main-scene composition
- dev popup family for hotkeys, move, settings, palette, and weather inspection

## Early / Contracted Organic Surfaces

- tree-stump-like hero scaffold as the current boot/main-scene organic anchor
- tropical vines as the current live flora prototype
- weather sprite source-art workflow kept separate from runtime rendering ownership

## Future Greenhouse Direction

- separate rooms and labs for plant development
- pots and bowls for growth procedures
- controlled-environment and biome-like themes
- multiple flora species with distinct anatomy and lifecycle behavior

## Non-Goals

- not a generic text UI toolkit
- not a screen-space-only layout engine
- not a second camera/viewport authority
- not a raster mask or fill system in place of world-space spatial relations
- not a single-vine special case; flora are a family of procedurally growing organisms with distinct anatomies and morphologies
- not a place to redefine core terms in multiple docs when [`docs/glossary.md`](docs/glossary.md) already owns the vocabulary

## Quick Links

- [`docs/README.md`](docs/README.md) - docs map
- [`docs/glossary.md`](docs/glossary.md) - shared terminology source of truth
- [`docs/weather-widget.md`](docs/weather-widget.md) - canonical weather-widget design brief
- [`docs/vines.md`](docs/vines.md) - pre-runtime vine ownership contract
- [`TODO.md`](TODO.md) - active backlog
- [`known_issues.md`](known_issues.md) - active unresolved issue tracker
- [`docs/LOG.md`](docs/LOG.md) - append-only repo log
- [`docs/scene-model.md`](docs/scene-model.md) - deterministic scene model
- [`docs/architecture.md`](docs/architecture.md) - architecture contract
- [`docs/rendering.md`](docs/rendering.md) - render order contract
- [`docs/audit.md`](docs/audit.md) - current repo audit
- [`docs/resource-map.md`](docs/resource-map.md) - future-development resource map
- [`docs/config.md`](docs/config.md) - scene config ownership note
- [`docs/archive/README.md`](docs/archive/README.md) - historical reports and reviews

## Runtime

- `yam` is the canonical launcher command
- `yam` prefers the live repo at `~/_git/yam` when present and opens YAM in a new Ghostty window on the current macOS setup
- `yam-sandbox` does the same for the sparse sandbox world
- `yam-rust` remains the direct runtime binary for debugging or manual execution
- `yam-install` rebuilds and reinstalls the fallback binary and launcher wrappers
- normal startup should hand off straight into the YAM boot/loading screen rather than printing runtime identity before the TUI takes over, and fresh boots should start from a clean non-dev UI state by default unless a future diagnostic run explicitly opts into preserving UI state
- `q` exits

## Current UI Surface

- dev mode exposes a compact popup family for `hotkeys`, `move`, and `settings`
- the settings popup currently owns:
  - `positions` for camera / hero / clock / weather / live `date` offsets, plus the still-reserved `calendar` seam
  - `ui` for world frame, axis, datum, scrollbars, and debug info panel visibility
  - `features` for persisted main-scene vine visibility policy
- hero, clock, weather, and the one-line Polish `date` companion are treated as one world-attached companion composition, while the sibling `calendar` seam remains reserved for future work or clock-cluster expansion
- the main scene now includes a first compact weather-layer scaffold using a cached weather snapshot and native Ratatui rendering; runtime currently attempts background `wttr.in` refresh and falls back to a static prototype snapshot if live fetch fails
- weather presentation is intentionally split into atlas / layout / localized-text seams so the current compact widget can evolve without re-owning fetch and normalization code, and sprite-shape trials can be inspected comparatively in the dedicated `[W]eather` dev popup
- the current compact weather presentation now leans toward a `wttr.in`-style left-sprite/right-facts structure while staying fully native to YAM
- runtime UI state is persisted at `~/.config/yam/state.json`, and clean startup now preserves these durable settings while still clearing transient dev/modal state
- popup chrome follows the BTAS/TNBA theme direction documented in [`docs/theme.md`](docs/theme.md)
- the canonical weather-widget direction lives in [`docs/weather-widget.md`](docs/weather-widget.md), with `wttr.in` as the preferred first provider, a provider-neutral internal model, and YAM-owned Ratatui sprite rendering

## Current Work

The active work order lives in [`TODO.md`](TODO.md), the current risk snapshot lives in [`docs/audit.md`](docs/audit.md), the append-only record of decisions lives in [`docs/LOG.md`](docs/LOG.md), and future-development resource scouting lives in [`docs/resource-map.md`](docs/resource-map.md).

Current priority is stability and efficiency first, with hero GIF aesthetics held steady and flora deferred until the system is prepared for it.

## Working Rules

- keep changes logged in `docs/LOG.md`
- keep terminology aligned with `docs/glossary.md`
- keep build output out of the repository
- avoid reintroducing old runtime artifacts unless explicitly needed
- reserve uppercase markdown filenames for the repo front door and other high-visibility entry points
- use `docs/README.md` when you need the docs map

## Environment Assumptions

- Terminal support for UTF-8 braille characters is required for hero rendering.
- Full color output is recommended.
- The app is tested primarily in Kitty and similar terminals.

## Maintenance

- the repo is pinned to the stable Rust toolchain through `rust-toolchain.toml`
- run `scripts/check.sh` before committing
- `cargo clippy -- -D warnings` must pass without warnings
- keep active behavior contracts in the relevant docs under `docs/`
- keep `TODO.md` execution-focused, `docs/audit.md` risk-focused, and `docs/LOG.md` append-only
- keep `known_issues.md` focused on active unresolved issues with timestamped tagged entries
