#!/bin/bash
set -e

BASE=$1
echo "🚀 Running Test Impact Analysis against base: $BASE"

# Detect OS and Arch
OS=$(uname -s)
ARCH=$(uname -m)

if [ "$OS" == "Linux" ]; then
    TARGET="x86_64-unknown-linux-gnu"
    EXT="tar.gz"
elif [ "$OS" == "Darwin" ]; then
    if [ "$ARCH" == "arm64" ]; then
        TARGET="aarch64-apple-darwin"
    else
        TARGET="x86_64-apple-darwin"
    fi
    EXT="tar.gz"
else
    # Assume Windows
    TARGET="x86_64-pc-windows-msvc"
    EXT="zip"
fi

echo "🔍 Detected target architecture: $TARGET"
DOWNLOAD_URL="https://github.com/dhia-bechattaoui/smart-diff-ci/releases/latest/download/smart-diff-ci-${TARGET}.${EXT}"

echo "📥 Downloading pre-compiled binary from $DOWNLOAD_URL"
curl -sL "$DOWNLOAD_URL" -o "smart-diff-ci.${EXT}"

if [ "$EXT" == "tar.gz" ]; then
    tar xzf "smart-diff-ci.tar.gz"
else
    unzip -q "smart-diff-ci.zip"
fi

chmod +x smart-diff-ci

echo "⚙️ Executing smart-diff-ci..."
./smart-diff-ci analyze --base "$BASE"
