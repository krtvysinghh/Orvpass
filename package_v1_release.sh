#!/bin/bash
set -e

mkdir -p release

cargo build --release

cp target/release/cli release/orvpass

chmod +x release/orvpass

shasum -a 256 release/orvpass > release/SHA256SUMS

echo "Artifact package complete"
