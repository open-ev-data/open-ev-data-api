#!/bin/bash
set -euo pipefail

# Error handler for better CI debugging
trap 'echo "❌ ERROR on line $LINENO: Command failed with exit code $?" >&2' ERR

VERSION="${1:-latest}"
REGISTRY="ghcr.io"
OWNER="${GITHUB_REPOSITORY_OWNER:-open-ev-data}"

echo "🐳 Building Docker images (version: $VERSION)..."
echo "::group::Docker Build - ev-server"

docker build -t "$REGISTRY/$OWNER/ev-server:$VERSION" -f docker/Dockerfile .

echo "::endgroup::"
echo "::group::Docker Build - ev-etl"

docker build -t "$REGISTRY/$OWNER/ev-etl:$VERSION" -f docker/Dockerfile.etl .

echo "::endgroup::"

docker tag "$REGISTRY/$OWNER/ev-server:$VERSION" "$REGISTRY/$OWNER/ev-server:latest"
docker tag "$REGISTRY/$OWNER/ev-etl:$VERSION" "$REGISTRY/$OWNER/ev-etl:latest"

echo "📤 Pushing Docker images..."
echo "::group::Docker Push"

docker push "$REGISTRY/$OWNER/ev-server:$VERSION"
docker push "$REGISTRY/$OWNER/ev-server:latest"
docker push "$REGISTRY/$OWNER/ev-etl:$VERSION"
docker push "$REGISTRY/$OWNER/ev-etl:latest"

echo "::endgroup::"
echo "✅ Docker builds complete!"
