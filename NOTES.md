# Notes

## Source of truth

`~/yam` is the source of truth.

Installed copies under `~/.config` and `~/.local` are runtime files, not the canonical copy.

## Sync rule

Preferred workflow:
1. Edit files in `~/yam`
2. Run `./install.sh`
3. Reopen Kitty and verify
4. Commit after the live setup looks right

If you edit a live file directly, copy that change back into `~/yam` before committing.

## Expected repo/runtime difference

`chafa/chafa_lab.sh` uses different default asset paths in the repo and installed copies:
- repo: `../assets/ives_yam.png`
- runtime: `assets/ives_yam.png`

That difference is expected and should not be “fixed” unless the install layout changes.
