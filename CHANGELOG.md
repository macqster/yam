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

### Fixed

- The real root cause of an intermittently-failing weather test: it was
  making live network calls to `wttr.in` from the test suite.
- Two panic-safety gaps following the same shape (an invariant enforced only
  at construction while backing fields stayed public and mutable):
  `GreenhouseState::active_room()` and `systems::fields::update_fields()`.
- A RUSTSEC vulnerability (`crossbeam-epoch`, via `image`'s unused default
  AVIF/OpenEXR/WebP features) and two lesser warnings (`paste`, `anyhow`) —
  see Security below.

### Security

- `image` trimmed to `default-features = false, features = ["gif"]` (the
  only format this crate decodes), dropping the dependency count from 300 to
  239 and removing the vulnerable `crossbeam-epoch`/`ravif`/`rav1e` chain
  entirely rather than just patching around it.
- GitHub Dependabot security updates enabled.

### Removed

- `src/scene/coords.rs`: the `core::spatial` compatibility shim, retired
  after confirming zero call sites outside its own tests.
- Dead root `install.sh` and `tools/experiments/check_golden.py`, both
  referencing infrastructure (a `visualizer/` Python app, a `cmd/yamv2` Go
  program) that no longer exists anywhere in this repo.
