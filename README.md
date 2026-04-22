# yam

`yam` is the application and runtime repo for the current terminal scene.

## Current focus

- `bin/yam` is the launcher
- the runtime now lives at the repository root
- `check_golden.py` and `scene_config.json` are rooted
- the Go runtime packages live at the repository root
- `yam-go` is the stable, feature-complete clock-only visualizer line
- `yam-rust` is the experimental engine-first line
- `docs/v2/` is the historical spec and tracking area
- [`docs/RELEASE_MODEL.md`](docs/RELEASE_MODEL.md) records the stable, `yam-go`, and `yam-rust` branch policy
- [`docs/FLATTENING_PLAN.md`](docs/FLATTENING_PLAN.md) records the move toward one canonical root runtime tree

## What moved out

Terminal startup assets such as Ghostty, Fastfetch, and Kitty live in the separate dotfiles repo.

See [`docs/DOTFILES_MIGRATION.md`](docs/DOTFILES_MIGRATION.md) for the split note.

## Working rules

- keep changes logged in `docs/v2/LOG.md`
- keep the live runtime path explicit
- do not reintroduce shell/bootstrap assets into this repo without a deliberate decision
- do not let the clock visualizer pick up engine-first complexity
- keep `yam-go` focused on the clock-only scene
