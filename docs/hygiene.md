# Repo Hygiene

## Baseline

- Rust code in `src/` is the active implementation tree
- `Cargo.toml`, `Cargo.lock`, `src/`, `assets/`, and `docs/` are the tracked surface
- the old reconstruction archive has been removed

## Rules

- keep changes logged in `docs/LOG.md`
- keep commits focused
- keep build output out of version control
- do not reintroduce old runtime artifacts without a clear reason
- avoid hidden coupling between engine, render, and UI layers
- use uppercase markdown filenames only for front-door docs or similarly high-visibility entry points; prefer lower-case names for routine contracts and archive notes
- "must" means an invariant, "should" means a recommendation, and "is" means a descriptive statement
- if a concept appears in more than one active doc, one doc must be canonical and the others must link to it
- any behavior change must update the owning contract doc in the same change
- terminology is part of the contract surface; do not introduce synonyms for `world-space`, `screen-space`, `vines`, or `RenderState`
- visual hygiene matters: preserve legibility for glyphs, letters, numerals, stats, and dense simulation data; avoid decorative clutter that harms scanability
- prefer existing Rust/ratatui tools, plugins, and scripts where they keep the interface cleaner and more maintainable
- keep Rust as the runtime core unless a different tool clearly improves research, registry authoring, or offline analysis; sidecar tools are acceptable for botanical research, species data preparation, and batch inspection, but they should not replace the live simulation/render path
- Lua plugins or scripts are acceptable as a bounded extension layer for species authoring or debug/dev tooling, provided Rust remains the host, the canonical state owner, and the deterministic runtime path stays unchanged
- temp files and shared runtime artifacts used by tests or helpers should be isolated per run when practical
- boundary changes should add a negative test when practical so forbidden behavior stays explicit
- use `cargo fmt --check` for docs-only or wording-only changes; run the full test suite for compositor, camera, overlay, or other shared-rendering behavior changes
- soft feature freeze: prefer polish and stability work only; avoid new features unless they fix a correctness bug, a regression, or a contract violation
- pre-new-feature gate: before starting any new feature, verify the modal/UI state is clean, camera behavior is explicit, hero rendering is stable, docs/logs match the current contract, and the relevant regression tests still pass
