# YAM Release Model

This repository uses two branches and continuous versioning.

## Branches

- `stable` holds the last released version.
- `experimental` holds the next release in progress.

## Versioning

- Releases advance monotonically: `0.2`, `0.3`, `0.4`, and so on.
- `stable` always corresponds to the latest released version.
- `experimental` always corresponds to the next version under active development.

## Workflow

1. Cut a stable release from `experimental`.
2. Tag the release.
3. Merge or fast-forward the result into `stable`.
4. Continue development in `experimental` for the next version.

## Repo Shape Goal

- Keep one canonical runtime tree in the repository root.
- Avoid version-number directory splits once the codebase is flattened.
- Keep old branch context in tags, logs, and release notes, not in directory names.
