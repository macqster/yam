#!/usr/bin/env bash
set -euo pipefail

# Drives the real release binary in a detached tmux session and prints the
# final rendered pane as text, for manual visual verification of interactive
# changes (see docs/hygiene.md's tmux verification rule). Not a regression
# test — write a real automated test if the same visual claim needs to be
# reasserted later.
#
# Usage: scripts/tmux-smoke.sh [--delay SECONDS] KEY [KEY...]
#
# The first KEY (normally " " to dismiss the boot splash) is sent only after
# an initial wait long enough to clear the boot animation's own Coalesce
# (1s) + Bar (3s) phases (src/ui/state.rs's BOOT_COALESCE/BOOT_BAR) — the
# space-to-continue keypress is silently dropped by the input guard until
# the loading state actually reaches AwaitStart, so sending it earlier does
# nothing and looks like a hang rather than an error. Every subsequent KEY
# is sent via `tmux send-keys` with a fixed --delay between sends (default
# 1.5s once past boot — deliberately generous, since two keys that gate on
# each other's state, e.g. `d` then `w`, can drop the second one if it's
# sent before the first key's UI-state mutation and re-render actually
# land). Use tmux's own key names for special keys ("Escape", "Enter", " ")
# and literal characters for everything else ("d", "w", "i").
#
# Example: boot, dismiss the loading screen, enter dev mode, cycle to
# Greenhouse twice, open the inspection popup, and print the result:
#   scripts/tmux-smoke.sh " " "d" "w" "w" "i"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

boot_wait="4.5"
delay="1.5"
if [[ "${1:-}" == "--delay" ]]; then
  delay="$2"
  shift 2
fi

if [[ $# -eq 0 ]]; then
  echo "usage: scripts/tmux-smoke.sh [--delay SECONDS] KEY [KEY...]" >&2
  exit 1
fi

bin="target/release/yam-rust"
if [[ ! -x "$bin" ]]; then
  echo "error: $bin not found or not executable; run 'cargo build --release' first" >&2
  exit 1
fi

session="yam-smoke-$$"
tmux new-session -d -s "$session" -x 200 -y 50 "./$bin"
trap 'tmux kill-session -t "$session" 2>/dev/null || true' EXIT

sleep "$boot_wait"
for key in "$@"; do
  tmux send-keys -t "$session" "$key"
  sleep "$delay"
done

tmux capture-pane -t "$session" -p
