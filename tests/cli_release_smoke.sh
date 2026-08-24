#!/bin/bash
set -e

BIN="./target/release/orvpass"

test -f "$BIN"

"$BIN" --help >/dev/null

echo "CLI SMOKE PASS"
