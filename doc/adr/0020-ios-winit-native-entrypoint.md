# ADR 0020: iOS Native Entrypoint bypassing SwiftUI App Wrapper

## Status
Approved

## Context
When running the Bevy application client on the iOS Simulator, the app initial launch would freeze. This was because the Xcode project wrapper used a SwiftUI `@main` application entrypoint (`App.swift`) that initialized a standard SwiftUI App lifecycle. Within this SwiftUI lifecycle, Bevy's entrypoint (`start_ios_client()`) was called asynchronously, which invoked Bevy's internal `app.run()`.

However:
1. SwiftUI's `@main` internally executes `UIApplicationMain`, setting up its own window and application event loop.
2. Bevy's `winit` windowing backend on iOS also executes `UIApplicationMain` and registers its own UIKit application delegate and window surface to run natively via Metal.
3. Running two competing event loops on the main thread led to a deadlock in `winit`'s initialization phase, resulting in a frozen application.

## Decision
We bypass SwiftUI entirely and utilize a raw Swift `main.swift` file as the sole entry point of the iOS app:
1. **Remove SwiftUI Wrapper**: Remove the `@main` attribute and delete `App.swift`.
2. **Top-Level main.swift**: Create `main.swift` in the iOS project containing direct top-level code executing the Bevy FFI bootstrapper `start_ios_client()` on the main thread.
3. **Winit Lifecycle Control**: This ensures `winit` is the first and only system to spin up `UIApplicationMain`, enabling it to cleanly bind its internal application delegate, manage lifecycle transitions (become active/resign active), and render natively via Metal.

## Consequences
- **Successful Launch**: The app boots cleanly in the simulator without freezes.
- **Improved Performance**: Eliminates the overhead of a SwiftUI runtime layer.
- **Metal Integration**: Bevy handles the window context and compiles shaders natively on the simulator GPU.
- **FFI Cleanliness**: The FFI bridge remains simple and standard.
