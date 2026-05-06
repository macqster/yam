#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN_DIR="$HOME/.local/bin"

install_wrappers() {
  mkdir -p "$BIN_DIR"
  cp "$ROOT/bin/yam" "$BIN_DIR/yam"
  cp "$ROOT/bin/yam-sandbox" "$BIN_DIR/yam-sandbox"
  chmod +x "$BIN_DIR/yam" "$BIN_DIR/yam-sandbox"
}

echo "[yam] updating dependencies..."
cargo update
echo "[yam] checking build..."
cargo check
echo "[yam] rebuilding + reinstalling..."
cargo install --path . --force --locked
echo "[yam] installing launcher wrappers..."
install_wrappers
echo "[yam] done."
