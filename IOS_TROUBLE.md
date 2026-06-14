# iOS Launch Crash Troubleshooting Guide

This guide details the steps to identify the root cause of instant startup crashes on physical iOS devices (which do not occur in the iOS Simulator).

## Common Startup Crash Causes on iOS Hardware

1. **Metal / wgpu Graphics Initialization Panic**: Physical device GPUs enforce strict Metal validation rules that the simulator (which bridges to macOS Metal drivers) bypasses.
2. **Launch Thread/Lifecycle Crash**: SpringBoard (the iOS shell) will instantly terminate any application that blocks the main thread at launch or crashes during winit's early boot sequence.
3. **Missing Entitlement or Dyld Mismatch**: Dynamic library loading or provisioning errors that only trigger on physical hardware.

---

## Step-by-Step Diagnostics using Xcode

To find the exact crash reason immediately, follow these steps to run the application with the debugger attached:

1. **Connect your iPhone**: Connect your physical iOS device to your Mac via USB-C or Lightning.
2. **Open Xcode**: Open the Xcode project `ios/ParadoxPlus.xcodeproj` on your Mac.
3. **Select your iPhone Target**: In the top scheme bar, select your connected physical iPhone as the active run destination.
4. **Run the App**: Click the **Play / Run** button (or press `Cmd + R`) to compile and install the debug version on your device.
5. **Capture the Crash Log**:
   - Xcode will automatically halt execution at the exact line of code that triggered the crash.
   - Review the **Debug Console** in the bottom pane of Xcode for any Rust panic traces, Swift assertions, or dynamic linker error logs.
   - If the crash occurs before Xcode attaches the debugger, view the device logs by going to **Window > Devices and Simulators > Open Console** in the top menu bar.
