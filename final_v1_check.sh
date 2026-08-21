#!/bin/bash
set -e

echo "=== ORVPASS FINAL V1 CHECK ==="

cargo fmt --all
cargo test --workspace
cargo build --release
cargo clippy --workspace -- -D warnings

./target/release/cli --version >/dev/null || true

echo "ALL CHECKS PASSED"
