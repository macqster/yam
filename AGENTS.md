# Agent Guide

This file is agent-facing operating guidance for work in this repository.
It describes how to work here, not the full YAM architecture.

## Read First

- [README.md](README.md) is the repo front door.
- [TODO.md](TODO.md) is the active execution backlog.
- [known_issues.md](known_issues.md) tracks active concrete issues only.
- [docs/architecture.md](docs/architecture.md) owns data flow and coupling rules.
- [docs/scene-model.md](docs/scene-model.md) owns scene/world behavior.
- [docs/rendering.md](docs/rendering.md) owns render order, layer, and UI/render contracts.
- [docs/audit.md](docs/audit.md) owns current risks and drift notes.
- [docs/LOG.md](docs/LOG.md) is append-only history.

## Operating Rules

- Inspect the current tree before editing.
- Treat an already-dirty worktree as user-owned context; do not revert unrelated changes.
- Keep edits narrow and consistent with existing Rust/Ratatui patterns.
- Do not add main-scene enrichment, greenhouse worlds, plant families, or large mechanics during prep work unless the request explicitly asks for implementation.
- Even after that request, discuss scope in narrow slices rather than the whole feature at once: propose the smallest next slice, confirm it, build it, then propose the next. This surfaces design questions (naming, ownership, UX) before code exists instead of after.
- When changing behavior, update the owning doc and append `docs/LOG.md` in the same batch.
- Keep `known_issues.md` empty unless there is a concrete unresolved user-visible or developer-visible issue.
- Prefer contract repairs, tests, docs alignment, and workflow cleanup over speculative abstractions.
- Treat `README.md` as a mostly-settled creative front door: preserve the intro tone, GIF, and compact orientation-sheet structure, and only make factual, hygiene, or clearly justified front-door changes unless explicitly asked for a broader rewrite.
- This maintainer's default working style for this repo is fast-paced and low-friction: once a review or eval has been reported and the maintainer says to act on it, work through the findings one by one without re-confirming each individually, favor direct fixes over asking which of several options to pick, and keep pace by not re-deriving already-established context. This is a pacing preference, not a standing waiver of judgment — it does not change what still needs explicit confirmation (anything publish-facing: merging PRs, commenting on issues, cutting tags/releases) or what stays off-limits (destructive git operations, secrets) regardless of how the request is phrased.

## Architecture Guardrails

- `core/` owns data and pure logic; it must not depend on UI, terminal, or render code.
- `systems/` may mutate `WorldState`; it must not render or depend on scene, render, UI, or terminal modules.
- `render/` and scene layers visualize state; they must not own simulation truth.
- `ui/` owns presentation state, settings, modals, and persisted UI preferences.
- World-attached content must flow through world/spatial/layer contracts.
- HUD and modal surfaces must stay screen-attached.
- Future greenhouse work must remain a separate simulation world or world-internal room model, not panel chrome on top of the main scene.
- New flora work must build from `core::organism`, `FloraState`, spatial guidance, species registry vocabulary, and per-organism journals.

## Verification

Use the smallest useful check while iterating, then finish maintenance batches with:

```bash
bash scripts/verify.sh
```

Useful narrower gates:

```bash
cargo fmt --check
bash scripts/check-docs.sh
bash scripts/check.sh
cargo test --quiet
```

`.github/workflows/verify.yml` runs the same `scripts/verify.sh` gate in CI on every push and pull request targeting `main`, and `main` requires it to pass before merging (branch protection). Treat that as a backstop, not a substitute for running it locally first.

If a command cannot be run, record that clearly in the handoff.

## Docs And Logs

- Keep `TODO.md` execution-focused.
- Keep `docs/audit.md` risk-focused.
- Keep `docs/LOG.md` historical and append-only.
- Keep architecture facts out of the backlog when an owning contract doc exists.
- Keep active markdown clean under repo-configured `markdownlint`, `markdownlint-cli2`, and `cspell`.

## Skills

Dedicated skills are useful only when they stay short and procedural.
They should point back to this file and the canonical docs above instead of copying the contracts.
Good candidates are maintenance, architecture review, docs hygiene, flora prep, and release handoff workflows.
The first repo-local skill drafts live in [skills/yam-maintenance/SKILL.md](skills/yam-maintenance/SKILL.md) and [skills/yam-architecture-review/SKILL.md](skills/yam-architecture-review/SKILL.md).
Each repo-local skill should keep matching `agents/openai.yaml` UI metadata, including a short description and a default prompt that explicitly names the skill.
`scripts/check-docs.sh` validates skill names, frontmatter descriptions, and required `agents/openai.yaml` interface fields.
