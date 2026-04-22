# YAM v2 Source Map

This document maps the new v2 source tree to the spec layers.

## Current Structure

```text
app.py
config.py
check_golden.py
scene_config.json
runtime/
  model.py
  messages.py
engine/
  ecosystem.py
  environment.py
  lifecycle.py
  species.py
  balance.py
morphology/
  model.py
shape/
  model.py
render/
  clock_font.py
  fonts/
    go_deco.txt
    Fender.flf
  framebuffer.py
  layers.py
  masks.py
  emitter.py
cmd/
  yamv2/
    main.go
    figlet_clock.go
ui/
  panels.py
  input.py
  keybindings.py
  commands.py
theme/
  model.py
```

## Notes

- This is the current filesystem contract.
- Keep module names stable once implementation starts.
- Add new files here before adding nontrivial runtime behavior.
- Prefer Bubble Tea core types in the runtime shell; only add optional upstream widgets when a module has a concrete need.
- Keep styling and layout primitives aligned with the dependency matrix instead of adding ad hoc third-party helpers.
- Consult [`DEPENDENCY_MATRIX.md`](DEPENDENCY_MATRIX.md) before introducing any new upstream UI package into the source tree.
- The live clock is rendered by the Go FIGlet engine in `cmd/yamv2`.
- `clock_font_name` in `scene_config.json` selects the live FIGlet font.
- The default live font is `Fender`, loaded from `render/fonts/Fender.flf`.
- The helper path is snapshot-only and should not define a second live clock renderer.
- `render/fonts/go_deco.txt` remains a legacy compatibility asset only.
