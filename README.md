# yam-rust

Terminal scene engine in Rust.

## Status

Active Rust rewrite with a persistent world, viewport camera, panel-based UI, anchor-based overlays, and a live terminal loop.

## Current layout

- `src/core/` - world, grid, cell, entity, fields
- `src/systems/` - tick pipeline and empty system scaffolding
- `src/render/` - clock, hero, fonts, and render helpers
- `src/ui/` - layout, panels, camera, viewport, scene composition, and debug overlays

## Runtime

- `yam-rust` is the installed command
- `yam-install` rebuilds and reinstalls the local binary
- `q` exits

## Hygiene

- keep changes logged in `docs/v2/LOG.md`
- keep `Cargo.toml`, `Cargo.lock`, `src/`, `assets/`, and docs under version control
- avoid reintroducing Go-era root artifacts unless explicitly needed for comparison
