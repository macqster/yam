# Repo Audit

Date: 2026-04-23

## Summary

The repository now tracks a Rust terminal scene engine as the active implementation.

The old Go-era root artifacts and the `docs/v2/` archive tree have been removed from the active layout.

## Keep

- `Cargo.toml`
- `Cargo.lock`
- `src/`
- `assets/`
- `docs/`
- `README.md`

## Gaps

- formal automated frame tests
- a stable camera and viewport contract doc
- release notes for the Rust binary and install command

## Next Actions

1. keep the docs current with implementation changes
2. keep the tree buildable after each edit
3. prune temporary output before committing
