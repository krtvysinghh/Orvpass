#!/bin/bash
set -e

echo "=== ARTIFACT CHECK ==="

test -f target/release/cli

echo "Binary exists"

echo "ARTIFACT PASS"
