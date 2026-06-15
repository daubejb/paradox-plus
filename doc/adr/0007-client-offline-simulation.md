# ADR 0007: Client-Side Offline Loopback Server Simulation

## Context
We need to support an offline, local single-player gameplay mode on the Green Course (Hole 1). In this mode, the client needs to process dice rolling, bunker escape resolutions, standard hazard penalties (Water, Out-of-Bounds resets), and putting strokes.

To prevent rules drift between offline and online play, the offline simulation must run the exact same state machine and terrain resolutions as the authoritative server. However, implementing a background OS thread (e.g. via `std::thread::spawn`) and blocking channel operations (e.g. `rx.recv()`) for the local server simulation loop fails on the `wasm32-unknown-unknown` target because web browsers do not support native OS threads or blocking channel calls.

## Decision
To ensure 100% compatibility across both WebAssembly and native platforms, we made the following decisions:
1. **System-Based Loopback Server:** The local offline server loop runs as a non-blocking system (`local_offline_server_system`) in Bevy's `PreUpdate` schedule on the main thread, rather than on a background OS thread.
2. **Non-Blocking Channels:** Communication between the Bevy client systems and the local server system is routed through std `mpsc` channels using non-blocking `try_recv()` calls.
3. **Zero-Allocation Postcard Parity:** The loopback channel transmits serialized postcard bytes, preserving end-to-end network code path validation. To eliminate WASM heap churn, the loopback system uses a pre-allocated serialization buffer (`send_buf: Mutex<Vec<u8>>`) with a capacity of 65,536 bytes.
4. **Submodule Separation:** To respect the 300-line budget limit, the simulation logic is split into granular files:
   - `loopback/mod.rs` (main PreUpdate loop)
   - `loopback/state.rs` (local server FSM state)
   - `loopback/handlers/` (dice rolls, putting, bunker escapes, and overshoot/undershoot movement direction laws).

## Consequences
* **WASM Target Safety:** The offline gameplay mode compiles and executes flawlessly on WebAssembly targets and browser sandboxes without crashing or requiring native thread APIs.
* **Zero Rules Drift:** Both offline play and online play utilize the identical postcard messages and the identical FSM/terrain resolutions defined in `crates/protocol`.
* **Zero Heap Churn:** Pre-allocated serialization buffers eliminate garbage collection pressure on the WASM heap during active gameplay ticks.
* **Testing Code-Path Realism:** The network serialization code paths are validated end-to-end since the loopback system serializes and deserializes payloads exactly like the QUIC network implementation.
