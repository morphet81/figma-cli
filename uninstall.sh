#!/usr/bin/env bash
set -euo pipefail

BOLD='\033[1m'
GREEN='\033[0;32m'
DIM='\033[2m'
RESET='\033[0m'

info() { echo -e "${BOLD}$1${RESET}"; }
success() { echo -e "${GREEN}$1${RESET}"; }

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
TARGET="$INSTALL_DIR/fcli"

if [[ -e "$TARGET" ]]; then
  rm -- "$TARGET"
  success "Removed $TARGET"
else
  info "No fcli at $TARGET (nothing to remove)."
  if command -v fcli &>/dev/null; then
    echo -e "${DIM}Another fcli is on your PATH: $(command -v fcli)${RESET}"
  fi
fi
