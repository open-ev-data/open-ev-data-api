#!/bin/bash
set -euo pipefail

VERSION="${1:-dev}"

echo "🔨 Building Linux x86_64 binaries (version: $VERSION)..."

cargo build --release -p ev-server -p ev-etl

echo "📦 Creating archives..."

mkdir -p dist

cd target/release

tar -czvf ../../dist/ev-server-x86_64-unknown-linux-gnu.tar.gz ev-server
tar -czvf ../../dist/ev-etl-x86_64-unknown-linux-gnu.tar.gz ev-etl

cd ../..

echo "✅ Linux builds complete!"
ls -la dist/
