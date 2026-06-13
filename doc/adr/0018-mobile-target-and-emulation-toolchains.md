# ADR 0018: Mobile Target Cross-Compilation and Emulation Toolchains

## Status
Approved

## Context
As we prepare to test the game client on both Android (Pixel 10 Pro) and iOS (iPhone 17 Pro), we need a standardized, reproducible build pipeline and emulator booting strategy inside the workspace. 
Prior to this ADR, the Bevy client app was configured only for desktop native and web target (`wasm32-unknown-unknown`) compilation, with no packaging configurations or FFI entry points to build for native iOS or Android apps.

## Decision
We implement a hybrid mobile packaging architecture:
1. **Android target**: The client crate compiles directly to a shared dynamic library (`cdylib`) and is packaged using `cargo-apk` as an APK wrapping Android's `NativeActivity` (using `android-activity` with the `game-activity` backend).
2. **iOS target**: The client crate compiles to a static library (`staticlib`). We scaffold a minimal native Swift/Xcode wrapper project under `ios/` that links to `libclient.a` and exposes a bridging FFI function `start_ios_client` called at application launch.
3. **Emulator Integration**: We integrate boot targets into the Makefile, utilizing:
   - `adb -e wait-for-device` and checking the `sys.boot_completed` Android property with a 60-second safety timeout.
   - Dynamic UDID extraction of iOS Simulators via `xcrun simctl list devices` to support any local macOS environment configuration.
4. **TestFlight Deployment**: TestFlight uploads utilize Apple's modern Transporter CLI (`transporter`) instead of the deprecated `altool` utility. TestFlight credentials are loaded dynamically from a local `.env` file or CLI variables.

## Consequences
- **Developer Experience**: Local developers can run `make android-emulator` or `make iphone-emulator` to spin up virtual testing devices, and run `make build-android` or `make build-iphone-release` to compile and package/install applications automatically.
- **Modularity**: All FFI and platform-specific entry points are kept isolated inside `crates/client/src/mobile.rs` to prevent bloating the core game client library or violating source file line limits.
- **Portability**: All Postcard messages and fixed-point physics configurations remain platform-agnostic, preserving bitwise FSM reproducibility across ARM64 (mobile) and x86_64/WASM (server/web) targets.
