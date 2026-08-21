#!/bin/bash
set -e

BIN="./target/release/cli"

test -x "$BIN"

echo "binary ok"

"$BIN" --help >/dev/null

echo "cli ok"
