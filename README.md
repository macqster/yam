# yam

`yam` is the application and runtime repo for the current terminal scene.

## Current focus

- `bin/yam` is the launcher
- the runtime is being flattened into the repository root
- `app.py`, `config.py`, and `check_golden.py` are already at the root
- `v2/` still holds the supporting packages during the flattening pass
- `docs/v2/` is the working specification and tracking area
- the legacy visualizer remains available only as a compatibility path
- [`docs/RELEASE_MODEL.md`](docs/RELEASE_MODEL.md) records the stable/experimental branch policy
- [`docs/FLATTENING_PLAN.md`](docs/FLATTENING_PLAN.md) records the move toward one canonical root runtime tree

## What moved out

Terminal startup assets such as Ghostty, Fastfetch, and Kitty live in the separate dotfiles repo.

See [`docs/DOTFILES_MIGRATION.md`](docs/DOTFILES_MIGRATION.md) for the split note.

## Working rules

- keep changes logged in `docs/v2/LOG.md`
- keep the live runtime path explicit
- do not reintroduce shell/bootstrap assets into this repo without a deliberate decision
