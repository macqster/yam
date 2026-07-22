#!/usr/bin/env bash
set -euo pipefail

echo "Running checks..."
if rg -n "crate::scene::" src/core -g "*.rs"; then
  echo "core must not depend on scene modules" >&2
  exit 1
fi
if rg -n 'crate::(scene|render|ui)::|ratatui|crossterm' src/systems -g "*.rs"; then
  echo "systems must not depend on scene, render, UI, or terminal modules" >&2
  exit 1
fi
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets
echo "All checks passed."
