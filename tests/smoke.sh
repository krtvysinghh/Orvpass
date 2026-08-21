#!/bin/bash

cargo test --workspace
cargo build --release

echo "SMOKE OK"

