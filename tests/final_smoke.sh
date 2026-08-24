#!/bin/bash
set -e

./target/release/orvpass --version >/dev/null

echo "FINAL SMOKE PASS"
