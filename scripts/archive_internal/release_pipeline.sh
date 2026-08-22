#!/bin/bash
set -e

echo "ORVPASS RELEASE PIPELINE"

cargo fmt --all
cargo test --workspace
cargo build --release
cargo clippy --workspace -- -D warnings

echo "PIPELINE PASS"
