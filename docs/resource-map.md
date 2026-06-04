# YAM Resource Map

<!-- cspell:ignore ntrospect0 -->

This doc tracks external tools, libraries, formats, and reference projects that are worth using, studying, emulating, or rejecting during future YAM development.

It is a research/reference surface, not an active backlog.

## Purpose

- keep future-development resource scouting out of `TODO.md`
- record which external tools are references, authoring environments, converters, or deliberate non-goals
- preserve one clear rule for runtime assets: keep them plain-text or YAM-native structured data, with deterministic Ratatui-owned rendering

## Runtime Asset Rule

- external formats such as ANSI, XBIN, or REXPaint `.xp` may be useful as source-art or interchange formats
- they should not become YAM runtime authorities by default
- runtime assets should remain plain text or YAM-native structured data with explicit contracts, tests, and semantic ownership

## Decision Categories

- `Adopt` means YAM can depend on the tool or crate directly
- `Emulate` means study the pattern or output grammar, then re-own the runtime behavior inside YAM
- `Reference` means keep it as inspiration, provenance, or optional offline workflow
- `Reject` means avoid using it as a runtime authority even if it remains useful as source material

## Current High-Value Resource Lanes

### Ratatui Ecosystem

Current YAM baseline:

- Ratatui is the live UI/render substrate
- YAM already owns its own world/HUD/layer model
- `tachyonfx` is already used for presentation effects

Recommended posture:

- `Adopt` Ratatui itself and small ecosystem helpers only when they reduce code without weakening YAM's scene contract
- `Emulate` popup, inspector, keyboard-dispatch, and settings-panel patterns from mature Ratatui apps
- `Reference` `tui-widgets`, `tui-scrollview`, and `ratatui-image` for future inspector growth or protocol experiments
- `Reject` generic dashboard-shell architecture as a replacement for YAM's world-space scene model

Implementation gate:

- only pull in more Ratatui ecosystem crates when a concrete YAM surface has outgrown the current custom implementation

### Dashboard TUI References

Current YAM baseline:

- YAM already uses Ratatui and several modal/dev surfaces, so it carries some
  natural dashboard gravity even though its product identity is world-space
  visualizer first
- the current greenhouse and world contracts explicitly reject panel-chrome
  ownership and room selection via UI tabs

Recommended posture:

- `Reference` Glint (`ntrospect0/glint`, studied at commit
  `8c10176787f9537cdc69f896088d7ce862cded8a`) as a mature Rust/Ratatui
  dashboard example with explicit widget registry, feature-gated widget
  families, polling/config/theme infrastructure, stack-cell composition,
  live-reload, and setup-wizard patterns
- `Emulate` only the infrastructure lessons that preserve YAM ownership:
  explicit extension descriptors, small extracted platform helpers,
  redraw/poll discipline, semantic theme roles, and humane setup/config flows
- `Reject` Glint's product ontology as a runtime template for YAM: pane-grid
  dashboard layout, widget-first composition, provider-heavy integrations,
  stack/tab worldview, command-bar-first identity, and "everything is a widget"
  architecture
- `Reject` borrowing dashboard chrome to solve greenhouse navigation, room
  selection, inspection, or world switching; those must stay owned by world
  state, room/place identity, and read-only inspect surfaces

Specific notes from the Glint study:

- its `Widget` trait, registry, and `docs/widget-sdk.md` are good references
  for how to document platform capabilities once repetition is real
- its feature-gated widget families are a good reminder that optional surfaces
  can stay compile-time explicit instead of leaking into one giant runtime blob
- its config watcher, wizard resume/finalize flow, and dirty/poll helpers are
  strong operational patterns for future YAM setup or authoring work
- its status bar, pane grid, and stack-cell composition are useful primarily as
  anti-drift examples for YAM's world/HUD/overlay contract

Implementation gate:

- if YAM adopts any lesson from Glint, promote it only as YAM-native
  infrastructure in service of world-space ownership
- do not add a generic widget shell, pane grid, stack tabs, or dashboard-style
  layout manager unless the architecture docs are intentionally changed first
- if a reusable YAM platform helper emerges from repeated code, document it as
  a YAM-owned contract rather than importing dashboard vocabulary wholesale

### Chafa And Terminal Image Rendering

Current YAM baseline:

- Chafa-backed hero rendering is the active baseline
- runtime rendering already avoids treating raw ANSI output as the final scene authority

Recommended posture:

- `Adopt` Chafa as the current hero renderer/compiler baseline
- `Emulate` useful frame-shape, color, and animation lessons in future YAM-owned frame caches
- `Reference` Chafa output as a seed for offline frame compilation, debugging, or manual correction workflows
- `Reject` raw ANSI stream embedding as the main Ratatui runtime path

Implementation gate:

- if hero rendering evolves beyond the current baseline, define a YAM-owned `CellGrid` / `HeroFrameSet` contract before introducing editor or cache tooling

### Weather Providers And Terminal Weather References

Current YAM baseline:

- `wttr.in` JSON is the live provider path
- YAM already owns normalized weather state, localized text, sprite atlas, layout, and Ratatui rendering

Recommended posture:

- `Adopt` `wttr.in` JSON as the current practical provider
- `Emulate` compact weather grammar, condition-family distinctions, and terminal-friendly fact ordering from weather TUIs
- `Reference` future provider options such as `Open-Meteo` only when provider expansion becomes real work
- `Reject` raw full-provider terminal output as runtime layout authority

Implementation gate:

- provider expansion should preserve a provider-neutral `WeatherSnapshot` seam and keep provider code out of scene/growth systems

### ANSI And ASCII Art Authoring Tools

Current YAM baseline:

- runtime weather sprites are plain-text assets
- source-art notes already live separately from runtime assets

Recommended posture:

- `Reference` Moebius / MoebiusXBIN as strong source-art environments for colored ANSI/XBIN exploration
- `Reference` REXPaint as a grid-discipline and interchange tool for text-mode composition experiments
- `Reference` `perkins` as a braille-oriented offline doctoring experiment for selected Chafa-derived hero frames
- `Emulate` useful composition and palette workflows from those tools while keeping runtime assets normalized for YAM
- `Reject` XBIN, `.ans`, or `.xp` files as direct runtime sprite atlas sources

Specific note on `perkins`:

- treat it as an experiment for manual cleanup of a few important Chafa-derived hero frames, especially when silhouette or braille-cell readability needs hand correction
- do not treat it as a replacement for Chafa, an animation-aware frame pipeline, or a runtime source-of-truth format
- if `perkins` ever becomes part of a real workflow, define a YAM-owned `CellGrid` / frame-cache contract first and treat `perkins` edits as offline source material only

Implementation gate:

- if an offline art workflow is promoted, each asset family should keep the `runtime/` vs `source-art/` split already used in `src/weather/assets/`

### Terminal Effects

Current YAM baseline:

- `tachyonfx` is already part of the runtime for presentation-only transitions/effects

Recommended posture:

- `Adopt` `tachyonfx` for loading transitions and small presentation-only polish
- `Emulate` restrained terminal animation patterns that do not obscure state truth
- `Reject` effects that own simulation state, projection semantics, or debug truth

Implementation gate:

- any new effect should degrade cleanly when disabled and stay documented as presentation-only

### Procedural Flora And Growth References

Current YAM baseline:

- vines are the first live flora prototype
- growth, density, constraints, aging, and tick systems already exist

Recommended posture:

- `Reference` `cbonsai` / `rbonsai` for terminal plant mood, slow emergence,
  glyph economy, and screensaver-like presence
- `Reference` `asciiquarium` for ambient terminal ecosystem staging and
  multiple organisms sharing one visual environment
- `Reference` HighGrow only as compact greenhouse structure: multiple small
  rooms, one to three planting sites, local fixture/climate affordances, and a
  magnifying-glass-like inspect precedent
- `Reference` Viridi for calm small-container mood, gentle check-in cadence,
  slow-time presence, and possible future personal attachment language
- `Reference` OpenAlea, L-Py, Arbaro, AmapSim, GreenLab, PowerPlant / pplant,
  ONETREE, and Algorithmic Botany / L-studio as deferred technical lineage for
  plant form, morphology, rule grammar, and environment-hook thinking
- `Reference` L-systems, space-colonization, cellular-automata, and agent-based growth literature as idea banks
- `Emulate` deterministic, art-directable, terminal-cell-friendly growth rules rather than biological simulation dogma
- `Reject` overbuilt realism, crop simulation, harvest loops, genetics, yield
  optimization, realistic cultivation instruction, or random ASCII painting
  that bypasses YAM's explicit world-space/growth contracts

Implementation gate:

- first greenhouse work should define functional space first: room, access
  paths, zones, fixtures, planting sites, symbolic environment, and read-only
  inspection
- future flora work should extend the existing systems vocabulary instead of
  bypassing it with one-off species logic
- L-systems or scientific plant-modeling references should not become runtime
  scope until the greenhouse room/site/environment contract is stable

### Theme And Palette References

Current YAM baseline:

- BTAS/TNBA semantic theme helpers already own color roles in code and docs

Recommended posture:

- `Reference` terminal palette conventions, ANSI fallback roles, and cel-animation palette studies
- `Emulate` useful contrast and semantic-token discipline
- `Reject` raw literal-color sprawl or editor-driven palette authority inside runtime code

Implementation gate:

- any future theme cleanup should preserve semantic naming first and keep raw-color literals limited to low-level render/test exceptions

### Apple Music Companion

Current YAM baseline:

- no music companion is implemented
- the May 2026 feasibility note lives in
  [`apple-music-companion.md`](apple-music-companion.md)

Recommended posture:

- `Reference` Apple MusicKit and MusicKit JS docs for platform capability and
  queue/playback semantics
- `Emulate` simple music-client list, queue, and now-playing ergonomics in a
  YAM-owned Ratatui companion
- `Adopt` macOS Music.app automation first if this becomes a real personal
  companion, because Music.app already owns account login, DRM, and playback
- `Reject` pure Rust Apple Music streaming as a practical target
- `Reject` Chrome/CDP/MusicKit playback as the first implementation path unless
  the goal is explicitly a playback-backend research project

Implementation gate:

- prove a small Music.app control backend before adding a full UI or any Apple
  Music API token flow

## Adopt / Emulate / Reference / Reject Summary

| Resource | Category | YAM role |
| --- | --- | --- |
| Ratatui | Adopt | core UI/render substrate |
| `tachyonfx` | Adopt | presentation-only loading/effect polish |
| Chafa | Adopt | current hero rendering/compiler baseline |
| `wttr.in` JSON | Adopt | current live weather provider |
| `ansi-to-tui` | Adopt | ANSI-to-Ratatui conversion seam when needed |
| Awesome Ratatui / exemplar TUIs | Emulate | popup, inspector, and keybinding patterns |
| weather TUIs / `wttr.in` presentation grammar | Emulate | compact fact ordering and condition-family inspiration |
| Moebius / MoebiusXBIN | Reference | offline source-art workflow |
| REXPaint | Reference | optional text-grid authoring/interchange workflow |
| `perkins` | Reference | offline Chafa-frame doctoring experiment for selected braille-heavy hero frames |
| `tui-widgets` / `tui-scrollview` | Reference | future inspector growth options |
| `ratatui-image` | Reference | future image-protocol experiments only |
| Apple MusicKit / MusicKit JS | Reference | capability and playback-semantics reference for possible music companion |
| macOS Music.app automation | Adopt candidate | safest first playback backend for possible `yam-music` |
| `cbonsai` / `rbonsai` | Reference | terminal plant mood and slow emergence lineage |
| `asciiquarium` | Reference | ambient terminal ecosystem staging |
| HighGrow | Reference | compact multi-room greenhouse and inspect precedent only |
| Viridi | Reference | calm small-container check-in mood |
| OpenAlea / L-Py / Arbaro | Reference | deferred plant morphology and grammar lineage |
| GreenLab / Algorithmic Botany | Reference | deferred technical plant-modeling lineage |
| Chrome/CDP/MusicKit playback | Reject first path | too fragile for the first YAM music companion slice |
| pure Rust Apple Music streaming | Reject | not a practical playback target |
| raw ANSI / XBIN / `.xp` runtime authority | Reject | not a runtime asset source by default |
| generic dashboard app shells | Reject | not a replacement for YAM's world-space scene model |

## Research Priorities

If future-development research resumes, prefer this order:

1. terminal art pipeline and a possible future YAM-owned `CellGrid` / frame-cache contract
2. Ratatui inspector/popup patterns for larger debug or art-review surfaces
3. weather-provider expansion research only when the current provider seam is no longer enough
4. procedural flora references for the next species/growth phase

## Notes For Future Feature Phases

- keep resource scouting separate from active maintenance work
- move research into implementation only when a concrete feature phase starts
- when a source-art or provider workflow graduates into active work, update the owning contract docs and add tests/invariants in the same batch
