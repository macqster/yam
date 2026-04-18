#!/bin/zsh

set -euo pipefail
unsetopt xtrace 2>/dev/null || true
setopt typesetsilent 2>/dev/null || true

ROOT="$(cd "$(dirname "$0")" && pwd)"
RECIPE_NAME="${1:-}"

shift || true

if [[ -z "$RECIPE_NAME" ]]; then
  echo "usage: $0 <recipe-name>" >&2
  echo "available recipes:" >&2
  for recipe in "$ROOT"/recipes/*.json; do
    [[ -e "$recipe" ]] || continue
    basename "${recipe%.json}" >&2
  done
  exit 1
fi

RECIPE_PATH="$ROOT/recipes/$RECIPE_NAME.json"
BASE_CONFIG="$ROOT/config/visualizer.json"

if [[ ! -f "$RECIPE_PATH" ]]; then
  echo "unknown recipe: $RECIPE_NAME" >&2
  echo "available recipes:" >&2
  for recipe in "$ROOT"/recipes/*.json; do
    [[ -e "$recipe" ]] || continue
    basename "${recipe%.json}" >&2
  done
  exit 1
fi

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required" >&2
  exit 1
fi

if ! command -v chafa >/dev/null 2>&1; then
  echo "chafa is required" >&2
  exit 1
fi

if [[ -f "$ROOT/.venv/bin/activate" ]]; then
  source "$ROOT/.venv/bin/activate"
fi

TMP_CONFIG="$(mktemp -t yam-visualizer-recipe)"
cleanup() {
  rm -f "$TMP_CONFIG"
}
trap cleanup EXIT INT TERM

python3 - "$BASE_CONFIG" "$RECIPE_PATH" "$TMP_CONFIG" <<'PY'
from __future__ import annotations

import json
import sys
from pathlib import Path

base_path = Path(sys.argv[1])
recipe_path = Path(sys.argv[2])
out_path = Path(sys.argv[3])

base = json.loads(base_path.read_text(encoding="utf-8"))
recipe = json.loads(recipe_path.read_text(encoding="utf-8"))

def merge(target: dict, overlay: dict) -> dict:
    for key, value in overlay.items():
        if key.startswith("_"):
            continue
        if isinstance(value, dict) and isinstance(target.get(key), dict):
            merge(target[key], value)
        else:
            target[key] = value
    return target

merged = merge(base, recipe)
out_path.write_text(json.dumps(merged, indent=2) + "\n", encoding="utf-8")
PY

export YAM_VISUALIZER_CONFIG_PATH="$TMP_CONFIG"
exec python3 "$ROOT/src/main.py" "$@"
