# Repo Hygiene

## Baseline

- Rust code in `src/` is the active implementation tree
- `Cargo.toml`, `Cargo.lock`, `src/`, `assets/`, and `docs/` are the tracked surface
- the old reconstruction archive has been removed

## Rules

- keep changes logged in `docs/LOG.md`
- keep commits focused
- keep `AGENTS.md` procedural and pointer-heavy; it should describe how agents work in the repo, while architecture facts stay in the owning docs
- keep repo-local `skills/*/SKILL.md` files short and workflow-focused; they should trigger repeatable work modes without becoming alternate contract docs, and their names and frontmatter descriptions must pass `scripts/check-docs.sh`
- keep repo-local skill `agents/openai.yaml` files present, short, generated from the matching skill name when practical, and valid under the interface metadata checks in `scripts/check-docs.sh`
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
- before trusting a new regression test, prove it can fail: temporarily revert the fix, confirm the test fails with the expected symptom (not an unrelated error), then restore the fix; this is cheap insurance against a test that passes for the wrong reason, not full TDD
- prefer explicit, non-wildcard match arms for core enums such as `WorldKind` (one arm per variant, or an explicit multi-variant arm, rather than a trailing `_ =>`); adding a variant then makes the compiler enumerate every call site that needs attention, which is a stronger completeness guarantee than grepping for call sites by hand
- this is a terminal application with no dev-server/browser equivalent for "run it and look at it"; verify interactive/visual changes by driving the real release binary in a detached `tmux` session (`tmux new-session -d`, `tmux send-keys` for input, `tmux capture-pane -p` to read the rendered terminal as text) and inspecting the captured output directly; treat this as a one-off manual spot-check, not a regression test, and write a real automated test if the same visual claim needs to be reasserted later
- use `cargo fmt --check` for docs-only or wording-only changes; run the full test suite for compositor, camera, overlay, or other shared-rendering behavior changes
- run `scripts/check.sh` for Rust hygiene; it fails if `src/core` imports `scene` modules or if `src/systems` imports scene, render, UI, or terminal modules, then runs format, clippy, and cargo check (the former `crate::scene::coords` isolation guard was removed on 2026-07-21 along with the now-retired `scene::coords` module it was guarding)
- run `scripts/verify.sh` for the full maintenance/release gate; it composes the active docs check, the Rust hygiene gate, and the full `cargo test --quiet` suite in one repo-owned command
- an opt-in `pre-push` hook that runs `scripts/verify.sh` before every push lives in `scripts/git-hooks/pre-push`; enable it once per clone with `git config core.hooksPath scripts/git-hooks` (skip a single push with `git push --no-verify`); it is not enabled by default because it adds real wall-clock time to every push and that tradeoff should be an explicit choice, not a silent default
- active markdown docs should stay clean under the repo-configured `cSpell` and `markdownlint` rules; fix real typos and malformed markdown, but prefer adding recurring valid repo vocabulary to shared config rather than repeatedly rewriting good technical terms
- run `scripts/check-docs.sh` for docs-hygiene batches; it enforces the active root/front-door docs, `AGENTS.md`, repo-local `skills/*/SKILL.md` workflow files, required skill frontmatter and `agents/openai.yaml` interface metadata, the first-level `docs/*.md` contract surface, the `docs/chatgpt-0.4-source-pack/*.md` upload-pack surface, `TODO.md` -> `known_issues.md` issue-link consistency, the canonical `Cargo.toml` -> `README.md` current-release version match, and README local-asset path validity directly, and it also runs repo-configured `cspell` / `markdownlint` checks when those CLIs are available in the local environment
- findings produced without a working Rust toolchain (no `cargo`, blocked network to crates.io/rustup, etc.) must be labeled as inferred/unverified in the doc that records them, not stated as settled fact; text-based inspection of `Cargo.lock` or source files is a reasonable fallback when no toolchain is available, but it has produced concretely wrong conclusions before (a duplicated-dependency-graph claim and a stale test count both turned out inaccurate once checked with real `cargo`) and should not carry the same confidence as a toolchain-verified finding
- dated readiness snapshots (`docs/audit.md`'s `Last reviewed`, `docs/greenhouse-roadmap.md`'s `Last checked`, and similar) should state which specific claims were re-verified in the current pass versus carried forward from a prior pass, rather than bumping the whole date stamp after only checking part of the snapshot; a partial re-check that silently reads as a full one is worse than an explicit partial note
- opt-in diagnostics logs should stay local-only and out of the repo; the current path is the user state dir (`$XDG_STATE_HOME/yam/diagnostics.ndjson` or `~/.local/state/yam/diagnostics.ndjson`) behind `YAM_DIAGNOSTICS=1`
- when diagnostics are enabled, prefer the installed `yam-diagnostics` helper for quick local inspection instead of ad hoc repo scripts; keep the format simple enough that raw NDJSON stays readable too
- prefer repo-level spell/lint configuration for recurring terminology; use file-local `cspell:ignore` comments only for narrow local exceptions that would add noise to the shared dictionary
- avoid link-like bracket labels such as `[fix]` or `[12:34]` in docs unless they are real markdown links or task checkboxes; use code-style labels like ``fix`` or ``12:34`` for lightweight tags and timestamps
- front-door and active contract docs should prefer broadly readable English, while archive/history docs may keep source-accurate jargon, tool names, and quoted vocabulary as long as the shared spell/lint config recognizes them
- front-door docs must not reference local preview/media assets unless those files are committed at the referenced paths
- soft feature freeze: prefer polish and stability work only; avoid new features unless they fix a correctness bug, a regression, or a contract violation
- pre-new-feature gate: before starting any new feature, verify the modal/UI state is clean, camera behavior is explicit, hero rendering is stable, docs/logs match the current contract, and the relevant regression tests still pass
