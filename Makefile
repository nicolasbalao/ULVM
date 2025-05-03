# Variables

BUILD_DIR=build
PACKAGE_DIR=dist
BIN_BASE_CLI_NAME=ulvm
BIN_BASE_SHIM_NAME=ulvm_shim
CARGO_BUILD_DIST=target/release
CARGO_BUILD_DIST_WINDOWS=target/x86_64-pc-windows-gnu/release

# TODO: refactor os detection uname doesn't exist on windows

# Commandes par défaut
.PHONY: help all clean build

help:
	@echo "Usage:"
	@echo "make build          Build project for your platform"
	@echo "make package        Build and packge the project for your platform"

all: linux windows


# build-linux:
# 	@echo "⛏️  Building"
# 	cargo build --release

# 	mkdir -p $(BUILD_DIR)/linux
# 	cp $(CARGO_BUILD_DIST)/$(BIN_CLI_NAME) $(BUILD_DIR)/linux/
# 	cp $(CARGO_BUILD_DIST)/$(BIN_SHIM_NAME) $(BUILD_DIR)/linux/
# 	cp scripts/install.sh $(BUILD_DIR)/linux/

build-windows:
	cross build --target x86_64-pc-windows-gnu --release

	mkdir -p $(BUILD_DIR)/x86_64-pc-windows-gnu
	cp $(CARGO_BUILD_DIST_WINDOWS)/$(BIN_BASE_CLI_NAME).exe $(BUILD_DIR)/x86_64-pc-windows-gnu/
	cp $(CARGO_BUILD_DIST_WINDOWS)/$(BIN_BASE_SHIM_NAME).exe $(BUILD_DIR)/x86_64-pc-windows-gnu/
	cp scripts/install.bat $(BUILD_DIR)/x86_64-pc-windows-gnu/

# package-linux:
# 	make build

# 	mkdir -p $(PACKAGE_DIR)
# 	tar czvf $(PACKAGE_DIR)/ulvm.tar.gz -C $(BUILD_DIR) .
# 	zip -r x86_64-pc-windows-gnu.zip $(BUILD_DIR)


clean:
	rm -rf $(BUILD_DIR) $(PACKAGE_DIR)