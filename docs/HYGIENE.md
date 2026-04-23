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
