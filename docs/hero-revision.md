# Hero Revision North Star

This document owns the product direction, acceptance bar, and staged roadmap
for the hero GIF and its terminal-art pipeline. It is the deciding authority
when a visual experiment, renderer change, cache optimization, or source-art
revision pulls in different directions.

## North Star

YAM's hero must read as intentional animated terminal art: a stable silhouette,
face, costume, and palette with transparent surroundings, coherent motion, and
recognizable dark red, brown, green, and black regions. It is not sufficient
for the pipeline to produce non-empty braille output, preserve alpha, or avoid
crashing. The rendered image itself is the product outcome.

## Current Problem

The long-running hero defect is unresolved: the active Chafa pipeline still
misrepresents dark reds, browns, greens, and blacks. The 2026-07-22 alpha and
`median` changes repaired a specific coverage-loss failure, but they did not
establish faithful dark-color reproduction. Do not describe the hero's color
fidelity as fixed until the acceptance bar below is met on the live main-scene
composition.

The failure is broader than one Chafa flag. The source GIF, alpha handling,
per-pixel correction, braille shape extraction, color sampling, terminal
appearance, cache provenance, and scene placement all participate in the
visible result. Treat dark-color fidelity as a first-class product constraint,
not as a final polish pass.

## Status (2026-07-27)

Decision: rebuild the hero source from a fully vector-redrawn original
instead of continuing to patch the existing raster GIF, specifically so
color fidelity is controlled at the source rather than compensated for in
code. As part of that decision, the one piece of code-side pixel correction
that existed (`tone_lift_dark_reds`, a narrow HSV lift for dark-red hues
only) was removed from `src/render/chafa.rs`; it is not being replaced by an
equivalent, on the bet that a clean vector source should not need it. This
is unconfirmed until real frames from the new source are reviewed.

The Phase 1 infrastructure below (manifest, package, offline compiler) has
landed as asset-independent groundwork and has now passed local compilation,
structural package validation, and a bounded live main-scene smoke -- see
[`hero-package.md`](hero-package.md) for the contract and
[`LOG.md`](LOG.md)'s 2026-08-01 entry for the verification. It has not been
reviewed against the full dark-color acceptance bar, and the new source asset
itself plus the preset retune against that source (Phase 2) are still
outstanding. Do not treat any part of this as fixing the dark-color problem
until a package compiled from the new source passes `scripts/tmux-smoke.sh`
review against the acceptance bar below.

## Product Goals

1. Preserve the intended silhouette and transparent surround without using a
   matte or background-colored substitute for alpha.
2. Render dark red, brown, green, and black regions as distinct visible color
   families; black costume/detail must not disappear into transparency or be
   mistaken for the scene background.
3. Keep face and hand readability, hair/leaf edges, and motion continuity at
   the fixed terminal footprint.
4. Make a hero revision reproducible: an approved source asset and explicit
   compiler preset must produce an identifiable frame package.
5. Make normal runtime playback independent of a live Chafa install and
   per-frame subprocess work.

## Non-Goals

- Do not replace the hero with dashboard chrome, a static screenshot, or a
  monochrome density ramp merely to make a check pass.
- Do not change world ownership, companion attachment, footer placement, or
  unrelated scene systems while solving hero fidelity.
- Do not promote a custom braille backend, editor, or file format based on a
  one-frame success. It must satisfy the whole animation acceptance bar.
- Do not mutate the frozen legacy Python reference tree as part of an active
  Rust hero revision unless its ownership is explicitly revisited.

## Acceptance Bar

A candidate is acceptable only when all of the following hold:

- It uses real alpha through source decode, compilation, and scene rendering.
- All frames have the declared fixed cell geometry and no placeholder frame.
- The approved representative-frame set visibly retains the four dark-color
  families: red, brown, green, and black.
- The face, hands, foliage/hair edges, and black costume regions remain
  legible without matte fill or background leakage.
- The animation loop has intentional timing; source frame durations are
  retained or an explicit authored playback timeline replaces them.
- The main-scene result is reviewed in a real supported terminal after a
  cache-clear or unique asset revision, not only through synthetic unit tests.
- The compiled frame package records its source identity, compiler preset,
  output geometry, timing, and schema revision.

## Canonical Direction

The end-state pipeline is:

```text
versioned source GIF -> explicit compiler preset -> CellGrid corrections
-> validated HeroFrameSet package -> runtime playback -> HeroLayer scene grid
```

Chafa remains a useful baseline compiler while it earns that role. It must not
remain the sole opaque authority over dark-color fidelity, and it must not be
required for ordinary runtime startup. A future custom braille backend may be
evaluated beside Chafa, but both feed the same `CellGrid` and package contract.

The source GIF and the compiled terminal package are different owner layers:

| Layer | Owner | Rule |
| --- | --- | --- |
| Source animation | versioned active hero asset | Carries intended art, alpha, and timing. |
| Compiler preset | hero-revision contract | Names every transform and tool version that affects output. |
| Cell corrections | structured `CellGrid` patches | Corrects cells explicitly; never hand-edits opaque ANSI captures. |
| Compiled package | intentional hero asset | Is validated, versioned, and runtime-readable. |
| User cache | runtime acceleration only | Is disposable and never the sole authority. |
| Scene placement | `HeroLayer` / spatial contract | Keeps hero world-attached and separate from art conversion. |

## Roadmap

### Phase 0 — Establish the reference

- Freeze a small representative-frame set covering face, hands, dark red,
  brown, green, black, alpha edges, and fast motion.
- Record the current source asset digest, `820x820` canvas, 64-frame sequence,
  and source timing as facts, not as the target design.
- Write down one canonical hero anchor semantic (`top-left`, `center`, or
  `baseline`) before changing dimensions or offsets.
- Treat current output as a comparison baseline, not as approval.

### Phase 1 — Make compilation explicit

- `landed 2026-07-27` Defined a manifest and package schema
  (`HeroManifest`/`HeroPackage` in `render/hero_manifest.rs` and
  `render/hero_package.rs`) carrying asset id/digest, dimensions, frame
  durations, loop mode, compiler id/version, the literal preset args, and a
  package schema revision. See [`hero-package.md`](hero-package.md).
- `landed 2026-07-27` Added an explicit offline compiler
  (`render/hero_compiler.rs`, invoked via `yam-rust --compile-hero`) and a
  validation report (`HeroPackage::validate()` / `PackageValidation`). Uses
  Chafa, and emits a reviewable package without entering the interactive
  runtime.
- `landed 2026-07-27` Added fixture tests for fixed geometry, placeholder
  (blank-frame) rejection, and package provenance, using synthetic frames
  rather than the real hero asset. Not yet covered: alpha and partial-frame
  disposal semantics at the manifest/package level (the existing
  `decoded_hero_frames_keep_full_canvas_geometry` chafa-level test covers
  alpha/geometry for the decode step, but nothing yet asserts on it through
  the compiled package).
- `verified locally 2026-08-01` The package/compiler batch passes the full
  repository gate, produces a valid current-source package, and reaches the
  main scene in a bounded live smoke. This does not close the dark-color
  fidelity acceptance bar or runtime package wiring.

### Phase 2 — Revise the source art and renderer together

- Create candidate GIF revisions only against the frozen frame set and
  compiler manifest.
- Compare Chafa presets and, if necessary, a true `2x4` braille backend using
  the same source frames and color acceptance bar.
- Keep dark-color policy explicit. Region-specific correction is acceptable
  only when named, reproducible, and validated across the animation.

### Phase 3 — Promote a stable runtime artifact

- Make runtime load the validated package first, with source timing and fixed
  geometry intact.
- Keep live compilation as an explicit developer rebuild/fallback operation,
  not the normal launch path.
- Verify the main scene in a supported terminal and retain compact, meaningful
  regression fixtures rather than generated cache files.

## Always-When-In-Doubt Rules

- Favor visible color fidelity over a smaller diff, faster cache, or a more
  convenient Chafa flag.
- If a change makes dark regions disappear, flatten, turn into background, or
  become indistinguishable, reject it even when tests pass.
- Preserve alpha; never solve dark-color loss by baking a matte into the art.
- Keep source art, compiled art, runtime cache, and scene placement as
  separate owner layers.
- Require a real-terminal visual check for any source, compiler, color,
  braille, timing, or cache-identity change.
- Do not call a color issue fixed from ANSI-code presence alone. Evaluate the
  rendered region in context and across motion.
- Keep the hero world-attached; art work must not turn it into HUD or modal
  presentation.
- When evidence conflicts, stop expanding the experiment and update this
  document with the observed result, the exact preset, and the next smallest
  falsifiable test.

## Routing

- [rendering.md](rendering.md) owns active renderer and scene behavior.
- [hero-cache.md](hero-cache.md) owns current runtime cache mechanics.
- [TODO.md](../TODO.md) owns the executable next steps.
- [audit.md](audit.md) records the active risk concisely.
- [LOG.md](LOG.md) records completed experiments and decisions.
