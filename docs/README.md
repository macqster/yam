# Documentation Index

This file is the docs map.  
The repo front door is [../README.md](../README.md).

## Start Here

- [../README.md](../README.md) - repo/runtime overview
- [../TODO.md](../TODO.md) - active execution backlog
- [../known_issues.md](../known_issues.md) - active unresolved issues only
- [LOG.md](LOG.md) - append-only project history
- [audit.md](audit.md) - current risk and drift snapshot

## Core Contracts

- [glossary.md](glossary.md) - shared terminology source of truth
- [architecture.md](architecture.md) - ownership and implementation architecture
- [scene-model.md](scene-model.md) - deterministic scene model
- [rendering.md](rendering.md) - render order, layering, and UI/render contracts
- [theme.md](theme.md) - reusable BTAS theme contract
- [hygiene.md](hygiene.md) - repo hygiene rules

## Active Surface Contracts

- [loading-screen.md](loading-screen.md) - boot/loading-screen contract
- [weather-widget.md](weather-widget.md) - weather-widget contract
- [vines.md](vines.md) - vine ownership/readiness contract
- [config.md](config.md) - config ownership and boot-frame note
- [hero-cache.md](hero-cache.md) - hero-frame cache design/runtime path

## Research And Policy

- [reference-sigye.md](reference-sigye.md) - reference study of `sigye`
- [release-model.md](release-model.md) - branch and release policy
- [resource-map.md](resource-map.md) - research/reference map

## Archive Entry Points

- [archive/README.md](archive/README.md) - archive index
- [REFERENCE_ARCHIVE.md](REFERENCE_ARCHIVE.md) - imported historical reference dump

## Where To Change Things

- repo/runtime overview -> [../README.md](../README.md)
- work order and execution checks -> [../TODO.md](../TODO.md)
- active unresolved issues -> [../known_issues.md](../known_issues.md)
- risk, drift, and next cleanup seams -> [audit.md](audit.md)
- historical record -> [LOG.md](LOG.md)
- terminology -> [glossary.md](glossary.md)
- ownership/architecture -> [architecture.md](architecture.md)
- scene behavior -> [scene-model.md](scene-model.md)
- rendering/layer/UI contracts -> [rendering.md](rendering.md)

## Role Rules

- `README.md` is the front door, not the full contract set
- `docs/README.md` is the map, not the backlog
- `TODO.md` stays execution-focused
- `known_issues.md` stays issue-focused and timestamped
- `docs/audit.md` stays risk-focused
- `docs/LOG.md` stays append-only
- if a term looks shared or ambiguous, check [glossary.md](glossary.md) before extending another doc
