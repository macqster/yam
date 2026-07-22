# Session Handoff: Internal Inspection Prep (2026-07-21)

Status: session handoff / non-authoritative
Purpose: prime a fresh Claude Code session (this one's context window is
nearly full) to critically re-evaluate a long stabilization + feature pass,
extract lessons, and reconcile docs — not to blindly continue building.
Do not treat this file's claims as verified fact; it is one participant's
summary of its own work and must be checked against the actual repo state.

## How to use this document

1. Read this whole file first for orientation.
2. Do **not** trust the summary below at face value — independently verify
   the load-bearing claims (list in section 3) against real code, real test
   runs, and real `git diff` output before acting on them.
3. Use your own task tracking for the inspection itself; this file is
   input, not a task list to execute mechanically.
4. The user's own framing for this pass was: "internal inspection of the
   work done so far: reevaluation, lessons learned, possible revisions of
   agents and skills, docs reconciliation." Treat that as the actual brief;
   this file exists to make that brief tractable without re-deriving
   everything from `git log`.

## 0. Critical state — read this before anything else

- Branch: `consolidate-spatial-and-harden-verification`.
- Draft PR already open: <https://github.com/macqster/yam/pull/5> — title
  "Retire scene::coords, harden panic-safety, fix flaky weather test",
  currently reflects only commit `4f5c6b4`. **It is stale**: it does not
  include the commit below, nor any of the substantial uncommitted work
  described in section 2.
- `git log` on this branch, newest first:
  - `d647343` "fix field and weather refresh invariants" (2026-07-21
    21:00:28+0200) — **authored solely by the user (Maciej Kuster), not by
    this session**. It exists locally, has *not* been pushed to
    `origin/consolidate-spatial-and-harden-verification` (local branch is
    "ahead 1"), and its actual content was **not independently reviewed by
    this session** beyond a `git show --stat`/diff skim just before writing
    this handoff. It touches `src/systems/fields.rs`, `src/ui/state.rs`,
    `docs/LOG.md`, `docs/audit.md`, `docs/greenhouse-roadmap.md`,
    `docs/weather-widget.md`. One concrete thing worth checking: it added a
    rule to `docs/weather-widget.md` — *"Tests may inject deterministic
    provider results, but they must exercise the same background worker and
    channel path used by production refreshes."* — which may or may not be
    in tension with this session's own `spawn_weather_snapshot_fetch`
    test/production split in `src/ui/state.rs` (see section 2). Reconcile
    this explicitly; do not assume they agree.
  - `4f5c6b4` "Retire scene::coords, harden panic-safety, fix flaky weather
    test" (2026-07-21 18:42:45Z) — this session's own commit, pushed, is
    what PR #5 currently contains.
  - Everything before that is pre-existing history, unrelated to this pass.
- **A large amount of further work sits uncommitted in the working tree
  right now** (see `git status --short` / `git diff --stat` for ground
  truth — do not trust any file list here, it will drift). Broad shape:
  flora-storage lock-in, `WorldState` greenhouse attachment, growth-dispatch
  generalization, species-profile lock-in, and a full `WorldKind::Greenhouse`
  feature (new render layer, hotkey wiring, persistence enum). Detailed in
  section 2.
- Nothing past `4f5c6b4` has been committed. Whether/how to commit
  (one commit vs. several, same PR vs. new one) is an open decision for the
  user — this session was explicitly told to only commit when asked, and
  was never asked past the PR-creation point.

## 1. What this session actually did, in order

Everything below is cross-referenced in `docs/LOG.md` under the
`2026-07-21` day headers (there are several, since the day's work spanned
many distinct sub-passes) — treat those timestamps as the authoritative,
detailed record; this is just an index into them.

1. **Spatial relation layer consolidation** (committed, `4f5c6b4`): retired
   `src/scene/coords.rs` after confirming zero external call sites.
2. **Verification-green restoration** (committed, `4f5c6b4`): fixed 4
   pre-existing clippy errors, a cspell gap, and — the substantial one — the
   real root cause of an intermittently-failing weather test (it was making
   live network calls to `wttr.in` in the test suite).
3. **Panic-safety hardening** (committed, `4f5c6b4`, plus more later
   uncommitted): `GreenhouseState::active_room()` fixed to return `Option`;
   later, a second real instance of the same bug shape was found and fixed
   in `systems::fields::update_fields()` (unbounded indexing from entity
   coordinates).
4. **HighGrow reference analysis** (uncommitted): installed HighGrow 4.20 via
   Wine/Sikarugir for inspection only (never executed as a "run the game"
   action beyond that), extracted its manual and resource strings, distilled
   transferable mechanics into
   `docs/archive/greenhouse-brainstorming/highgrow_reference_analysis.md`.
5. **Flora storage lock-in** (uncommitted): `FloraState` migrated from a
   bespoke `vines: Vec<VineInstance>` field to an enum-backed
   `FloraInstance` family store (`organisms: Vec<FloraInstance>`), with
   `vines()`/`vines_mut()`/`push_vine()` accessors. Every call site migrated.
6. **`core::greenhouse` → `WorldState` attachment** (uncommitted): added
   `greenhouse: Option<GreenhouseState>`, populated via `greenhouse_for_kind`.
7. **Growth-dispatch generalization** (uncommitted): `run_growth` was found
   to only grow the one vine matching a hard-coded ID, silently ignoring any
   other vine instance — fixed to iterate all vines (matching what
   `run_aging` already did correctly).
8. **Species-profile data-shape lock-in** (uncommitted): locked "static Rust
   fixtures" as the format, validated by a new test proving `SpeciesRegistry`
   holds multiple distinct profiles at once (prior coverage only ever
   registered one).
9. **`WorldKind::Greenhouse`** (uncommitted, the biggest single piece): a
   real, selectable third world — profile, capabilities, hotkey cycle
   (`w`), persisted `WorldKindSnapshot` enum, and a new minimal
   `GreenhouseLayer` (room-bounds outline + fixture markers only, no
   labels). Verified via `tmux` driving the actual release binary, not just
   unit tests — confirmed the room renders and the cycle survives a full
   lap back to `MainScene` intact.

Steps 5–9 were each explicitly discussed with the user before
implementation (scope proposed, confirmed, then built) rather than assumed.
That back-and-forth is itself worth evaluating — see section 4.

## 2. Load-bearing claims to independently verify (do not just trust these)

- `bash scripts/verify.sh` was reported clean at 267/267 tests, clippy, fmt,
  and docs, as of the last run this session. **Re-run it yourself.** Do not
  assume it's still true — the working tree may have drifted, and this
  session's own hygiene notes explicitly warn against trusting
  toolless/unverified claims.
- The `FloraInstance` migration claims "every call site migrated" — verify
  by grepping for `.vines` (the old field name) repo-wide; there should be
  zero remaining direct references outside the accessor methods themselves.
- The `WorldKind::Greenhouse` exhaustiveness claim ("the compiler found 8
  sites, all fixed") — verify with a clean `cargo check --all-targets`; if
  anything non-exhaustive slipped through with a wildcard `_ =>` arm instead
  of an explicit one, the compiler would not have caught it, and this
  session might have missed a spot that silently does the wrong thing for
  `Greenhouse` (worth an explicit `rg` for `WorldKind::` match sites with
  wildcard arms).
- The claim that the greenhouse render was "verified end-to-end in the
  running app via tmux" — this was real (a `tmux capture-pane` output was
  inspected showing the room outline and markers), but it was one visual
  spot-check at one terminal size, not a regression-proof method. Consider
  whether this warrants a repeatable test rather than a one-off manual check.
- The camera does **not** auto-center on the greenhouse room when switching
  to it — this was a deliberate scope decision (documented as "not fixing a
  pre-existing camera-reset-on-switch gap that affects all worlds, not just
  greenhouse"), not an oversight, but re-evaluate whether that reasoning
  still holds.

## 3. Lessons learned this session — candidate material for AGENTS.md / skills

These are patterns that recurred enough this session to be worth codifying,
if they hold up under scrutiny. None of these have been written into
`AGENTS.md` or the skills yet — that's explicitly part of what's being
asked for in this inspection pass.

- **"Discuss scope before implementing" for anything crossing into new
  feature territory.** `AGENTS.md` already gates greenhouse-world
  implementation behind an explicit ask; this session additionally
  discovered that even after the ask, *staged* scope discussion (narrowest
  slice → confirm → next slice → confirm) caught real design questions
  (camera semantics, `WorldComposition` variant vs. reuse, selection UX)
  before any code was written. Worth checking whether `AGENTS.md` should
  describe this staged pattern explicitly, not just the initial gate.
- **"Reproduce the bug before trusting the regression test."** Used
  repeatedively (the `systems::fields` bounds fix, the `run_growth`
  dispatch fix): before declaring a new test meaningful, revert the fix
  temporarily, confirm the test fails with the expected error, then restore
  the fix. This caught nothing wrong this session, but it's a cheap
  discipline that would have caught a subtly-wrong test. Worth writing into
  `docs/hygiene.md` explicitly as a standard for regression tests specifically
  (not full TDD, just: prove the test can fail before trusting that it can't).
- **Compiler-driven exhaustiveness for enum additions.** Adding
  `WorldKind::Greenhouse` without wildcard arms let the compiler enumerate
  every site needing attention, rather than hunting via grep (which can
  miss non-obvious call sites). Worth a short note in `docs/architecture.md`
  or a skill recommending non-wildcard matches on core enums specifically
  *because* this property is valuable, not just as a style preference.
- **TUI apps have no browser-based "start the dev server and look at it"
  equivalent** — the general instruction to verify UI changes in a browser
  doesn't apply here. This session improvised a `tmux`-based approach
  (detached session, `send-keys`, `capture-pane` to read real terminal
  output as text). This is probably worth documenting as the actual
  verification method for this project specifically — maybe in
  `docs/hygiene.md` or a project skill — so it isn't reinvented ad hoc next
  time, and so its limits (one-off, not a regression test) are named.
- **Toolless-session unreliability** was already formalized into
  `docs/hygiene.md` earlier this session (after two separate incidents
  where sandbox-without-cargo inference turned out wrong). Check whether
  that guidance is sufficient or needs sharper enforcement — e.g., should
  `docs/audit.md` entries have a required "verified how" tag?
- **Doc-thrash risk.** `docs/greenhouse-roadmap.md` was edited in at least
  six separate passes this session (flora storage, greenhouse attachment,
  growth dispatch, species-profile, WorldKind::Greenhouse, plus whatever is
  in the unreviewed `d647343` commit), each time touching different
  subsections locally. No single top-to-bottom read-through happened after
  all edits landed. This is the single highest-risk area for accumulated
  small contradictions, duplicate bullets, or a "Locked First-Pass
  Decisions" section that no longer reads as a coherent list. **This is
  probably the single most valuable concrete task for the new session.**

## 4. Docs reconciliation — where to actually look

In rough priority order:

1. `docs/greenhouse-roadmap.md` — full read top to bottom. It's long (900+
   lines) with several distinct sections (ChatGPT Brainstorming Reference,
   Current Baseline, 0.4 Readiness Snapshot, 0.4 Gate Checklist, Locked
   First-Pass Decisions, North Star, Hard Rules, Domain Model,
   Functional-Space Contract, several `*_Ingest` historical sections, an
   Operation Plan with Phases 0–11, Open Decisions, Creative Brief). Check
   specifically: does the Gate Checklist table agree with the prose in
   Locked First-Pass Decisions? Do the Phase 4 status notes agree with the
   Current Baseline bullets? Are there now two slightly different
   descriptions of the same fact (e.g. `FloraInstance`, `WorldKind::Greenhouse`)
   phrased inconsistently in different sections?
2. `docs/audit.md` — this session added several `**Superseded**` /
   `**Fixed**` / `**Added**` annotated entries rather than deleting old
   ones. `docs/audit.md`'s own stated rule is "keep resolved detail in
   `docs/LOG.md`... rather than re-accumulating it here" — check whether
   this session followed that rule or violated it by leaving too much
   resolved-but-annotated detail in place. A pruning pass may be warranted.
3. `TODO.md` — check for any other stale items resembling the ones already
   closed this session (search for language like "decide", "inspect",
   "biased toward" as a smell for undecided things that may since have been
   decided without the TODO entry being updated).
4. `docs/vines.md`, `docs/scene-model.md` — spot-check for staleness from
   the `FloraInstance`/growth-dispatch changes; these weren't touched this
   session and may now describe the old `vines: Vec<VineInstance>` shape.
5. `d647343`'s doc changes specifically (`docs/LOG.md`, `docs/audit.md`,
   `docs/greenhouse-roadmap.md`, `docs/weather-widget.md`) — these were
   never reconciled against this session's own later edits to the same
   files, since they predate this session's awareness of them. Diff them in.

## 5. Skills and `AGENTS.md` — not reviewed this session, need a real look

This session touched **zero** files under `skills/` and did not re-read
`AGENTS.md` after its initial load. Both exist:

- `AGENTS.md` (73 lines, repo root)
- `skills/yam-architecture-review/SKILL.md` (+ `agents/openai.yaml`)
- `skills/yam-maintenance/SKILL.md` (+ `agents/openai.yaml`)

Concrete things to check, since none of these were verified this session:

- Do either skill or `AGENTS.md` reference `scene::coords` (retired this
  session, before this handoff's own timeframe even) as if it still exists?
- Do they describe `FloraState`/flora storage in terms of the old
  `vines: Vec<VineInstance>` shape?
- Do they have any awareness of `WorldKind::Greenhouse` as a selectable
  world, or do they still describe only `Boot`/`MainScene`/`Sandbox`?
- Given the "lessons learned" in section 3, do the maintenance/review
  skills already encode any of those patterns (staged-discussion-before-
  features, reproduce-before-trusting-a-test, tmux-based TUI verification)?
  If not, this is the natural place to add them — but confirm the skills'
  actual current scope and tone first rather than assuming what they say.

## 6. Explicitly deferred / not done

State this plainly to whoever picks this up next — these are known gaps,
not oversights to "discover":

- No growth dispatch exists for greenhouse organisms (there are none to
  dispatch for yet — no organism occupies a planting site).
- No inspection UI (popups, per-fixture detail) for the greenhouse.
- The hero-rendering offline-compiler direction (`docs/rendering.md`) is
  untouched, deliberately deferred all session.
- Camera does not auto-center when switching into the greenhouse world
  (see section 2).
- `run_growth`'s growth *rule* itself is still vine-specific code — only
  its *dispatch loop* was generalized this session, not the rule.

## 7. Suggested first moves for the new session

1. `git status` / `git diff --stat` for real, current ground truth — do not
   rely on this file's file lists.
2. `bash scripts/verify.sh` fresh.
3. `git show d647343` in full and reconcile it against this session's
   understanding of the same files.
4. Ask the user directly: commit the uncommitted work (as one commit or
   several?), and if so, onto PR #5 or a new PR? This session deliberately
   did not decide this unilaterally.
5. Only then move into the docs-reconciliation and skills/AGENTS.md review
   work described above.
