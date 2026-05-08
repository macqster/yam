# Known Issues

This file tracks active, user-visible, and developer-visible issues that are not yet resolved.

## Contract

- use explicit date-and-time stamps for issue entries and meaningful updates
- keep entries closely integrated with `README.md`, `docs/README.md`, `TODO.md`, and `docs/LOG.md` so front-door docs, work order, and history stay aligned
- use a clean tag/flag grammar on every issue so scope is obvious at a glance: status, severity, surface/layer, subsystem/logic, and any focused traits such as alignment, rendering, loading, input, or persistence

## Tag Model

- `status:` current state such as `open`, `watch`, `blocked`, `resolved`
- `severity:` impact level such as `minor`, `moderate`, `major`, `critical`
- `surface:` user-facing area or layer such as `ui`, `loading`, `scene`, `render`, `docs`
- `system:` owning subsystem or logic seam such as `alignment`, `layout`, `input`, `persistence`, `projection`
- `notes:` optional short freeform flags when a tighter cue helps

## Active Issues

### 2026-05-08 06:49 CEST

- `status: open` `severity: minor` `surface: ui` `system: alignment`
  loading-screen prompt alignment remains slightly off: `press [space] to continue` still reads as visually non-symmetrical relative to the rest of the centered boot assembly and needs another centering pass rather than further blind nudging.
