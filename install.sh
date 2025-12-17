#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# --- Navigate to the script's directory (assumed to be the project root) ---
# This resolves the "could not find Cargo.toml" error by ensuring cargo runs
# from the directory containing the top-level Cargo.toml file.
SCRIPT_DIR="$(dirname "$(realpath "$0")")"
echo "Navigating to project root: $SCRIPT_DIR"
cd "$SCRIPT_DIR"

# --- 1. Install System Dependencies ---

echo "Installing required system dependencies for Pop!_OS (Ubuntu/Debian)..."
# Dependencies are for OpenSSL, ALSA (audio backend), DBus (media control),
# and XCB (windowing/graphics dependencies).
sudo apt update
sudo apt install -y \
  pkg-config \
  libssl-dev \
  libasound2-dev \
  libdbus-1-dev \
  libxcb-shape0-dev \
  libxcb-xfixes0-dev \
  git

echo "System dependencies installed successfully."

# --- 2. Build the Project ---

echo "Building spotify_player in release mode..."
# The --workspace flag tells cargo to build all members defined in the root Cargo.toml.
cargo build --release --workspace

echo "Build successful! Binary location: target/release/spotify_player"

# --- 3. Run Tests ---

echo "Running tests with a full feature set..."
# Features are taken from the project's CI configuration.
cargo test --workspace --no-default-features --features "rodio-backend,media-control,image,notify,fzf"

echo "Tests completed."

# --- 4. Next Steps ---

echo "---"
echo "Next Steps:"
echo "1. To run the application, navigate to the project root and execute:"
echo "   ./target/release/spotify_player"
echo "2. Remember to run the initial authentication step:"
echo "   ./target/release/spotify_player authenticate"
echo "3. For more details on installation and features, refer to the README.md."
