#!/bin/bash
set -e

echo "Checking formatting"
cargo fmt --all --check

echo "Running tests"
cargo test --workspace

echo "Building release"
cargo build --release

echo "Release verification passed"

