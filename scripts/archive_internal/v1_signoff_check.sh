#!/bin/bash
set -e

echo "=== ORVPASS V1 SIGNOFF CHECK ==="

cargo test --workspace
cargo build --release
cargo clippy --workspace -- -D warnings

echo "SIGNOFF PASS"
