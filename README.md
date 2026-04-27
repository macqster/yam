# yam-rust

Terminal scene engine in Rust.

## Status

Persistent world, camera, viewport, panels, and live terminal rendering.

## Layout

- `src/core/` - world, grid, cell, entity, fields
- `src/systems/` - tick pipeline and system scaffolding
- `src/render/` - clock, hero, fonts, and render helpers
- `src/ui/` - camera, viewport, panels, layout, scene composition, and debug overlays
- `docs/` - documentation index, active contracts, and archive pointers
- this repository is the active Rust runtime tree; the older `/Users/maciejkuster/_git/yam` checkout has been removed

## Runtime

- `yam-rust` is the installed command
- `yam-install` rebuilds and reinstalls the binary
- `q` exits

## Hygiene

- keep changes logged in `docs/LOG.md`
- keep build output out of the repository
- avoid reintroducing old runtime artifacts unless explicitly needed
- reserve uppercase markdown filenames for the repo front door and other high-visibility entry points

## Maintenance

- run `scripts/check.sh` before committing
- `cargo clippy -- -D warnings` must pass without warnings
- keep the render order contract in `docs/RENDERING.md`
- start with `docs/README.md` when you need the docs map
