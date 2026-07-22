# YAM Release Model

This repository currently uses one active Rust maintenance branch. It does
not use release cuts in the conventional sense — see Distribution below.

## Distribution

- No public release will ever be provided for this project: no GitHub
  Releases, no git tags marking a release point, no prebuilt binaries, no
  installers, and no distribution channel of any kind.
- The only way to run YAM is to clone the repository and build it from
  source with `cargo`, using the local launchers described in `README.md`.
- This is a deliberate, standing decision, not a temporary gap waiting on
  tooling or CI work. A request for prebuilt binaries (`#4` on GitHub,
  2026-05-21) was closed as not planned on that basis.
- The version numbers below (`Cargo.toml`, `README.md`'s current-release
  line) are internal development milestones only. They track how far the
  runtime has progressed for maintenance and docs purposes; they do not
  correspond to, and never will correspond to, a published release artifact.
  Do not read a version bump as an announcement that something is available
  to download — nothing ever will be.

## Active Branch

- `main` is the active branch for ongoing Rust runtime maintenance.
- Current stabilization, review, and feature-prep work happens directly on `main`.
- Historical branch models such as `stable` / `experimental` are not the active workflow for the current repo state unless a future process explicitly reinstates them.

## Versioning

- The development version advances monotonically: `0.2`, `0.3`, `0.4`, and so on — see Distribution above for what these numbers do and do not mean.
- The current version should stay explicit in the repo and docs, with `Cargo.toml` and the root `README.md` kept in sync as the canonical package/front-door pair.
- `docs/LOG.md` and `CHANGELOG.md` carry the long-term change history in place of tags or release notes; the branch itself stays focused on the active maintenance line.

## Workflow

1. Keep `main` green under `bash scripts/verify.sh`. CI (`.github/workflows/verify.yml`) runs the same gate on every push and pull request, and `main` is branch-protected to require it before merging.
2. Land maintenance batches directly on `main`, or through a short-lived branch/PR for a larger consolidated pass, with docs/tests/log updates in the same change either way.
3. When a branch merges through a PR, use a real merge commit — the repo only allows that merge method (squash and rebase-merge are disabled) so a consolidated pass's individually-verified commits stay visible in `main`'s history rather than collapsing into one blob. Head branches delete automatically on merge.
4. Document any future change to this model, including the Distribution decision above, here before treating it as active workflow.

## Repo Shape Goal

- Keep one canonical runtime tree in the repository root.
- Avoid version-number directory splits once the codebase is flattened.
- Keep old branch context in `docs/LOG.md` and `CHANGELOG.md`, not in directory names or tags.
