#!/usr/bin/env bash
set -euo pipefail

echo "[yam] updating dependencies..."
cargo update
echo "[yam] checking build..."
cargo check
echo "[yam] rebuilding + reinstalling..."
cargo install --path . --force --locked
echo "[yam] done."
