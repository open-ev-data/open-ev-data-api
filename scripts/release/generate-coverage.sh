#!/bin/bash
set -euo pipefail

echo "📊 Generating test coverage report..."

mkdir -p dist/coverage

cargo llvm-cov --all-features --workspace --html --output-dir dist/coverage
cargo llvm-cov --all-features --workspace --json --output-path dist/coverage-summary.json
cargo llvm-cov --all-features --workspace --lcov --output-path dist/lcov.info

echo "📦 Creating coverage archive..."
cd dist
tar -czvf coverage-report.tar.gz coverage/
cd ..

echo "✅ Coverage report generated!"
ls -lh dist/coverage*.* dist/lcov.info 2>/dev/null || echo "Coverage files created"
