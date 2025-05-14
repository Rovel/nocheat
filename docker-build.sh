#!/bin/bash
set -e

echo "Building Docker image for cross-compilation..."
docker build -t nocheat-builder .

echo "Running cross-compilation in Docker container..."
docker run --rm -v "$(pwd)":/app nocheat-builder

echo "Build process completed. Check the target/release-builds/ directory for all platform binaries."