# Hero Cache Plan

This note defines the disposable runtime-facing `HeroFrameSet` seam within the
current package-then-cache startup path.

## Goal

Keep the current Chafa-rendered hero baseline visually stable while removing the common-path startup cost of:

- GIF decode
- temporary frame PNG writes
- per-frame `chafa` process spawns

The intended direction is:

1. compile or refresh hero frames offline or on-demand
2. persist them as a runtime-owned cache
3. let normal startup load that cache directly

As of 0.4.9 this cache is the *second* choice, not the first: `hero_frames_cached_from` prefers a validated `HeroPackage` when one is present, then this cache, then the live chafa path. See [hero-package.md](hero-package.md). The distinction matters because a package is validated on the source's SHA-256 digest and its preset id, while this cache can only compare mtimes - art swapped in with an older timestamp defeats the cache but not a package.

Runtime cache files live in the user cache directory, one per hero source:

- `$XDG_CACHE_HOME/yam/<stem>.r<revision>.<width>x<height>.frame_cache.json` when `XDG_CACHE_HOME` is set
- otherwise `~/.cache/yam/<stem>.r<revision>.<width>x<height>.frame_cache.json`

`<stem>` and `<revision>` come from the `HeroSource` descriptor
(`src/render/hero_source.rs`), so the default `IVY_VECTOR` source resolves to
`hero_gif_2.r5.96x48.frame_cache.json` and the previous `IVY` default to
`hero_gif_1.r6.96x48.frame_cache.json`. The per-source key prevents different
assets from sharing a cache, while the revision prevents a renderer,
compiler-preset, or serialized-contract change from silently reusing frames
produced by older behavior.

Live compilation writes a cache only when neither a valid package nor a valid
cache exists for the selected source. `hero_gif_1.r6.*` is therefore written
only when something actually renders `IVY` without a prepared artifact — a
test, an explicit `Hero::from_source` call, or a launch with
`YAM_HERO_SOURCE=hero_gif_1`. Each source keeps its own cache, so switching
between sources never reuses another source's frames. Old revision-keyed files
may remain in the user cache directory, but the current descriptor will not
select them.

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

## Landed Migration

The original cache migration completed in stages:

1. The serializable `HeroFrameSet` / `CellGrid` format was defined and tested.
2. Runtime loading was added.
3. Live Chafa was retained as the rebuild and fallback seam.
4. Ordinary startup began preferring prepared artifacts.

The current runtime order is validated package first, disposable frame cache
second, and live Chafa third. The package contract belongs in
[`hero-package.md`](hero-package.md); this note owns only the disposable cache.

## Acceptance Bar

- Runtime startup should avoid GIF decode plus temporary-frame writes and
  per-frame process spawning whenever a valid package or cache is available.
- Visible hero geometry, frame count, and color stability should stay aligned with the current Chafa baseline.
- The cache should remain a runtime-owned representation, not a second independent rendering authority.
- The cache freshness rule should stay simple and explicit: cached hero frames are reusable when the cache file is at least as new as the source GIF. If the compile-time source path is no longer reachable after a build tree is moved or removed, an existing revision-matched cache remains reusable because it is the only path to real art.
- On a fresh machine without `chafa`, startup should degrade explicitly rather than panic: the uncached path may fall back to a visible placeholder frame, but the runtime should remain alive and the cache loader should still be preferred whenever a valid cache already exists.
