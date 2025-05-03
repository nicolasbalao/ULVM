# Variables

BUILD_DIR=build
PACKAGE_DIR=package
BIN_BASE_CLI_NAME=ulvm
BIN_BASE_SHIM_NAME=ulvm_shim
CARGO_BUILD_DIST=target/release
CARGO_BUILD_DIST_WINDOWS=target/x86_64-pc-windows-gnu/release

# TODO: refactor os detection uname doesn't exist on windows

# Commandes par défaut
.PHONY: help clean build package

help:
	@echo "Usage:"
	@echo "make build          Build project create /build with exec and install file"
	@echo "make package        Build and packge the project creating archive file"


build:
	@echo "⛏️  Building"

	cargo build --release

	mkdir -p $(BUILD_DIR)
	cp $(CARGO_BUILD_DIST)/$(BIN_BASE_CLI_NAME) $(BUILD_DIR)/
	cp $(CARGO_BUILD_DIST)/$(BIN_BASE_SHIM_NAME) $(BUILD_DIR)/
	cp scripts/install.sh $(BUILD_DIR)/



build-windows:
	cross build --target x86_64-pc-windows-gnu --release

	mkdir -p $(BUILD_DIR)/x86_64-pc-windows-gnu
	cp $(CARGO_BUILD_DIST_WINDOWS)/$(BIN_BASE_CLI_NAME).exe $(BUILD_DIR)/
	cp $(CARGO_BUILD_DIST_WINDOWS)/$(BIN_BASE_SHIM_NAME).exe $(BUILD_DIR)/

	cp scripts/install.bat $(BUILD_DIR)/

package:
	make build
	mkdir -p $(PACKAGE_DIR)
	tar czvf $(PACKAGE_DIR)/ulvm.tar.gz -C $(BUILD_DIR)/ .

package-windows:
	make build-windows
	mkdir -p $(PACKAGE_DIR)
	zip -r $(PACKAGE_DIR)/ulvm_x86_64-pc-windows-gnu.zip $(BUILD_DIR)/


install:
	make build
	sh $(BUILD_DIR)/install.sh

clean:
	rm -rf $(BUILD_DIR) $(PACKAGE_DIR)