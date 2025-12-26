#!/bin/bash
set -euo pipefail

# Error handler for better CI debugging
trap 'echo "❌ ERROR on line $LINENO: Command failed with exit code $?" >&2' ERR

echo "📊 Generating test coverage report..."
echo "::group::Coverage Generation"

mkdir -p dist/coverage

cargo llvm-cov --all-features --workspace --html --output-dir dist/coverage
cargo llvm-cov --all-features --workspace --json --output-path dist/coverage-summary.json
cargo llvm-cov --all-features --workspace --lcov --output-path dist/lcov.info

echo "::endgroup::"
echo "📦 Creating coverage archive..."
cargo llvm-cov --all-features --workspace --json --output-path dist/coverage-summary.json
cargo llvm-cov --all-features --workspace --lcov --output-path dist/lcov.info

echo "📦 Creating coverage archive..."
cd dist
tar -czvf coverage-report.tar.gz coverage/
cd ..

echo "✅ Coverage report generated!"
ls -lh dist/coverage*.* dist/lcov.info 2>/dev/null || echo "Coverage files created"
