# Hero Cache Plan

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

The current code seam for this shape lives in [hero_cache.rs](/Users/mcq/_git/yam/src/render/hero_cache.rs:1).

## Migration Order

1. Define and test the serializable cache format.
2. Add a runtime loader that can hydrate hero frames from `HeroFrameSet`.
3. Keep the current Chafa path as the rebuild and fallback seam.
4. Switch ordinary startup to prefer cached frames only after the cache proves stable.

## Acceptance Bar

- Runtime startup should avoid the current GIF decode plus temp-frame plus per-frame process-spawn cost on the common path.
- Visible hero geometry, frame count, and color stability should stay aligned with the current Chafa baseline.
- The cache should remain a runtime-owned representation, not a second independent rendering authority.
- The cache freshness rule should stay simple and explicit: cached hero frames are reusable only when the cache file is at least as new as the source GIF.
- On a fresh machine without `chafa`, startup should degrade explicitly rather than panic: the uncached path may fall back to a visible placeholder frame, but the runtime should remain alive and the cache loader should still be preferred whenever a valid cache already exists.
