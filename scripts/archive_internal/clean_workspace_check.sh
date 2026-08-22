#!/bin/bash
set -e

echo "=== CLEAN WORKSPACE CHECK ==="

git diff --exit-code

echo "WORKSPACE CLEAN"
