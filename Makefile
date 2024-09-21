# Variables
export CARGO_TARGET_DIR ?= ./target
DESTDIR ?= /
PREFIX ?= /usr/local
BINARY := mwifi
TARGET := $(CARGO_TARGET_DIR)/release/$(BINARY)
INSTALL_PATH := $(DESTDIR)$(PREFIX)/bin/$(BINARY)

# Default target (build release)
.PHONY: all build install uninstall clean

all: build

# Build the project in release mode
build:
	@echo "Building $(BINARY) in release mode..."
	cargo build -p $(BINARY) --release

# Install the binary to the system
install: build
	@echo "Installing $(BINARY) to $(INSTALL_PATH)..."
	@install -Dm755 $(TARGET) $(INSTALL_PATH)
	@echo "$(BINARY) installed successfully!"

# Uninstall the binary from the system
uninstall:
	@echo "Uninstalling $(BINARY) from $(INSTALL_PATH)..."
	@if [ -f $(INSTALL_PATH) ]; then \
		rm -f $(INSTALL_PATH); \
		echo "$(BINARY) uninstalled successfully!"; \
	else \
		echo "$(INSTALL_PATH) does not exist, skipping uninstall."; \
	fi

# Clean the build artifacts
clean:
	@echo "Cleaning project..."
	cargo clean