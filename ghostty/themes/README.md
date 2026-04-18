# Ghostty themes

Ghostty loads themes from `~/.config/ghostty/themes/` by name.

This repo tracks the active theme files under:

- `~/_git/yam/ghostty/themes/`

Current theme files:

- `yam-dark` - the baseline dark theme used by the shell setup

When adding a new theme:

- keep the file name equal to the theme name you want to use in `config`
- keep the file in this directory so the installer can mirror it into `~/.config/ghostty/themes/`
- prefer theme-only settings here; keep behavior and keybinds in `ghostty/config`

