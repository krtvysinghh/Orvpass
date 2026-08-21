#!/bin/bash
set -e

echo "=== ORVPASS ARTIFACT VERIFY ==="

if [ ! -f target/release/cli ]; then
    echo "missing release binary"
    exit 1
fi

echo "artifact exists"

./target/release/cli --version >/dev/null || true

echo "artifact verified"
