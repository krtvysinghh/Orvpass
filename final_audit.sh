#!/bin/bash
set -e

echo "=== ORVPASS FINAL AUDIT ==="

cargo fmt --all --check
cargo test --workspace
cargo build --release

echo "AUDIT PASSED"

