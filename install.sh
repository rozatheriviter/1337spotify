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

echo "Installing required system dependencies for Fedora..."
# Dependencies are for OpenSSL, ALSA (audio backend), DBus (media control),
# XCB (windowing/graphics dependencies), and GTK4.
sudo dnf check-update || true
sudo dnf install -y \
  pkgconf-pkg-config \
  openssl-devel \
  alsa-lib-devel \
  dbus-devel \
  libxcb-devel \
  gtk4-devel \
  git

echo "System dependencies installed successfully."

# --- 2. Build LinAmp ---

echo "Building LinAmp (spotify_player with GTK support) in release mode..."
# The --workspace flag tells cargo to build all members defined in the root Cargo.toml.
cargo build --release --workspace --features gtk

echo "Build successful! Binary location: target/release/spotify_player"

# --- 3. Create linamp executable ---

echo "Creating linamp symlink..."
ln -sf spotify_player target/release/linamp

# --- 4. Run Tests ---

echo "Running tests with GTK and full feature set..."
# Features are taken from the project's CI configuration, adding 'gtk'.
cargo test --workspace --no-default-features --features "rodio-backend,media-control,image,notify,fzf,gtk"

echo "Tests completed."

# --- 5. Next Steps ---

echo "---"
echo "Next Steps:"
echo "1. To run the application with the LinAmp (GTK) UI, type:"
echo "   ./target/release/linamp --ui-type gtk"
echo "2. Remember to run the initial authentication step:"
echo "   ./target/release/linamp authenticate"
echo "3. For more details on installation and features, refer to the README.md."
