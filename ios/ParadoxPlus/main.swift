import Foundation

// Declare the Rust FFI function
@_silgen_name("start_ios_client")
func start_ios_client()

// Call Bevy/Rust entry point directly on the main thread.
// This allows winit's iOS backend to natively initialize UIApplication and manage the window lifecycle.
start_ios_client()
