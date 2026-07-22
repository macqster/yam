# YAM Release Model

This repository currently uses one active Rust maintenance branch with lightweight release cuts.

## Active Branch

- `main` is the active branch for ongoing Rust runtime maintenance.
- Current stabilization, review, and release-prep work happens directly on `main`.
- Historical branch models such as `stable` / `experimental` are not the active workflow for the current repo state unless a future release process explicitly reinstates them.

## Versioning

- Releases advance monotonically: `0.2`, `0.3`, `0.4`, and so on.
- The current branch version should stay explicit in the repo, docs, and release notes, with `Cargo.toml` and the root `README.md` kept in sync as the canonical package/front-door pair.
- Tags and release notes should carry the long-term release history; the branch itself stays focused on the active maintenance line.

## Workflow

1. Keep `main` green under `bash scripts/verify.sh`. CI (`.github/workflows/verify.yml`) runs the same gate on every push and pull request, and `main` is branch-protected to require it before merging.
2. Land maintenance batches directly on `main`, or through a short-lived branch/PR for a larger consolidated pass, with docs/tests/log updates in the same change either way.
3. Cut and tag releases from the clean pushed `main` state when the branch is ready.
4. Document any future branch-model change here before treating it as active workflow.

## Repo Shape Goal

- Keep one canonical runtime tree in the repository root.
- Avoid version-number directory splits once the codebase is flattened.
- Keep old branch context in tags, logs, and release notes, not in directory names.
