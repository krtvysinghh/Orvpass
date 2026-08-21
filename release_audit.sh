#!/bin/bash
set -e

echo "=== ORVPASS RELEASE AUDIT ==="

git status --short

cargo test --workspace

cargo build --release

cargo clippy --workspace -- -D warnings

echo "AUDIT PASS"
