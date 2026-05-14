# Known Issues

This file tracks active unresolved issues only.

## Contract

- use explicit date-and-time stamps for issue entries and meaningful updates
- keep entries user-visible or developer-visible, not broad project wishes
- keep each issue small enough to link cleanly from `README.md`, `docs/README.md`, `TODO.md`, and `docs/LOG.md`
- use one stable issue id per entry so backlog items can refer to it directly

## Tag Model

- `id:` stable issue id such as `KI-001`
- `status:` current state such as `open`, `watch`, `blocked`
- `severity:` impact level such as `minor`, `moderate`, `major`, `critical`
- `surface:` affected area such as `ui`, `loading`, `scene`, `render`, `docs`
- `system:` owning seam such as `alignment`, `layout`, `input`, `persistence`, `projection`
- `links:` optional link back to the matching backlog or contract note

## Active Issues

### KI-001

- `status: open` `severity: minor` `surface: loading` `system: alignment` `links: TODO.md immediate maintenance`
- first recorded: `2026-05-08 06:49 CEST`
- summary:
  loading-screen prompt alignment remains slightly off: `press [space] to continue` still reads as visually non-symmetrical relative to the rest of the centered boot assembly and needs one deliberate centering pass rather than further blind nudging.
