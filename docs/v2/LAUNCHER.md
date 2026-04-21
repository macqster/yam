# YAM v2 Launcher

This document records the current launch contract.

## Default

```bash
yam
```

Launches the v2 scaffold.

The default v2 scaffold now uses the Go Bubble Tea runtime shell and redraws the live FIGlet clock scene.
The live scene uses the terminal alternate screen and hides the cursor until exit.

## Runtime Selection

```bash
yam --version v2 --runtime bubbletea
```

Launches the Go Bubble Tea runtime shell inside `v2/`.

```bash
yam --version v2 --runtime python
```

Launches the legacy Python fallback path, which is retained only as a thin verification helper.

## Compatibility

```bash
yam --version v1
```

Launches the legacy visualizer stack.

## Scene Config Commands

```bash
yam --scene-config show
yam --scene-config edit
yam --scene-config reset
yam --scene-set theme_name=monochrome
```

These commands operate on `v2/scene_config.json`.

## Notes

- the launcher is versioned explicitly
- v1 recipe handling remains available behind the compatibility flag
- the v2 launcher can choose `python` or `bubbletea`
- the default v2 runtime is `bubbletea`
- the Python entrypoint is retained as a fallback verifier and uses `python3 -m v2.app`
- the Bubble Tea entrypoint uses `go run ./cmd/yamv2` from `v2/`
- scene config tuning is exposed through launcher subcommands
- the default scene is the day-plus-hour FIGlet clock
- `--scene-set` updates one supported key at a time
- see [`DEPENDENCY_MATRIX.md`](DEPENDENCY_MATRIX.md) for the approved upstream tool categories behind this launcher contract
