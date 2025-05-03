#!/bin/bash

# Couleurs
GREEN="\033[0;32m"
CYAN="\033[0;36m"
BOLD="\033[1m"
RESET="\033[0m"

# Variables
CLI_BIN_DIR="$HOME/.local/bin"
ULVM_BIN_DIR="$HOME/.ulvm/bin"

# Détermine le chemin absolu du dossier contenant ce script
SCRIPT_DIR="$(realpath "$0" | sed 's|\(.*\)/.*|\1|')"

echo "${BOLD}${CYAN}📦 ULVM Installer${RESET}"
echo

# Création des répertoires
echo  "📁 Creating ${BOLD}$CLI_BIN_DIR${RESET}"
mkdir -p "$CLI_BIN_DIR"

echo  "📁 Creating ${BOLD}$ULVM_BIN_DIR${RESET}"
mkdir -p "$ULVM_BIN_DIR"

# Copie du binaire CLI
echo  "🚚 Installing CLI binary to ${BOLD}$CLI_BIN_DIR${RESET}"
cp "$SCRIPT_DIR/ulvm" "$CLI_BIN_DIR/ulvm"

# Copie du shim
echo  "🔧 Installing shim to ${BOLD}$ULVM_BIN_DIR${RESET}"
cp "$SCRIPT_DIR/ulvm_shim" "$ULVM_BIN_DIR/ulvm_shim"

# Fin
echo
echo  "${GREEN}✅ Installation complete!${RESET}"
echo  "👉 Make sure ${BOLD}~/.local/bin${RESET} is in your PATH."
