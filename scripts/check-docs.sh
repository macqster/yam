#!/usr/bin/env bash
set -euo pipefail

echo "Running docs checks..."

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

active_docs=(
  "README.md"
  "TODO.md"
  "known_issues.md"
  "docs/README.md"
  "docs/LOG.md"
  "docs/audit.md"
  "docs/architecture.md"
  "docs/glossary.md"
  "docs/hygiene.md"
  "docs/rendering.md"
  "docs/scene-model.md"
)

for path in "${active_docs[@]}"; do
  if [[ ! -f "$path" ]]; then
    echo "Missing active doc: $path" >&2
    exit 1
  fi
done

crate_version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
if [[ -z "$crate_version" ]]; then
  echo "Could not determine crate version from Cargo.toml" >&2
  exit 1
fi

readme_version="$(sed -n 's/^Current release: `\(.*\)`$/\1/p' README.md | head -n 1)"
if [[ -z "$readme_version" ]]; then
  echo "README.md is missing the canonical Current release line." >&2
  exit 1
fi

if [[ "$crate_version" != "$readme_version" ]]; then
  echo "Version mismatch: Cargo.toml=$crate_version README.md=$readme_version" >&2
  exit 1
fi

todo_issue_ids="$(rg -o 'KI-[0-9]+' TODO.md | sort -u || true)"
known_issue_ids="$(rg -o '^### KI-[0-9]+' known_issues.md | awk '{print $2}' | sort -u || true)"

if [[ -n "$todo_issue_ids" ]]; then
  while IFS= read -r issue_id; do
    [[ -z "$issue_id" ]] && continue
    if ! grep -qx "$issue_id" <<<"$known_issue_ids"; then
      echo "TODO references missing known issue id: $issue_id" >&2
      exit 1
    fi
  done <<<"$todo_issue_ids"
fi

run_if_available() {
  local tool="$1"
  shift
  if command -v "$tool" >/dev/null 2>&1; then
    "$tool" "$@"
  else
    echo "Skipping $tool: not installed in current environment."
  fi
}

run_if_available markdownlint --config .markdownlint.jsonc "${active_docs[@]}"
run_if_available markdownlint-cli2 "${active_docs[@]}"
run_if_available cspell --config .cspell.json "${active_docs[@]}"

echo "All docs checks passed."
