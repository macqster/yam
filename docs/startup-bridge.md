# Startup Bridge

This document explains how the `yam` startup scene is wired end to end.

## Execution chain

1. The shell starts in Ghostty.
2. The tracked shell profile sources `~/.config/fastfetch/startup.zsh`.
3. The startup hook checks that the session is interactive, inside Ghostty, and not already bootstrapped.
4. The hook calls `~/.local/bin/fastfetch-chafa`.
5. `fastfetch-chafa` renders the Chafa logo into a temporary raw-logo file.
6. Fastfetch reads `~/.config/fastfetch/config.jsonc` and renders the system panel beside the logo.
7. Control returns to the shell prompt.

## File map

| Repo file | Installed target | Role |
| --- | --- | --- |
| `fastfetch/startup.zsh` | `~/.config/fastfetch/startup.zsh` | Startup gate and invocation point |
| `bin/fastfetch-chafa` | `~/.local/bin/fastfetch-chafa` | Raw logo renderer and Fastfetch launcher |
| `chafa/chafa_lab.sh` | `~/.local/share/fastfetch-chafa/chafa_lab.sh` | Chafa render study and logo generator |
| `fastfetch/config.jsonc` | `~/.config/fastfetch/config.jsonc` | Fastfetch panel layout |

## Notes

- The bridge is intentionally one-way: shell profile -> startup hook -> renderer -> Fastfetch
- Ghostty does not own this behavior directly; it only provides the terminal environment
- If the startup scene changes, update this document along with `fastfetch/README.md` and the top-level `README.md`

