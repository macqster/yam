#!/bin/zsh

# Fastfetch startup hook for interactive Kitty shells.
# Kept separate from ~/.zshrc so it can be backed up and restored cleanly.

if [[ -o interactive && -n "${KITTY_WINDOW_ID:-}" && -z "${TMUX:-}" && -z "${FASTFETCH_BOOTSTRAPPED:-}" ]]; then
  export FASTFETCH_BOOTSTRAPPED=1
  printf '\n\n'
  [[ -x "$HOME/.local/bin/fastfetch-chafa" ]] && "$HOME/.local/bin/fastfetch-chafa"
  printf '\n'
fi
