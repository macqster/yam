#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "Running full verification..."

bash scripts/check-docs.sh
bash scripts/check.sh
cargo test --quiet

echo "Full verification passed."
