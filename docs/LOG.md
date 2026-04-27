# YAM Log

## 2026-04-26

- added `docs/SCENE_MODEL.md` to formalize the deterministic scene layer above ratatui, including layer order, coordinate spaces, masking, and the frame pipeline
- aligned the scene docs with the terminal presentation split: world, HUD, and overlay
- compiled the `new/` concept notes into the repo-wide `TODO.md` backlog at the repository root
- tightened `TODO.md` wording so it matches the scene and architecture docs without introducing a second projection or presentation vocabulary
- added `docs/REFERENCE_ARCHIVE.md` to catalog the `yam-rust_debugging_260424-2009_dump` notes as reference-only archive material
- marked the archived dump notes as reference-only in `TODO.md` so the active backlog stays separate from historical context
- added an archive-derived ideas section to `TODO.md` so the useful design patterns stay visible without collapsing the archive back into active work
- added a concrete next-steps section to `TODO.md` for projection, invariance, and hero-rendering follow-up work
- replaced the TODO implementation order with the explicit work order: ui -> main scene stabilisation -> hero gif -> main scene stabilisation -> main scene vines -> main scene stabilisation
- expanded the `ui` phase in `TODO.md` with a checklist that keeps projection ownership, world-state boundaries, and footer/HUD/overlay responsibilities explicit
- added a main scene stabilisation checklist to `TODO.md` covering singular projection, world/HUD boundaries, and resize invariance before hero work resumes
- added a hero GIF checklist to `TODO.md` that keeps frame ownership, geometry stability, and cached rendering decisions explicit before the vines phase
- added explicit main scene stabilisation exit criteria to `TODO.md` so resize, camera, footer, and border-probe behavior have a hard checkpoint before hero or vines work continues
- added a main scene vines checklist to `TODO.md` that keeps vine work world-attached and constrained by the same projection and masking rules as the rest of the scene
- clarified that every stabilization checkpoint in `TODO.md` must end with the scene matching the current presentation contract before the next phase starts
- added a final TODO alignment rule so the backlog stays subordinate to `SCENE_MODEL.md` and `ARCHITECTURE.md` whenever the work order changes
- archived the `todo_review.md` critique and promoted its strongest gaps into `TODO.md` as explicit projection, invariants, RenderState, validation, and hero decision-gate contracts
- promoted archive ideas into `TODO.md` as explicit projection, layering, determinism, and greenhouse integration contracts
- promoted archive stress-test ideas into `TODO.md` as startup invariants, projection round-trip checks, and static-input determinism validation
- moved dated audit/update/issue reports into `docs/archive/reports/` and added an archive index so active docs stay contract-focused
- demoted `NOTES.md` into a short working-notes file and archived the old `DOTFILES_MIGRATION.md` report with the other dated docs
- clarified the front-door docs split: root `README.md` for orientation, `docs/README.md` for navigation, and `NOTES.md` for non-authoritative working notes
- documented the filename convention that reserves uppercase markdown names for high-visibility entry points and prefers lower-case names for routine docs
- renamed the routine contract docs in `docs/` to lowercase filenames so the tree reserves uppercase names for the few front-door entry points
- flattened the root `README.md` into the master repo doc so it now owns the active backlog and working guidance while `TODO.md` and `NOTES.md` act as compatibility pointers
- removed the root `TODO.md` and `NOTES.md` stubs after folding their useful content into the master `README.md`
- restored `TODO.md` as the active backlog, trimmed `README.md` back toward a front door, and repaired stale docs links to the lowercase contract filenames
- rewrote `docs/audit.md` into a current risk snapshot, trimmed `TODO.md` back to backlog pointers, and archived the completed flattening/version-map notes

## 2026-04-23

- pruned Go-era root artifacts from the repo
- moved the active documentation surface to `docs/`
- removed the legacy `docs/v2/` tree
- updated the root README and hygiene notes for the Rust baseline
- committed the Rust baseline and the documentation hygiene refresh
- removed dead `visualizer/` ignore entries and normalized the repo hygiene wording
- verified `cargo clippy -- -D warnings` and `cargo run -- --version` on the Rust baseline
- added architecture contract docs, module headers, and a strict repo check script
- enforced the module boundary contract and verified `scripts/check.sh`
- added the render layer contract and scene-level render order header
- integrated a cached chafa-backed hero frame path with escape-sequence stripping
- switched the hero frame path to ANSI-preserving ratatui text and derived hero bounds from the rendered frame size
- replaced the hero snapshot path with a persistent chafa stream and UI-owned frame updates
- added a compile-time/runtime identity stamp to prove `cargo run` and `yam-rust` share the same source tree and build markers
- removed the chafa stream signature gate so frame snapshots are no longer collapsed before reaching the hero state
- aligned the crate/package version with the runtime footer so `cargo run` and `yam-rust` both report `v0.3.0`
- replaced PTY-based hero capture with deterministic GIF decoding plus per-frame single-image chafa rendering
- fixed temp GIF frame export to deterministic PNG filenames and verified the hero buffer loads 64 frames in tests
- preserved ANSI-derived style spans while clipping hero frame text in the renderer
- introduced a minimal scene + layer compositor and routed the existing render order through it without changing output

## 2026-04-24

- isolated legacy Python runtime, render, engine, hero, morphology, shape, theme, and UI artifacts under `tools/legacy-python/`
- moved experimental Python entrypoints under `tools/experiments/`
- normalized `src/theme/` into palette/style/glyph-oriented styling modules
- moved rendering authority from `ui` toward `scene` layers and removed legacy panel rendering from the active path
- added fixed-size hero frame normalization and control-character cleanup to reduce frame jitter
- introduced grid-backed composition primitives, scene-level `LayerOutput`, mask plumbing, and a hero mask probe path
- added persisted runtime UI state for hero offsets, clock offsets, clock font, camera position, and hero animation FPS
- added debug-only editing controls, debug telemetry, and split normal/debug footer text
- added `scripts/update.sh` and CLI update/check-update entrypoints
- added build identity plumbing for runtime footer/version reporting
- documented current coordinate/camera/rendering caveats in the repo audit and rendering docs
- standardized the active camera/viewport path so fullscreen can lock to a datum-centered crop while windowed mode keeps mutable panning, and aligned the debug border mapping to it
- removed render-time hero/clock anchor writes from the active layer path and made debug reconstruct its telemetry from pure state helpers
- removed the legacy `Layer::render(...)` path from the scene contract and dropped the `(0,0)` hero sentinel from runtime state
- made the grid writer grapheme/display-width aware for safer text placement
- verified the tree with `cargo check` and `cargo test` during the audit pass
- removed the centered viewport-tier placement path from `Scene::render` so resize/fullscreen transitions no longer introduce a second framing rule
- added a read-only per-frame `FrameContext` so hero, clock, and debug now share one projection snapshot instead of recomputing it inside layers
- added coordinate invariance tests for anchor/world/screen composition and screen-space overlays
- removed stale repo-split wording after deleting the legacy `/Users/maciejkuster/_git/yam` checkout
- ingested the 2026-04-24 chatgpt audit report and folded its findings into the repo audit trail
- renamed the per-frame snapshot to `RenderState` so the render path has one explicit read-only frame contract
- aligned repo docs with the `RenderState` snapshot contract and removed stale `UiState` side-effect wording
- corrected the active hero/clock placement contract so both remain world-pinned and camera movement no longer reprojects them
- tightened the world datum contract to define `(0, 0)` as the centered origin with signed quadrants around it
- recorded the coordinate orientation split: world is Cartesian, terminal/screen is downward-y
- recorded the hero geometry contract: `820x820` source GIF renders to a fixed `96x48` terminal footprint
- replaced the dotted debug world-edge probe with a stable ASCII frame border probe
- documented the intentional debug border padding row and side padding cell for future UI placement
- documented that the bottom one-row padding is currently occupied by the footer
- clarified that camera is the world-space origin of the crop and viewport is the terminal-sized crop rectangle
- recorded the world-ui vs hud-ui split: world-ui stays attached to world entities; hud-ui stays attached to the viewport/camera/terminal frame
- clarified that world-ui stays world-pinned while hud-ui stays screen-attached, and encoded that split in `src/scene/coords.rs`
- reclassified the clock as world-ui and the footer/status bar as hud-ui so the active contract matches runtime behavior
- split `RenderState` into explicit `world` and `hud` sections and added a resize-invariance test for the frame builder
- encoded the footer bottom-row rule in `footer_row(height)` and added a unit test for the HUD row contract
- restored the ASCII world border probe as a datum-centered world-space indicator and aligned its camera-projected meaning with the active contract
- added a current-issues report covering the remaining static-vs-dynamic contract confusion points
- tightened the current-issues report with an explicit static/dynamic placement table for world-ui, hud-ui, camera, and viewport semantics
- clarified the fullscreen lock contract in the current issues, audit, and rendering docs so windowed panning and fullscreen datum-centered static framing are distinct rules
- implemented the fullscreen lock as a render-state rule and added tests that pin fullscreen datum-centered lock vs windowed-pan behavior
- fixed the current-issues note to call out the debug telemetry mismatch where `Clock screen` was still reported from raw world data instead of the projected value
- added `RenderState::clock_screen()` so clock rendering and debug telemetry share one projected clock position
- added tests for projected clock telemetry and fullscreen clock-screen invariance under stored camera motion
- updated the current-issues and rendering docs to mark the clock telemetry mismatch corrected
- changed the default windowed starting camera baseline to `(-69, -17)` for the `124x32` terminal screenshots
- clamped windowed runtime and render-state camera crops to at most one cell of overscan beyond the world border/frame and recorded that architecture rule
- added tests for the `124x32` starting baseline and windowed camera overscan clamp
- expanded and flattened decoded GIF subimage frames to an opaque full `820x820` logical canvas before chafa rendering to prevent frame 15 and frame 30 vertical overstretch
- added a regression test for full-canvas opaque hero frame geometry across partial GIF frames

## Log Rules

- append entries in date order
- keep entries factual and short
- record material repository changes, not speculative notes
