# Changelog

All notable user- or developer-visible changes to this project are recorded
here, in the style of [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

This file is a curated summary. For the full, detailed, append-only record of
every maintenance batch, see [`docs/LOG.md`](docs/LOG.md).

No release has been tagged yet; everything below is accumulating under
`Unreleased` ahead of the first real `0.x` tag.

## [Unreleased]

### Added

- `WorldKind::Greenhouse`: a real, selectable third world (cycled with the
  same `w` hotkey as `Sandbox`), rendering one inert nursery room via a
  minimal read-only `GreenhouseLayer` (bounds outline plus fixture markers,
  no labels). No growth dispatch, mutation, or inspection UI yet.
- CI (`.github/workflows/verify.yml`): runs `scripts/verify.sh` on every push
  and pull request; `main` requires it via branch protection.
- `cargo audit` wired into CI; `.github/dependabot.yml` for routine `cargo`
  and `github-actions` dependency freshness.
- `.github/PULL_REQUEST_TEMPLATE.md`.

### Changed

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
