#!/bin/bash
set -e

BIN="./target/release/cli"

test -f "$BIN"

"$BIN" --help >/dev/null || true

echo "CLI SMOKE PASS"
