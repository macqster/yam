# yam-rust

Terminal scene engine in Rust.

## What This Repo Is

This repository contains the active Rust runtime tree for YAM-Rust.

## What YAM Is

YAM is a centered, datum-first Rust terminal simulation engine for a main visualiser scene and a greenhouse simulation space, driven by explicit world-space projection, spatial debugging, and a shared terminology glossary.

The current Ghostty baseline uses a hidden titlebar with a `120x31` window config, which opens to about `124x32` usable cells on the current macOS setup.

Terminology lives in [`docs/glossary.md`](docs/glossary.md); scene and render contracts point back to it rather than redefining terms inline.

## Main Scene

- hero GIF
- tree-stump-like hero scaffolding for the animated figure to sit on
- clock widget
- weather widget
- procedurally generated vines as the organic frame of the composition

## Greenhouse

- separate rooms and labs for plant development
- pots and bowls for growth procedures
- controlled-environment and biome-like themes
- multiple flora species with distinct anatomy and lifecycle behavior

## Current Flora

- pre-generated tree-stump hero scaffolding at boot
- tropical vines framing the composition
- monstera-like plant with large aesthetic growing leaves

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
- [`docs/vines.md`](docs/vines.md) - pre-runtime vine ownership contract
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

The active work order lives in [`TODO.md`](TODO.md), the current risk snapshot lives in [`docs/audit.md`](docs/audit.md), and the append-only record of decisions lives in [`docs/LOG.md`](docs/LOG.md).

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

- run `scripts/check.sh` before committing
- `cargo clippy -- -D warnings` must pass without warnings
- keep active behavior contracts in the relevant docs under `docs/`
- keep `TODO.md` execution-focused, `docs/audit.md` risk-focused, and `docs/LOG.md` append-only
