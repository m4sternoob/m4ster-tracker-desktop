#!/bin/bash
# Sync the React frontend from m4ster-roadmap-tracker and build it.
# The desktop app bundles the web UI locally for fast load + offline shell;
# all data/API calls go to the Vercel backend (https://m4ster-tracker.vercel.app).
set -e
cd "$(dirname "$0")/.."

FRONTEND_DIR="frontend-src"
DIST_DIR="dist"

if [ ! -d "$FRONTEND_DIR" ]; then
  echo "Cloning m4ster-roadmap-tracker..."
  git clone --depth 1 https://github.com/m4sternoob/m4ster-roadmap-tracker.git "$FRONTEND_DIR"
else
  echo "Updating m4ster-roadmap-tracker..."
  cd "$FRONTEND_DIR" && git pull --depth 1 origin main && cd ..
fi

echo "Installing frontend dependencies..."
cd "$FRONTEND_DIR" && npm install && cd ..

echo "Building frontend..."
cd "$FRONTEND_DIR" && npm run build && cd ..

echo "Copying dist/ to $DIST_DIR..."
rm -rf "$DIST_DIR"
cp -r "$FRONTEND_DIR/dist" "$DIST_DIR"

echo "Frontend ready in $DIST_DIR/"
