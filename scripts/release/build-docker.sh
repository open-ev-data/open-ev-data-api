#!/bin/bash
set -euo pipefail

trap 'echo "❌ ERROR on line $LINENO: Command failed with exit code $?" >&2' ERR

VERSION="${1:-latest}"
REGISTRY="ghcr.io"
OWNER="${GITHUB_REPOSITORY_OWNER:-open-ev-data}"

echo "🔍 Pre-flight checks..."
if [ ! -f "target/release/ev-server" ]; then
    echo "❌ Binary not found: target/release/ev-server"
    exit 1
fi

if [ ! -f "target/release/ev-etl" ]; then
    echo "❌ Binary not found: target/release/ev-etl"
    exit 1
fi

echo "✅ Binaries found"
echo "🐳 Building Docker images (version: $VERSION)..."
echo "::group::Docker Build - ev-server"

docker build -t "$REGISTRY/$OWNER/ev-server:$VERSION" -f docker/Dockerfile . || {
    echo "❌ Failed to build ev-server Docker image"
    exit 1
}

echo "::endgroup::"
echo "::group::Docker Build - ev-etl"

docker build -t "$REGISTRY/$OWNER/ev-etl:$VERSION" -f docker/Dockerfile.etl . || {
    echo "❌ Failed to build ev-etl Docker image"
    exit 1
}

echo "::endgroup::"

docker tag "$REGISTRY/$OWNER/ev-server:$VERSION" "$REGISTRY/$OWNER/ev-server:latest"
docker tag "$REGISTRY/$OWNER/ev-etl:$VERSION" "$REGISTRY/$OWNER/ev-etl:latest"

echo "📤 Pushing Docker images..."
echo "::group::Docker Push"

echo "Pushing ev-server:$VERSION..."
docker push "$REGISTRY/$OWNER/ev-server:$VERSION" 2>&1 || {
    echo "❌ Failed to push ev-server:$VERSION"
    echo "Registry: $REGISTRY"
    echo "Owner: $OWNER"
    exit 1
}

echo "Pushing ev-server:latest..."
docker push "$REGISTRY/$OWNER/ev-server:latest"

echo "Pushing ev-etl:$VERSION..."
docker push "$REGISTRY/$OWNER/ev-etl:$VERSION"

echo "Pushing ev-etl:latest..."
docker push "$REGISTRY/$OWNER/ev-etl:latest"

echo "::endgroup::"
echo "✅ Docker builds complete!"
