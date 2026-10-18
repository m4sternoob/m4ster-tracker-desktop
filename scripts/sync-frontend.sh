#!/bin/bash
# Sync the React frontend from m4ster-roadmap-tracker and build it.
# The desktop app bundles the web UI locally for fast load + offline shell;
# all data/API calls go to the Vercel backend (https://m4ster-tracker.vercel.app).
#
# Pinned to a specific commit for reproducible builds. Bump PINNED_COMMIT
# when you want to pick up new web UI changes.
set -e
cd "$(dirname "$0")/.."

FRONTEND_DIR="frontend-src"
DIST_DIR="dist"
PINNED_COMMIT="305d01d1eb637a8673e6ef3eda766186c035dbf5"

if [ ! -d "$FRONTEND_DIR" ]; then
  echo "Cloning m4ster-roadmap-tracker at $PINNED_COMMIT..."
  git clone https://github.com/m4sternoob/m4ster-roadmap-tracker.git "$FRONTEND_DIR"
fi

echo "Checking out pinned commit $PINNED_COMMIT..."
cd "$FRONTEND_DIR" && git fetch origin main && git checkout "$PINNED_COMMIT" && cd ..

echo "Installing frontend dependencies..."
cd "$FRONTEND_DIR" && npm install && cd ..

echo "Building frontend..."
cd "$FRONTEND_DIR" && npm run build && cd ..

echo "Copying dist/ to $DIST_DIR..."
rm -rf "$DIST_DIR"
cp -r "$FRONTEND_DIR/dist" "$DIST_DIR"

echo "Frontend ready in $DIST_DIR/ (pinned to $PINNED_COMMIT)"
