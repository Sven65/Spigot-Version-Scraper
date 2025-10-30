# Makefile for Rust project with macOS and Linux releases

PROJECT_NAME := spigot_version_scraper
TARGET_LINUX := x86_64-unknown-linux-gnu
BUILD_DIR_LINUX := target/$(TARGET_LINUX)/release
ZIP_NAME_LINUX := $(PROJECT_NAME)-linux.zip

# --------------------
# macOS / current platform
# --------------------
release:
	cargo build --release
	@echo "Release built for current platform at target/release/$(PROJECT_NAME)"

# --------------------
# Linux release using cross
# --------------------
release-linux:
	@echo "Starting cross-compile for $(TARGET_LINUX) using Docker..."
	docker run --rm --platform linux/amd64 -v "$(PWD)":/usr/src/myapp -w /usr/src/myapp rust:latest \
		bash -c "rustup target add $(TARGET_LINUX) && cargo build --release --target $(TARGET_LINUX)"
	@echo "Cross-compile finished."
	@echo "Zipping Linux binary..."
	@cd $(BUILD_DIR_LINUX) && zip -r $(ZIP_NAME_LINUX) $(PROJECT_NAME)
	@echo "Linux release ready at $(BUILD_DIR_LINUX)/$(ZIP_NAME_LINUX)"

# --------------------
# Clean
# --------------------
clean:
	cargo clean
	rm -f $(BUILD_DIR_LINUX)/$(ZIP_NAME_LINUX)

# --------------------
# Run locally on macOS
# --------------------
run-local:
	cargo run
