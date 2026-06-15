import UIKit

// Declare the Rust FFI function
@_silgen_name("start_ios_client")
func start_ios_client(_ width: Float, _ height: Float)

// Query logical screen size in points using UIKit
let bounds = UIScreen.main.bounds
let width = Float(bounds.size.width)
let height = Float(bounds.size.height)

// Call Bevy/Rust entry point directly on the main thread with logical screen dimensions.
start_ios_client(width, height)
