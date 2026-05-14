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
- run `scripts/verify.sh` for the full maintenance/release gate; it composes the active docs check, the Rust hygiene gate, and the full `cargo test --quiet` suite in one repo-owned command
- active markdown docs should stay clean under the repo-configured `cSpell` and `markdownlint` rules; fix real typos and malformed markdown, but prefer adding recurring valid repo vocabulary to shared config rather than repeatedly rewriting good technical terms
- run `scripts/check-docs.sh` for docs-hygiene batches; it enforces the active-doc inventory, `TODO.md` -> `known_issues.md` issue-link consistency, and the canonical `Cargo.toml` -> `README.md` current-release version match directly, and it also runs repo-configured `cspell` / `markdownlint` checks when those CLIs are available in the local environment
- opt-in diagnostics logs should stay local-only and out of the repo; the current path is the user state dir (`$XDG_STATE_HOME/yam/diagnostics.ndjson` or `~/.local/state/yam/diagnostics.ndjson`) behind `YAM_DIAGNOSTICS=1`
- when diagnostics are enabled, prefer the installed `yam-diagnostics` helper for quick local inspection instead of ad hoc repo scripts; keep the format simple enough that raw NDJSON stays readable too
- prefer repo-level spell/lint configuration for recurring terminology; use file-local `cspell:ignore` comments only for narrow local exceptions that would add noise to the shared dictionary
- avoid link-like bracket labels such as `[fix]` or `[12:34]` in docs unless they are real markdown links or task checkboxes; use code-style labels like ``fix`` or ``12:34`` for lightweight tags and timestamps
- front-door and active contract docs should prefer broadly readable English, while archive/history docs may keep source-accurate jargon, tool names, and quoted vocabulary as long as the shared spell/lint config recognizes them
- soft feature freeze: prefer polish and stability work only; avoid new features unless they fix a correctness bug, a regression, or a contract violation
- pre-new-feature gate: before starting any new feature, verify the modal/UI state is clean, camera behavior is explicit, hero rendering is stable, docs/logs match the current contract, and the relevant regression tests still pass
