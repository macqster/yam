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

### Changed

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

- `src/scene/coords.rs`: the `core::spatial` compatibility shim, retired
  after confirming zero call sites outside its own tests.
- Dead root `install.sh` and `tools/experiments/check_golden.py`, both
  referencing infrastructure (a `visualizer/` Python app, a `cmd/yamv2` Go
  program) that no longer exists anywhere in this repo.
