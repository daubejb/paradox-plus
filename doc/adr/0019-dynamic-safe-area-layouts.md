# ADR 0019: Dynamic Safe Area Layouts in Bevy UI

## Status
Approved

## Context
Mobile devices (including the Pixel 10 Pro and iPhone 17 Pro) feature native safe areas (e.g. status bar notches, gesture home indicators) where game UI elements should not encroach to avoid overlapping system overlays or getting cropped.
Initially, the implementation used compile-time platform target conditional checks (`cfg!(any(target_os = "android", target_os = "ios"))`) to apply hardcoded safe-area padding and container heights. However, this compile-time approach has significant downsides:
1. **WebAssembly Limitations**: WebAssembly builds compile to `target_os = "unknown"`. Mobile browsers (Safari/Chrome) running the WASM client would not trigger the `cfg!` target checks, rendering a squished or cut-off UI.
2. **Local Testing Constraints**: Desktop builds are compile-gated out of safe-area spacing, meaning developers cannot test, debug, or preview mobile layouts locally by resizing the desktop window to mobile aspect ratios.

## Decision
We replace compile-time environment gating with a runtime-responsive safe-area adjustment system:
1. **Dynamic Safe-Area System**: We implement a Bevy ECS system `update_ui_safe_areas_system` that queries the primary `Window` dimensions on every frame (in the `Update` schedule).
2. **Mobile Detection Heuristics**: The system flags the viewport as mobile if the compile-target OS is natively mobile (Android/iOS) OR if the screen aspect ratio is portrait (height > width) and the screen width is narrow (width < 600px).
3. **Reactive Layout Styling**: When mobile status is detected, the system applies safe-area padding (44.0px top padding for status bar, 24.0px bottom padding for home indicator) and updates the container heights of the `TopHudNode` and `BottomBarNode` styles dynamically. If desktop native/landscape is detected, it reverts back to default padding (10.0px top, 10.0px bottom).

## Consequences
- **Robustness**: WASM client builds running on mobile browsers now dynamically inherit safe-area paddings.
- **Developer Cycle Efficiency**: Desktop developers can instantly preview mobile layouts by dragging the window border to portrait dimensions.
- **Core Hygiene**: Layout components (`top_hud.rs`, `bottom_bar.rs`) remain free of platform-specific compile-time configuration gates and retain their single-responsibility styling logic. Modified files stay well under the strict 300-line source budget.
