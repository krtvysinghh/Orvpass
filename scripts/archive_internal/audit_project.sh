#!/bin/bash

echo "=== ORVPASS AUDIT ==="

cargo test --workspace
cargo clippy --workspace -- -D warnings || true

echo "Audit finished"

