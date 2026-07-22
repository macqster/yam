# YAM 0.4 Source Pack Manifest

## Purpose

This manifest gives the external ChatGPT project enough provenance to know what
repo state this source pack came from.

This file does not make the pack authoritative. The real authority remains the
main YAM repository and its owning docs.

## Export Metadata

- Export date: `2026-07-22`
- Source repo: `https://github.com/macqster/yam.git`
- Source branch: `main`
- Source commit: `c49d4f03847a0af4c3b1a8942c2de5824ea24037`
- Pack scope: YAM `0.4` planning context only
- Pack authority: derived context, not implementation authority

## Included Pack Files

- `source-pack-manifest.md`
- `project-brief.md`
- `architecture-constraints.md`
- `readiness-and-gates.md`
- `greenhouse-brief.md`
- `repo-workflow-brief.md`
- `chatgpt-usage-guide.md`

## Not Included

This source pack does not include:

- source code
- tests
- CI configuration
- full backlog state
- full audit history
- generated assets
- runtime screenshots or other runtime evidence

It is suitable for planning-context alignment, not implementation auditing.

## Owning Repo Docs

These repo docs are the current source authorities behind this pack:

- `README.md`
- `TODO.md`
- `docs/architecture.md`
- `docs/scene-model.md`
- `docs/rendering.md`
- `docs/glossary.md`
- `docs/hygiene.md`
- `docs/audit.md`
- `docs/greenhouse-roadmap.md`
- `docs/LOG.md`

## Refresh Rule

Refresh this pack whenever one of these changes materially:

- greenhouse strategy or first-pass contract
- spatial ownership or projection rules
- flora storage direction or organism vocabulary
- 0.4 readiness gates
- world-selection/profile contract

Also refresh this manifest whenever the export branch or source commit changes.

## Export Method

Generated manually from the current repo planning and contract docs unless
otherwise stated here.

If this later becomes a scripted export, record the script path and command in
this manifest.

## Verification Note

The phrase `verification green` refers to the owning repository's verification
state at the source commit, not to this exported pack by itself.

## Pack Integrity

Future exports may include SHA-256 checksums for each included pack file if
stronger drift detection becomes useful.

## Usage Rule

Use this pack to help ChatGPT produce candidate planning material.

Do not treat pack text or ChatGPT output as source of truth unless it is later
promoted back into the owning YAM docs.
