#!/bin/bash
set -e

echo "=== ORVPASS VERSION ALIGNMENT FIX ==="

echo "[1/4] Update cli dependency"

sed -i '' 's/orvpass-core = "\^0.1.0"/orvpass-core = "1.0.0"/g' cli/Cargo.toml


echo "[2/4] Regenerate lockfile"

cargo update


echo "[3/4] Validate"

cargo fmt --all

cargo test --workspace

cargo build --release

cargo clippy --workspace -- -D warnings


echo "[4/4] Commit checkpoint"

git add -A

git commit -m "fix: align core and cli versions for v1.0.0"

git log -1 --oneline

echo "=== VERSION ALIGNMENT COMPLETE ==="

