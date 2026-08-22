#!/bin/bash
set -e

echo "=== SECURITY RELEASE CHECK ==="

cargo test --workspace security

echo "SECURITY CHECK PASS"
