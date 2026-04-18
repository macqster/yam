# Ghostty

This directory holds the Ghostty part of `yam`.

The layout is intentionally split into two parts:

- `config` for Ghostty behavior and window/session settings
- `themes/` for the color theme(s) Ghostty loads by name

## Source of truth

- Repo config: `~/_git/yam/ghostty/config`
- Installed config on macOS: `~/Library/Application Support/com.mitchellh.ghostty/config.ghostty`
- Repo themes: `~/_git/yam/ghostty/themes/`
- Installed themes: `~/.config/ghostty/themes/`

## Current setup

- `config` keeps font, spacing, shell integration, and general behavior
- `themes/yam-dark` holds the tracked terminal palette
- the base config uses `theme = yam-dark`
- macOS Option is explicitly treated as Alt so terminal shortcuts can reach tmux and other shell tools
- the live macOS Ghostty config is repo-backed through App Support so XDG and App Support do not fight each other
- tmux uses the raw `Ctrl+A` prefix directly; Ghostty does not synthesize that prefix

## Key layout

Current Ghostty split/navigation bindings are Cmd-based:

- `Cmd+D` split right
- `Cmd+Shift+D` split down
- `Cmd+Alt+Arrow` move between splits
- `Cmd+Ctrl+Arrow` resize splits

The `Option` setting matters for terminal input, but it does not turn the split shortcuts into Option-based shortcuts. If a key combo is missing or inconsistent, prefer Cmd-based bindings or remove the conflicting macOS system shortcut.

## Notes

- Ghostty themes are regular Ghostty config files
- There is no separate Ghostty plugin system here
- Anything shell-related beyond Ghostty itself belongs in the shell profile or the `fastfetch/` layer
- On macOS, App Support is the canonical runtime location for the active config
