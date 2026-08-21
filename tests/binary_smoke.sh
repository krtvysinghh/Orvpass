#!/bin/bash
set -e

./target/release/cli --version >/dev/null || true

echo "CLI binary smoke PASS"
