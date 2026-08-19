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

Runtime cache files live in the user cache directory, one per hero source:

- `$XDG_CACHE_HOME/yam/<stem>.r<revision>.<width>x<height>.frame_cache.json` when `XDG_CACHE_HOME` is set
- otherwise `~/.cache/yam/<stem>.r<revision>.<width>x<height>.frame_cache.json`

`<stem>` and `<revision>` come from the `HeroSource` descriptor
(`src/render/hero_source.rs`), so the default `IVY_VECTOR` source resolves to
`hero_gif_2.r1.96x48.frame_cache.json` and the previous `IVY` default to
`hero_gif_1.r2.96x48.frame_cache.json`. The per-source key prevents different
assets from sharing a cache, while the revision prevents a renderer,
compiler-preset, or serialized-contract change from silently reusing frames
produced by older behavior.

An ordinary launch compiles only the default source, so `hero_gif_1.r2.*` is
written only when something actually renders `IVY` — a test, an explicit
`Hero::from_source` call, or a launch with `YAM_HERO_SOURCE=hero_gif_1`. Each
source keeps its own cache, so switching between them with that variable costs
one cold chafa compile per source and is warm from then on. The first launch
after upgrading to 0.4.1 pays that cold compile for `hero_gif_2`, because no
existing cache matches the new default.

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
- The cache freshness rule should stay simple and explicit: cached hero frames are reusable when the cache file is at least as new as the source GIF. If the compile-time source path is no longer reachable after a build tree is moved or removed, an existing revision-matched cache remains reusable because it is the only path to real art.
- On a fresh machine without `chafa`, startup should degrade explicitly rather than panic: the uncached path may fall back to a visible placeholder frame, but the runtime should remain alive and the cache loader should still be preferred whenever a valid cache already exists.
