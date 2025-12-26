#!/bin/bash
set -euo pipefail

# Error handler for better CI debugging
trap 'echo "❌ ERROR on line $LINENO: Command failed with exit code $?" >&2' ERR

VERSION="${1:-dev}"

echo "🔨 Building Linux x86_64 binaries (version: $VERSION)..."
echo "::group::Linux Build"

cargo build --release -p ev-server -p ev-etl

echo "::endgroup::"
echo "📦 Creating archives..."

mkdir -p dist

cd target/release

tar -czvf ../../dist/ev-server-x86_64-unknown-linux-gnu.tar.gz ev-server
tar -czvf ../../dist/ev-etl-x86_64-unknown-linux-gnu.tar.gz ev-etl

cd ../..

echo "✅ Linux builds complete!"
ls -lh dist/
