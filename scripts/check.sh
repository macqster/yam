#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "Running checks..."

# The boundary checks below use `grep` rather than `rg` deliberately.
#
# They were previously written as `if rg …; then fail; fi`, which reads a
# *missing* ripgrep (exit 127) as "no matches" and reports success without
# having inspected a single file. A wrong working directory (exit 2) passed
# the same way. That is the silent-pass failure `scripts/check-docs.sh`
# already guards against for its linters, and it mattered more here: these two
# checks are the only automated enforcement the architecture contract has, so
# a machine without ripgrep got no boundary checking at all while still being
# told "All checks passed." CI only escaped because ubuntu-latest happens to
# ship ripgrep; nothing in the workflow installs it.
#
# grep is in POSIX, so it needs no availability guard, and check_boundary
# distinguishes all three exit states instead of collapsing them into a
# boolean: 0 (matches, so a violation), 1 (clean), anything else (the check
# could not run, which fails loudly rather than passing quietly).
check_boundary() {
  local dir="$1" pattern="$2" message="$3"
  local hits status=0 scanned

  # Establish the directory and the file count up front, independently of grep.
  #
  # grep's exit status alone cannot carry the "could not run" case: BSD grep,
  # the macOS default and so the platform this repo is developed on, exits 1
  # for a *missing* directory - the same code it uses for "no matches" - so a
  # mistyped path, a moved module tree, or a wrong working directory would
  # otherwise read as a clean pass. (GNU grep and ugrep exit 2 there, which is
  # why the status check further down is still worth keeping.) Counting first
  # makes the gate prove it had something to inspect, whichever grep is
  # installed.
  #
  # The directory is tested on its own rather than through `find`'s exit status
  # because under `set -o pipefail` a failing `find` at the head of the count
  # pipeline makes the assignment fail, and `set -e` then kills the script
  # before it can print why - the same silent exit this function exists to stop.
  if [[ ! -d "$dir" ]]; then
    echo "boundary check could not run: $dir is not a directory" >&2
    exit 1
  fi

  scanned="$(find "$dir" -type f -name '*.rs' | wc -l | tr -d '[:space:]')"
  if ((scanned == 0)); then
    echo "boundary check could not run: no .rs files found under $dir" >&2
    exit 1
  fi

  hits="$(grep -rnE "$pattern" "$dir" --include='*.rs')" || status=$?

  if ((status == 0)); then
    printf '%s\n' "$hits" >&2
    echo "$message" >&2
    exit 1
  elif ((status != 1)); then
    echo "boundary check could not run over $dir (grep exited $status)" >&2
    exit 1
  fi

  echo "  $dir: $scanned files checked, no forbidden imports"
}

# `docs/architecture.md` forbids `core -> ui` and `core -> render` alongside
# `core -> scene`, and `src/core/mod.rs` claims no ratatui/crossterm usage, but
# only the scene half was ever enforced. `core/` and `systems/` are held to the
# same upward-dependency rule, so they share one pattern.
#
# ratatui/crossterm are matched through `::` or a `use` statement rather than
# bare, so the "No ratatui/crossterm usage" note in `src/core/mod.rs` is not
# itself reported as a violation.
forbidden_upward='crate::(scene|render|ui)::|(^|[^[:alnum:]_])(ratatui|crossterm)::|use[[:space:]]+(ratatui|crossterm)[[:space:]]*[;{]'

check_boundary src/core "$forbidden_upward" \
  "core must not depend on scene, render, UI, or terminal modules"
check_boundary src/systems "$forbidden_upward" \
  "systems must not depend on scene, render, UI, or terminal modules"

# `render/` owns terminal primitives, so unlike core/systems it may use ratatui
# and crossterm freely - but it sits below scene and UI and must not reach up
# into them. That edge existed until 2026-09-02 (`render/clock.rs` imported
# `UiState`, and `render/render_state.rs` imported scene's `Camera`/`Viewport`)
# and is guarded here so it cannot come back unnoticed.
check_boundary src/render 'crate::(scene|ui)::' \
  "render must not depend on scene or UI modules"

cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets
echo "All checks passed."
