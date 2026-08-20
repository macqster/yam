# Hero Package Contract

This note defines the validated, versioned "compiled package" owner layer
named in [`hero-revision.md`](hero-revision.md)'s pipeline table, and is
distinct from the disposable runtime cache described in
[`hero-cache.md`](hero-cache.md).

## Why Two Formats

- `render::hero_cache::HeroFrameSet` is runtime acceleration only: it may be
  missing, stale, or deleted at any time, and the runtime silently falls
  back to live Chafa compilation when it is. It carries no provenance.
- `render::hero_package::HeroPackage` is the intentional, reviewable
  artifact produced by the offline compiler. It carries a full
  `HeroManifest` (source identity, compiler identity/version, exact preset
  args, timing, schema revision) and is validated before it is trusted.

Both serialize frames through the same shared `render::cell_grid::CellGrid`
format so neither has to redefine cell/style serialization independently.

## Producing A Package

```bash
cargo build --release
target/release/yam-rust --compile-hero [SOURCE_GIF_PATH]
```

`SOURCE_GIF_PATH` defaults to whichever source an ordinary launch would
render - `hero_source::DEFAULT`, or whatever `YAM_HERO_SOURCE` selects - so
the compiler and the runtime cannot disagree about which asset they mean. The
compiler itself never enters the interactive runtime; it decodes the source
GIF, renders every frame through the same `chafa_preset_args()` the ordinary
runtime path uses (`render/chafa.rs` is the single source of truth for that
preset, so runtime and offline compiler cannot silently drift apart), builds a
manifest, validates the result, and writes the package where the runtime looks
for it: `<cache dir>/<stem>.hero_package.json`.

Geometry and `absent_color` come from the source's descriptor rather than
constants, so a package cannot be rendered against a different drop reference
than the runtime uses for the same asset.

## Manifest Shape

`HeroManifest` (`render/hero_manifest.rs`):

- `asset_id` - stable logical name derived from the source file stem
- `asset_digest` - a stable SHA-256 digest of the source file's raw bytes;
  identifies which exact bytes produced this package
- `canvas_width` / `canvas_height` - decoded GIF canvas size
- `frame_count` / `frame_durations_ms` - per-frame authored timing
- `loop_mode`
- `compiler_id` / `compiler_version` - e.g. `"chafa"` / captured `chafa
  --version` output, or `"unknown"` if it could not be captured
- `preset_id` - human-readable name for the exact chafa preset used
- `compiler_args` - the literal preset arguments, so a package's exact
  invocation is reconstructable without cross-referencing source code
- `render_width` / `render_height`
- `schema_revision` - `HERO_PACKAGE_SCHEMA_REVISION` in
  `render/hero_manifest.rs`

## Validation

`HeroPackage::validate()` (`render/hero_package.rs`) only checks objective,
machine-checkable facts:

- frame count matches the manifest
- every frame's geometry matches the manifest's render dimensions
- every frame contains exactly `width * height` serialized cells
- no frame is entirely blank (unstyled spaces), which would indicate a
  placeholder or a failed compile step
- provenance fields (`compiler_id`, `preset_id`, `asset_digest`) are present
- the package schema revision is supported by the current reader

It returns every issue found, not just the first, as a `PackageValidation`
report.

## What This Does Not Prove

Per [`hero-revision.md`](hero-revision.md): "ANSI-code presence and
non-placeholder frame counts are insufficient." Passing `validate()` means
the package is structurally sound, not that dark reds, browns, greens, and
blacks render correctly. A real-terminal review via
[`scripts/tmux-smoke.sh`](../scripts/tmux-smoke.sh) after any source,
compiler, or preset change is still required before trusting a package's
visible output.

## Current Status

Runtime wiring landed in 0.4.9: `hero_frames_cached_from` prefers a validated
package over the frame cache and the live Chafa path, in that order. A package
is used only when all of these hold, and any failure falls through silently
rather than erroring, because a package is an optional acceleration and the
live path can always rebuild:

- schema revision matches `HERO_PACKAGE_SCHEMA_REVISION`
- `preset_id` matches the runtime's current `HERO_PRESET_ID`
- render geometry matches what the caller asked for
- the source file's SHA-256 digest matches the manifest's `asset_digest`
- `validate()` reports no issues

That digest check is what makes a package safer than the frame cache it
supersedes. The cache can only compare mtimes, so art swapped in with an older
timestamp is served as trusted; a package is validated on content.

The hero's dark-color defect was fixed separately on 2026-08-19 - see
[`chafa-drop-rule.md`](chafa-drop-rule.md). Structural validation still cannot
judge color, so the real-terminal review above remains required.
