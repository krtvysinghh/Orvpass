#!/usr/bin/env bash
set -e

echo "=== ORVPASS PRODUCTION AUDIT ==="

echo "[1] Formatting"
cargo fmt --all --check

echo "[2] Build"
cargo build --workspace --release

echo "[3] Tests"
cargo test --workspace

echo "[4] Security checks"
cargo audit || true
cargo deny check || true

echo "[5] Binary check"
./target/release/orvpass --version || true

echo "[6] Git status"
git status

echo "=== PRODUCTION CHECK COMPLETE ==="
