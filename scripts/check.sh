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
if rg -n "project_world_to_screen" src -g "*.rs" -g "!src/scene/coords.rs"; then
  echo "project_world_to_screen must stay isolated in scene/coords.rs compatibility code" >&2
  exit 1
fi
if rg -n "crate::scene::coords" src -g "*.rs" -g "!src/scene/coords.rs"; then
  echo "scene::coords compatibility aliases must stay isolated in scene/coords.rs" >&2
  exit 1
fi
cargo fmt --check
cargo clippy -- -D warnings
cargo check
echo "All checks passed."
