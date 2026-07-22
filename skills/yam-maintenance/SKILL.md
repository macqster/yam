---
name: yam-maintenance
description: Use when doing maintenance work in the YAM repository, including cleanup, stabilization, docs/log updates, hygiene checks, dependency/tooling adjustments, or pre-expansion prep that should not add new runtime features.
---

# YAM Maintenance

## Read First

- Read [../../AGENTS.md](../../AGENTS.md) for repo operating rules.
- Use [../../TODO.md](../../TODO.md) for the active work order.
- Use [../../docs/audit.md](../../docs/audit.md) for risk status.
- Use [../../docs/hygiene.md](../../docs/hygiene.md) for repo hygiene rules.

## Workflow

1. Inspect `git status --short --branch` before editing.
2. Read the narrow code/docs surface affected by the request.
3. Keep changes scoped to the maintenance goal.
4. Update owning docs and append [../../docs/LOG.md](../../docs/LOG.md) when behavior, contracts, tooling, or workflow changes.
5. Keep [../../known_issues.md](../../known_issues.md) empty unless there is a concrete unresolved issue.
6. Run the smallest useful checks while iterating.
7. Finish with `bash scripts/verify.sh` for maintenance handoff unless the user explicitly asks for a narrower pass.

## Guardrails

- Do not introduce greenhouse worlds, new plant families, or main-scene enrichment during maintenance prep unless the request explicitly asks for that implementation (matching `AGENTS.md`'s Operating Rules) — maintenance framing should not be read as a blanket ban once the user has actually asked for the feature.
- Do not duplicate architecture facts in `TODO.md`, this skill, or logs; point to the owning docs.
- Do not revert unrelated dirty worktree changes.
- Prefer existing scripts and local patterns over new tooling.
