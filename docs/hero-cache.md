# Hero Cache Plan

> **0.4 main goal.** This document sits inside the hero track, the primary
> development focus for 0.4 (set 2026-07-25) — see [`audit.md`](audit.md)'s Hero
> Track section for the full scope. Expect this note to move faster than most
> contract docs while that track is active. Two consequences worth stating up
> front: the source art is now in scope for deliberate change, so the freshness
> rule below will see real cache invalidations rather than only theoretical
> ones; and the offline compiler / `CellGrid` direction in Migration Order is
> now active direction rather than a parked idea.

This note defines the first runtime-facing `HeroFrameSet` seam and the current cache-first startup path.

## Goal

Keep the current Chafa-rendered hero baseline visually stable while removing the common-path startup cost of:

- GIF decode
- temporary frame PNG writes
- per-frame `chafa` process spawns

The intended direction is:

1. compile or refresh hero frames offline or on-demand
2. persist them as a runtime-owned cache
3. let normal startup load that cache directly

The currently wired runtime cache file lives in the user cache directory:

- `$XDG_CACHE_HOME/yam/hero_gif_1.96x48.frame_cache.json` when `XDG_CACHE_HOME` is set
- otherwise `~/.cache/yam/hero_gif_1.96x48.frame_cache.json`

## Runtime Shape

The initial runtime cache contract is:

- `HeroFrameSet`
  - `render_width`
  - `render_height`
  - `frames: Vec<CellGrid>`
- `CellGrid`
  - `width`
  - `height`
  - `cells: Vec<CachedCell>`
- `CachedCell`
  - `symbol`
  - `style`
- `CachedStyle`
  - `fg`
  - `bg`
  - `add_modifier`
  - `sub_modifier`

The current code seam for this shape lives in [hero_cache.rs](../src/render/hero_cache.rs).

## Migration Order

1. Define and test the serializable cache format.
2. Add a runtime loader that can hydrate hero frames from `HeroFrameSet`.
3. Keep the current Chafa path as the rebuild and fallback seam.
4. Switch ordinary startup to prefer cached frames only after the cache proves stable.

## Acceptance Bar

- Runtime startup should avoid the current GIF decode plus temp-frame plus per-frame process-spawn cost on the common path.
- Visible hero geometry, frame count, and color stability should stay aligned with the current Chafa baseline.
- The cache should remain a runtime-owned representation, not a second independent rendering authority.
- The cache freshness rule should stay simple and explicit: a cache is stale
  only when the source GIF can be shown to be newer than it.
- On a fresh machine without `chafa`, startup should degrade explicitly rather than panic: the uncached path may fall back to a visible placeholder frame, but the runtime should remain alive and the cache loader should still be preferred whenever a valid cache already exists.

## Source Reachability

`HERO_GIF_PATH` in [chafa.rs](../src/render/chafa.rs) is an absolute
compile-time path (`CARGO_MANIFEST_DIR`), and the GIF is read at runtime rather
than embedded in the binary. A binary therefore stays bound to the tree it was
built from: move, rename, or delete that tree — including building from a
throwaway git worktree — and the source GIF becomes unreachable to the
installed binary.

The freshness rule above is written so that this degrades to *cached art*
rather than to nothing. An unreachable source cannot prove the cache stale, so
an existing cache is kept and rendered; only a missing cache falls through to
the live compile path, which needs that same unreachable GIF and will produce a
placeholder frame. A source that reappears resumes ordinary mtime comparison on
the next launch.

This is a mitigation, not a fix for the underlying path binding: a binary
installed from a since-deleted tree still cannot *rebuild* the cache, so a
resolution change or a cache wipe leaves it with placeholder frames. Embedding
the asset (`include_bytes!`) or resolving it relative to the installed binary
would close that gap properly, at the cost of a ~4.3MB binary or an install
layout contract respectively.
