# Notes

Working notes for repo/runtime sync.

## Rules

- Treat `~/ _git/yam` as the canonical source tree for the Rust app.
- Treat installed copies under `~/.config` and `~/.local` as runtime artifacts, not authoritative source.
- Keep live runtime tweaks copied back into the repo before committing.
- Use `./install.sh` only when you need to resync runtime copies or startup assets.

## Visualizer

- The current visualizer maintenance snapshot lives in `visualizer/STATUS.md` and `visualizer/VOCABULARY.md`.
- Ghostty is the preferred terminal target.
- Ghostty config source of truth is `ghostty/config`.
- Installed runtime path is `~/Library/Application Support/com.mitchellh.ghostty/config.ghostty`.
- Ghostty should not inject a synthetic `Ctrl+A`; tmux receives the raw control key directly.
- Treat `visualizer/src/main.py`, `visualizer/src/chafa_pipeline.py`, `visualizer/src/info_panel.py`, `visualizer/src/renderer.py`, and `visualizer/src/terminal.py` as stable unless there is a clear bug.
- Treat `visualizer/src/layout.py` and `visualizer/src/vines_engine.py` as the future seam for outside vines code.

## Runtime Difference

- `chafa/chafa_lab.sh` supports both repo and installed asset layouts.
- The dual-path behavior is expected and should not be “fixed” unless the install layout changes.
