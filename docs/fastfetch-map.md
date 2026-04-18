# Fastfetch Map

This document maps the Fastfetch files in `yam` to the runtime files they install.

## Current mapping

| Repo file | Installed target | Purpose |
| --- | --- | --- |
| `fastfetch/config.jsonc` | `~/.config/fastfetch/config.jsonc` | Visible Fastfetch panel layout and module configuration |
| `fastfetch/startup.zsh` | `~/.config/fastfetch/startup.zsh` | Shell hook that decides when the startup scene runs |

## Organization rules

- Keep all Fastfetch-specific startup logic in `fastfetch/startup.zsh`
- Keep the panel appearance in `fastfetch/config.jsonc`
- Keep the Ghostty-specific gate in the startup hook, not in `~/.zshrc`
- Keep the repo copy as the source of truth and the installed copy as a runtime artifact

## Startup path

The current contract is:

1. the shell profile sources `~/.config/fastfetch/startup.zsh` when it exists
2. the startup hook checks that the shell is interactive
3. the hook checks that the terminal is Ghostty
4. the hook avoids running twice in the same session
5. the hook calls `~/.local/bin/fastfetch-chafa`

## Notes

- This layer intentionally lives beside Ghostty, not inside it
- If the startup behavior changes, update this map and `fastfetch/README.md` together
- If the render panel changes, update `config.jsonc` and the higher-level `yam` README together

