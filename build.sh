#!/bin/bash

# Set error handling
set -e

echo "Building release version..."
cargo build --release

# Create artifacts directory if it doesn't exist
if [ ! -d "artifacts" ]; then
  echo "Creating artifacts directory..."
  mkdir -p artifacts
fi

# Detect OS and copy the appropriate executable
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" || "$OSTYPE" == "cygwin" ]]; then
  # Windows
  echo "Windows platform detected"
  cp target/release/joker_cli.exe artifacts/
  echo "Executable available at: artifacts/joker_cli.exe"
else
  # Linux/macOS
  echo "Unix-like platform detected"
  cp target/release/joker_cli artifacts/
  chmod +x artifacts/joker_cli
  echo "Executable available at: artifacts/joker_cli"
fi

echo "Build process completed successfully!"
