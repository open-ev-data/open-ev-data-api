#!/bin/bash
set -euo pipefail

COVERAGE_FILE="${1:-dist/coverage-summary.json}"

if [ ! -f "$COVERAGE_FILE" ]; then
    echo "❌ Coverage file not found: $COVERAGE_FILE"
    exit 1
fi

COVERAGE_PERCENT=$(jq -r '.data[0].totals.lines.percent' "$COVERAGE_FILE")

if [ -z "$COVERAGE_PERCENT" ] || [ "$COVERAGE_PERCENT" == "null" ]; then
    echo "❌ Failed to extract coverage percentage"
    exit 1
fi

COVERAGE_ROUNDED=$(printf "%.2f" "$COVERAGE_PERCENT")

echo "📊 Coverage: ${COVERAGE_ROUNDED}%"

if [ -f "README.md" ]; then
    sed -i "s/coverage-[0-9.]*%25/coverage-${COVERAGE_ROUNDED}%25/g" README.md
    echo "✅ README.md updated with coverage: ${COVERAGE_ROUNDED}%"
else
    echo "❌ README.md not found"
    exit 1
fi
