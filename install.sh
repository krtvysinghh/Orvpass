#!/bin/sh
set -e

# Orvpass One-Line Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/krtvysinghh/Orvpass/main/install.sh | sh

echo "⚡ Installing Orvpass CLI (Zero-Knowledge Secrets Manager)..."

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
  x86_64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="arm64" ;;
  *) echo "❌ Unsupported architecture: $ARCH"; exit 1 ;;
esac

TAG="v5.0.0"
if [ "$OS" = "darwin" ]; then
  FILE="orvpass-${TAG}-macos-${ARCH}.tar.gz"
elif [ "$OS" = "linux" ]; then
  FILE="orvpass-${TAG}-linux-${ARCH}.tar.gz"
else
  echo "❌ Unsupported operating system: $OS"; exit 1
fi

URL="https://github.com/krtvysinghh/Orvpass/releases/download/${TAG}/${FILE}"
TMP_DIR="$(mktemp -d)"

echo "📥 Downloading $URL..."
curl -fsSL "$URL" -o "${TMP_DIR}/${FILE}" || {
  echo "Building from cargo source fallback..."
  cargo install --git https://github.com/krtvysinghh/Orvpass.git orvpass-cli
  echo "✅ Orvpass installed successfully via cargo!"
  exit 0
}

tar -xzf "${TMP_DIR}/${FILE}" -C "${TMP_DIR}"

INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
  INSTALL_DIR="${HOME}/.local/bin"
  mkdir -p "$INSTALL_DIR"
fi

cp "${TMP_DIR}/orvpass" "${INSTALL_DIR}/orvpass"
chmod +x "${INSTALL_DIR}/orvpass"
rm -rf "$TMP_DIR"

echo "✨ Orvpass installed successfully to ${INSTALL_DIR}/orvpass!"
echo "🚀 Run 'orvpass' to launch interactive TUI dashboard or 'orvpass --help' for CLI commands."
