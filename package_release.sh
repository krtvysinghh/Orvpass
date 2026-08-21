#!/bin/bash
set -e

mkdir -p release

cargo build --release

cp target/release/cli release/orvpass

echo "Package ready"

