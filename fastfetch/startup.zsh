#!/bin/zsh

# Fastfetch startup hook for interactive Ghostty shells.
# Kept separate from ~/.zshrc so it can be backed up and restored cleanly.

if [[ -o interactive && "${TERM_PROGRAM:-}" == "ghostty" && -z "${TMUX:-}" && -z "${FASTFETCH_BOOTSTRAPPED:-}" ]]; then
  export FASTFETCH_BOOTSTRAPPED=1
  printf '\n\n'
  [[ -x "$HOME/.local/bin/fastfetch-chafa" ]] && "$HOME/.local/bin/fastfetch-chafa"
  printf '\n'
fi
