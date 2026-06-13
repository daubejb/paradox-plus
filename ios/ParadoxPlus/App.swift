import SwiftUI

// Declare the Rust FFI function
@_cdecl("start_ios_client")
func start_ios_client()

@main
struct ParadoxPlusApp: App {
    init() {
        // Run the Bevy engine initialization on the main loop
        DispatchQueue.main.async {
            start_ios_client()
        }
    }
    
    var body: some Scene {
        WindowGroup {
            Text("Initializing Paradox Plus...")
        }
    }
}
