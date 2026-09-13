#!/usr/bin/env bash
# Fast test tier (§8.8): fmt check, clippy -D warnings, unit + integration
# tests. The pre-commit gate: nothing commits with this red.
set -euo pipefail
cd "$(dirname "$0")/.."

echo "== AURORA fast tier =="
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
echo "== fast tier green =="
