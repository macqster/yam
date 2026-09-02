# Changelog

All notable user- or developer-visible changes to this project are recorded
here, in the style of [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

This file is a curated summary. For the full, detailed, append-only record of
every maintenance batch, see [`docs/LOG.md`](docs/LOG.md).

This project has no public release process — see
[`docs/release-model.md`](docs/release-model.md)'s Distribution section: no
GitHub Releases, tags, or prebuilt binaries will ever be provided, and the
only way to run YAM is to build it from source. `Unreleased` below is a
permanent heading, not a holding area for a future tag; it accumulates the
full change history in one running section instead of per-version ones.

## [Unreleased]

### Added

- Per-phase boot toggles on the dev settings popup's `runtime` tab: `coalesce`,
  `bar`, `dissolve`, and `hold` each switch on and off independently, and the
  choice persists. Left means off and Right means on rather than either key
  flipping the value, so a repeated keypress settles on a state instead of
  oscillating.

  A disabled phase is skipped while the rest keep their normal timings, so the
  sequence shortens rather than changing shape. Every boot transition — start,
  ordinary progression, and the spacebar acknowledgement — now routes through
  one `enter_boot_phase` entry point, so a phase cannot be skipped on some paths
  and played on others; boot order lives on `BootLoadingPhase::next` instead of
  being restated in each `update_loading` arm.

  `AwaitStart` is deliberately not toggleable: it is the wait for a person, not
  an animation, and belongs to `--auto-start`. The two stay independent —
  switching every phase off in manual mode still shows the prompt and waits,
  while the same settings under `--auto-start` reached the first world in 136ms
  against 5661ms with all phases on.

- `--auto-start`, which lets a launch advance through the boot screen without
  someone pressing `[space]`. Manual remains the default and the interactive
  contract. `YAM_AUTO_START=1` is an environment fallback for launchers that
  find a variable easier to set than an argument vector; the flag wins when both
  are present, and only `1`/`true`/`yes`/`on` enable it.

  Automatic mode skips the wait for a person, not the animation: coalesce, bar,
  dissolve, and hold all run at their normal timings, and it reaches `AwaitStart`
  the ordinary way before calling the same `acknowledge_loading_start`
  transition the spacebar calls, so that phase keeps exactly one exit. No key
  event is synthesized, and the chosen policy is recorded in the `boot_start`
  diagnostics event rather than as a fake keypress.

  The `press [space] to continue` prompt is not drawn under automatic mode.
  `showing_start_prompt()` deliberately stays true through `Dissolve` so the
  prompt fades with the rest of the screen — right when a person did press the
  key, but it would otherwise leave the prompt up for the whole one-second
  dissolve telling the user to do something already done for them.

  `scripts/tmux-smoke.sh` gained `--auto-start`, which launches with the flag,
  needs no keys, and polls for the first world instead of sleeping a fixed
  interval — a cold hero cache still adds several seconds before the phases
  begin, so a hard-coded wait would be flaky or needlessly slow.

- Runtime package loading, completing the offline-compiler path started in
  0.4.8: `hero_frames_cached_from` now prefers a validated `HeroPackage` over
  the frame cache and the live chafa path, in that order. A package is used
  only when its schema revision, `preset_id`, render geometry, and the source
  file's SHA-256 digest all match, and `validate()` reports no issues; any
  failure falls through silently. `--compile-hero` writes to where the runtime
  looks, `<cache dir>/<stem>.hero_package.json`, so compile-then-run works
  without arguments.

- Phase 1 offline hero compiler, landed from `agent/hero-revision-contract`
  where it had been unmerged since 2026-07-27. `render::hero_package` is the
  validated, versioned artifact; `render::hero_manifest` records provenance
  (SHA-256 source digest, geometry, per-frame timing, compiler id/version,
  preset id, literal chafa args); `render::hero_compiler` is reachable as
  `yam-rust --compile-hero` and never sits on the ordinary startup path.
  `render::cell_grid` extracts the shared cell shape so the disposable runtime
  cache and the package format serialize through one type.
- `chafa_preset_args`: one authoritative chafa flag list shared by runtime
  rendering and the offline compiler, so the two cannot drift apart. It carries
  the source's own `absent_color`, which is what stops an offline package being
  rendered against a different drop reference than the runtime uses for the
  same asset. `HERO_PRESET_ID` is recorded verbatim in every manifest.

- `docs/chafa-drop-rule.md`: how `--bg` decides which hero colors are drawn at
  all, the batched patch-grid measurement harness, the procedure for choosing
  `HeroSource::absent_color` when new art is registered, per-family
  darkest-survivable values, and the chafa settings measured to have no effect
  at 24-bit color so they are not re-tuned in the expectation of one.

- `HeroSource::absent_color`: the colour handed to chafa as `--bg`, now owned
  per asset. Under `--fg-only` that value is never painted; it is the colour
  chafa treats as already on screen, so art resembling it is discarded rather
  than drawn. It must be absent from the asset, and
  `absent_color_is_actually_absent_from_every_source` enforces that with a 128
  Euclidean-RGB floor.

- Second registered hero source, `IVY_VECTOR` (`assets/hero_gif_2.gif`,
  `1080x1080`, 48 frames): the same character and pose cycle as `IVY`,
  redrawn as flat vector art in Moho. Registered first as a probe so
  `hero_source::ALL` and the three asset-swap gates would run against a
  genuinely second asset instead of checking one file against itself, then
  promoted to `hero_source::DEFAULT` in 0.4.1 once it had been looked at in
  the running app.
- `YAM_HERO_SOURCE=<stem>`: selects any registered hero source for one
  launch, resolved in `Hero::new` via `hero_source::resolve_from_env` and
  falling back to the default when unset or unknown. This is the smallest
  form of the selection surface — enough to look at candidate art in the
  running app; a world- or settings-owned selector does not exist yet.
- `HeroSource::min_frame0_coverage_percent`: the live-render coverage floor
  is now per-source rather than one shared constant, because cell density is
  a property of the art (flat vector fills light fewer braille dots than cel
  shading at the same requested size).
- `WorldKind::Greenhouse`: a real, selectable third world (cycled with the
  same `w` hotkey as `Sandbox`), rendering one inert nursery room via a
  minimal read-only `GreenhouseLayer` (bounds outline plus fixture markers,
  no labels).
- Greenhouse growth dispatch: a first `OrganismFamily::Seedling` occupies the
  nursery's `left_tray` planting site (a soft `PlantingSite::occupant`
  reference, not ownership) and advances `Dormant -> Growing -> Mature` on
  its own 6-tick cadence via `systems::growth::run_greenhouse_growth`.
- Greenhouse inspection: a read-only `GreenhouseInspectLayer` (`i` hotkey,
  dev-mode and Greenhouse-world gated) surfaces the active room's
  `inspection_refs` (room, bench, fixture, and planting-site descriptions).
- CI (`.github/workflows/verify.yml`): runs `scripts/verify.sh` on every push
  and pull request; `main` requires it via branch protection.
- `cargo audit` wired into CI; `.github/dependabot.yml` for routine `cargo`
  and `github-actions` dependency freshness.
- `.github/PULL_REQUEST_TEMPLATE.md`.
- `scripts/tmux-smoke.sh`: wraps the repo's manual `tmux`-based interactive
  verification recipe into a reusable script (boots the release binary,
  waits out the boot animation, sends a key sequence, prints the final
  rendered pane).
- Offline hero package compiler (`yam-rust --compile-hero [SOURCE]`) and a
  validated, versioned `HeroPackage`/`HeroManifest` format
  (`render/hero_package.rs`, `render/hero_manifest.rs`), distinct from the
  disposable runtime hero cache — see `docs/hero-package.md`. **Unverified**:
  written without a local Rust toolchain; needs `cargo build`/`cargo
  test`/`cargo clippy` before it can be trusted.

### Changed

- `scripts/check.sh`'s architecture boundary checks can no longer pass without
  running, and now enforce the whole contract rather than a third of it. They
  were written as `if rg …; then fail; fi`, which reads a missing ripgrep
  (exit 127) as "no matches" and reports success having inspected no files; a
  wrong working directory passed the same way. CI only escaped because
  `ubuntu-latest` happens to ship ripgrep. They use `grep` now, which is in
  POSIX and needs no guard, and each check counts the `.rs` files it scanned
  and refuses to pass on zero — necessary because BSD grep, the macOS default,
  returns the same exit code for a missing directory as for a clean one. A
  passing run reports the file count it checked.
  `core/` is also held to the same rule as `systems/` now: `docs/architecture.md`
  forbids `core -> ui` and `core -> render` alongside `core -> scene`, and
  `src/core/mod.rs` claims no ratatui/crossterm usage, but only the `scene`
  half was ever enforced. No violation existed — the tree was already clean —
  so this closes an unguarded invariant rather than fixing a broken boundary.

- Development version bumped to `0.4.11` (from `0.4.10`). Opens a maintenance
  cycle for the follow-ups from the 2026-09-02 repository assessment. The bump
  itself changes no runtime behavior; each finding lands as its own entry as it
  is addressed.

- Development version bumped to `0.4.10` (from `0.4.9`). `--compile-hero`'s
  optional argument now selects a registered source by stem, full path, or
  bare filename, instead of overriding only the source path. Overriding the
  path alone compiled the named GIF against the *default* source's
  `absent_color` and geometry and wrote it under the default source's package
  filename — a package the runtime could then never validate. Unregistered art
  is refused with an error listing the registered stems, because its
  `absent_color` and geometry are what the descriptor owns.

- Development version bumped to `0.4.9` (from `0.4.8`). The digest check is
  what makes a package safer than the frame cache it takes precedence over:
  the cache can only compare mtimes, so art swapped in with an older timestamp
  is served as trusted, while a package is validated on content.

- Development version bumped to `0.4.8` (from `0.4.7`). The compiler was
  adapted on landing rather than merged as authored: the branch predated
  per-source descriptors, so `CompileOptions` takes geometry and `absent_color`
  from a `HeroSource`, exposes `for_source`, and honors `YAM_HERO_SOURCE`.
  Four globals the descriptor had superseded are gone (`HERO_GIF_PATH`,
  `HERO_DISPLAY_BG`, `HERO_CACHE_REVISION`, `HERO_RENDER_WIDTH`/`HEIGHT`).
  No cache revision bump: the preset refactor is output-neutral, verified by
  comparing a fresh compile byte-for-byte against a cache written by 0.4.7.
- `docs/hero-revision.md` no longer describes the dark-color defect as
  unresolved. It was fixed on 2026-08-19, and that document's prediction that
  the failure was "broader than one Chafa flag" was wrong — it was one flag.
  The wrong prediction is recorded rather than deleted, because the reasoning
  is the useful part.

- Development version bumped to `0.4.7` (from `0.4.6`): consolidation pass, no
  runtime behavior change. `docs/release-model.md` step 2 said maintenance
  batches could land directly on `main`; that stopped being true on 2026-08-20
  when admin bypass was disabled, so it now describes the PR-only workflow and
  records why. `docs/audit.md` review date refreshed. The two chafa flags
  measured inert at 24-bit color (`--color-space`, `--dither`) carry comments
  saying so, so they are not tuned in the expectation of an effect.

- Development version bumped to `0.4.6` (from `0.4.5`): hero rendering uses
  `--color-extractor=average` instead of `median`. Measured at the shipped
  backgrounds it renders exactly the same cells, frame for frame, at lower
  reconstruction error (`hero_gif_2` 122 to 112, `hero_gif_1` 125 to 96), and
  it does not move any drop boundary — `median` and `average` produce identical
  darkest-survivable tables, so the change cannot cost a color. The 2026-07-22
  switch away from `average` was against the then-flattened opaque canvas,
  which no longer exists. Cache revisions bumped: `IVY` `r5` -> `r6`,
  `IVY_VECTOR` `r4` -> `r5`.

- Development version bumped to `0.4.5` (from `0.4.4`): **dev-mode positions
  now persist across launches.** Saved offsets are reseeded only on an
  explicit `--hard-reset`, or automatically when `~/.config/yam/state.json`
  carries a different crate version than the running binary — the upgrade
  case, so new art or a new default composition is not fought by offsets
  tuned against the previous one.
- Through 0.4.4 every launch reseeded, so move mode could not persist
  anything. Worse, because the reset ran at startup and a save then wrote the
  whole offsets struct, any later save silently overwrote a previously-saved
  hero position with the default.
- `--preserve-ui-state` is gone; preserving is now the default. `--clean-launch`
  is dropped from `bin/yam` and `bin/yam-sandbox`: nothing ever parsed it, and
  leaving it there would read as forcing a reset now that `--hard-reset` does.
- `~/.config/yam/state.json` gains a `version` field. Files without one (any
  written before 0.4.5, including the pre-snapshot bare-offsets format) read as
  versionless and reseed. The stamp is only written on save, so a session that
  reseeds and then quits without saving reseeds again next launch — harmless,
  because it reseeds to the same defaults, but it is not a one-time event.

- Development version bumped to `0.4.4` (from `0.4.3`): `IVY_VECTOR`
  (`hero_gif_2`, the default hero) uses `absent_color` `#336699`,
  which deliberately overlaps its own palette. The asset is ten flat colours,
  flat fills render as fully-lit `⣿`, and under `--fg-only` a uniform cell is
  all eight dots or none — so discarding the darkest tiers is the only way to
  keep the hero open rather than a solid mass. The value is chromatic rather
  than neutral because `7c0307` and `332a29` have identical RGB sums (134
  each): no grey or black separates them, a blue does, keeping the dark red
  while still dropping the leggings and line art. Frame-0 coverage is
  923/4608 (20.0%) against an unchanged 10% floor. `IVY` is untouched and
  still uses a non-overlapping `#00e000`. Cache revision `r3` -> `r4`.
- `absent_color_is_actually_absent_from_every_source` gained an
  `ACCEPTED_OVERLAP` list. A source that overlaps its palette on purpose has
  its overlap pinned exactly (`hero_gif_2`, 259464 pixels in the worst frame)
  rather than being required to separate, so any drift in the palette, the
  radius, or the chosen colour still fails the gate.

- Development version bumped to `0.4.3` (from `0.4.2`): `absent_color`
  retuned from `#00ff00` to `#00e000` on both sources. Distance from the
  palette is a trade rather than a maximum — on partially transparent edge
  cells the value bleeds into chafa's foreground pick, and that bleed grows
  with distance — so the right value is the least clearance that still clears
  the drop radius. Off-palette edge cells fall from 15 to 8 across the two
  assets, with slightly better reconstruction error on both.
- `absent_color_is_actually_absent_from_every_source` now ignores colours
  thinner than half a rendered cell, which cell averaging discards anyway.
  Without that floor the gate was measuring the GIF exporter's anti-aliasing
  fringe rather than the art: on `hero_gif_1` that fringe is 108 of 249
  distinct colours, and it was setting the reported clearance.
- Hero cache revisions bumped again for the retune: `IVY` `r4` -> `r5`,
  `IVY_VECTOR` `r2` -> `r3`.

- Development version bumped to `0.4.2` (from `0.4.1`): the hero renders its
  dark regions for the first time. `HERO_DISPLAY_BG` (`#100100`, a dark red)
  was telling chafa that every dark region was already on screen, so the hair
  shadow, the leggings, and every outline were discarded rather than drawn —
  a mechanism separate from, and untouched by, the 2026-07-22 matte/extractor
  fix. Replaced by a per-source `absent_color` of `#00ff00`. Frame-0 coverage
  roughly doubles on both assets (`hero_gif_1` 932 -> 1923 cells,
  `hero_gif_2` 706 -> 1725), every solid cell of the hero is now drawn
  (1468/1468, previously 621), and there is no spill outside the silhouette.
  `--fg-only` is unchanged: the scene still shows through the unlit dots.
- Hero cache revisions bumped because every rendered frame changes: `IVY`
  `r2` -> `r4` and `IVY_VECTOR` `r1` -> `r2`. `IVY` skips `r3` deliberately —
  a stale `hero_gif_1.r3.*` from a 2026-08-17 experiment can still be present
  in `~/.cache/yam` and would be accepted as fresh, serving pre-fix frames.

- Development version bumped to `0.4.1` (from `0.4.0`): the rendered hero is
  now `assets/hero_gif_2.gif` (`IVY_VECTOR`) rather than `assets/hero_gif_1.gif`
  (`IVY`). `IVY` stays registered and gated, so `YAM_HERO_SOURCE=hero_gif_1`
  returns to the previous art without a rebuild. The first launch after the
  upgrade pays one cold `chafa` compile, because no existing frame cache
  matches the new default. See `docs/release-model.md` for what this number
  does and does not mean (no tagged release follows).
- `rendered_hero_frames_contain_real_content_not_placeholders` now iterates
  `hero_source::ALL` instead of only the default source, so registering hero
  art also enrols it in the live-render gate. Its coverage assertion reads
  the per-source floor rather than a hard-coded 20%.
- Withdrew the 41.5% frame-0 coverage figure that the hero content test and
  `docs/rendering.md` cited for `assets/hero_gif_1.gif`; re-measured through
  the test's own helper against chafa 1.18.2 it is 932/4608 cells (20.2%),
  which leaves that source's unchanged 20% floor with 0.2 points of headroom
  rather than the roughly 2x the note assumed. The floor is deliberately not
  recalibrated here — see the `medium` item in `docs/audit.md`.
- Runtime and the new offline hero compiler now share one authoritative
  chafa preset (`chafa_preset_args()` in `render/chafa.rs`) instead of two
  independently maintained flag lists.
- Removed the code-side `tone_lift_dark_reds` hue/saturation/value
  correction for dark reds from the hero rendering path
  (`src/render/chafa.rs`), as part of rebuilding the hero source from a
  vector-redrawn original — see `docs/hero-revision.md`.

- Development version bumped to `0.4.0` (from `0.3.9`) now that the
  Greenhouse world, growth dispatch, and read-only inspection have all
  landed and `bash scripts/verify.sh` is green — see `docs/release-model.md`
  for what this number does and does not mean (no tagged release follows).
- `scripts/check.sh`'s clippy and cargo-check invocations broadened to
  `--all-targets --all-features` / `--all-targets`, so lints and compile
  errors inside `#[cfg(test)]` modules are actually enforced by CI instead of
  only checking the default binary target.

- Flora storage locked to an enum-backed `FloraInstance` family store
  (`FloraState::organisms`, one `Vine` variant today), replacing the old
  bespoke `vines: Vec<VineInstance>` field.
- `systems::growth::run_growth` now iterates every vine instance instead of
  one hard-coded seed id, matching `systems::aging::run_aging`.
- Species-profile data format locked as static Rust fixtures.
- Repo merge policy: merge-commit only (squash and rebase-merge disabled),
  branches auto-delete on merge.
- Hero source asset (`assets/hero_gif_1.gif`) swapped for a working master
  that carries real per-pixel alpha (previously fully opaque, with a flat
  matte background baked in); the render pipeline now preserves that alpha
  end to end instead of compositing every frame onto an opaque fill, and
  `--color-extractor` switched from `average` to `median`. Together these
  fix a long-standing coverage bug: `average` against the flattened canvas
  was dropping roughly 80% of the frame grid as "no coverage" (any
  low-contrast dark region, not only reds), rather than merely desaturating
  it.
- Hero frame cache now written as compact JSON instead of pretty-printed,
  cutting the generated cache file from 81MB to 27MB with no behavior
  change (it is machine-only, so the indentation was pure overhead).

### Fixed

- An instant boot recorded no `world_ready` event, and `runtime_exit` then
  reported `boot_completed: false` for a run that had booted correctly. The
  runtime reports readiness on a `Some -> None` boot-phase change, which never
  happens when every phase is disabled because the boot finishes inside
  `start_loading_boot`. Unreachable until boot phases became toggleable, since
  the sequence previously had a 4.5s floor.

- Saved UI state is written atomically and honors `XDG_CONFIG_HOME`. The write
  was a plain `fs::write`, which truncates in place, so a crash or kill
  mid-write left a half-written `state.json` that `load_or_new` then discarded
  silently — losing saved positions with no message. It now writes a sibling
  temp file and renames over the target. The path also ignored
  `XDG_CONFIG_HOME` and hard-coded `~/.config`, unlike diagnostics
  (`XDG_STATE_HOME`) and the hero cache (`XDG_CACHE_HOME`); and it resolved
  `HOME` with `unwrap_or_default()`, so with `HOME` unset the whole path went
  relative and state landed in whatever directory the app was launched from.

- The test suite runs in about 10 seconds instead of about 111. The cause was
  not what it looked like: the two slowest tests shell out to `chafa` once per
  GIF frame, but one `chafa` spawn costs ~48 ms, and the real bottleneck was
  decoding the two large hero GIFs at the default `opt-level = 0` — roughly
  20 s per decode of the 1080x1080 48-frame source, in `GifDecoder` plus
  `frame_to_canvas`'s per-pixel loop. Those sources are decoded eleven times
  across the file, so nearly the whole suite was unoptimized pixel work. A
  `[profile.test] opt-level = 2` trades a slower cold compile for an 11x faster
  suite, which is what makes `scripts/verify.sh` runnable on every batch as the
  workflow asks.

- A stale claim in `chafa_is_available`'s doc comment, which said CI does not
  install `chafa`. It does, so those content assertions do run in CI and the
  skip is a local-developer affordance rather than a permanent CI exemption.

- `--compile-hero` and the `chafa` runtime requirement are now in the README.
  The validated hero-package layer — compiler, manifest, provenance and schema
  validation, and a package-first startup path — was fully built and wired, but
  no user could ever have a package: the runtime never writes one, and the only
  command that does appeared in no user-facing doc. The README also never said
  `chafa` is required at all, though the hero renders from the source GIF
  through it and degrades to blank placeholder frames without it. Measured
  rather than assumed: with a package and `chafa` off `PATH` the hero renders;
  with neither, it renders nothing.

- `Ctrl+C` now exits. Raw mode is enabled for the whole run, so the terminal
  never turns it into SIGINT — it arrived as an ordinary key event that nothing
  handled, because `KeyModifiers::CONTROL` was not tested anywhere in the
  codebase. `q` was the only way out, and `q` is gated behind mode checks and a
  confirmation modal, so a wedged overlay had no escape at all; in dev free-roam
  `Ctrl+C` fell through to the character catch-all and recalled the camera home.
  It is checked before any mode dispatch, so it works from the loading screen, a
  modal, and settings edit alike, and it exits without saving or playing the
  quit dissolve. `q` is unchanged as the graceful path.

- Three dead branches in the dev-mode character handler. A `c == 'd'` arm was
  shadowed by the unguarded `Char('d')` arm above it, and the shift-qualified
  variants on the font and FPS chords tested for `'='`/`'-'` *with* SHIFT — but
  no keyboard enhancement flags are pushed, so crossterm reports the resulting
  character (`'+'`, `'_'`), and none of them could ever match. One of those dead
  tests also duplicated a chord the font arm above had already claimed, so two
  arms disagreed about the same keypress while both were unreachable. The
  literal `{`/`}` and `-`/`+` chords that actually worked are unchanged.

- Three README claims that contradicted the runtime, and the gap in
  `scripts/check-docs.sh` that let one of them drift. The version badge sat at
  `0.4.0` while the canonical `current release` line four lines below it
  tracked every bump through `0.4.10`, because the gate checked the line and
  not the badge; the badge carries the version in both its shields.io URL and
  its alt text, and both are gated now. The snapshot also described the
  greenhouse as "not yet growth-dispatched" and listed "greenhouse growth
  dispatch, inspection UI" under `future_surfaces`, though both landed on
  2026-07-22: `systems::tick` calls `run_greenhouse_growth` every tick and the
  read-only `GreenhouseInspectLayer` is bound to `i`. `TODO.md` and
  `docs/architecture.md` already recorded the landing; only the front door
  missed it. What actually remains — the curation/transfer write-path and
  richer per-fixture inspection — now matches the `next_track` line above it.

- The real root cause of an intermittently-failing weather test: it was
  making live network calls to `wttr.in` from the test suite.
- Two panic-safety gaps following the same shape (an invariant enforced only
  at construction while backing fields stayed public and mutable):
  `GreenhouseState::active_room()` and `systems::fields::update_fields()`.
- A RUSTSEC vulnerability (`crossbeam-epoch`, via `image`'s unused default
  AVIF/OpenEXR/WebP features) and two lesser warnings (`paste`, `anyhow`) —
  see Security below.
- Greenhouse inspect popup describing the left tray site as awaiting "one
  future nursery occupant" when a seedling already occupies it; a test now
  fails if an occupied planting site's inspection text drifts back to
  describing it as empty.
- `bin/yam` and `bin/yam-sandbox` missing the executable bit, so `./bin/yam`
  failed from a fresh clone (the installed copies were unaffected, since
  `scripts/update.sh` chmods them).
- Hero frames silently failing to render (`chafa unavailable`-shaped
  placeholder output) since the `image` dependency trim below: that trim's
  premise ("the only format this crate decodes") was incomplete, since the
  render pipeline also *encodes* temp frames to PNG before shelling out to
  `chafa`, and PNG encoding is a separate `image` feature not implied by
  `gif`. Fixed by adding `"png"` back to the feature list.

### Security

- `image` trimmed to `default-features = false, features = ["gif", "png"]`
  (the two formats this crate actually decodes/encodes), dropping the
  dependency count from 300 to 241 and removing the vulnerable
  `crossbeam-epoch`/`ravif`/`rav1e` chain entirely rather than just patching
  around it.
- GitHub Dependabot security updates enabled.

### Removed

- `scene_config.json`'s cross-language coupling. A `#[cfg(test)]` test in
  `src/main.rs` pinned ten of its field values, so changing a Python tooling
  preset meant editing a Rust test — for a file `docs/config.md` says twice is
  not authoritative for the Rust runtime, and which no non-test Rust code reads
  or embeds. `bin/yam` and `bin/yam-sandbox` also listed it among the mtime
  inputs that trigger a full Rust reinstall, a rebuild that could not change the
  binary's behavior. The file itself stays: `tools/experiments/config.py` still
  reads it, so the tooling that uses it now owns it alone.

- A legacy direct-to-`Frame` drawing API that the grid/layer pipeline had
  already replaced: all of `render/clock.rs` (only `clock_lines` was live, and
  it was a one-line wrapper over `fonts.render`), and `render/hero.rs`'s
  `draw_hero`, `draw_hero_at`, `draw_hero_debug`, `draw_hero_debug_at`,
  `debug_rect`, `render_lines_clipped` and `clip_line`. Also
  `compositor::merge_grid_legacy` with the `MaskMode` enum that only fed it, and
  `theme`'s `hero_overlay` style and `HERO_CENTER_MARKER` glyph, which existed
  only for the deleted hero debug overlay. The `Hero` struct and its animation
  methods are unchanged.

- `src/scene/coords.rs`: the `core::spatial` compatibility shim, retired
  after confirming zero call sites outside its own tests.
- Dead root `install.sh` and `tools/experiments/check_golden.py`, both
  referencing infrastructure (a `visualizer/` Python app, a `cmd/yamv2` Go
  program) that no longer exists anywhere in this repo.
