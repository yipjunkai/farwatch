#!/bin/sh
# Install farwatch CLI (direct binary download)
#
# Usage: curl -fsSL https://raw.githubusercontent.com/yipjunkai/farwatch/main/install.sh | sh
#
# Maintenance:
# - Release URL pattern is derived from REPO and GitHub Releases — no changes
#   needed unless the repository moves.
# - The Homebrew formula (Formula/farwatch.rb) is the other install path
#   and is auto-updated by CI (.github/workflows/release.yml).

set -e

REPO="yipjunkai/farwatch"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}" in
  Darwin)  os="apple-darwin" ;;
  Linux)   os="unknown-linux-gnu" ;;
  *)       echo "Unsupported OS: ${OS}"; exit 1 ;;
esac

case "${ARCH}" in
  x86_64)  arch="x86_64" ;;
  aarch64|arm64) arch="aarch64" ;;
  *)       echo "Unsupported architecture: ${ARCH}"; exit 1 ;;
esac

TARGET="${arch}-${os}"

echo "Fetching latest release..."
TAG=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "${TAG}" ]; then
  echo "Error: could not determine latest release"
  exit 1
fi

ARCHIVE="farwatch-${TAG}-${TARGET}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${TAG}/${ARCHIVE}"

echo "Downloading farwatch ${TAG} for ${TARGET}..."
TMPDIR=$(mktemp -d)
trap 'rm -rf "${TMPDIR}"' EXIT

curl -fsSL "${URL}" -o "${TMPDIR}/${ARCHIVE}"
tar xzf "${TMPDIR}/${ARCHIVE}" -C "${TMPDIR}"

BINARY="${TMPDIR}/farwatch-${TAG}-${TARGET}/farwatch"
if [ ! -f "${BINARY}" ]; then
  echo "Error: binary not found in archive"
  exit 1
fi

if [ -w "${INSTALL_DIR}" ]; then
  cp "${BINARY}" "${INSTALL_DIR}/farwatch"
else
  echo "Installing to ${INSTALL_DIR} (requires sudo)..."
  sudo cp "${BINARY}" "${INSTALL_DIR}/farwatch"
fi

chmod +x "${INSTALL_DIR}/farwatch"

echo "farwatch ${TAG} installed to ${INSTALL_DIR}/farwatch"
echo ""
echo "Get started:"
echo "  farwatch start"
