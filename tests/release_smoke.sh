#!/bin/bash
set -e

BIN="./target/release/orvpass"

test -x "$BIN"

echo "binary ok"

"$BIN" --help >/dev/null

echo "cli ok"
