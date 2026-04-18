# Chafa Map

This document maps the Chafa renderer pieces used by the `yam` startup scene.

## Current mapping

| Repo file | Installed target | Role |
| --- | --- | --- |
| `bin/fastfetch-chafa` | `~/.local/bin/fastfetch-chafa` | Creates the raw logo and launches Fastfetch |
| `chafa/chafa_lab.sh` | `~/.local/share/fastfetch-chafa/chafa_lab.sh` | Controls the Chafa render parameters |
| `assets/ives_yam.png` | `~/.local/share/fastfetch-chafa/assets/ives_yam.png` | Source image used for the current portrait |

## Organization rules

- Keep startup orchestration in `bin/fastfetch-chafa`
- Keep image rendering parameters in `chafa/chafa_lab.sh`
- Keep the source portrait in `assets/`
- Do not hardcode Desktop or Downloads paths into the live startup path

## Notes

- `fastfetch-chafa` renders a still image for startup use
- `chafa_lab.sh` can still be used for manual experimentation with the same source image
- If the portrait changes, update this map and the `yam` README together

