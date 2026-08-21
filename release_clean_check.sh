#!/bin/bash
set -e

echo "=== RELEASE CLEAN CHECK ==="

git status --short

cargo fmt --all --check

echo "CLEAN CHECK PASS"
