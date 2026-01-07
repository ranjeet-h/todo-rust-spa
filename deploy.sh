#!/bin/bash

# Exit on any error
set -e

# Get the project root directory
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "🚀 Starting todoapp deployment build..."

# Build frontend locally (fast)
echo "📦 Building frontend..."
(cd "$ROOT_DIR/frontend" && npm run build)

echo 
echo "🎯 Select build target:"
echo "1. Docker: Local ARM64 (Multi-stage build)"
echo "2. Docker: Production AMD64 (Multi-stage build)"
echo "3. Local Host: Build and Run on your current OS (Mac)"
echo

# Check if non-interactive
if [ -t 0 ]; then
    read -p "Choose option (1-3): " REPLY
    echo
else
    REPLY="1" 
    echo "Non-interactive mode: Choosing option 1"
fi

case "$REPLY" in
    1)
        BUILD_PLATFORM="linux/arm64"
        echo "🐳 Building Docker image for ARM64..."
        docker buildx build \
          --build-arg CACHEBUST=$(date +%s) \
          --platform "$BUILD_PLATFORM" \
          -f Dockerfile \
          -t todoapp:latest \
          --load \
          .
        echo "✅ Docker build complete! Run with: docker-compose up -d"
        ;;
    2)
        BUILD_PLATFORM="linux/amd64"
        echo "🐳 Building Docker image for AMD64..."
        docker buildx build \
          --build-arg CACHEBUST=$(date +%s) \
          --platform "$BUILD_PLATFORM" \
          -f Dockerfile \
          -t todoapp:latest \
          --load \
          .
        echo "✅ Docker build complete!"
        ;;
    3)
        echo "🍎 Running locally on your Mac..."
        # Force a re-build to ensure new frontend assets are embedded
        touch "$ROOT_DIR/backend/src/main.rs"
        cd "$ROOT_DIR/backend"
        cargo run
        ;;
    *)
        echo "❌ Invalid option."
        exit 1
        ;;
esac
