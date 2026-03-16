# SPDX-FileCopyrightText: 2026 The mwifiex_nl80211_test_mode authors 
# SPDX-License-Identifier: GPL-2.0-only

CARGO_RUNNER		?= cross
CROSS_CONTAINER_ENGINE	= podman
TARGET_PROFILE		?= release

default-target-list	:= aarch64-unknown-linux-musl arm-unknown-linux-musleabihf x86_64-unknown-linux-musl

all: check-toolchain build
build: $(addsuffix -build,$(default-target-list))

help:
	@echo "MWIFIEX test mode cross compilation Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  help              - Show this help message"
	@echo "  check-toolchain   - Verify all required CLI tools are installed"
	@echo "  all               - Build binaries for all default targets"
	@echo "  <target>-build    - Build binaries for all default targets"
	@echo "  <target>          - Build for a specific target (e.g., aarch64-unknown-linux-musl)"
	@echo ""
	@echo "Default targets: $(default-target-list)"
	@echo ""
	@echo "Configuration variables:"
	@echo "  CARGO_RUNNER           - Cargo wrapper for cross-compilation (default: $(CARGO_RUNNER))"
	@echo "  CROSS_CONTAINER_ENGINE - Container engine for cross (default: $(CROSS_CONTAINER_ENGINE))"
	@echo "  TARGET_PROFILE         - Build profile to use (current: $(TARGET_PROFILE))"
	@echo ""
	@echo "Examples:"
	@echo "  make check-toolchain                    - Verify dependencies"
	@echo "  make all                                - Build everything"
	@echo "  make aarch64-unknown-linux-musl-build   - Build for aarch64 only"
	@echo "  make CROSS_CONTAINER_ENGINE=docker all  - Use Docker instead of Podman"

check-toolchain:
	@echo "Checking for required CLI tools..."
	@command -v $(CARGO_RUNNER) >/dev/null 2>&1 || { echo "Error: $(CARGO_RUNNER) is not installed. Please install it to continue."; exit 1; }
	@command -v rustup >/dev/null 2>&1 || { echo "Error: rustup is not installed. Please install it from https://rustup.rs/"; exit 1; }
	@command -v $(CROSS_CONTAINER_ENGINE) >/dev/null 2>&1 || { echo "Error: $(CROSS_CONTAINER_ENGINE) is not installed. Please install podman or set CROSS_CONTAINER_ENGINE to docker."; exit 1; }
	@echo "All required tools are available!"

clean:
	cargo clean

%-build: check-toolchain
	CROSS_CONTAINER_ENGINE=$(CROSS_CONTAINER_ENGINE) $(CARGO_RUNNER) build --target $* $(if $(TARGET_PROFILE),--profile $(TARGET_PROFILE))

.PHONY: all build clean check-toolchain help
