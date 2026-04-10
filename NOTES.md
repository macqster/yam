# Notes

## Source of truth

`~/yam` is the source of truth.

Installed copies under `~/.config` and `~/.local` are runtime files, not the canonical copy.

## Sync rule

Preferred workflow:
1. Edit files in `~/yam`
2. For visualizer-only iteration, run `yam` directly because the launcher now prefers `~/yam/visualizer`
3. Run `./install.sh` when runtime copies or startup assets need to be synced
4. Reopen Kitty and verify
5. Commit after the live setup looks right

If you edit a live file directly, copy that change back into `~/yam` before committing.

## Expected repo/runtime difference

`chafa/chafa_lab.sh` supports both repo and installed asset layouts:
- repo path candidate: `../assets/ives_yam.png`
- runtime path candidate: `assets/ives_yam.png`

That dual-path behavior is expected and should not be “fixed” unless the install layout changes.

## Visualizer maintenance

The current maintenance snapshot for the visualizer lives in:

- `visualizer/STATUS.md`

Treat these visualizer modules as stable unless there is a clear bug:

- `visualizer/src/main.py`
- `visualizer/src/chafa_pipeline.py`
- `visualizer/src/info_panel.py`
- `visualizer/src/renderer.py`
- `visualizer/src/terminal.py`

Treat these as the future integration seam for outside ivy code:

- `visualizer/src/layout.py`
- `visualizer/src/ivy_engine.py`

## Visualizer runtime rule

The `yam` command now prefers:

- `~/yam/visualizer`

and only falls back to:

- `~/.local/share/yam-visualizer`

when the repo copy is unavailable.

That is intentional and exists so visualizer iteration does not require reinstall for every change.
