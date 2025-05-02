#!/bin/bash

# Variables
CLI_BIN_DIR="$HOME/.local/bin"
ULVM_BIN_DIR="$HOME/.ulvm/bin"

# Create directory
echo "Creating local bin dir: $CLI_BIN_DIR"
mkdir -p "$CLI_BIN_DIR"

echo "Creating ulvm home bin dir: $ULVM_BIN_DIR"
mkdir -p "$ULVM_BIN_DIR"

# Copy CLI bin  
echo "Copy ulvm to $CLI_BIN_DIR"
cp ./ulvm "$CLI_BIN_DIR/ulvm"

# Copy shims to ulvm bin
echo "Setup shim to $ULVM_BIN_DIR"
cp ./ulvm_shim "$ULVM_BIN_DIR/ulvm_shim"

echo "Installation complete"


