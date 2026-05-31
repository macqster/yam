# YAM 0.4 Expansion — ChatGPT Preflight Check

Purpose: give Codex a compact, execution-ingestable checkpoint before any new YAM 0.4 feature work begins.

Scope: pre-feature readiness only. Do not implement new visible features in this pass.

## Operating Rule

Before adding new runtime features, stabilize the 0.3.9 -> 0.4.0 transition baseline.

The repo is not ready for broad feature expansion until the following gates are green:

```text
docs aligned
verification green
spatial ownership stable
flora storage decision made
greenhouse/world contract decided
hero/render pipeline failure modes hardened
```

## Hard Constraints

- Do not add new visible greenhouse runtime features yet.
- Do not add new plant families yet.
- Do not introduce a second projection/world-to-screen model.
- Do not weaken `scripts/verify.sh`, `scripts/check.sh`, or existing architecture guardrails.
- Keep `known_issues.md` for concrete active bugs only.
- Keep `TODO.md` execution-oriented, not a speculative design document.
- Keep broad/open risks in `docs/audit.md`.
- Prefer small, reviewable patches.

## Immediate Baseline Checks

Run locally in the real dev environment:

```bash
bash scripts/verify.sh
```

Expected verification spine:

```text
scripts/check-docs.sh
scripts/check.sh
cargo test --quiet
```

`scripts/check.sh` is expected to protect these boundaries:

```text
core must not depend on scene modules
systems must not depend on scene/render/UI/terminal modules
project_world_to_screen must stay isolated
scene::coords compatibility aliases must stay isolated
cargo fmt --check
cargo clippy -- -D warnings
cargo check
```

If verification fails, fix verification before any feature expansion.

## Pre-Feature Work Batches

### Batch A — Verification Baseline

1. Run `bash scripts/verify.sh`.
2. Fix any fmt/clippy/check/test/doc-check failures.
3. Confirm whether `known_issues.md` is empty or contains only concrete current issues.
4. Add a concise `docs/LOG.md` entry noting the 0.4 pre-expansion baseline.

Done when:

```text
bash scripts/verify.sh passes locally
```

### Batch B — Contract Alignment

Review and align these files:

```text
TODO.md
docs/audit.md
docs/greenhouse-roadmap.md
docs/vines.md
docs/scene-model.md
docs/architecture.md
docs/rendering.md
docs/hygiene.md
known_issues.md
```

Goals:

- One active truth for what is implemented.
- One active truth for what is planned.
- One active truth for what is blocked.
- No duplicated stale design promises.
- No speculative feature work promoted into implementation language.

Done when:

```text
TODO = execution backlog
audit = risks/open gates
LOG = history
known_issues = concrete active bugs only
```

### Batch C — Architecture Decisions

Make these decisions before new feature implementation:

#### C1. Spatial Layer

Current intended direction:

```text
core::spatial is canonical
scene::coords is compatibility-only
projection remains singular
world/screen/anchor spaces remain type-distinct
```

Gate:

```text
No new world-attached renderable is added until its projection/anchor/guide relationship routes through core::spatial or an explicitly allowed compatibility seam.
```

#### C2. Flora Storage

Problem:

```text
FloraState is still vine-shaped in storage, while the project now has shared organism vocabulary.
```

Decide one direction:

```text
Option 1: enum-backed family store
Option 2: organism registry
Option 3: small explicit multi-family structure
```

Current recommendation:

```text
Prefer enum-backed family store first.
Reason: enough for vines + greenhouse seedling/cutting prototypes without prematurely building a generic ECS-like registry.
```

Gate:

```text
Document the chosen multi-family flora storage direction before adding a new plant family.
```

#### C3. Greenhouse Contract

Decide the first implementation shape before runtime work:

```text
WorldKind::Greenhouse later
first room: greenhouse_nursery / propagation room
environment: per-room symbolic profile
inspection: popup/read-only first
species definitions: static Rust fixtures first
```

Name ownership before implementation:

```text
GreenhouseState
Room
Zone
Fixture
Environment
PlantingSite
```

Gate:

```text
No selectable Greenhouse world until room/environment/planting-site ownership is documented.
```

#### C4. Hero / Render Pipeline Failure Contract

Expected rule:

```text
hero rendering is renderer-owned and cache-first
live Chafa failures degrade to placeholder frames, not panic
fixed hero frame geometry stays stable across resize
```

Gate:

```text
Missing GIF / temp write failure / chafa failure / cache miss has a known fallback path.
```

### Batch D — Tiny Enabling Tests

Before feature expansion, prefer small tests/assertions for:

```text
WorldKind profile/selectability
Boot remains non-selectable
MainScene/Sandbox profile invariants
projection isolation guardrails
FloraState family identity/count adapters
SpeciesRegistry / OrganismJournal basics
greenhouse data fixtures once introduced
```

## Not Yet In Scope

Do not start these until the pre-feature gates are complete:

```text
new visible greenhouse world
new plant family runtime
main-scene flowers/fruit/particles
large scaffold enrichment
manual hero CellGrid editor
XP import/export
game-like greenhouse progression
persistent curation/retirement mechanics
```

## Suggested First Codex Task

Task name:

```text
YAM 0.4 pre-expansion readiness pass
```

Instruction:

```text
Audit the current repo against greenhouse_chatgpt_preflight_check.md. Do not add new features. Run the existing verification gate, report failures, and propose the smallest patch set needed to align TODO/audit/docs/known_issues with the 0.4 pre-expansion posture. Preserve existing guardrails.
```

Expected output:

```text
1. Verification result
2. Any failures with file/path evidence
3. Proposed minimal patch list
4. Any architecture decision still requiring human choice
5. No feature implementation unless explicitly approved afterward
```

---

# AGENTS.md + Skills Optimization Review

Purpose: capture the instruction-surface review in a Codex-ingestable form before changing repo guidance.

Scope: review and optimize `AGENTS.md`, repo-local skills, and docs validation rules. Do not rewrite the instruction system wholesale.

## Reviewed Surfaces

Review these repo surfaces together:

```text
AGENTS.md
skills/yam-maintenance/SKILL.md
skills/yam-architecture-review/SKILL.md
skills/*/agents/openai.yaml
scripts/check-docs.sh
```

## Optimization Principle

The target is not more instructions.

The target is:

```text
less ambiguity
lower context cost
cleaner task routing
clearer handoff output
less duplicated phase-specific guidance
```

Keep durable repo guidance in `AGENTS.md`. Keep phase-specific execution work in `TODO.md`, risks/open gates in `docs/audit.md`, and history in `docs/LOG.md`.

## Current Assessment

### AGENTS.md

Current strengths:

```text
clear read-first hierarchy
strong dirty-worktree rule
explicit no-feature-prep rule
clear module ownership boundaries
concrete verification commands
separated docs/log roles
skills section avoids duplicating detailed contracts
```

Main optimization risk:

```text
AGENTS.md currently mixes always-on repo rules, 0.4 pre-expansion constraints, and skill-authoring guidance.
```

This is acceptable now, but should be controlled so `AGENTS.md` does not become a duplicate backlog.

### yam-maintenance skill

Current strengths:

```text
correctly scoped to maintenance/stabilization/docs hygiene
clear workflow
strong no-runtime-feature guardrail during prep
little architecture duplication
```

Optimization target:

```text
Shorten the frontmatter description so it acts as routing text, not a mini-spec.
```

Suggested description:

```yaml
description: Use for YAM cleanup, verification, docs/log alignment, hygiene checks, tooling upkeep, and pre-expansion prep that must not add runtime features.
```

### yam-architecture-review skill

Current strengths:

```text
useful pre-feature review mode
explicitly review-first, implement-only-if-asked
covers the right 0.4 pressure points: greenhouse, flora, world modes, spatial, render layers, UI ownership
```

Optimization target:

```text
Shorten and sharpen the description so it routes architecture reviews without becoming a broad planning bucket.
```

Suggested description:

```yaml
description: Use for YAM architecture reviews before greenhouse, flora, spatial, render-layer, world-mode, or broad coherence changes.
```

## Recommended Minimal Patch Set

Make a small docs-only patch:

```text
1. Add AGENTS.md “Task Routing” section.
2. Add AGENTS.md “Scope Discipline” rule.
3. Shorten both SKILL.md frontmatter descriptions.
4. Add or normalize Handoff sections in both skills.
5. Append a docs/LOG.md note.
6. Run bash scripts/check-docs.sh.
7. Run bash scripts/verify.sh if the environment supports it.
```

No Rust changes.
No new runtime features.
No new skill yet.

## Suggested AGENTS.md Addition — Task Routing

Add a compact section similar to:

```markdown
## Task Routing

- Use `$yam-maintenance` for cleanup, verification, docs/log alignment, hygiene, and pre-expansion prep.
- Use `$yam-architecture-review` for design review before changing spatial, flora, world, greenhouse, render-layer, or UI ownership.
- For normal feature implementation, read this file plus the owning docs; do not use a review skill as permission to implement broad changes.
```

## Suggested AGENTS.md Addition — Scope Discipline

Add a compact section similar to:

```markdown
## Scope Discipline

- Keep this file durable. Put phase-specific work orders in `TODO.md`, risk snapshots in `docs/audit.md`, and one-off execution briefs outside the repo or in explicitly named planning docs.
- Prefer pointers over copied policy blocks when an owning doc already exists.
- Add new AGENTS.md rules only after repeated friction or verified project need.
```

## Suggested Skill Handoff Sections

### yam-maintenance

Add or normalize:

```markdown
## Handoff

- Summarize changed files.
- List verification commands run and results.
- Note any skipped checks with reasons.
- Identify remaining blockers without proposing feature work.
```

### yam-architecture-review

Add or normalize:

```markdown
## Handoff

- Findings first, ordered by severity.
- Include file/path evidence.
- Separate must-fix blockers from optional cleanup.
- End with the smallest safe next action.
```

## Optional Validator Tweaks

Consider only small validation additions later:

```text
SKILL.md should mention ../../AGENTS.md
SKILL.md frontmatter description should preferably be <= 300 chars
```

Do not overbuild `scripts/check-docs.sh` yet.

## Do Not Add More Skills Yet

Current skill set is enough:

```text
yam-maintenance
yam-architecture-review
```

Potential future skills should wait until workflows repeat at least twice:

```text
yam-flora-prep
yam-greenhouse-planning
yam-release-handoff
yam-docs-hygiene
```

Reason:

```text
Adding skills too early increases routing ambiguity and context overhead.
```

## Suggested Codex Task

Task name:

```text
YAM agent/skill instruction-surface optimization
```

Instruction:

```text
Audit AGENTS.md and repo-local skills against greenhouse_chatgpt_preflight_check.md. Make only a small docs/instruction patch: add task routing and durable scope-discipline guidance to AGENTS.md, shorten skill routing descriptions, normalize skill handoff sections, and update docs/LOG.md. Do not add runtime features, do not create new skills, and do not weaken verification guardrails.
```

Expected output:

```text
1. Files changed
2. Exact instruction-surface changes made
3. Validation commands run and results
4. Any skipped validation with reason
5. Remaining risks or decisions, if any
```

---

# Codex Preflight Bundle Completion Addendum

Purpose: make this file a complete launch brief for Codex without turning it into a duplicate repo manual.

This addendum should help Codex start by observing the repository state, identify decisions it must not silently make, stop safely when blocked, and verify that the preflight/instruction-surface pass is complete.

## Preflight Bundle Authority

This file is a preflight/handoff bundle.

It does not replace these owning repo surfaces:

```text
AGENTS.md
TODO.md
docs/audit.md
docs/architecture.md
docs/scene-model.md
docs/rendering.md
docs/greenhouse-roadmap.md
docs/vines.md
known_issues.md
skills/*/SKILL.md
```

If this file conflicts with an owning repo doc, Codex must report the conflict instead of silently choosing one.

Authority rule:

```text
AGENTS.md = durable always-on repo guidance
skills/*/SKILL.md = task-specific reusable workflows
TODO.md = execution backlog
docs/audit.md = risks/open gates
docs/LOG.md = history
this file = external preflight launch brief
```

Do not copy large policy blocks from this file into repo docs unless they are intentionally promoted to an owning surface.

Prefer pointers over duplicated contracts.

## Repo State Intake Checklist

Before editing, Codex should inspect the actual repo state.

Run or attempt:

```bash
git status --short
git branch --show-current
git log -1 --oneline
find . -maxdepth 3 \( -name AGENTS.md -o -name SKILL.md \) -print
bash scripts/check-docs.sh
bash scripts/verify.sh
```

Report:

```text
current branch
dirty/untracked files
latest commit
available AGENTS.md / SKILL.md surfaces
check-docs result
verify result
whether cargo/rust tooling is available
```

If `bash scripts/verify.sh` cannot run because the environment lacks Rust/Cargo or required runtime dependencies, do not guess success. Report the skipped check and exact reason.

## Human Decision Log

These decisions are named but not fully implemented policy yet. Codex must not silently settle them during preflight cleanup.

| Decision | Default for now | Codex authority |
|---|---|---|
| Flora storage model | Enum-backed family store preferred | Propose/document only unless explicitly approved for implementation |
| Greenhouse first room | Nursery / propagation room | Document-only until approved |
| Environment ownership | Per-room symbolic profile | Document-only until approved |
| Inspection mode | Popup/read-only first | Document-only until approved |
| Species source | Static Rust fixtures first | Implement only after approval |
| `WorldKind::Greenhouse` | Later, not selectable yet | Do not add runtime world yet |
| New plant families | Not yet | Do not add runtime plant family yet |
| Feature expansion start | After preflight gates pass | Do not begin features in this pass |

If a requested patch requires choosing among these, stop and report the smallest decision needed.

## Patch Discipline / Stop Conditions

Use small, reviewable patches.

Prefer docs/instruction edits only for this preflight pass.

Do not modify runtime Rust files unless explicitly asked after the preflight pass.

Stop and report instead of patching if:

```text
verification fails for reasons unrelated to the intended docs/instruction patch
cargo/rust toolchain is unavailable and verification cannot be completed
repo has unrelated dirty files
an owning doc contradicts this preflight file
implementation would require Rust/runtime changes
a human-choice-required decision blocks progress
the requested change would create a duplicate backlog or duplicate architecture contract
```

When stopped, output:

```text
1. blocker
2. evidence path/command
3. smallest safe next action
4. no speculative patch
```

## Preflight Pass Acceptance Criteria

The preflight/instruction-surface pass is complete only when:

```text
AGENTS.md contains task-routing guidance or already has equivalent routing clarity
AGENTS.md contains durable scope-discipline guidance or already has equivalent scope clarity
skills have short routing-oriented descriptions
skills have clear handoff sections
skills still point back to repo-level guidance rather than duplicating it
scripts/check-docs.sh passes
bash scripts/verify.sh passes, or skipped verification is explicitly justified by environment limits
known_issues.md remains concrete-only
TODO.md remains execution-only
no runtime feature files changed
no new skills added
no greenhouse runtime world added
no new plant family runtime added
docs/LOG.md records the instruction-surface/preflight update if repo files were changed
```

## Final Codex Response Shape

For this preflight pass, Codex should finish with:

```text
1. Summary
2. Files changed
3. Validation commands run
4. Validation results
5. Skipped checks and exact reasons
6. Remaining blockers or human decisions
7. Smallest safe next action
```

Do not end with a broad feature roadmap.

Do not propose multiple implementation branches unless a human decision is actually required.

Do not claim verification passed unless the command actually passed.

---

# Codex Operator Cues

Purpose: provide short prompts and behavioral cues for running Codex against YAM 0.4 without causing scope drift.

These cues are session-level operating patterns. They do not replace `AGENTS.md`, repo-local skills, or owning docs.

## Preferred Session Pattern

Use this pattern for non-trivial work:

```text
observe -> summarize -> propose smallest patch -> approve/patch -> verify -> report
```

For ambiguous or architectural work, use Plan mode first.

Do not start in implementation mode when the task touches:

```text
spatial ownership
flora storage
greenhouse world/state
render layer ordering
hero pipeline
UI modes / inspection surfaces
skills or AGENTS.md
```

## Good First Prompt Shape

Use prompts shaped like:

```text
Read AGENTS.md and the relevant owning docs first. Inspect repo state. Do not edit yet.

Report:
1. current branch and dirty files
2. relevant files/docs read
3. verification status if run
4. smallest safe patch proposal
5. decisions that need human approval
```

Then approve a specific patch.

## Good Patch Prompt Shape

Use prompts shaped like:

```text
Make the smallest patch that does X. Do not change runtime behavior. Do not add new features.

After patching, run Y. Report changed files, validation results, skipped checks, and remaining blockers.
```

## Bad Prompt Shapes To Avoid

Avoid broad prompts like:

```text
improve the architecture
clean this up
make greenhouse work
add flora support
prepare 0.4
fix the docs
```

Replace them with bounded prompts:

```text
Audit docs/audit.md and TODO.md for duplicated 0.4 preflight work. Propose a docs-only patch. Do not edit runtime files.
```

## Approval Cues

Use explicit approval language:

```text
approved: docs-only patch
approved: test-only patch
approved: runtime patch in these files only: ...
approved: run verification only
approved: report-only, no edits
```

If approval is not explicit, Codex should prefer report-only mode for architecture-sensitive tasks.

## Stop Cues

Use these to halt drift:

```text
stop after repo-state intake
stop after patch proposal
stop before runtime changes
stop before creating new files
stop before changing verification scripts
stop if cargo/clippy/test failures are unrelated
```

## Owning Doc Cue

Before changing behavior, Codex should identify the owning doc and summarize the relevant contract.

Use this shape:

```text
Owning doc:
Relevant contract:
Files likely affected:
Patch authority:
Verification command:
```

If no owning doc exists, Codex should report that gap instead of inventing a new contract in code.

## Report-Only Default

Architecture-sensitive tasks are report-only unless the prompt explicitly says:

```text
patch approved
```

This applies especially to:

```text
spatial layer changes
flora storage changes
greenhouse state/world changes
render layer changes
hero pipeline changes
UI mode changes
agent/skill instruction changes
```

## File-Scope Approval Cue

For safer patches, approve exact file scope:

```text
Only edit these files:
- AGENTS.md
- skills/yam-maintenance/SKILL.md
- skills/yam-architecture-review/SKILL.md
- docs/LOG.md
```

If Codex needs another file, it should stop and ask/report before editing it.

## Handoff Cue

Every Codex response should end with:

```text
Smallest safe next action:
...
```

Do not end with a broad roadmap unless explicitly asked.

## Verification Honesty Cue

Codex must distinguish:

```text
passed
failed
not run
skipped because ...
```

Do not claim verification passed unless the command actually passed.

```
