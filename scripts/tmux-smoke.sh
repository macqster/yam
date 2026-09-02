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
auto_start=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --delay) delay="$2"; shift 2 ;;
    # Launch with YAM's own auto-start policy instead of sending a Space.
    # Nothing is injected: the runtime acknowledges its own boot prompt, so
    # this path never has to guess when AwaitStart has been reached.
    --auto-start) auto_start=1; shift ;;
    --) shift; break ;;
    *) break ;;
  esac
done

# A cold hero cache makes boot far slower than the animation phases alone:
# the runtime shells out to `chafa` once per source frame (64 for the current
# hero) before the loading screen can finish, which takes several seconds.
# Waiting only for the boot animation in that case captures the loading
# screen instead of the scene and reads as a hang. Detect the cold-cache case
# up front and extend the initial wait rather than making every caller
# remember to pass a bigger --delay.
hero_cache_dir="${XDG_CACHE_HOME:-$HOME/.cache}/yam"
if ! compgen -G "$hero_cache_dir/*.frame_cache.json" >/dev/null; then
  boot_wait="20"
  echo "note: no hero frame cache in $hero_cache_dir; waiting ${boot_wait}s for a cold chafa rebuild" >&2
fi

# Keys are optional under --auto-start: capturing the first world with no
# input at all is exactly what that mode is for.
if [[ $# -eq 0 && "$auto_start" -eq 0 ]]; then
  echo "usage: scripts/tmux-smoke.sh [--delay SECONDS] [--auto-start] KEY [KEY...]" >&2
  exit 1
fi

bin="target/release/yam-rust"
if [[ ! -x "$bin" ]]; then
  echo "error: $bin not found or not executable; run 'cargo build --release' first" >&2
  exit 1
fi

session="yam-smoke-$$"
launch="./$bin"
[[ "$auto_start" -eq 1 ]] && launch="./$bin --auto-start"
tmux new-session -d -s "$session" -x 200 -y 50 "$launch"
trap 'tmux kill-session -t "$session" 2>/dev/null || true' EXIT

if [[ "$auto_start" -eq 1 ]]; then
  # Poll for the footer rather than sleeping a fixed interval. Auto-start
  # removes the keypress but not the variable part of boot: a cold hero cache
  # still adds several seconds of chafa work before the phases even begin, so a
  # hard-coded wait would be either flaky or needlessly slow.
  deadline=$((SECONDS + 60))
  until tmux capture-pane -t "$session" -p 2>/dev/null | grep -qF '[q]uit'; do
    if (( SECONDS >= deadline )); then
      echo "error: auto-start did not reach the first world within 60s" >&2
      tmux capture-pane -t "$session" -p >&2
      exit 1
    fi
    sleep 1
  done
else
  sleep "$boot_wait"
fi
for key in "$@"; do
  tmux send-keys -t "$session" "$key"
  sleep "$delay"
done

tmux capture-pane -t "$session" -p
