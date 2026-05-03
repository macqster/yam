# YAM Glossary

This glossary is the shared vocabulary for YAM’s plant anatomy, morphology, and spatial terminology.

## Authority

- `strict` - botanically grounded terms that should stay close to standard plant meaning
- `inferred` - YAM-specific design interpretations built from plant structure and terminal constraints
- `provisional` - placeholders that may still change after further botany or spatial research

## Core Terms

| Term | Authority | YAM Meaning | Use For |
| --- | --- | --- | --- |
| `stem` | strict | main supporting shoot axis | trunk-like or shoot-like support |
| `branch` | strict | side axis arising from another axis | offshoots and secondary growth paths |
| `stub` | inferred | short pruned or broken remnant of a stem or branch | scaffold remnants, deadwood-like supports |
| `node` | strict | attachment point on a stem where organs arise | leaf, bud, or branch attachment sites |
| `internode` | strict | segment between nodes | repeated stem segments |
| `leaf` | strict | photosynthetic organ attached to the plant body | foliage, fenestration, leaf lifecycle |
| `bud` | strict | undeveloped growth point | future leaf, flower, or branch before opening |
| `meristem` | strict | active growth region | tip growth and branching growth origins |
| `apical meristem` | strict | tip growth region | elongation and terminal growth |
| `lateral meristem` | strict | side growth region | branching and thickness growth |
| `axis` | inferred | directional branch system carrying repeated metamers | branch hierarchy and structural runs |
| `metamer` | inferred | repeating structural unit of a plant axis | modular plant construction |
| `insertion` | strict | point where an organ connects to an axis | organ attachment |
| `attachment` | inferred | practical YAM synonym for insertion | organ connection point in docs/UI |
| `organ` | strict | generic bucket for plant outputs | leaf, flower, fruit, branch |
| `petiole` | strict | leaf stalk | connector between a leaf blade and its axis |
| `petal` | strict | flower blade | blossom parts and flower lifecycle |
| `root` | strict | below-ground anchor and uptake organ | anchoring and subterranean growth |
| `crown` | strict | upper leafy/superstructure portion | top mass of a plant |
| `canopy` | strict | outward leafy spread of the crown | shade-forming foliage mass |
| `flower` | strict | reproductive organ cluster | bloom output and pollination stages |
| `fruit` | strict | mature reproductive structure | post-flower output and fruiting |
| `habit/form` | inferred | broad growth style of a species | registry entries and species summaries |
| `growth mode` | inferred | how a plant tends to extend or branch | species registry and debugging |
| `branching pattern` | inferred | how branches are produced or arranged | species registry and growth rules |
| `lifecycle tuning` | inferred | species-specific timing and progression values | registry fields and journals |
| `support habit` | inferred | how the plant supports itself or others | scaffold, vine, or aroid forms |
| `morphology notes` | inferred | descriptive shape notes for a species | registry entries and inspection |
| `ecology cues` | inferred | environmental tendencies or responses | registry metadata and greenhouse tuning |
| `node` in spatial capture | provisional | do not use for guide capture unless formally expanded later | keep spatial work on points, anchors, guides, lines, polylines |

## Scene / UI Terms

| Term | Authority | YAM Meaning | Use For |
| --- | --- | --- | --- |
| `camera` | inferred | world-space crop origin or framing helper | projection and viewport control |
| `viewport` | strict | the visible terminal-sized crop area | what is currently being shown |
| `HUD` | inferred | screen-attached heads-up display layer | footer, hints, status, passive overlays |
| `suite` | inferred | a grouped set of related UI panels or controls | modal shell families, settings groups, or inspection panel collections |
| `shell` | inferred | a bounded UI container that wraps a group of controls | modal popups and panel families |
| `overlay` | strict | a top-most presentation layer above scene and HUD | modals, inspectors, and temporary control surfaces |
| `screen space` | strict | terminal-attached coordinate space | HUD and overlay placement |
| `world space` | strict | simulation-space coordinate system centered on the datum | entities, guides, flora, and placement logic |
| `frame` | strict | one full rendered terminal frame | the complete per-tick output |
| `layout` | inferred | how screen regions are arranged relative to each other | scene zoning and UI composition |
| `panel` | inferred | a bounded UI region inside the screen | status areas, inspectors, or modal content areas |
| `layer` | strict | one ordered render stratum in the scene stack | world base, HUD, debug, and modal ordering |
| `zone` | inferred | a stable screen region with a defined purpose | main scene, footer, debug/inspect, modal overlay |
| `modal` | strict | an interaction state that captures focus until dismissed | hotkeys, move, settings, and other popups |
| `footer` | strict | the bottom status strip of the terminal frame | compact mode hints and runtime reminders |
| `status bar` | inferred | the always-visible status/footer strip | runtime reminders, mode labels, and version info |
| `debug overlay` | inferred | an always-available diagnostic presentation layer | camera, pointer, and world readouts |
| `inspect` | inferred | focused reading and drill-down interaction mode | entity detail and read-only investigation |
| `dev mode` | inferred | gated editing and mutation surface | developer controls and simulation tooling |
| `command palette` | inferred | search-based action hub | rare actions, jumps, and toggles |
| `main scene` | strict | the primary world-visualization area | hero, flora, guides, and world-tied content |
| `greenhouse` | inferred | the dedicated multi-species simulation space | plant development, rooms, and labs |

## YAM Guidance

- use `node` only for plant anatomy and morphology
- use `points`, `anchors`, `guides`, `lines`, and `polylines` for spatial authoring
- keep structural terms separate from reproductive terms
- prefer strict botanical terms when available, and reserve inferred terms for YAM-specific abstractions
- use `camera`, `viewport`, `HUD`, `suite`, `overlay`, `screen space`, and `world space` consistently with the scene-model and rendering contracts
- use `shell` when referring to the shared modal container for hotkeys, move, and settings
- use `layer`, `zone`, `modal`, `footer`, `status bar`, `debug overlay`, `inspect`, `dev mode`, `command palette`, `main scene`, and `greenhouse` consistently with the presentation and interaction contracts
