#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN_DIR="$HOME/.local/bin"

diagnostics_enabled() {
  local value="${YAM_DIAGNOSTICS:-}"
  value="$(printf '%s' "$value" | tr '[:upper:]' '[:lower:]')"
  [[ -n "$value" && "$value" != "0" && "$value" != "false" && "$value" != "off" ]]
}

diagnostics_path() {
  if [[ -n "${YAM_DIAGNOSTICS_PATH:-}" ]]; then
    printf '%s\n' "$YAM_DIAGNOSTICS_PATH"
    return 0
  fi
  if [[ -n "${XDG_STATE_HOME:-}" ]]; then
    printf '%s\n' "$XDG_STATE_HOME/yam/diagnostics.ndjson"
    return 0
  fi
  printf '%s\n' "$HOME/.local/state/yam/diagnostics.ndjson"
}

ensure_diagnostics_session() {
  if [[ -z "${YAM_DIAGNOSTICS_SESSION:-}" ]]; then
    export YAM_DIAGNOSTICS_SESSION="$(python3 - <<'PY'
from datetime import datetime
import os
print(f"{datetime.now().astimezone().isoformat(timespec='milliseconds')}-{os.getpid()}")
PY
)"
  fi
}

append_diagnostics_event() {
  local kind="$1"
  shift

  diagnostics_enabled || return 0
  ensure_diagnostics_session

  local path
  path="$(diagnostics_path)"
  mkdir -p "$(dirname "$path")"

  python3 - "$path" "$kind" "$YAM_DIAGNOSTICS_SESSION" "$$" "$@" <<'PY'
import json
import sys
from datetime import datetime

path, kind, session, pid, *pairs = sys.argv[1:]
payload = {
    "ts": datetime.now().astimezone().isoformat(timespec="seconds"),
    "session": session,
    "pid": int(pid),
    "kind": kind,
}
for pair in pairs:
    key, value = pair.split("=", 1)
    if value.isdigit():
      payload[key] = int(value)
    elif value in ("true", "false"):
      payload[key] = value == "true"
    else:
      payload[key] = value
with open(path, "a", encoding="utf-8") as handle:
    handle.write(json.dumps(payload) + "\n")
PY
}

now_ms() {
  python3 - <<'PY'
import time
print(int(time.time() * 1000))
PY
}

install_wrappers() {
  mkdir -p "$BIN_DIR"
  cp "$ROOT/bin/yam" "$BIN_DIR/yam"
  cp "$ROOT/bin/yam-sandbox" "$BIN_DIR/yam-sandbox"
  cp "$ROOT/bin/yam-install" "$BIN_DIR/yam-install"
  cp "$ROOT/bin/yam-diagnostics" "$BIN_DIR/yam-diagnostics"
  chmod +x "$BIN_DIR/yam" "$BIN_DIR/yam-sandbox" "$BIN_DIR/yam-install" "$BIN_DIR/yam-diagnostics"
}

run_cargo_with_offline_fallback() {
  local description="$1"
  shift

  local start_ms
  start_ms="$(now_ms)"
  echo "[yam] ${description} (offline-first)..."
  if cargo "$@" --offline; then
    local end_ms
    end_ms="$(now_ms)"
    append_diagnostics_event \
      install_step \
      description="$description" \
      mode=offline-first \
      duration_ms="$((end_ms - start_ms))"
    return 0
  fi

  echo "[yam] offline path unavailable; retrying with network..."
  start_ms="$(now_ms)"
  cargo "$@"
  local end_ms
  end_ms="$(now_ms)"
  append_diagnostics_event \
    install_step \
    description="$description" \
    mode=network-fallback \
    duration_ms="$((end_ms - start_ms))"
}

cd "$ROOT"

append_diagnostics_event \
  install_start \
  version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)" \
  root="$ROOT"

run_cargo_with_offline_fallback "checking build" check --locked
run_cargo_with_offline_fallback "rebuilding + reinstalling" install --path . --force --locked
echo "[yam] installing launcher wrappers..."
install_wrappers
echo "[yam] done."
append_diagnostics_event install_done wrappers=true
