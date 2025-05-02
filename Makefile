# Variables

BUILD_DIR=build
PACKAGE_DIR=dist
BIN_BASE_CLI_NAME=ulvm
BIN_BASE_SHIM_NAME=ulvm_shim
CARGO_BUILD_DIST=target/release

# TODO: refactor os detection uname doesn't exist on windows
TARGET ?= $(shell uname -s | tr A-Z a-z)

ifeq ($(TARGET),msys)
	PLATFORM=windows
else ifeq ($(TARGET),windowsnt)
	PLATFORM=windows
else ifeq ($(TARGET),linux)
	PLATFORM=linux
else
	PLATFORM=unknown
endif

ifeq ($(PLATFORM),windows)
	BIN_CLI_NAME=$(BIN_BASE_CLI_NAME).exe
	BIN_SHIM_NAME=$(BIN_BASE_SHIM_NAME).exe
else
	BIN_CLI_NAME=$(BIN_BASE_CLI_NAME)
	BIN_SHIM_NAME=$(BIN_BASE_SHIM_NAME)
endif

# Commandes par défaut
.PHONY: help all linux windows clean build

help:
	@echo "Usage:"
	@echo "make build          Build project for your platform"
	@echo "make package        Build and packge the project for your platform"

all: linux windows

build:
	@echo "⛏️  Build pour $(PLATFORM)"
	make build-$(PLATFORM)


package:
	@echo "📦 Package pour $(PLATFORM)"
	make package-$(PLATFORM)


build-linux:
	cargo build --release

	mkdir -p $(BUILD_DIR)/linux
	cp $(CARGO_BUILD_DIST)/$(BIN_CLI_NAME) $(BUILD_DIR)/linux/
	cp $(CARGO_BUILD_DIST)/$(BIN_SHIM_NAME) $(BUILD_DIR)/linux/
	cp scripts/install.sh $(BUILD_DIR)/linux/


package-linux:
	make build

	mkdir -p $(PACKAGE_DIR)
	tar czvf $(PACKAGE_DIR)/ulvm.tar.gz -C $(BUILD_DIR) .

clean:
	rm -rf $(BUILD_DIR) $(PACKAGE_DIR)