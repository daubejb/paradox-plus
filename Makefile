-include .env
export

# Auto-detect Android SDK and NDK paths if not explicitly defined
ANDROID_HOME ?= $(HOME)/Library/Android/sdk
ANDROID_NDK_ROOT ?= $(shell ls -d $(ANDROID_HOME)/ndk/* 2>/dev/null | sort -V | tail -n 1)

export ANDROID_HOME
export ANDROID_NDK_ROOT

# Variable definitions
ADB ?= $(shell which adb || echo $(ANDROID_HOME)/platform-tools/adb)
EMULATOR ?= $(shell which emulator || echo $(ANDROID_HOME)/emulator/emulator)
IOS_SIM_DEVICE ?= "iPhone 17 Pro"

# Detect host architecture for iOS Simulator compilation
UNAME_M := $(shell uname -m)
ifeq ($(UNAME_M),x86_64)
  IOS_SIM_TARGET := x86_64-apple-ios
else
  IOS_SIM_TARGET := aarch64-apple-ios-sim
endif

.PHONY: help critique-plan android-emulator iphone-emulator build-android build-iphone-sim build-iphone-release check-env-testflight deploy-testflight mac run-iphone-sim

help:
	@echo "🟢 Paradox Plus Mobile Build & Emulation Makefile"
	@echo "================================================="
	@echo "Available targets:"
	@echo "  make mac                   - Run native macOS client"
	@echo "  make android-emulator      - Boot Android AVD and poll completion"
	@echo "  make iphone-emulator       - Boot iOS Simulator for $(IOS_SIM_DEVICE)"
	@echo "  make run-iphone-sim        - Compile, build, and launch on iOS Simulator"
	@echo "  make build-android         - Compile for Android & install via USB-C"
	@echo "  make build-iphone-sim      - Compile static library for iOS Simulator"
	@echo "  make build-iphone-release  - Compile static library for physical iOS"
	@echo "  make deploy-testflight     - Upload IPA bundle to TestFlight via Transporter"
	@echo "  make critique-plan         - Run automated implementation plan critique"

mac:
	cargo run -p client

run-iphone-sim: build-iphone-sim iphone-emulator
	@echo "Locating simulator..."
	@UDID=$$(xcrun simctl list devices | grep -m 1 $(IOS_SIM_DEVICE) | grep -o '[A-F0-9]\{8\}-[A-F0-9]\{4\}-[A-F0-9]\{4\}-[A-F0-9]\{4\}-[A-F0-9]\{12\}'); \
	if [ -z "$$UDID" ]; then \
		echo "ERROR: Could not find iOS simulator matching '$(IOS_SIM_DEVICE)'."; \
		exit 1; \
	fi; \
	echo "Building Xcode project for Simulator..."; \
	xcodebuild -project ios/ParadoxPlus.xcodeproj -scheme ParadoxPlus -configuration Debug -sdk iphonesimulator -derivedDataPath ios/build/derivedData -arch arm64; \
	echo "Installing App on Simulator..."; \
	xcrun simctl install "$$UDID" ios/build/derivedData/Build/Products/Debug-iphonesimulator/ParadoxPlus.app; \
	echo "Launching App on Simulator..."; \
	xcrun simctl launch "$$UDID" com.paradox.plus.client

critique-plan:
	@echo "Running automated implementation plan critique..."
	cargo run -p critique

android-emulator:
	@echo "Checking Android emulator status..."
	@if [ -z "$$($(ADB) devices | grep -v "List" | grep "emulator")" ]; then \
		AVD=$$($(EMULATOR) -list-avds | grep -v -i "wear" | head -n 1); \
		if [ -z "$$AVD" ]; then \
			echo "ERROR: No suitable Android Virtual Device (AVD) found. Please create one in Android Studio."; \
			exit 1; \
		fi; \
		echo "Booting Android emulator: $$AVD"; \
		$(EMULATOR) -avd "$$AVD" -no-snapshot-load > /dev/null 2>&1 & \
		echo "Waiting for device connection..."; \
		$(ADB) wait-for-device; \
		BOOTED=0; \
		for i in $$(seq 1 30); do \
			if [ "$$($(ADB) shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = "1" ]; then \
				BOOTED=1; \
				break; \
			fi; \
			sleep 2; \
		done; \
		if [ $$BOOTED -ne 1 ]; then \
			echo "ERROR: Android emulator failed to complete boot sequence within 60 seconds."; \
			exit 1; \
		fi; \
		echo "Android emulator booted successfully."; \
	else \
		echo "Android emulator already running."; \
	fi

iphone-emulator:
	@echo "Locating simulator for device: $(IOS_SIM_DEVICE)"
	@UDID=$$(xcrun simctl list devices | grep -m 1 $(IOS_SIM_DEVICE) | grep -o '[A-F0-9]\{8\}-[A-F0-9]\{4\}-[A-F0-9]\{4\}-[A-F0-9]\{4\}-[A-F0-9]\{12\}'); \
	if [ -z "$$UDID" ]; then \
		echo "ERROR: Could not find iOS simulator matching '$(IOS_SIM_DEVICE)'."; \
		exit 1; \
	fi; \
	echo "Booting iOS Simulator: $$UDID"; \
	xcrun simctl boot "$$UDID" 2>/dev/null || true; \
	open -a Simulator; \
	echo "Synchronizing boot state..."; \
	xcrun simctl bootstatus "$$UDID"

build-android:
	@echo "Adding Rust Android target..."
	rustup target add aarch64-linux-android
	@echo "Installing cargo-apk if not present..."
	@cargo install --list | grep -q cargo-apk || cargo install cargo-apk
	@echo "Building APK..."
	cargo apk build --lib --manifest-path crates/client/Cargo.toml
	@echo "Installing APK to connected device..."
	$(ADB) install -r -d target/debug/apk/Paradox.apk

build-iphone-sim:
	@echo "Adding Rust iOS Simulator target..."
	rustup target add $(IOS_SIM_TARGET)
	@echo "Compiling for iOS Simulator ($(IOS_SIM_TARGET))..."
	cargo build --release --target $(IOS_SIM_TARGET) --manifest-path crates/client/Cargo.toml

build-iphone-release:
	@echo "Adding Rust iOS target..."
	rustup target add aarch64-apple-ios
	@echo "Compiling for iOS Device (aarch64-apple-ios)..."
	cargo build --release --target aarch64-apple-ios --manifest-path crates/client/Cargo.toml
	@if [ -d "ios/ParadoxPlus.xcodeproj" ]; then \
		echo "Archiving iOS project..."; \
		TEAM_ARG=""; \
		if [ ! -z "$(DEVELOPMENT_TEAM)" ]; then \
			TEAM_ARG="DEVELOPMENT_TEAM=$(DEVELOPMENT_TEAM)"; \
		fi; \
		if xcodebuild -project ios/ParadoxPlus.xcodeproj -scheme ParadoxPlus -configuration Release -archivePath ios/build/ParadoxPlus.xcarchive $$TEAM_ARG archive; then \
			echo "Exporting IPA bundle..."; \
			xcodebuild -exportArchive -archivePath ios/build/ParadoxPlus.xcarchive -exportOptionsPlist ios/exportOptions.plist -exportPath build/ios/ipa; \
		else \
			echo ""; \
			echo "========================================================================"; \
			echo "Rust static library compiled successfully!"; \
			echo "iOS App Archive failed due to Xcode signing requirements."; \
			echo "To package the IPA, please either:"; \
			echo "1. Set DEVELOPMENT_TEAM in your .env file or pass it to make:"; \
			echo "   make build-iphone-release DEVELOPMENT_TEAM=XXXXXXXXXX"; \
			echo "2. Open the project in Xcode (ios/ParadoxPlus.xcodeproj) and"; \
			echo "   configure your Development Team under Signing & Capabilities."; \
			echo "========================================================================"; \
			echo ""; \
			exit 0; \
		fi; \
	else \
		echo "Warning: ios/ParadoxPlus.xcodeproj not found, compiled static library only."; \
	fi

check-env-testflight:
ifndef APPLE_ID
	$(error APPLE_ID is undefined. Please set it in your environment or .env file)
endif
ifndef APP_SPECIFIC_PASSWORD
	$(error APP_SPECIFIC_PASSWORD is undefined. Please set it in your environment or .env file)
endif

deploy-testflight: check-env-testflight
	@if [ -f "build/ios/ipa/ParadoxPlus.ipa" ]; then \
		echo "Uploading ParadoxPlus.ipa to TestFlight..."; \
		xcrun transporter -m upload -u "$(APPLE_ID)" -p "$(APP_SPECIFIC_PASSWORD)" -f build/ios/ipa/ParadoxPlus.ipa; \
	else \
		echo "ERROR: build/ios/ipa/ParadoxPlus.ipa not found. Run 'make build-iphone-release' first."; \
		exit 1; \
	fi
