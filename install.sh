#!/bin/zsh

set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"

mkdir -p "$HOME/.config/kitty" "$HOME/.config/fastfetch" "$HOME/.local/bin" "$HOME/.local/share/fastfetch-chafa"
mkdir -p "$HOME/.local/share/fastfetch-chafa/assets"
mkdir -p "$HOME/.local/share/yam-visualizer"

cp "$ROOT/kitty/kitty.conf" "$HOME/.config/kitty/kitty.conf"
cp "$ROOT/kitty/current-theme.conf" "$HOME/.config/kitty/current-theme.conf"
cp "$ROOT/fastfetch/config.jsonc" "$HOME/.config/fastfetch/config.jsonc"
cp "$ROOT/fastfetch/startup.zsh" "$HOME/.config/fastfetch/startup.zsh"
cp "$ROOT/bin/fastfetch-chafa" "$HOME/.local/bin/fastfetch-chafa"
cp "$ROOT/bin/yam" "$HOME/.local/bin/yam"
cp "$ROOT/chafa/chafa_lab.sh" "$HOME/.local/share/fastfetch-chafa/chafa_lab.sh"
cp "$ROOT/assets/ives_yam.png" "$HOME/.local/share/fastfetch-chafa/assets/ives_yam.png"
rm -rf "$HOME/.local/share/yam-visualizer/.venv" \
       "$HOME/.local/share/yam-visualizer/src/__pycache__" \
       "$HOME/.local/share/yam-visualizer/assets/frames_raw" \
       "$HOME/.local/share/yam-visualizer/assets/frames_chafa" \
       "$HOME/.local/share/yam-visualizer/assets/source.gif" \
       "$HOME/.local/share/yam-visualizer/visualizer"
cp -R "$ROOT/visualizer/." "$HOME/.local/share/yam-visualizer/"

chmod +x "$HOME/.local/bin/fastfetch-chafa" \
         "$HOME/.local/bin/yam" \
         "$HOME/.local/share/fastfetch-chafa/chafa_lab.sh" \
         "$HOME/.local/share/yam-visualizer/run_visualizer.sh" \
         "$HOME/.config/fastfetch/startup.zsh"

python3 -m venv "$HOME/.local/share/yam-visualizer/.venv"
source "$HOME/.local/share/yam-visualizer/.venv/bin/activate"
python3 -m pip install -r "$HOME/.local/share/yam-visualizer/requirements.txt"

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
  3. Logo source image is installed to:
     ~/.local/share/fastfetch-chafa/assets/ives_yam.png
  4. Reopen Kitty.
  5. Launch the visualizer any time with:
     yam
     By default this now prefers ~/yam/visualizer when that repo exists.

Repo source of truth:
  ~/yam
EOF
