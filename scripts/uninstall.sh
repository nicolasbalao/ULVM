#!/bin/bash

# Couleurs
GREEN="\033[0;32m"
CYAN="\033[0;36m"
BOLD="\033[1m"
RESET="\033[0m"

# Variables
CLI_BIN_DIR="$HOME/.local/bin"
ULVM_DIR="$HOME/.ulvm"


# Création des répertoires
echo  "📁 Removing ${BOLD}$CLI_BIN_DIR/ulvm${RESET}"
rm    "$CLI_BIN_DIR/ulvm"

echo  "📁 Removing ${BOLD}$ULVM_DIR${RESET}"
rm -rf "$ULVM_DIR"
# Fin
echo
echo  "${GREEN}✅ Uninstallation complete!${RESET}"
echo  "👉 You can remove ${BOLD}~/.local/bin${RESET} of your PATH."