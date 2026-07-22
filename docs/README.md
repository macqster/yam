# Documentation Index

This file is the docs map.  
The repo front door is [../README.md](../README.md).

## Start Here

- [../AGENTS.md](../AGENTS.md) - agent-facing operating guidance
- [../skills/yam-maintenance/SKILL.md](../skills/yam-maintenance/SKILL.md) - repo-local maintenance workflow skill
- [../skills/yam-architecture-review/SKILL.md](../skills/yam-architecture-review/SKILL.md) - repo-local architecture review workflow skill
- [../README.md](../README.md) - repo/runtime overview
- [../TODO.md](../TODO.md) - active execution backlog
- [../known_issues.md](../known_issues.md) - active unresolved issues only
- [LOG.md](LOG.md) - append-only project history
- [../CHANGELOG.md](../CHANGELOG.md) - curated user/developer-facing change summary
- [audit.md](audit.md) - current risk and drift snapshot

## Core Contracts

- [glossary.md](glossary.md) - shared terminology source of truth
- [architecture.md](architecture.md) - ownership and implementation architecture
- [scene-model.md](scene-model.md) - deterministic scene model
- [rendering.md](rendering.md) - render order, layering, and UI/render contracts
- [theme.md](theme.md) - reusable BTAS theme contract
- [hygiene.md](hygiene.md) - repo hygiene rules

## Active Surface Contracts

- [chatgpt-0.4-source-pack/README.md](chatgpt-0.4-source-pack/README.md) - compact upload pack for the external YAM 0.4 ChatGPT project
- [greenhouse-roadmap.md](greenhouse-roadmap.md) - greenhouse expansion roadmap and implementation gates
- [main-scene-scaffold.md](main-scene-scaffold.md) - main-scene hero support scaffold direction
- [loading-screen.md](loading-screen.md) - boot/loading-screen contract
- [weather-widget.md](weather-widget.md) - weather-widget contract
- [vines.md](vines.md) - vine ownership/readiness contract
- [config.md](config.md) - config ownership and boot-frame note
- [hero-cache.md](hero-cache.md) - hero-frame cache design/runtime path
- [soft-line-atlas.md](soft-line-atlas.md) - linework glyph grammar for guides and future mask edges

## Research And Policy

- [apple-music-companion.md](apple-music-companion.md) - Apple Music companion feasibility and architecture note
- [reference-sigye.md](reference-sigye.md) - reference study of `sigye`
- [release-model.md](release-model.md) - branch/version policy; no public release, ever (build from source only)
- [resource-map.md](resource-map.md) - research/reference map
- [palette-reference.md](palette-reference.md) - human-friendly BTAS/TNBA palette chain summary
- [palette-sheet-reference.md](palette-sheet-reference.md) - palette-sheet source/export handoff note

## Archive Entry Points

- [archive/README.md](archive/README.md) - archive index

## Doc Role Table

| Need | Owning Surface |
| --- | --- |
| Repo/runtime overview | [../README.md](../README.md) |
| Agent-facing work guidance | [../AGENTS.md](../AGENTS.md) |
| Agent-facing workflow procedures | [../skills/](../skills/) |
| Active execution order and checks | [../TODO.md](../TODO.md) |
| Concrete unresolved issues | [../known_issues.md](../known_issues.md) |
| Current risks, drift, and cleanup seams | [audit.md](audit.md) |
| Historical record | [LOG.md](LOG.md) |
| Shared terminology | [glossary.md](glossary.md) |
| Ownership and architecture boundaries | [architecture.md](architecture.md) |
| Scene/world behavior | [scene-model.md](scene-model.md) |
| Rendering, layers, and UI/render contracts | [rendering.md](rendering.md) |
| External ChatGPT 0.4 upload context | [chatgpt-0.4-source-pack/README.md](chatgpt-0.4-source-pack/README.md) |
| External ChatGPT 0.4 provenance and workflow cues | [chatgpt-0.4-source-pack/source-pack-manifest.md](chatgpt-0.4-source-pack/source-pack-manifest.md) |
| Greenhouse strategy and operation plan | [greenhouse-roadmap.md](greenhouse-roadmap.md) |
| Main-scene hero support scaffold direction | [main-scene-scaffold.md](main-scene-scaffold.md) |
| Vine ownership and staged readiness | [vines.md](vines.md) |
| Weather widget ownership | [weather-widget.md](weather-widget.md) |
| Hero cache design/runtime path | [hero-cache.md](hero-cache.md) |
| Config ownership and boot-frame note | [config.md](config.md) |

## Role Rules

- `README.md` is the front door, not the full contract set
- `AGENTS.md` is the agent work guide, not an architecture contract
- `skills/*/SKILL.md` files are procedural workflow helpers, not contract docs
- `docs/README.md` is the map, not the backlog
- `TODO.md` stays execution-focused
- `known_issues.md` stays issue-focused and timestamped
- `docs/audit.md` stays risk-focused
- `docs/LOG.md` stays append-only
- if a term looks shared or ambiguous, check [glossary.md](glossary.md) before extending another doc
