#!/bin/bash
set -euo pipefail

VERSION="${1:-latest}"
REGISTRY="ghcr.io"
OWNER="${GITHUB_REPOSITORY_OWNER:-open-ev-data}"

echo "🐳 Building Docker images (version: $VERSION)..."

docker build -t "$REGISTRY/$OWNER/ev-server:$VERSION" -f docker/Dockerfile .
docker build -t "$REGISTRY/$OWNER/ev-etl:$VERSION" -f docker/Dockerfile.etl .

docker tag "$REGISTRY/$OWNER/ev-server:$VERSION" "$REGISTRY/$OWNER/ev-server:latest"
docker tag "$REGISTRY/$OWNER/ev-etl:$VERSION" "$REGISTRY/$OWNER/ev-etl:latest"

echo "📤 Pushing Docker images..."

docker push "$REGISTRY/$OWNER/ev-server:$VERSION"
docker push "$REGISTRY/$OWNER/ev-server:latest"
docker push "$REGISTRY/$OWNER/ev-etl:$VERSION"
docker push "$REGISTRY/$OWNER/ev-etl:latest"

echo "✅ Docker builds complete!"
