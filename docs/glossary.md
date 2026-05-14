# YAM Glossary

This glossary is the shared vocabulary for YAM-specific nomenclature, recurring Rust/runtime terms, and the small amount of general developer jargon that appears repeatedly across the repo docs.

## Authority

- `strict` - standard domain meaning that YAM should stay close to
- `inferred` - stable YAM-specific interpretation built around the current architecture
- `repo` - repo-local wording or shorthand used intentionally in docs/reviews/backlog work
- `provisional` - a placeholder that may still change after further research or implementation

## How To Use This File

- prefer the terms here over inventing near-synonyms in contract docs
- if a term is shared across multiple docs, define it here once and point back to it
- keep this file definitional, not historical or backlog-shaped
- if a term is ordinary Rust or general developer jargon but keeps recurring in YAM docs, it belongs here too

## 1. YAM Core Terms

| Term | Authority | YAM Meaning | Use For |
| --- | --- | --- | --- |
| `YAM` | repo | the overall terminal scene/simulation project | project-wide reference |
| `yam-rust` | repo | the active Rust runtime tree and runtime crate | repo/runtime naming |
| `datum` | inferred | the shared absolute world reference at `(0, 0)` | projection, guides, border probes, anchor math |
| `world space` | strict | simulation-space coordinates centered on the datum | entities, flora, guides, attachment logic |
| `screen space` | strict | terminal-attached coordinates after projection | HUD and overlay placement |
| `anchor space` | inferred | offsets resolved relative to another object or entity | attachment and companion placement |
| `main scene` | inferred | the primary visualiser world | hero, scaffold, companions, vines |
| `sandbox` | inferred | the sparse alternate world for experiments | stripped test scene surface |
| `greenhouse` | inferred | the future multi-species simulation space | rooms, labs, growth procedures |
| `world-attached` | repo | content that is positioned in world space and then projected | hero, companions, vines, guides |
| `screen-attached` | repo | content that stays fixed to the terminal frame | footer, HUD indicators, modal placement |
| `companion` | inferred | a world-attached UI-like scene element attached to the hero composition | clock, weather, date, reserved calendar seam |
| `calendar seam` | repo | the still-reserved sibling companion path that exists in offsets/state but is not yet a live rendered surface | reserved future work |
| `dev mode` | inferred | the gated runtime surface for debug, inspection, and live adjustment tools | help, move, settings, pointer probe |
| `metamechanics` | inferred | subordinate runtime control/observation state inside `ui/`, not world ownership | dev toggles, modal state, debug visibility |
| `surface family` | repo | a grouped set of related visible UI surfaces with one coherent role language | help/move/settings/footer/quit-confirm cleanup |
| `front door` | repo | the highest-visibility entry doc for the repo | `README.md` ownership |
| `docs map` | repo | the navigation file for the active docs set | `docs/README.md` ownership |

## 2. Scene And UI Terms

| Term | Authority | YAM Meaning | Use For |
| --- | --- | --- | --- |
| `camera` | inferred | the world-space crop origin or framing helper | projection and viewport control |
| `viewport` | strict | the visible terminal-sized crop area | what is currently shown |
| `frame` | strict | one full rendered terminal frame | per-tick output |
| `HUD` | inferred | screen-attached presentation layer above the world but below modal overlays | footer, status, passive indicators |
| `overlay` | strict | top-most presentation layer above scene and HUD | modals, inspectors, transient control surfaces |
| `modal` | strict | an interaction state that captures focus until dismissed | help, settings, quit-confirm, active move surface |
| `shell` | inferred | the shared bordered container used by popup/overlay surfaces | modal chrome |
| `footer` | strict | the bottom status strip of the terminal frame | compact mode hints and version/status stamp |
| `status stamp` | repo | the right-aligned runtime identity label in the footer | version/build label |
| `debug panel` | inferred | the left diagnostic readout with grouped facts | runtime/hero/companions/vines tabs |
| `tab row` | repo | a lightweight in-surface row used to switch grouped content | settings tabs, debug tabs, move target chips |
| `move strip` | repo | the lower-band modal surface for explicit world-attached movement | target cycling and arrow-key motion |
| `help popup` | repo | the modal discoverability surface opened by `?` | controls overview |
| `settings popup` | repo | the tabbed modal owner for editable persisted presentation/runtime settings | positions/ui/features/gif/theme |
| `pointer probe` | inferred | the dev-only blinking world-space marker used for exact coordinate inspection | guide authoring and absolute readout |
| `world frame` | repo | the world-border indicator rendered in world space | border probe and crop understanding |
| `world crosshair` | repo | the datum/axis helper shown in dev UI | orientation and projection checks |

## 3. Spatial And Render Terms

| Term | Authority | YAM Meaning | Use For |
| --- | --- | --- | --- |
| `projection` | strict | the mapping from world-space coordinates into terminal coordinates | camera/view/render math |
| `projection path` | repo | the live code path that performs coordinate conversion and placement resolution | audits and cleanup notes |
| `relation layer` | repo | the canonical shared spatial ownership layer that should unify datum, anchors, guides, and attachment math | architecture work |
| `guide` | inferred | a world-space annotation primitive used for debugging and future flora/mask guidance | linework, authoring, inspection |
| `guide set` | inferred | a named collection of guides addressed as one group | grouped linework and future tooling |
| `anchor` | inferred | an attachment reference resolved relative to an entity or pose | companion and attachment math |
| `attachment` | inferred | practical YAM term for a resolved anchored relation or offset | entity and companion placement |
| `mask` | inferred | a selective render-application surface, not a raster image substitute | hero occlusion and constrained merges |
| `render layer` | strict | one ordered stratum in the composed scene output | field, hero, companions, debug, overlays |
| `z-index` | repo | the numeric ordering key for render layers | stacking order |
| `Grid` | repo | the intermediate full-frame cell buffer used before Ratatui lines are emitted | layer output and composition |
| `Cell` | repo | one terminal cell in a `Grid`, including glyph and style | composition and cache storage |
| `scratch grid` | repo | a reusable per-layer `Grid` buffer owned by `Scene` | allocation reuse path |
| `final frame grid` | repo | the reusable composed `Grid` that merges all active layers before terminal output | runtime render-loop efficiency |
| `opaque backdrop write` | repo | a space-plus-background cell write that intentionally clears what is behind it | modal shells and overlay readability |
| `ASCII chrome` | repo | guaranteed-ASCII UI strings and borders where the fast ASCII write path is safe | footer, debug labels, world label |

## 4. Flora And Morphology Terms

| Term | Authority | YAM Meaning | Use For |
| --- | --- | --- | --- |
| `stem` | strict | main supporting shoot axis | trunk-like or shoot-like support |
| `branch` | strict | side axis arising from another axis | offshoots and secondary growth paths |
| `stub` | inferred | short pruned or broken remnant of a stem or branch | scaffold remnants, deadwood-like supports |
| `node` | strict | attachment point on a stem where organs arise | leaf, bud, or branch attachment sites |
| `internode` | strict | segment between nodes | repeated stem segments |
| `leaf` | strict | photosynthetic organ attached to the plant body | foliage and leaf lifecycle |
| `bud` | strict | undeveloped growth point | future leaf, flower, or branch before opening |
| `meristem` | strict | active growth region | tip growth and branching origins |
| `apical meristem` | strict | tip growth region | elongation and terminal growth |
| `lateral meristem` | strict | side growth region | branching and thickness growth |
| `axis` | inferred | directional branch system carrying repeated metamers | branch hierarchy and structural runs |
| `metamer` | inferred | repeating structural unit of a plant axis | modular plant construction |
| `insertion` | strict | point where an organ connects to an axis | organ attachment |
| `attachment` | inferred | practical YAM synonym for insertion in plant-facing docs | organ connection wording |
| `organ` | strict | generic bucket for plant outputs | leaf, flower, fruit, branch |
| `petiole` | strict | leaf stalk | connector between blade and axis |
| `flower` | strict | reproductive organ cluster | bloom output and lifecycle |
| `fruit` | strict | mature reproductive structure | post-flower output |
| `habit/form` | inferred | broad growth style of a species | species registry summaries |
| `growth mode` | inferred | how a plant tends to extend or branch | species registry and debugging |
| `support habit` | inferred | how a plant supports itself or another structure | scaffold, vine, or self-supporting forms |
| `flora runtime` | repo | the future broader plant simulation machinery beyond the current vine prototype | architecture and backlog notes |

## 5. Rust And Runtime Terms

| Term | Authority | YAM Meaning | Use For |
| --- | --- | --- | --- |
| `crate` | strict | a Rust compilation unit/package | repo structure and build discussion |
| `module` | strict | a Rust namespace/file subtree | code organization |
| `trait` | strict | a Rust behavior interface | `Layer`, shared renderer contracts |
| `impl` | strict | a Rust implementation block | type or trait implementation |
| `enum` | strict | a tagged Rust sum type | runtime modes, tabs, world kinds |
| `struct` | strict | a Rust data type with named fields | state, render, and config types |
| `shim` | repo | a temporary compatibility layer retained during a migration | projection/spatial transition work |
| `compatibility path` | repo | an older-but-still-supported implementation route kept during refactors | migration notes and audits |
| `fallback` | repo | the alternative path used when the preferred one is unavailable or invalid | cache load failure, no-`chafa` behavior |
| `cache-first` | repo | runtime behavior that attempts a prepared cached artifact before recompiling or rebuilding | hero startup path |
| `hot path` | repo | code that runs frequently enough that allocation or branching cost matters | render loop and startup optimization |
| `allocation churn` | repo | repeated avoidable allocation work in a frequently executed path | render-loop cleanup |
| `wrapper overhead` | repo | time spent in tool/launcher layers around the real app runtime | `cargo run` vs direct binary timing |
| `regression test` | repo | a targeted test added to prevent a fixed behavior from quietly breaking again | maintenance and contract repair |

## 6. Docs And Maintenance Terms

| Term | Authority | YAM Meaning | Use For |
| --- | --- | --- | --- |
| `contract doc` | repo | the doc that owns the active behavior/ownership definition for a surface | architecture/rendering/scene-model work |
| `backlog` | repo | the active execution checklist | `TODO.md` |
| `audit` | repo | the current risk, drift, and next-cleanup snapshot | `docs/audit.md` |
| `append-only log` | repo | the historical record where new entries are added rather than rewritten into a status board | `docs/LOG.md` |
| `issue register` | repo | the small file of active unresolved issues with stable ids | `known_issues.md` |
| `maintenance pass` | repo | a deliberately narrow cleanup batch focused on wording, ownership, or light structural tightening | docs/UI/code hygiene work |
| `role tightening` | repo | cleanup that makes one surface or file do one clearer job without redesigning the system | UI/docs cleanup notes |
| `surface drift` | repo | mismatch between how a surface is implemented, described, and named | audit findings |
| `soft feature freeze` | repo | current rule that stability/polish/contract repair takes priority over new features | planning and review work |

## Usage Guidance

- use `node` only for plant anatomy and morphology
- use `points`, `anchors`, `guides`, `lines`, and `polylines` for spatial authoring
- use `move strip` for the lower `[m]` surface, not `move popup`
- use `front door` for `README.md` and `docs map` for `docs/README.md`
- use `backlog`, `audit`, `append-only log`, and `issue register` consistently for `TODO.md`, `docs/audit.md`, `docs/LOG.md`, and `known_issues.md`
- use `write_string` for the fully general grapheme-aware compositor path and `write_ascii_string` only for known ASCII-only chrome
- if a shared term is missing from this file, add it here before letting multiple docs define it differently
