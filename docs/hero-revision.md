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

## Current Problem: Resolved 2026-08-19

The dark-color defect this document was written against is fixed. The cause
was the `--bg` value: under `--fg-only` chafa treats it as the color already
on screen and discards art resembling it, and `HERO_DISPLAY_BG` was a dark
red (`#100100`). The pipeline was being instructed to throw away the exact
content that kept going missing. See [`chafa-drop-rule.md`](chafa-drop-rule.md)
for the mechanism, the measurement harness, and the procedure for choosing a
value when new art is registered.

This section previously asserted that "the failure is broader than one Chafa
flag" and listed eight participating subsystems. That was wrong: it was one
flag. Recorded rather than quietly corrected, because the reasoning is the
useful part - each of those subsystems was a plausible suspect, and the defect
was only isolated by measuring them out one at a time rather than by
reasoning about which was most likely.

The 2026-07-22 alpha and `median` changes did repair a real and separate
coverage-loss failure. They were necessary; they were just never sufficient,
which is what this section correctly warned about.

The offline package direction below is now implemented through package-first
startup. What remains open is future art or renderer revision work and any
curated cell-editing workflow. The acceptance bar still stands for every
source, preset, timing, or cell-content change: structural checks cannot judge
color, so a real-terminal review is still required.

## Status (2026-08-22)

The vector source is registered as `IVY_VECTOR` and has been the default since
0.4.1. The dark-color defect was isolated and fixed through the per-source
`absent_color` contract rather than by restoring the removed
`tone_lift_dark_reds` correction. Both registered sources remain independently
selectable and covered by source-geometry, coverage, palette-overlap, and
runtime-fallback tests.

The manifest, package, shared `CellGrid`, and offline compiler are landed.
Runtime startup prefers a package only when schema, preset, geometry, source
digest, and package validation all agree; it then falls back to the disposable
frame cache and finally live Chafa compilation. This closes the original
package-wiring roadmap, but it does not turn machine checks into visual
approval. See [`hero-package.md`](hero-package.md) for the current contract.

## Product Goals

1. Preserve the intended silhouette and transparent surround without using a
   matte or background-colored substitute for alpha.
2. Preserve each approved source's intentional dark-color composition without
   accidental loss into transparency or the scene background. The vector
   source may deliberately cull named dark tiers through its documented
   `absent_color` policy; unapproved loss of red, brown, green, or black detail
   remains a defect.
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
- The approved representative-frame set retains every dark-color region not
  explicitly culled by that source's documented `absent_color` policy.
- The face, hands, foliage/hair edges, and retained costume/detail regions
  remain legible without matte fill or background leakage.
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
-> validated HeroPackage -> runtime playback -> HeroLayer scene grid
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
- Record each registered source's digest, declared canvas and frame count, and
  source timing as facts, not as the target design. The current pair is
  `IVY_VECTOR` at `1080x1080` / 48 frames and `IVY` at `820x820` / 64 frames.
- Write down one canonical hero anchor semantic (`top-left`, `center`, or
  `baseline`) before changing dimensions or offsets.
- Treat current output as a comparison baseline, not as approval.

### Phase 1 — Make compilation explicit

- `implemented on branch 2026-07-27; landed on main 2026-08-20` Defined a
  manifest and package schema
  (`HeroManifest`/`HeroPackage` in `render/hero_manifest.rs` and
  `render/hero_package.rs`) carrying asset id/digest, dimensions, frame
  durations, loop mode, compiler id/version, the literal preset args, and a
  package schema revision. See [`hero-package.md`](hero-package.md).
- `implemented on branch 2026-07-27; landed on main 2026-08-20` Added an
  explicit offline compiler
  (`render/hero_compiler.rs`, invoked via `yam-rust --compile-hero`) and a
  validation report (`HeroPackage::validate()` / `PackageValidation`). Uses
  Chafa, and emits a reviewable package without entering the interactive
  runtime.
- `implemented on branch 2026-07-27; landed on main 2026-08-20` Added fixture
  tests for fixed geometry, placeholder
  (blank-frame) rejection, and package provenance, using synthetic frames
  rather than the real hero asset. Not yet covered: alpha and partial-frame
  disposal semantics at the manifest/package level (the existing
  `decoded_hero_frames_keep_full_canvas_geometry` chafa-level test covers
  alpha/geometry for the decode step, but nothing yet asserts on it through
  the compiled package).
- `verified locally 2026-08-20` The package/compiler batch passes the full
  repository gate, produces valid packages for registered sources, and reaches
  the main scene in bounded live use. Package-first startup is wired; visual
  acceptance remains change-specific rather than implied by this structural
  gate.

### Phase 2 — Revise the source art and renderer together

- `landed 2026-08-19` Registered `IVY_VECTOR` beside `IVY`, promoted it to the
  default source, and kept source selection reversible through
  `YAM_HERO_SOURCE`.
- `landed through 2026-08-20` Made the Chafa preset and per-source
  `absent_color` policy explicit, measured their visible effects, and covered
  both registered sources with geometry, alpha, content, and palette guards.
- `ongoing` Create any future GIF revision against the frozen frame set and
  compiler manifest; compare preset or backend candidates under the same
  color acceptance bar.
- `ongoing` Keep region-specific correction named, reproducible, and validated
  across the animation rather than embedding an undocumented shared transform.

### Phase 3 — Promote a stable runtime artifact

- `landed 2026-08-20` Runtime loads a matching validated package first, with
  fixed geometry and provenance checks intact, then falls through to the frame
  cache and live compiler.
- `landed 2026-08-20` Live compilation is the final rebuild/fallback seam, not
  the normal launch path when a valid package or cache exists.
- `ongoing` Verify the main scene in a supported terminal after every visible
  source, preset, timing, or cell-content change, and retain compact regression
  fixtures rather than generated cache files.

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
