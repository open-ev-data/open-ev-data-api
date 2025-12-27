# Production Dockerfile - Uses pre-compiled binaries from target/release/
# This Dockerfile is optimized for CI/CD pipelines where binaries are built
# separately before Docker image creation, significantly reducing build time.
#
# Prerequisites: cargo build --release must be run before building this image
# Usage: docker build -t ev-server -f docker/Dockerfile .

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY target/release/ev-server /app/ev-server

# Create non-root user
RUN useradd -r -s /bin/false appuser && \
    chown -R appuser:appuser /app
USER appuser

# Default environment variables
ENV DATABASE_URL=/app/vehicles.db
ENV PORT=3000
ENV HOST=0.0.0.0
ENV LOG_LEVEL=info
ENV ENABLE_COMPRESSION=true
ENV ENABLE_OPENAPI=true

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/api/v1/health || exit 1

# Run server
ENTRYPOINT ["/app/ev-server"]
