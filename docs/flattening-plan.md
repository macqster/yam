# Flattening Plan

This note records the completed move to a single Rust runtime tree.

## Goal

- Keep one runtime source tree at the repository root.
- Keep release history in git rather than in folder names.

## Status

- The Rust tree in `src/` is canonical.
- The old `docs/v2/` archive has been removed.
- The repo should stay root-first and Rust-first.

## Rules

- keep edits small and buildable
- update docs when structure changes
- remove build output before commit
