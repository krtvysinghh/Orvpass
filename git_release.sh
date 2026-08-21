#!/bin/bash
set -e

git status
git add .
git commit -m "Orvpass V1 RC1"
echo "Ready for release"

