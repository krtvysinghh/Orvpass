#!/bin/bash
set -e

./target/release/cli --version >/dev/null

echo "FINAL SMOKE PASS"
