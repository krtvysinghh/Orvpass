#!/bin/bash
set -e

echo "=== ORVPASS V1 FINAL CHUNK 32: VERSION FREEZE ==="

echo "[1/5] Update package versions"

sed -i '' 's/version = "0.1.0"/version = "1.0.0"/g' Cargo.toml
sed -i '' 's/version = "0.0.0"/version = "1.0.0"/g' cli/Cargo.toml
sed -i '' 's/version = "0.1.0"/version = "1.0.0"/g' core/Cargo.toml

echo "[2/5] Update lockfile"

cargo update

echo "[3/5] Format"

cargo fmt --all

echo "[4/5] Full validation"

cargo test --workspace
cargo build --release
cargo clippy --workspace -- -D warnings

echo "[5/5] Commit"

git add -A
git commit -m "release: freeze version 1.0.0"

git log -1 --oneline

echo "=== CHUNK 32 COMPLETE ==="
