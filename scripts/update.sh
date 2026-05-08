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

run_cargo_with_offline_fallback() {
  local description="$1"
  shift

  echo "[yam] ${description} (offline-first)..."
  if cargo "$@" --offline; then
    return 0
  fi

  echo "[yam] offline path unavailable; retrying with network..."
  cargo "$@"
}

cd "$ROOT"

run_cargo_with_offline_fallback "checking build" check --locked
run_cargo_with_offline_fallback "rebuilding + reinstalling" install --path . --force --locked
echo "[yam] installing launcher wrappers..."
install_wrappers
echo "[yam] done."
