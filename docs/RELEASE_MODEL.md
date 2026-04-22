# YAM Release Model

This repository uses a stable release branch, a current Go development branch, and continuous versioning.

## Branches

- `stable` holds the last released version of `yam-go`.
- `yam-go` is the current Go development branch.
- `yam-rust` is the separate experimental track for the engine-first rewrite.

## Versioning

- Releases advance monotonically: `0.2`, `0.3`, `0.4`, and so on.
- `stable` always corresponds to the latest released version.
- `yam-go` always corresponds to the next version under active development.

## Workflow

1. Cut a stable release from `yam-go`.
2. Tag the release.
3. Merge or fast-forward the result into `stable`.
4. Continue development in `yam-go` for the next version.

## Repo Shape Goal

- Keep one canonical runtime tree in the repository root for `yam-go`.
- Avoid version-number directory splits once the codebase is flattened.
- Keep old branch context in tags, logs, and release notes, not in directory names.
- Keep the Rust rework isolated to the `yam-rust` branch until it proves its own value.
