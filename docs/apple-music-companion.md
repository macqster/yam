# Apple Music Companion Assessment

This note records the May 2026 assessment of whether YAM should grow a small
Apple Music companion surface. It is future-reference material, not an active
implementation commitment.

## Summary

A basic Rust/Ratatui Apple Music front end is feasible as a YAM companion if
YAM owns the terminal UI and delegates protected playback to a platform-owned
player. A pure Rust Apple Music streamer is not a practical target.

Best first shape:

- add a separate `yam-music` companion binary or mode inside the YAM repo
- keep Rust responsible for UI, selection, queue display, and state
- let macOS Music.app handle playback through AppleScript, JXA, or native macOS
  automation
- add Apple Music API or MusicKit metadata later only if the Music.app bridge is
  useful in daily use

Do not start by rebuilding the Vibez Chrome/CDP/MusicKit playback path unless
the explicit goal is to research that fragile backend.

## Context

The assessment came after testing `simonepelosi/vibez` on macOS. Vibez could
authenticate, read the user's Apple Music library, browse feeds, and populate
queues, but playback did not reliably start. Earlier attempts only reached
30-second previews. Later diagnostic builds reached library/feed/vibe browsing
but still failed around the MusicKit queue/play path.

Representative failure shape:

- Apple Music auth and library API access worked
- Vibez saw library songs, playlists, feeds, and generated vibe queues
- Chrome opened as the playback host
- MusicKit queue setup repeatedly failed or skipped tracks
- no full-track audio started

That makes the browser playback backend the risky part, not the terminal UI.

## Platform Facts

Apple's public MusicKit material supports:

- Apple Music metadata and library access through the Apple Music API
- playback through MusicKit on Apple platforms
- browser playback through MusicKit on the Web / MusicKit JS

The practical implication for YAM is that full Apple Music playback should be
treated as a platform/player integration problem. Rust should not expect to
receive raw protected audio stream URLs and play them directly.

References:

- <https://developer.apple.com/musickit/>
- <https://js-cdn.music.apple.com/musickit/v1/index.html>

## Recommended Architecture

Preferred path:

```text
yam-music TUI
    -> controller trait
        -> macOS Music.app controller
            -> AppleScript/JXA/native automation
        -> optional future metadata provider
            -> Apple Music API / MusicKit-related auth
```

Keep the companion separate from the main scene at first. A sibling binary is
cleaner than embedding a music UI into the current visualizer runtime before the
backend is proven.

Suggested surfaces:

- now-playing row
- playback controls: play/pause, next, previous, seek if available
- library or playlist list
- queue list
- simple search if it can be backed by Music.app or a small metadata provider
- compact status/errors panel

Suggested key shape:

| Key | Action |
| --- | --- |
| `Enter` | play selected item |
| `Space` | play/pause |
| `n` / `p` | next / previous |
| `/` | search or filter |
| `q` | quit |

## Effort Estimate

| Scope | Difficulty | Estimate |
| --- | --- | --- |
| Spike: play/pause/next/now-playing through Music.app | low | 2-4 hours |
| Minimal `yam-music` Ratatui front end | medium-low | 1-2 days |
| Browse Music.app library/playlists and play selected items | medium | 2-4 days |
| Apple Music API metadata plus Music.app playback | medium-high | 4-7 days |
| Chrome/MusicKit playback backend controlled from Rust | high/risky | 1-3 weeks |
| Pure Rust Apple Music streaming client | not recommended | open-ended / likely blocked |

Practical personal-use estimate:

- useful local companion: 2-4 days
- polished YAM-native companion: 1-2 weeks
- Vibez-style Chrome/MusicKit backend: research project, not a quick app slice

## Decision Notes

- Use Music.app first because it already owns the Apple Music account, DRM, and
  platform playback path.
- Treat MusicKit JS as a possible metadata/playback reference, not as the first
  backend to reimplement.
- Treat Apple Music API work as metadata and library management unless a later
  proof confirms reliable playback control.
- Keep this outside the main YAM scene until the controls are reliable enough to
  justify a companion surface.
- If this becomes active work, add a small backend trait before building the UI
  so Music.app automation, mock playback, and any future MusicKit experiment can
  be tested separately.

## First Implementation Slice

If the idea resumes, start with a tiny terminal spike:

1. Read current Music.app track, artist, album, player state, and position.
2. Send play/pause/next/previous commands.
3. Wrap those commands in a Rust controller trait.
4. Build a small Ratatui screen around that controller.
5. Only then evaluate library browsing and search.

The spike should prove control reliability before any YAM visual integration or
Apple Music API token work.
