# Notes

## Source of truth

`~/_git/yam` is the source of truth.

Installed copies under `~/.config` and `~/.local` are runtime files, not the canonical copy.

## Sync rule

Preferred workflow:
1. Edit files in `~/_git/yam`
2. For visualizer-only iteration, run `yam` directly because the launcher now prefers `~/_git/yam/visualizer`
3. Run `./install.sh` when runtime copies or startup assets need to be synced
4. Reopen Ghostty and verify
5. Commit after the live setup looks right

If you edit a live file directly, copy that change back into `~/_git/yam` before committing.

## Expected repo/runtime difference

`chafa/chafa_lab.sh` supports both repo and installed asset layouts:
- repo path candidate: `../assets/ives_yam.png`
- runtime path candidate: `assets/ives_yam.png`

That dual-path behavior is expected and should not be “fixed” unless the install layout changes.

## Visualizer maintenance

The current maintenance snapshot for the visualizer lives in:

- `visualizer/STATUS.md`
- `visualizer/VOCABULARY.md`

Current terminal target for the visualizer:

- Ghostty first

Ghostty config source of truth:

- `ghostty/config`

Installed runtime path:

- `~/Library/Application Support/com.mitchellh.ghostty/config.ghostty`

The installer now symlinks that file back to the repo copy through App Support on macOS instead of copying it.
tmux prefix handling stays native: Ghostty should not inject a synthetic `Ctrl+A`; tmux receives the raw control key directly.

Treat these visualizer modules as stable unless there is a clear bug:

- `visualizer/src/main.py`
- `visualizer/src/chafa_pipeline.py`
- `visualizer/src/info_panel.py`
- `visualizer/src/renderer.py`
- `visualizer/src/terminal.py`

Treat these as the future integration seam for outside vines code:

- `visualizer/src/layout.py`
- `visualizer/src/vines_engine.py`

## Visualizer runtime rule

The `yam` command now prefers:

- `~/_git/yam/visualizer`

and only falls back to:

- `~/.local/share/yam-visualizer`

when the repo copy is unavailable.

That is intentional and exists so visualizer iteration does not require reinstall for every change.
