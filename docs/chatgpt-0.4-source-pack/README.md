# YAM 0.4 ChatGPT Source Pack

This folder is a compact source pack for the separate ChatGPT project used for
YAM 0.4 expansion planning.

The goal is to give ChatGPT enough real project context to be useful without
uploading the entire repo or forcing it to infer architecture from scattered
notes.

## Intended Use

Upload this pack when the external ChatGPT project needs:

- current YAM baseline and direction
- architecture and ownership constraints
- 0.4 readiness and non-goals
- greenhouse/world expansion context
- repo workflow and authority cues
- bounded creative prompts that stay compatible with the repo

## Recommended Upload Order

Use this order for a fresh ChatGPT project:

1. `source-pack-manifest.md`
2. `project-brief.md`
3. `architecture-constraints.md`
4. `readiness-and-gates.md`
5. `greenhouse-brief.md`
6. `repo-workflow-brief.md`
7. `chatgpt-usage-guide.md`

The first five files give ChatGPT the actual project context. The workflow and
usage notes help keep the later conversation disciplined.

## Files

- [`source-pack-manifest.md`](source-pack-manifest.md) - provenance for this
  export: repo, branch, commit, owning docs, and refresh rule
- [`project-brief.md`](project-brief.md) - what YAM is, what exists now, and
  what 0.4 is trying to protect
- [`architecture-constraints.md`](architecture-constraints.md) - the rules
  ChatGPT should not violate when proposing features or structures
- [`readiness-and-gates.md`](readiness-and-gates.md) - 0.4 readiness gates,
  current defaults, and open decisions
- [`greenhouse-brief.md`](greenhouse-brief.md) - canonical greenhouse
  direction, first-pass contract, and preserved vocabulary
- [`repo-workflow-brief.md`](repo-workflow-brief.md) - minimal source-control,
  authority, and checkpoint context for external planning
- [`chatgpt-usage-guide.md`](chatgpt-usage-guide.md) - how to ask ChatGPT for
  useful output that can be promoted back into YAM docs

## Pack Design

This pack is intentionally small and layered:

- `project-brief.md` explains what YAM is
- `architecture-constraints.md` explains what must not be broken
- `readiness-and-gates.md` explains what is currently allowed
- `greenhouse-brief.md` explains what the greenhouse direction actually is
- `repo-workflow-brief.md` explains how the planning context relates to the
  real repository
- `chatgpt-usage-guide.md` explains how to ask for output that can be reused

## Rules

- Treat this pack as planning context, not direct implementation authority.
- The real repo authority still lives in the main YAM docs.
- Check [`source-pack-manifest.md`](source-pack-manifest.md) before using the
  pack for a new planning batch.
- If the pack drifts, refresh it from the owning docs before using it for a new
  serious planning batch.
