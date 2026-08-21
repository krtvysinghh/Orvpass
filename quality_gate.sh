#!/bin/bash
set -e

echo "=== ORVPASS QUALITY GATE ==="

cargo test --workspace
cargo build --release
cargo clippy --workspace -- -D warnings

echo "QUALITY GATE PASS"
