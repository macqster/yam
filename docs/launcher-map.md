# Launcher Map

This document maps the `yam` launcher to the repo and runtime locations it understands.

## Current behavior

The launcher prefers the repo copy of the visualizer when possible:

1. check `YAM_VISUALIZER_ROOT` if it is set
2. otherwise check `YAM_REPO` and use its `visualizer/` directory
3. otherwise fall back to `~/.local/share/yam-visualizer`

## Current mapping

| Repo file | Installed target | Role |
| --- | --- | --- |
| `bin/yam` | `~/.local/bin/yam` | Launches the visualizer from the repo copy or the installed bundle |
| `visualizer/run_visualizer.sh` | `~/_git/yam/visualizer/run_visualizer.sh` or `~/.local/share/yam-visualizer/run_visualizer.sh` | Visualizer entrypoint |

## Notes

- The launcher is intentionally repo-copy-first so active visualizer iteration stays local
- The runtime bundle remains the fallback when the repo copy is unavailable
- If the repo root changes, update this map and the launcher together

