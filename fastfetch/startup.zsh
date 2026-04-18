#!/bin/zsh

# Fastfetch startup hook for interactive Ghostty shells.
# Kept separate from ~/.zshrc so it can be backed up and restored cleanly.
# This hook is intentionally Ghostty-only and intentionally skips tmux shells.
# The startup screen belongs in the Ghostty terminal session, not inside tmux.

if [[ -o interactive && "${TERM_PROGRAM:-}" == "ghostty" && -z "${TMUX:-}" && -z "${FASTFETCH_BOOTSTRAPPED:-}" ]]; then
  export FASTFETCH_BOOTSTRAPPED=1
  printf '\n\n'
  [[ -x "$HOME/.local/bin/fastfetch-chafa" ]] && "$HOME/.local/bin/fastfetch-chafa"
  printf '\n'
fi
