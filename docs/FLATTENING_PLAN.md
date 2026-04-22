# YAM Flattening Plan

This note records the path from the current versioned tree to a single root runtime tree.

## Goal

- Keep one runtime source tree at the repository root.
- Keep the release model separate from the source tree layout.
- Keep `yam-go` scoped to a clock-only visualizer.
- Keep `yam-rust` separate for the engine-first biological simulation experiment.

## Steps

1. Lock the branch policy and release numbering.
2. Move the live runtime modules from `v2/` to the repository root.
3. Update the launcher to target the flattened tree.
4. Keep only the helper modules that are still useful for verification.
5. Prune obsolete version-specific directory references from docs and scripts.
6. Preserve history in git rather than in directory names.

## Safety Rules

- Move code in small batches.
- Keep the tree buildable after each batch.
- Update the golden frame after any layout-affecting change.
- Log each move in `docs/v2/LOG.md` until the old versioned path is gone.

## Exit Criteria

- `yam` launches from one canonical source tree.
- The repo no longer needs `v1/` or `v2/` directory names for active development.
- Stable releases are described by branch/tag/version policy, not by folder names.

## Current Status

- Batch 1 is complete: the Python entrypoints and scene config now live at the repository root.
- Batch 2 is complete: the live Go support packages now live in root-level package directories.
- The old `v2/` source tree has been removed.
- The Go runtime is now the clock-only visualizer baseline.
