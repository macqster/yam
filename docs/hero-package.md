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
target/release/yam-rust --compile-hero [STEM_OR_PATH]
```

The install/update path uses the registry-wide form to refresh every source
that has no reviewed portable package:

```bash
target/release/yam-rust --compile-all-heroes
```

This command intentionally ignores `YAM_HERO_SOURCE`; it is a machine refresh
of the complete registry, not a probe of the source selected for one launch.
It deliberately skips a source with a canonical package: recompiling that
source locally would create a second visual authority.

`STEM_OR_PATH` names a *registered* source, by stem (`hero_gif_1`), by full
path, or by bare filename. Omitted, it is whichever source an ordinary launch
would render - `hero_source::DEFAULT`, or whatever `YAM_HERO_SOURCE` selects -
so the compiler and the runtime cannot disagree about which asset they mean.

Unregistered art is refused rather than compiled. Its `absent_color`, render
geometry and coverage floor are exactly what the descriptor owns, so there is
nothing to compile it correctly against; the error lists the registered stems.
Until 0.4.10 the argument overrode only the source path, which compiled the
named GIF against the *default* source's drop reference and filed it under the
default source's package name - a package that could then never validate. The
compiler itself never enters the interactive runtime; it decodes the source
GIF, renders every frame through the same `chafa_preset_args()` the ordinary
runtime path uses (`render/chafa.rs` is the single source of truth for that
preset, so runtime and offline compiler cannot silently drift apart), builds a
manifest, validates the result, and writes the package where the runtime looks
for it: `<cache dir>/<stem>.hero_package.json`.

Geometry and `absent_color` come from the source's descriptor rather than
constants, so a package cannot be rendered against a different drop reference
than the runtime uses for the same asset.

## Canonical Default Package

The default `hero_gif_2` has a source-owned, gzip-compressed package at
`assets/hero_packages/hero_gif_2.r7.rgb-median-diffusion-fgonly-braille-v3.json.gz`.
Its adjacent `.sha256` file verifies the compressed bytes before the runtime
decodes it. The default loader uses this artifact before any machine-local
package, cache, or live Chafa compilation, and then applies the ordinary
manifest, source-digest, geometry, literal-argument, and structural checks.

This exception exists because the MBP arm64 and iMac x86_64 Chafa 1.18.2
executables produced different `CellGrid` payloads from identical GIF bytes
and literal arguments; the iMac-local result visibly created a large dark
mass. Nominal Chafa version text is therefore provenance, not proof of
cross-host visual equivalence. The reviewed MBP package is the one fleet
authority for this default; it is not a cache and must not be overwritten by
the updater.

Refresh it only on the accepted MBP renderer after a real-terminal review of
the candidate, using the release compiler and deterministic gzip (`gzip -9 -n`).
Update the `.sha256` sidecar in the same reviewed change, run the full gate,
and deploy the resulting source checkout. Do not copy an iMac-compiled default
package into this path. `IVY` has no canonical package and remains on the
normal local compiler path.

## Manifest Shape

`HeroManifest` (`render/hero_manifest.rs`):

- `asset_id` - stable logical name derived from the source file stem
- `asset_digest` - a stable SHA-256 digest of the source file's raw bytes;
  identifies which exact bytes produced this package. Hex-encoded lowercase
  and zero-padded with no separators, so always 64 characters. That encoding is part
  of the on-disk format rather than a presentation choice: the value is
  compared verbatim on load, and a mismatch falls through silently, so drift in
  case or padding would invalidate every existing package without reporting
  anything. `digest_uses_the_stable_sha256_hex_shape` pins it to canonical
  vectors for that reason.
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
- provenance fields (`compiler_id`, `preset_id`, `compiler_args`, `asset_digest`) are present
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

Runtime wiring landed in 0.4.9. For a source declaring one, the runtime first
tries the checksum-verified canonical package; otherwise it prefers a validated
machine-local package, then the frame cache, and finally the live Chafa path.
Every package candidate is used only when all of these hold, and a failed
candidate falls through rather than erroring:

- schema revision matches `HERO_PACKAGE_SCHEMA_REVISION`
- `preset_id` matches the runtime's current `HERO_PRESET_ID`
- `compiler_args` exactly match the runtime's current source-owned Chafa preset
- render geometry matches what the caller asked for
- the source file's SHA-256 digest matches the manifest's `asset_digest`
- `validate()` reports no issues

That digest check is what makes a package safer than the frame cache it
supersedes. The cache can only compare mtimes, so art swapped in with an older
timestamp is served as trusted; a package is validated on content.

The default hero's current `#336699` overlap is deliberate and source-owned;
its exact compiler arguments and pinned overlap guard are documented in
[`chafa-drop-rule.md`](chafa-drop-rule.md). The September 9 non-overlapping
experiment is historical rather than active policy. Structural validation still
cannot judge color, so the real-terminal review above remains required.
