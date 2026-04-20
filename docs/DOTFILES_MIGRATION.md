# Dotfiles Migration

The terminal-startup assets that used to live in this repo have been moved out of `yam`.

## Moved out

- `ghostty/`
- `fastfetch/`
- `kitty/`
- `bin/fastfetch-chafa`
- `chafa/`
- the related startup bridge docs

## Why

- keep `yam` focused on the v1/v2 application and its docs
- keep shell, terminal, and startup customization in a dedicated dotfiles repository
- reduce cross-coupling between app development and workstation bootstrap assets

## What remains here

- `bin/yam`
- `v1/`
- `v2/`
- `docs/v2/`

## Notes

- if you need the old startup scene, retrieve it from the dotfiles repo rather than this project
- keep this document short and update it only when the split changes
