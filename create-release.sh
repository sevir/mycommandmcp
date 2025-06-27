#!/bin/bash
# Custom release script for MyCommandMCP
# This script creates GitHub releases with cross-platform binaries

set -e

echo "🚀 Starting MyCommandMCP Release Process..."

# Get version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo "📦 Version: $VERSION"

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean
rm -rf dist/
mkdir -p dist/

# Setup cross-compilation
echo "🔧 Setting up cross-compilation..."
./setup-cross-compilation.sh

# Build for all platforms
echo "🔨 Building for Linux x86_64..."
cargo build --release --target x86_64-unknown-linux-gnu

echo "🔨 Building for Windows x86_64..."
./build-windows.sh

echo "🔨 Building for macOS Intel..."
cargo build --release --target x86_64-apple-darwin || echo "⚠️  macOS Intel build skipped"

echo "🔨 Building for macOS ARM64..."
cargo build --release --target aarch64-apple-darwin || echo "⚠️  macOS ARM build skipped"

# Copy binaries to dist
echo "📦 Copying binaries to dist..."
cp target/x86_64-unknown-linux-gnu/release/mycommandmcp dist/mycommandmcp_${VERSION}_linux_amd64 || echo "⚠️  Linux binary not found"
cp target/x86_64-pc-windows-gnu/release/mycommandmcp.exe dist/mycommandmcp_${VERSION}_windows_amd64.exe || echo "⚠️  Windows binary not found"
cp target/x86_64-apple-darwin/release/mycommandmcp dist/mycommandmcp_${VERSION}_darwin_amd64 || echo "⚠️  macOS Intel binary not found"
cp target/aarch64-apple-darwin/release/mycommandmcp dist/mycommandmcp_${VERSION}_darwin_arm64 || echo "⚠️  macOS ARM binary not found"

# Create archives
echo "📁 Creating archives..."
cd dist

# Create Linux archive
if [ -f "mycommandmcp_${VERSION}_linux_amd64" ]; then
    tar -czf "mycommandmcp_${VERSION}_linux_amd64.tar.gz" "mycommandmcp_${VERSION}_linux_amd64" ../README.md ../LICENSE.txt ../USAGE.md ../mycommand-tools.yaml ../mycommand-tools-extended.yaml
    echo "✅ Linux archive created"
fi

# Create Windows archive
if [ -f "mycommandmcp_${VERSION}_windows_amd64.exe" ]; then
    zip "mycommandmcp_${VERSION}_windows_amd64.zip" "mycommandmcp_${VERSION}_windows_amd64.exe" ../README.md ../LICENSE.txt ../USAGE.md ../mycommand-tools.yaml ../mycommand-tools-extended.yaml
    echo "✅ Windows archive created"
fi

# Create macOS Intel archive
if [ -f "mycommandmcp_${VERSION}_darwin_amd64" ]; then
    tar -czf "mycommandmcp_${VERSION}_darwin_amd64.tar.gz" "mycommandmcp_${VERSION}_darwin_amd64" ../README.md ../LICENSE.txt ../USAGE.md ../mycommand-tools.yaml ../mycommand-tools-extended.yaml
    echo "✅ macOS Intel archive created"
fi

# Create macOS ARM archive
if [ -f "mycommandmcp_${VERSION}_darwin_arm64" ]; then
    tar -czf "mycommandmcp_${VERSION}_darwin_arm64.tar.gz" "mycommandmcp_${VERSION}_darwin_arm64" ../README.md ../LICENSE.txt ../USAGE.md ../mycommand-tools.yaml ../mycommand-tools-extended.yaml
    echo "✅ macOS ARM archive created"
fi

cd ..

# Generate checksums
echo "🔐 Generating checksums..."
cd dist
sha256sum * > checksums.txt
cd ..

# List created files
echo "📋 Release artifacts created:"
ls -la dist/

echo "🎉 Release process completed!"
echo "💡 To create a GitHub release:"
echo "   1. git tag v${VERSION}"
echo "   2. git push origin v${VERSION}"
echo "   3. Upload files from dist/ to GitHub release"
