# YAM v2 Source Map

This document maps the new v2 source tree to the spec layers.

## Proposed Structure

```text
v2/
  app.py
  assets/
    fonts/
      Gothic.flf
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
    framebuffer.py
    layers.py
    masks.py
    emitter.py
  ui/
    panels.py
    input.py
    keybindings.py
    commands.py
  theme/
    model.py
```

## Notes

- This is the first-pass filesystem contract.
- Keep module names stable once implementation starts.
- Add new files here before adding nontrivial runtime behavior.
- Prefer Bubble Tea core types in the runtime shell; only add optional upstream widgets when a module has a concrete need.
- Keep styling and layout primitives aligned with the dependency matrix instead of adding ad hoc third-party helpers.
- Consult [`DEPENDENCY_MATRIX.md`](DEPENDENCY_MATRIX.md) before introducing any new upstream UI package into the source tree.
- The clock font source lives under `v2/render/fonts/` and is shared between the Go runtime and Python verifier.
