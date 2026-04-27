# Repo Hygiene

## Baseline

- Rust code in `src/` is the active implementation tree
- `Cargo.toml`, `Cargo.lock`, `src/`, `assets/`, and `docs/` are the tracked surface
- the old reconstruction archive has been removed

## Rules

- keep changes logged in `docs/LOG.md`
- keep commits focused
- keep build output out of version control
- do not reintroduce old runtime artifacts without a clear reason
- avoid hidden coupling between engine, render, and UI layers
- use uppercase markdown filenames only for front-door docs or similarly high-visibility entry points; prefer lower-case names for routine contracts and archive notes
