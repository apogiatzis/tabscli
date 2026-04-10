#!/usr/bin/env bash
set -euo pipefail

REPO="apogiatzis/tabscli"
INSTALL_DIR="${HOME}/.local/bin"
BINARY="tabscli"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}" in
    Linux)  os="unknown-linux-gnu" ;;
    Darwin) os="apple-darwin" ;;
    *)      echo "Error: unsupported OS '${OS}'" >&2; exit 1 ;;
esac

case "${ARCH}" in
    x86_64|amd64)  arch="x86_64" ;;
    aarch64|arm64) arch="aarch64" ;;
    *)             echo "Error: unsupported architecture '${ARCH}'" >&2; exit 1 ;;
esac

TARGET="${arch}-${os}"

# Get latest release tag
VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)"
if [ -z "${VERSION}" ]; then
    echo "Error: could not determine latest release" >&2
    exit 1
fi

URL="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY}-${VERSION}-${TARGET}.tar.gz"

echo "Installing ${BINARY} ${VERSION} (${TARGET})..."

# Create install directory
mkdir -p "${INSTALL_DIR}"

# Download and extract
TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT

curl -fsSL "${URL}" -o "${TMP}/${BINARY}.tar.gz"
tar xzf "${TMP}/${BINARY}.tar.gz" -C "${TMP}"
mv "${TMP}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
chmod +x "${INSTALL_DIR}/${BINARY}"

echo "Installed ${BINARY} to ${INSTALL_DIR}/${BINARY}"

# Check if install dir is in PATH
if ! echo "${PATH}" | tr ':' '\n' | grep -qx "${INSTALL_DIR}"; then
    echo ""
    echo "Warning: ${INSTALL_DIR} is not in your PATH."
    echo "Add it by appending this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo ""
    echo "  export PATH=\"\${HOME}/.local/bin:\${PATH}\""
fi
