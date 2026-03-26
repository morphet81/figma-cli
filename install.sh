#!/usr/bin/env bash
set -euo pipefail

BOLD='\033[1m'
GREEN='\033[0;32m'
RED='\033[0;31m'
DIM='\033[2m'
RESET='\033[0m'

info() { echo -e "${BOLD}$1${RESET}"; }
success() { echo -e "${GREEN}$1${RESET}"; }
error() { echo -e "${RED}$1${RESET}" >&2; exit 1; }

if ! command -v cargo &>/dev/null; then
    info "Rust toolchain not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

info "Building fcli in release mode..."
cargo build --release

BINARY="$SCRIPT_DIR/target/release/fcli"

if [ ! -f "$BINARY" ]; then
    error "Build failed — binary not found at $BINARY"
fi

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
mkdir -p "$INSTALL_DIR"
cp "$BINARY" "$INSTALL_DIR/fcli"
chmod +x "$INSTALL_DIR/fcli"

if ! echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
    echo ""
    info "Add $INSTALL_DIR to your PATH by appending this to your shell profile:"
    echo -e "${DIM}  export PATH=\"$INSTALL_DIR:\$PATH\"${RESET}"
    echo ""

    SHELL_NAME="$(basename "$SHELL")"
    case "$SHELL_NAME" in
        zsh)  PROFILE="$HOME/.zshrc" ;;
        bash) PROFILE="$HOME/.bashrc" ;;
        *)    PROFILE="" ;;
    esac

    if [ -n "$PROFILE" ]; then
        read -rp "Add it to $PROFILE now? [Y/n] " answer
        if [[ -z "$answer" || "$answer" =~ ^[Yy] ]]; then
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$PROFILE"
            success "Added to $PROFILE. Restart your shell or run: source $PROFILE"
        fi
    fi
fi

echo ""
success "fcli installed successfully!"
echo -e "${DIM}$("$INSTALL_DIR/fcli" --version)${RESET}"
echo ""
info "Quick start:"
echo "  fcli auth login                  # authenticate with Figma"
echo "  fcli auth status                 # check auth status"
echo "  fcli file inspect --url \"...\"    # inspect a Figma file"
echo ""
echo -e "${DIM}Set FIGMA_ACCESS_TOKEN to skip login: export FIGMA_ACCESS_TOKEN=figd_...${RESET}"
