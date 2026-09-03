#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "Running full verification..."

# The pre-push hook is on by default as of 2026-09-03. It is enabled from here
# rather than left to a setup step, because this script is the one command the
# workflow already asks for on every maintenance batch and so is the earliest
# point a fresh clone reliably reaches.
#
# Deliberately loud. docs/hygiene.md argued that enabling the hook should be an
# explicit choice rather than a silent default; flipping the default does not
# make silent config mutation acceptable, so this says exactly what it changed.
# It never overwrites a core.hooksPath already pointing somewhere else - that is
# a deliberate local setup and the developer's call, so it is reported and left
# alone rather than treated as a verification failure.
#
# Skipped under CI: that checkout is ephemeral and never pushes, so installing a
# hook there would be noise with no effect.
maybe_enable_pre_push_hook() {
  [[ -z "${CI:-}" ]] || return 0
  git rev-parse --git-dir >/dev/null 2>&1 || return 0

  local current
  current="$(git config --local --get core.hooksPath || true)"
  [[ "$current" != "scripts/git-hooks" ]] || return 0

  if [[ -n "$current" ]]; then
    echo "note: this clone sets core.hooksPath=$current; leaving it alone, so the" >&2
    echo "      pre-push hook is not enabled here (see docs/hygiene.md)" >&2
    return 0
  fi

  echo "enabling the pre-push verification hook for this clone (default since 2026-09-03;"
  echo "skip a single push with 'git push --no-verify')"
  bash scripts/install-hooks.sh || echo "note: could not enable the pre-push hook; continuing" >&2
}

maybe_enable_pre_push_hook

bash scripts/check-docs.sh
bash scripts/check.sh
cargo test --quiet

echo "Full verification passed."
