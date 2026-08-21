#!/bin/bash
set -e

cargo test --workspace
cargo build --release

echo "ORVPASS V1 SMOKE PASS"
