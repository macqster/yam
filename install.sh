#!/bin/zsh

set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"

mkdir -p "$HOME/.config/kitty" "$HOME/.config/fastfetch" "$HOME/.local/bin" "$HOME/.local/share/fastfetch-chafa"

cp "$ROOT/kitty/kitty.conf" "$HOME/.config/kitty/kitty.conf"
cp "$ROOT/kitty/current-theme.conf" "$HOME/.config/kitty/current-theme.conf"
cp "$ROOT/fastfetch/config.jsonc" "$HOME/.config/fastfetch/config.jsonc"
cp "$ROOT/fastfetch/startup.zsh" "$HOME/.config/fastfetch/startup.zsh"
cp "$ROOT/bin/fastfetch-chafa" "$HOME/.local/bin/fastfetch-chafa"
cp "$ROOT/chafa/chafa_lab.sh" "$HOME/.local/share/fastfetch-chafa/chafa_lab.sh"

chmod +x "$HOME/.local/bin/fastfetch-chafa" "$HOME/.local/share/fastfetch-chafa/chafa_lab.sh" "$HOME/.config/fastfetch/startup.zsh"

if ! grep -Fq 'source "$HOME/.config/fastfetch/startup.zsh"' "$HOME/.zshrc" 2>/dev/null; then
  printf '\n[[ -f "$HOME/.config/fastfetch/startup.zsh" ]] && source "$HOME/.config/fastfetch/startup.zsh"\n' >> "$HOME/.zshrc"
fi

cat <<'EOF'
Installed Kitty + Fastfetch + Chafa startup bundle.

Next steps:
  1. Ensure dependencies are installed:
     brew install kitty fastfetch chafa
  2. Ensure the font exists:
     JetBrainsMono Nerd Font
  3. Edit the logo source image in:
     ~/.local/share/fastfetch-chafa/chafa_lab.sh
  4. Reopen Kitty.

Repo source of truth:
  ~/yam
EOF
