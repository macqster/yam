# YAM Log

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
- standardized the active camera/viewport path on top-left offset semantics and aligned the debug border mapping to it
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

## Log Rules

- append entries in date order
- keep entries factual and short
- record material repository changes, not speculative notes
