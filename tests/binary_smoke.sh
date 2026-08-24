#!/bin/bash
set -e

./target/release/orvpass --version >/dev/null

echo "CLI binary smoke PASS"
