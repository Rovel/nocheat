#!/bin/bash
set -e

# Create output directory
mkdir -p target/release-builds

# Build for Linux (x86_64, static linking with musl)
echo "Building for Linux (x86_64)..."
cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/libnocheat.so target/release-builds/libnocheat_linux_x86_64.so

# Build for Windows (x86_64)
echo "Building for Windows (x86_64)..."
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/nocheat.dll target/release-builds/nocheat_windows_x86_64.dll

# Build for macOS (x86_64)
echo "Building for macOS (x86_64)..."
cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/libnocheat.dylib target/release-builds/libnocheat_macos_x86_64.dylib

# Build for macOS (ARM64/M1)
echo "Building for macOS (ARM64/M1)..."
cargo build --release --target aarch64-apple-darwin
cp target/aarch64-apple-darwin/release/libnocheat.dylib target/release-builds/libnocheat_macos_arm64.dylib

echo "All builds completed successfully!"
echo "Binaries are available in target/release-builds/"