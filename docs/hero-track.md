# Hero Track: Traced Baseline

<!-- cspell:ignore RAII Rgba -->

This document is the prep artifact for the 0.4 hero revision. It traces the
hero pipeline as it actually runs today, records measured numbers rather than
estimates, and evaluates each stage. It is a **baseline and evaluation**, not a
phase plan — no implementation order is committed here.

The decisions the rework needs are recorded as `HQ-1` through `HQ-6` in
Resolution Points, to be settled *during* the rework phase. They are
deliberately left open: each states its resolution criteria and owning doc, and
none is pre-judged. The rankings in Opportunities reflect measured cost, not a
chosen plan.

Traced 2026-07-25 against `611f7ca`, on a 10-core darwin host with
`chafa 1.18.2`.

Owning docs this one defers to: [`rendering.md`](rendering.md) for render/layer
contracts, [`hero-cache.md`](hero-cache.md) for the cache path,
[`architecture.md`](architecture.md) for the geometry contract, and
[`audit.md`](audit.md) for risk status.

## Scope Of The Traced Path

The whole chain from source art to terminal cells:

`assets/hero_gif_1.gif` → GIF decode → per-frame canvas expansion + color
pre-pass → temp PNG → `chafa` subprocess → ANSI → `ratatui` `Line`/`Span` →
frame cache (JSON) → `Hero` in UI state → `HeroLayer` → `Grid` → compositor →
terminal.

## Measured Baseline

Every number below was measured, not estimated. Method is named so each can be
re-run.

### Source asset

| Fact | Value |
| --- | --- |
| File | `assets/hero_gif_1.gif`, 4.31 MB |
| Canvas | 820x820, palette mode (`P`) |
| Frames | 64 |
| Declared frame delay | 0 ms |
| Frame 0 fully-transparent pixels | 411,951 / 672,400 (61.3%) |

The declared 0 ms delay is inert: the runtime never reads GIF timing and drives
animation from its own cadence (see below). Worth knowing before anyone
"fixes" the delay expecting it to change playback.

### Cold compile (no cache)

| Stage | Measurement | Method |
| --- | --- | --- |
| Total, launch → cache written | **4.74 s** | wall-clock poll for the cache file with `XDG_CACHE_HOME` pointed at a scratch dir |
| Single `chafa` invocation | **81 ms**, 26,452 bytes stdout | `time chafa` with the exact production flag set on an exported frame |
| Implied 64-frame cost | ~5.2 s | 64 x 81 ms, consistent with the 4.74 s total |
| Subprocess spawns | **64** | one per frame, `chafa.rs:77-84` |
| Temp PNG writes | **64** | `render_image_frame`, `chafa.rs:316-319` |

`chafa` spawning dominates the cold path. Nothing else in the chain is close.

### Warm path and steady state

| Fact | Value |
| --- | --- |
| Cache file | 27.0 MB JSON |
| Cached cells | 294,912 (64 frames x 4,608) |
| Bytes per cell | ~92 |
| Style runs across all frames | 70,982 |
| Style runs per frame | ~1,109 (avg 23.1 spans per line) |
| Render loop target | **120 FPS** (`runtime.rs:90`) |
| Hero animation cadence | **2 FPS** (`hero_fps` default, `state.rs:677`) |
| Idle CPU, main scene | ~29% of one core |
| Idle CPU, greenhouse (hero + companions hidden) | ~23.5% of one core |
| RSS | ~35.5 MB |

## Stage-By-Stage Trace

### 1. Entry seam

Production reaches this whole subsystem through exactly **one** function:
`chafa::hero_frames_cached(HERO_RENDER_WIDTH, HERO_RENDER_HEIGHT)`, called once
from `Hero::new` (`hero.rs:26-27`), plus the two dimension constants. Every
other public item in `chafa.rs` is internal to the module's own call graph.

That is a genuinely narrow seam and worth preserving through the revision — it
is what makes replacing the interior (offline compiler, different converter)
tractable.

### 2. Decode and canvas expansion

`decode_gif_frames` (`chafa.rs:101-115`) reads the GIF and expands each frame
onto a full 820x820 **transparent** canvas via `frame_to_canvas`
(`chafa.rs:198-210`). GIF frames are often subimages with offsets; this
normalizes them to full canvas geometry so downstream sizing is stable.

Transparency is load-bearing here — this is the 2026-07-22 dark-region fix.
The canvas is `Rgba([0,0,0,0])`, and alpha is preserved to `chafa` rather than
flattened onto an opaque matte.

### 3. Color pre-pass

`tone_lift_dark_reds` (`chafa.rs:253-271`) runs per pixel during canvas
expansion: RGB → HSV, and if the pixel reads as dark red (hue ≤20° or ≥340°,
saturation ≥0.45, value ≤0.42) its value is lifted by +0.08, capped at 0.45,
with saturation nudged 1.02x.

**Evaluation: this is the most questionable stage in the chain.** It predates
the 2026-07-22 fix, which addressed the same symptom (dark reds not surviving
to the terminal) at a different and more correct layer — real alpha plus
`--color-extractor=median`. Two corrections for one symptom, one of which is a
hand-tuned magic-number color transform applied to 672,400 pixels per frame.
It has unit coverage of its own behavior but no test asserting it is still
*needed*. Tracked as `HQ-1`; not decided here.

### 4. Temp PNG + subprocess

`render_image_frame` (`chafa.rs:309-325`) writes each frame to
`yam_frame_NNNN.png` in a per-process temp dir, then shells out to `chafa` with
the path. The temp dir is RAII-cleaned by `TempFrameDir`'s `Drop`
(`chafa.rs:303-307`).

Flags (`chafa_output`, `chafa.rs:48-66`): braille symbols, full RGB color,
median extractor, no dither, `--fg-only`, `--bg=#100100`, `--animate=off`.
`HERO_DISPLAY_BG` is a *display hint only* — with `--fg-only` it tells `chafa`
what background to assume, it does not fill anything.

**Measured alternative:** `chafa` reads from stdin via `-`. I verified that
piping the PNG bytes with the identical production flag set produces
**byte-identical output** (`cmp` clean, same md5, 26,452 bytes both ways). So
the 64 temp-file writes are not required by the tool.

### 5. ANSI → ratatui

`render_frame_with_command` (`chafa.rs:24-46`) parses `chafa`'s stdout with
`ansi-to-tui` into `Text<'static>`, returning `text.lines`. Parse failure
degrades to a literal `ANSI_PARSE_ERROR` text rather than an error — note this
is *not* in the placeholder-detection list (see stage 6), so an ANSI parse
failure would be cached as if it were valid art.

### 6. Cache

`hero_frames_cached` (`chafa.rs:87-99`) prefers the cache, else compiles and
writes one. Guards, all of which are sound:

- `load_cached_hero_frames` validates freshness, exact render dimensions, and
  non-emptiness before accepting a cache.
- `hero_frames_are_cacheable` / `is_placeholder_frame` (`chafa.rs:132-151`)
  refuse to persist a batch containing any known placeholder string, which is
  what closes the 2026-07-22 silent-regression class properly.
- Freshness (`chafa.rs:157-180`) is "stale only when the source can be shown to
  be newer", so an unreachable source keeps the cache instead of discarding it.

Format is `serde_json` compact. At ~92 bytes per cell for what is fundamentally
a char plus two optional colors plus two u16 modifier bitfields, the encoding is
roughly an order of magnitude larger than the information it carries.

### 7. Runtime frame ownership

`Hero::new` (`hero.rs:23-68`) derives `width`/`height` from frame 0 and
normalizes **every** frame to that size via `hard_lock_frame` → `normalize_line`
(`hero.rs:260`, `hero.rs:218`). After construction, all 64 frames are already
exactly `width` x `height`.

### 8. Layer draw

`HeroLayer::render_into_grid` (`hero_layer.rs:41-97`) runs on every
`terminal.draw`, which is unconditional per loop iteration (`runtime.rs:374`).
Line 59:

```rust
let normalized = normalize_lines(hero.frame().clone(), hero.width, hero.height);
```

This clones the current frame (~1,109 `Span`s, each owning a `String`) and then
rebuilds it through `normalize_lines`/`normalize_line`
(`hero_layer.rs:114-163`) — logic that duplicates `hero.rs`'s
`hard_lock_frame`/`normalize_line` almost line for line, applied to data
`Hero::new` already normalized to exactly those dimensions.

At 120 FPS against a 2 FPS hero cadence, **each hero frame is cloned and
rebuilt about 60 times before it changes.**

## Evaluation

### Solid — do not disturb without cause

- The single-function entry seam (`hero_frames_cached`) and the fixed-geometry
  contract. This is what makes the interior replaceable.
- Failure handling: every stage degrades to a visible placeholder rather than
  panicking, and placeholders are structurally prevented from poisoning the
  cache.
- Cache validation: freshness, dimensions, non-emptiness, and placeholder
  rejection are each necessary and none is redundant.
- Alpha preservation through canvas expansion. This is the hard-won part of the
  2026-07-22 fix and is easy to break by "simplifying" `frame_to_canvas`.

### Fragile

- **Compile-time absolute asset path.** `HERO_GIF_PATH` binds a binary to its
  build tree. Mitigated for the cached case (2026-07-25) but not fixed; a
  binary from a deleted tree still cannot rebuild.
- **`ANSI_PARSE_ERROR` is not treated as a placeholder.** Unlike the five
  recognized failure strings, a parse failure produces a one-line frame that
  `is_placeholder_frame` does not match, so it would be written to cache and
  then loaded back as trusted art. Narrow, but it is the same silent-failure
  shape that already bit this repo once.
- **Two color corrections for one symptom** (stage 3), one of them
  magic-numbered.
- **Normalization logic duplicated across two modules** with no test asserting
  they agree.

### Unproven

- The offline compiler / `CellGrid` direction remains documented but not yet built.
  Note that `CellGrid` already exists and is exercised — as the *cache* format
  (`hero_cache.rs:41`). Any offline-compiler work should start from that fact
  rather than designing a second grid type.

## Opportunities, Ranked By Measured Value

Ranked by what the measurements actually support, not by how interesting the
work is.

1. **Cap or gate the render loop — the single largest lever.** ~23.5 of the
   ~29 idle CPU points survive with the hero and companions hidden, so the
   dominant cost is the unconditional 120 FPS redraw itself, not the hero. For
   a screensaver-like diorama whose fastest content moves at 2 FPS, 120 FPS is
   ~60x more redraws than the content justifies. This is the finding a naive
   reading would get backwards, which is exactly why it was worth measuring.
2. **Stop cloning and re-normalizing the hero frame every draw**
   (`hero_layer.rs:59`). Worth ~5.5 CPU points combined with companions, and
   the work is provably redundant — `Hero::new` already normalized to the same
   dimensions. Either cache the normalized frame or borrow it. Also collapses
   the duplicated normalization logic.
3. **Pipe frames to `chafa` via stdin instead of temp PNGs.** Verified
   byte-identical output. Removes 64 file writes, the temp-dir lifecycle, and
   three entire failure modes (`hero temp dir unavailable`, `failed to write
   temp image`, `temp path not utf-8`).
4. **Parallelize the 64 spawns.** They are independent subprocesses on a
   10-core host and currently run sequentially via `.map()`. Bounded by cores,
   this should take the cold path from ~4.7 s toward ~1 s. Cheaper and far less
   risky than replacing `chafa`, and it stays useful even if `chafa` is later
   replaced.
5. **Reconsider `tone_lift_dark_reds`** now that the real fix is in. If it is
   redundant, deleting it removes a per-pixel HSV round-trip over 672,400
   pixels x 64 frames and a set of magic numbers from the color path.
6. **Shrink the cache format.** 27 MB of JSON for 294,912 cells. Only worth
   doing if warm-load time actually matters; it was measured at ~150 ms
   previously, so this is a size and tidiness concern more than a speed one.
7. **Close the asset path binding** (`include_bytes!` or binary-relative
   resolution). Already tracked in `TODO.md`.

Items 1 and 2 are runtime cost; 3, 4, and 5 are compile-path cost and
robustness; 6 and 7 are hygiene. Item 1 is not hero-specific but was found by
tracing the hero, and it dominates everything else here.

## Resolution Points For The Rework Phase

Open decisions to resolve **during** the rework phase. Nothing here is decided,
and nothing here should be read as leaning toward an outcome — each point states
the question, why it is genuinely open, what evidence would settle it, what it
blocks, and which doc owns the answer once made. `HQ-` ids are stable so backlog
items and log entries can cite a single point rather than restating it.

Status vocabulary: `open` (no decision), `resolved` (decision made and recorded
per the protocol below). All points are `open`.

### HQ-1 — Does `tone_lift_dark_reds` stay?

- `status:` open
- `question:` Is the per-pixel HSV dark-red lift (`chafa.rs:253-271`) still
  contributing to the rendered result now that real alpha plus
  `--color-extractor=median` landed (2026-07-22)?
- `why open:` Two corrections address the same symptom at different layers. The
  pre-pass predates the other one. Its tests assert what it does, not that it is
  still required, so neither "keep" nor "remove" currently has evidence behind
  it.
- `resolved by:` An A/B of rendered output — cold-compile the frame set with the
  pre-pass disabled, capture with `capture-pane -p -e`, and compare dark-red
  coverage and overall color distribution against the current path. Both
  outcomes are legitimate; if it stays, the constants need an explanation, and
  if it goes, the removal needs the same live verification.
- `blocks:` Nothing structurally. Independent of every other point.
- `answer lands in:` `docs/rendering.md`, plus the `chafa.rs` doc comment if it
  stays.

### HQ-2 — What drives redraws, and is 120 FPS intentional?

- `status:` open
- `question:` The render loop targets 120 FPS unconditionally (`runtime.rs:90`,
  `runtime.rs:374`) while the fastest content moves at 2 FPS. Is that
  deliberate, and if it changes, what should drive redraws?
- `why open:` No recorded rationale exists for the figure, and the measurement
  (~23.5 of ~29 idle CPU points) means the answer has real cost either way. A
  deliberate 120 FPS for input latency or effect smoothness is a defensible
  answer that simply needs writing down.
- `resolved by:` Establishing intent first, then — only if it changes — choosing
  among a lower fixed cap, dirty-region tracking, or event-driven redraw, each
  measured against idle CPU and against input responsiveness and the `tachyonfx`
  loading/quit effects, which are the surfaces most likely to suffer from fewer
  redraws.
- `blocks:` The value of HQ-3 partly, since a much lower redraw rate changes how
  much per-draw hero cost matters.
- `answer lands in:` `docs/rendering.md`. Not hero-owned — this is a render-loop
  decision surfaced by the hero trace.

### HQ-3 — Does `chafa` remain a subprocess?

- `status:` open
- `question:` Does the rework keep shelling out to `chafa`, or move conversion
  in-process (the offline compiler / `CellGrid` direction)?
- `why open:` The ranked opportunities include both subprocess-preserving work
  (piping via stdin, parallelizing spawns) and a replacement direction. These
  are not contradictory but the ordering depends on this answer, and an
  in-process converter would make the stdin and parallelism work moot.
- `resolved by:` Deciding whether YAM owns pixel-to-cell conversion. Note
  `CellGrid` already exists and is exercised as the cache format
  (`hero_cache.rs:41`), so a replacement starts from an existing type rather
  than a blank design. Whichever way this goes, output fidelity must be compared
  live against the current braille/median baseline.
- `blocks:` The sequencing of opportunities 3, 4, and 5. Settle this before
  investing in either branch.
- `answer lands in:` `docs/rendering.md` and `docs/hero-cache.md`.

### HQ-4 — Does the fixed 96x48 geometry survive?

- `status:` open
- `question:` Does the hero stay at a fixed 96x48 cell footprint, or become
  responsive to terminal size?
- `why open:` The cache is keyed by render dimensions
  (`hero_gif_1.{w}x{h}.frame_cache.json`), so responsive sizing implies a cache
  entry per observed size and a recompile on resize — at the cold cost measured
  above unless HQ-3 changes it.
- `resolved by:` Deciding whether the hero is a fixed-composition element or a
  scaling one. This is a composition question first and a performance question
  second; the scaffold and companion layout assume the current footprint.
- `blocks:` Cache format and key design; interacts with HQ-3 and HQ-5.
- `answer lands in:` `docs/architecture.md`'s Hero Geometry Contract, and
  `docs/hero-cache.md` for the keying consequence.

### HQ-5 — Is the source art shape itself up for revision?

- `status:` open
- `question:` Now that art is in scope, does the 64-frame / 820x820 / 4.31 MB
  source shape change?
- `why open:` Frame count drives the entire cold compile cost linearly, so this
  is the one point that can move the performance numbers without any code change
  at all. It was out of scope while the art was frozen and has never been
  examined.
- `resolved by:` An art decision, not a measurement — though the trace supplies
  the cost per frame if frame count is traded against smoothness. Any change
  must move `assets/hero_gif_1.gif` and
  `tools/legacy-python/hero/assets/hero_go.gif` together, per the existing
  byte-identical-duplicate rule in `docs/audit.md`.
- `blocks:` Nothing, but it changes the baseline every other point is measured
  against, so resolving it early avoids re-measuring.
- `answer lands in:` `docs/architecture.md`'s Hero Geometry Contract and
  `docs/audit.md`'s asset-duplication note.

### HQ-6 — How is the `ANSI_PARSE_ERROR` cacheability gap closed?

- `status:` open
- `question:` The gap itself is a bug to fix, not a decision. The open part is
  the approach: add the string to `is_placeholder_frame`'s list, or replace the
  in-band magic string with a typed error so the failure cannot be mistaken for
  content.
- `why open:` The list approach is a one-line fix that keeps the existing shape;
  the typed-error approach removes the class but touches the return type of the
  render path and so interacts with HQ-3.
- `resolved by:` Choosing between a minimal fix now and a structural fix folded
  into whatever HQ-3 decides. Either way the fix needs a test asserting a parse
  failure is never persisted.
- `blocks:` Nothing. Can be fixed independently at any point.
- `answer lands in:` `src/render/chafa.rs` and `docs/audit.md`'s matching risk
  note.

### Resolution protocol

When a point is resolved: record the decision and its reasoning in
`docs/LOG.md`, update the owning doc named in that point, flip its `status:` here
to `resolved` with the date, and drop the matching `TODO.md` item. Keep the point
in place rather than deleting it — the question and the evidence behind the
answer are worth more later than a clean list.

## Constraints That Must Hold Through The Revision

- Any change touching `src/render/chafa.rs` requires live
  `scripts/tmux-smoke.sh` verification. CI has no `chafa`, and every live-path
  test tolerates placeholder frames by design, so CI cannot catch this class.
- The cache stays a runtime-owned representation, not a second rendering
  authority.
- Placeholder frames must never be persisted as trusted cache.
- Alpha must survive canvas expansion.
- Aesthetic changes are deliberate and reviewed; incidental drift from
  non-hero work remains a regression.

## Re-Running This Trace

The measurements above are reproducible. Point `XDG_CACHE_HOME` at a scratch
directory to exercise the cold path without touching the real cache, then use
`scripts/tmux-smoke.sh` (or a raw `tmux` session plus
`capture-pane -p -e`) to inspect rendered color output. `ps -o %cpu=,rss=`
against the running pid gives the idle cost; switching to the greenhouse world
hides the hero and companions and gives the attribution baseline.
