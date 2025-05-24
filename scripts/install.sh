#!/bin/bash

# Couleurs
GREEN="\033[0;32m"
CYAN="\033[0;36m"
BOLD="\033[1m"
RESET="\033[0m"

# Variables
CLI_BIN_DIR="$HOME/.local/bin"

# Détermine le chemin absolu du dossier contenant ce script
SCRIPT_DIR="$(realpath "$0" | sed 's|\(.*\)/.*|\1|')"

echo "${BOLD}${CYAN}📦 ULVM Installer${RESET}"
echo

# Création des répertoires
echo  "📁 Creating ${BOLD}$CLI_BIN_DIR${RESET}"
echo
mkdir -p "$CLI_BIN_DIR"


# Copie du binaire CLI
echo  "🚚 Installing CLI binary to ${BOLD}$CLI_BIN_DIR${RESET}"
echo
cp "$SCRIPT_DIR/ulvm" "$CLI_BIN_DIR/ulvm"

# Vérifie si ~/.local/bin est dans le PATH
if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
  echo
  echo "${BOLD}⚠ ~/.local/bin is not in your PATH — cannot run 'ulvm' directly.${RESET}"
  echo "👉 You can run it manually later or add ~/.local/bin to your PATH."
fi