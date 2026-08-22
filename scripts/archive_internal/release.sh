#!/bin/bash
set -e

cargo fmt --all
cargo test --workspace
cargo build --release

echo "Release build complete"

