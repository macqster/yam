# yam

`yam` is the repo for the application, runtime, and spec work around the YAM v1/v2 split.

## Current focus

- `v2/` is the default runtime path
- `v1/` remains the legacy baseline
- `docs/v2/` is the working specification and tracking area
- `bin/yam` is the launcher

## What moved out

Terminal startup assets such as Ghostty, Fastfetch, and Kitty now live in a separate dotfiles repo.

See [`docs/DOTFILES_MIGRATION.md`](docs/DOTFILES_MIGRATION.md) for the split note.

## Working rules

- keep changes logged in `docs/v2/LOG.md`
- keep the runtime split explicit
- do not reintroduce shell/bootstrap assets into this repo without a deliberate decision
