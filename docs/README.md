# Documentation Index

## Active Docs

- [`../README.md`](../README.md) - current repo and runtime overview
- [`../TODO.md`](../TODO.md) - active backlog
- [`LOG.md`](LOG.md) - current repository log
- [`hygiene.md`](hygiene.md) - repo hygiene rules
- [`audit.md`](audit.md) - current repo audit
- [`glossary.md`](glossary.md) - YAM glossary and terminology source of truth
- [`architecture.md`](architecture.md) - implementation architecture contract and current vs intended spatial model
- [`scene-model.md`](scene-model.md) - deterministic scene model above ratatui
- [`rendering.md`](rendering.md) - render order contract
- [`release-model.md`](release-model.md) - branch and release policy
- [`config.md`](config.md) - scene config ownership note and the `124x32` boot/start frame size
## Notes

- `docs/` is the active documentation surface.
- `README.md` is the repo front door; `docs/README.md` is the docs map.
- If unsure where to make a change, start here.
- `README.md` carries the one-line YAM slogan and non-goals; `docs/architecture.md` and `docs/scene-model.md` carry the detailed contracts.
- `docs/glossary.md` is the shared terminology source of truth; the other docs should point back to it instead of redefining terms.
- if a term looks shared or ambiguous, check `docs/glossary.md` before extending a contract doc.
- historical reconstruction notes were consolidated into the current docs set.
- uppercase markdown filenames are reserved for the highest-visibility entry points; lower-case names are preferred for most active contracts and archive reports.

## Archive Entry Points

- [`REFERENCE_ARCHIVE.md`](REFERENCE_ARCHIVE.md) - reference-only dump of imported historical notes
- [`archive/README.md`](archive/README.md) - archive index for older reports and reviews

## Where to Change Things

- If unsure where to change something, start here.
- Conceptual behavior changes -> `docs/scene-model.md`
- Ownership and architecture changes -> `docs/architecture.md`
- Rendering and layer-order changes -> `docs/rendering.md`
- Work order and validation changes -> `TODO.md`
- Risk and status changes -> `docs/audit.md`

## Archive Notes

- [`archive/notes/flattening-plan.md`](archive/notes/flattening-plan.md) - completed flattening note
- [`archive/notes/version-map.md`](archive/notes/version-map.md) - runtime/version mapping note
