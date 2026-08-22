#!/bin/bash
set -e

echo "=== ORVPASS FINAL PIPELINE ==="

cargo fmt --all
cargo test --workspace
cargo build --release
cargo clippy --workspace -- -D warnings

./target/release/cli --version >/dev/null || true

echo "FINAL PIPELINE PASS"
