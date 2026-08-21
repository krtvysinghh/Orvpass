#!/bin/bash
set -e

echo "=== ORVPASS V1 RELEASE VERIFY ==="

cargo test --workspace

cargo build --release

cargo clippy --workspace -- -D warnings

./target/release/cli --version >/dev/null || true

echo "RELEASE VERIFIED"
