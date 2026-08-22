#!/bin/bash
set -e

cargo fmt --all
cargo test --workspace
cargo build --release
cargo clippy --workspace -- -D warnings

echo "ORVPASS RELEASE CHECK PASS"
