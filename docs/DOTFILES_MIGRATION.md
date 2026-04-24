# Dotfiles Migration

The terminal-startup assets that used to live in the older `yam` checkout have been moved out of this repo.

## Moved out

- `ghostty/`
- `fastfetch/`
- `kitty/`
- `bin/fastfetch-chafa`
- `chafa/`
- the related startup bridge docs

## Why

- keep this repository focused on the current Rust application and its docs
- keep shell, terminal, and startup customization in a dedicated dotfiles repository
- reduce cross-coupling between app development and workstation bootstrap assets

## What remains here

- `yam-rust`
- `yam-install`
- the runtime helper modules
- `docs/`

## Notes

- if you need the old startup scene, retrieve it from the dotfiles repo rather than this project
- keep this document short and update it only when the split changes
