#!/usr/bin/env bash
set -euo pipefail

echo "Running checks..."
cargo fmt --check
cargo clippy -- -D warnings
cargo check
echo "All checks passed."
