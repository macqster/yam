# YAM v2 Repo Hygiene

This file defines the working discipline for the v2 branch.

## Current Baseline

- the Rust rewrite in `/src` is the active implementation tree
- Go-era root artifacts have been pruned from the repo baseline
- `Cargo.toml`, `Cargo.lock`, `src/`, `assets/`, and docs are the canonical tracked surface

## Source of Truth

- `docs/v2/` is the v2 spec anchor
- code must follow the spec, not the reverse
- any divergence should be documented before implementation

## Tracking

- every material change should be reflected in `docs/v2/LOG.md`
- long-running work should have a visible roadmap update
- avoid untracked side paths or one-off experiments in the main tree

## File Discipline

- keep files in the smallest logical home
- avoid ambiguous names
- preserve stable paths once established
- do not rename files casually

## Commit Discipline

- make focused commits
- keep each commit tied to one logical change
- do not bundle documentation, refactors, and feature work unless they are inseparable

## Review Discipline

- prefer small diffs
- keep generated or temporary output out of version control unless it is part of the spec
- call out any deviation from the roadmap in the log

## Operational Rules

- no destructive rewrites
- no unlogged branch churn
- no undocumented file moves
- no hidden coupling between engine, render, and UI layers
- remove build output before committing unless it is intentionally tracked
