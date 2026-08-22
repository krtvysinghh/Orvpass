#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

echo "Building Orvpass..."

cargo build --release

mkdir -p release/artifacts

cp target/release/orvpass release/artifacts/ 2>/dev/null || true

echo "Build complete"
