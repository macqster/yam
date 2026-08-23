#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
expected_hooks_path="scripts/git-hooks"

cd "$repo_root"

mode="install"
if [[ "${1:-}" == "--check" ]]; then
  mode="check"
elif [[ $# -gt 0 ]]; then
  echo "usage: $0 [--check]" >&2
  exit 2
fi

current_hooks_path="$(git config --local --get core.hooksPath || true)"

if [[ "$mode" == "check" ]]; then
  if [[ "$current_hooks_path" != "$expected_hooks_path" ]]; then
    echo "pre-push hook is not enabled for this clone (core.hooksPath=${current_hooks_path:-unset})" >&2
    exit 1
  fi
  [[ -x "$expected_hooks_path/pre-push" ]] || {
    echo "configured hook is missing or not executable: $expected_hooks_path/pre-push" >&2
    exit 1
  }
  echo "pre-push hook enabled for this clone: $current_hooks_path/pre-push"
  exit 0
fi

if [[ -n "$current_hooks_path" && "$current_hooks_path" != "$expected_hooks_path" ]]; then
  echo "refusing to overwrite this clone's existing core.hooksPath: $current_hooks_path" >&2
  echo "inspect it and change it deliberately with git config --local if appropriate" >&2
  exit 1
fi

[[ -x "$expected_hooks_path/pre-push" ]] || {
  echo "tracked pre-push hook is missing or not executable: $expected_hooks_path/pre-push" >&2
  exit 1
}

git config --local core.hooksPath "$expected_hooks_path"
"$repo_root/scripts/install-hooks.sh" --check
