#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/../.."

mkdir -p release

shasum -a 256 release/artifacts/* > release/SHA256SUMS
