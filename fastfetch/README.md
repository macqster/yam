# Fastfetch

This directory holds the Fastfetch half of the `yam` startup scene.

Fastfetch is responsible for the visible system-info panel that appears after the Ghostty shell starts.
It is intentionally Ghostty-only and does not run inside tmux.

## Source of truth

- Repo config: `~/_git/yam/fastfetch/config.jsonc`
- Repo startup hook: `~/_git/yam/fastfetch/startup.zsh`
- Installed config: `~/.config/fastfetch/config.jsonc`
- Installed startup hook: `~/.config/fastfetch/startup.zsh`

## What each file does

### `config.jsonc`

Owns:

- the panel layout
- custom separators and framing
- the module order
- the commands used for the custom hardware and disk lines
- the visual tone of the startup panel

Edit this file when you want to change:

- which system facts are shown
- how the panel is grouped
- the spacing and framing of the startup output

### `startup.zsh`

Owns:

- when the Fastfetch scene runs
- the shell and terminal guards around startup
- the small blank-line padding before and after the render
- the call into `~/.local/bin/fastfetch-chafa`

Edit this file when you want to change:

- whether the startup panel runs in a given shell
- whether it should be limited to Ghostty
- how much spacing appears around the scene

## Runtime contract

The startup path is intentionally simple:

1. Ghostty starts a shell
2. the shell sources `~/.config/fastfetch/startup.zsh`
3. the hook checks that the shell is interactive, inside Ghostty, and not already bootstrapped
4. the hook calls `~/.local/bin/fastfetch-chafa`
5. Fastfetch renders the panel with the repo-tracked config

## Notes

- The Fastfetch layer does not own Ghostty behavior
- The Fastfetch layer does not own the visualizer app
- The panel is meant to be readable and stable, not maximally dynamic
- If you change the startup policy, keep the Ghostty-only and tmux-skip rule explicit in both this README and `startup.zsh`
