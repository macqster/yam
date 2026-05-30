#!/usr/bin/env bash
set -euo pipefail

echo "Running docs checks..."

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

shopt -s nullglob

active_docs=(
  "AGENTS.md"
  "README.md"
  "TODO.md"
  "known_issues.md"
  docs/*.md
  skills/*/SKILL.md
)
skill_dirs=(skills/*)
skill_agent_files=(skills/*/agents/openai.yaml)

for path in "${active_docs[@]}"; do
  if [[ ! -f "$path" ]]; then
    echo "Missing active doc: $path" >&2
    exit 1
  fi
done

for skill_dir in "${skill_dirs[@]}"; do
  [[ -d "$skill_dir" ]] || continue
  skill_md="$skill_dir/SKILL.md"
  skill_yaml="$skill_dir/agents/openai.yaml"
  if [[ ! -f "$skill_md" ]]; then
    echo "Missing skill file: $skill_md" >&2
    exit 1
  fi
  skill_name="$(sed -n 's/^name: \(.*\)$/\1/p' "$skill_md" | head -n 1 | tr -d '"')"
  if [[ -z "$skill_name" ]]; then
    echo "Missing skill frontmatter name: $skill_md" >&2
    exit 1
  fi
  if [[ ! "$skill_name" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]]; then
    echo "Skill name must be lowercase hyphen-case: $skill_md" >&2
    exit 1
  fi
  if [[ "$skill_name" != "$(basename "$skill_dir")" ]]; then
    echo "Skill name mismatch: $skill_dir has name $skill_name" >&2
    exit 1
  fi
  skill_description="$(sed -n 's/^description: \(.*\)$/\1/p' "$skill_md" | head -n 1)"
  skill_description="${skill_description%\"}"
  skill_description="${skill_description#\"}"
  if [[ -z "$skill_description" ]]; then
    echo "Missing skill frontmatter description: $skill_md" >&2
    exit 1
  fi
  if (( ${#skill_description} > 1024 )); then
    echo "Skill description must be 1024 chars or fewer: $skill_md" >&2
    exit 1
  fi
  if [[ "$skill_description" == *"<"* || "$skill_description" == *">"* ]]; then
    echo "Skill description must not contain angle-bracket placeholders: $skill_md" >&2
    exit 1
  fi
  if [[ ! -f "$skill_yaml" ]]; then
    echo "Missing skill metadata: $skill_yaml" >&2
    exit 1
  fi
  if ! grep -qx "interface:" "$skill_yaml"; then
    echo "Skill metadata is missing interface section: $skill_yaml" >&2
    exit 1
  fi
  display_name="$(sed -n 's/^  display_name: "\(.*\)"$/\1/p' "$skill_yaml" | head -n 1)"
  short_description="$(sed -n 's/^  short_description: "\(.*\)"$/\1/p' "$skill_yaml" | head -n 1)"
  default_prompt="$(sed -n 's/^  default_prompt: "\(.*\)"$/\1/p' "$skill_yaml" | head -n 1)"
  if [[ -z "$display_name" ]]; then
    echo "Skill metadata is missing display_name: $skill_yaml" >&2
    exit 1
  fi
  if [[ -z "$short_description" ]]; then
    echo "Skill metadata is missing short_description: $skill_yaml" >&2
    exit 1
  fi
  if (( ${#short_description} < 25 || ${#short_description} > 64 )); then
    echo "Skill short_description must be 25-64 chars: $skill_yaml" >&2
    exit 1
  fi
  if [[ -z "$default_prompt" ]]; then
    echo "Skill metadata is missing default_prompt: $skill_yaml" >&2
    exit 1
  fi
  if [[ "$default_prompt" != *"\$$skill_name"* ]]; then
    echo "Skill default_prompt must mention \$$skill_name: $skill_yaml" >&2
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

while IFS= read -r asset_path; do
  [[ -z "$asset_path" ]] && continue
  if [[ ! -e "$asset_path" ]]; then
    echo "README.md references missing local asset: $asset_path" >&2
    exit 1
  fi
done < <(sed -n 's/.*src="\([^"]*\)".*/\1/p' README.md)

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
run_if_available cspell --config .cspell.json "${active_docs[@]}" "${skill_agent_files[@]}"

echo "All docs checks passed."
