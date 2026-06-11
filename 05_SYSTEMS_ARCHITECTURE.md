# Paradox Plus Systems Architecture

This document defines the core technical architecture, engine selections, UI styling strategies, and networking topologies for **Paradox Plus**.

---

## 🗺️ High-Level System Topology

```mermaid
graph TD
    subgraph Shared Logic (paradox_core)
        Rules[FSM & Game Rules Engine]
        Physics[Terrain Strategies & SlideTracker]
        Solver[MDP AI Bot Solver]
    end

    subgraph Dedicated Server (Online Mode)
        Server[Authoritative Quinn QUIC Server]
        Lobby[Lobby Matchmaker]
        Server --> Rules
        Server --> Solver
    end

    subgraph Client App (Offline Mode)
        ClientLocal[Bevy Engine Game Client]
        LocalThread[Local Game Loop]
        ClientLocal --> LocalThread
        LocalThread --> Rules
        LocalThread --> Solver
    end

    subgraph Client App (Online Mode)
        ClientOnline[Bevy Engine Game Client]
        Net[Async Quinn Net Poller]
        ClientOnline --> Net
    end

    Net -- Raw Action Request (Postcard) --> Server
    Server -- State Sync Broadcast (Postcard) --> Net
```

---

## 🛠️ The Tech Stack

| Component | Selected Technology | Rationale |
| :--- | :--- | :--- |
| **Engine & Core** | **Bevy Engine** (Pure Native & Web targets) | Bypasses DOM/HTML virtualization layers to render UI directly to Metal/Vulkan. Runs parallel ECS systems natively. |
| **Shared Game Logic** | **`paradox_core` Crate** | Houses the game rules, physics, and bot solvers, allowing identical execution locally (offline) or in the cloud (online). |
| **User Interface** | **`bevy_ui` + Taffy (Flexbox solver)** | Provides game-grade visual control (WGSL fragment shaders, particle effects, UI animations) inside Bevy's ECS scene graph. |
| **Serialization** | **Postcard** (Binary Protocol) | Combines the bandwidth efficiency of flat byte arrays with 100% compile-time type-safety for Rust payloads. |
| **Networking** | **Quinn** (QUIC on UDP) / WebTransport | Resolves UDP packet-loss delays via multi-stream multiplexing and cellular connection migration. |
| **Topology** | **Hybrid Authoritative** | Server-authoritative for online multiplayer, client-authoritative for local offline single-player. |

---

## 🚀 1. Engine & Graphics: Bevy Engine (Pure Native & Web)

To achieve maximum visual looks and performance with zero DOM (no HTML, CSS, JS, or WebViews), the engine is split into two primary compile pathways:

### Mobile Targets (`aarch64-apple-ios` & `aarch64-linux-android`)
* The entire game engine compiles directly to native machine instructions.
* **iOS Integration**: Rust code is compiled into a static library (`.a`) and linked inside a Xcode Swift project. Swift instantiates the window surface and hands control to Bevy.
* **Android Integration**: Rust code compiles into a dynamic library (`.so`) packaged using Android's NativeActivity.
* Graphics commands execute directly on the phone's native GPU interfaces (**Metal** for iOS, **Vulkan** for Android) via Bevy's `wgpu` backend.

### Web Target (`wasm32-unknown-unknown`)
* The same Rust codebase compiles to WebAssembly (using Trunk) to run inside web browsers, falling back to WebGL2 if WebGPU is unavailable.

---

## 🎨 2. User Interface: `bevy_ui` + Taffy (Flexbox)

For a premium user interface, we reject application-grade widgets (like egui or Slint) and build directly within Bevy's rendering context:

* **Taffy Solver**: UI nodes are positioned using Bevy's built-in Flexbox and Grid layout systems (powered by the Taffy crate), allowing responsive layouts for different mobile screen aspects.
* **Visual Effects**:
  * **WGSL Shaders**: Custom fragment shaders are attached to UI panels to render modern, dynamic backgrounds like frosted glass (glassmorphism), neon border glows, or moving gradients.
  * **Particle Integration**: GPU particle emitters are nested directly within UI component transforms (e.g. producing particle sparks when hovering or tapping buttons).
  * **ECS Animation**: Layout animations (sliding panels, scaling icons, fading alerts) are driven directly by Bevy systems mutating UI components over time.

---

## 📦 3. Serialization: Postcard (Binary Protocol)

Network data transmission requires a highly compact, type-safe serialization format:

* **Shared Crate**: We define shared network data structures inside a common `protocol` library crate:

```rust
// Spec for protocol/src/messages.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ClientAction {
    // Lobby Matching Actions
    CreateRoom,
    JoinRoom { code: String },
    LeaveRoom,
    
    // Marker Placement Phase
    DraftCard { card_type: u8, cell_index: u32 },
    SkipPlacement,
    
    // Normal Play Phase
    RollDice { dice_count: u8 },
    
    // Banana Choice Transition
    ChooseBananaSlide { step_count: u8 },
    
    // Game/UI alerts
    AcknowledgeAlert,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ServerUpdate {
    // Room Lobbies updates
    RoomCreated { code: String },
    RoomJoined { players: Vec<PlayerInfo> },
    
    // Game FSM State Sync
    StateSync {
        sequence: u64,
        game_state: GameStateEnum,
        active_player_id: u64,
        current_hole: u8,
        player_positions: Vec<u32>,
        player_scores: Vec<Scorecard>,
        placed_wagers: Vec<WagerToken>,
    },
    
    // Immediate action events (dice roll triggers, slide sequences, putting outcomes)
    DiceRollOutcome { roll_values: Vec<u8> },
    SlideTransition { path: Vec<u32> },
    AlertTriggered { alert_message: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PlayerInfo {
    pub player_id: u64,
    pub name: String,
    pub is_ready: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Scorecard {
    pub running_strokes: u16,
    pub total_strokes: u16,
    pub earned_cards: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WagerToken {
    pub card_type: u8,
    pub owner_id: u64,
    pub cell_index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStateEnum {
    Lobby,
    MarkerPlacement,
    AwaitingTurn,
    Rolling,
    Moving,
    BananaChoice,
    HazardAlert,
    HoleCompleted,
    MatchCompleted,
}
```

* **Postcard Driver**: Using `postcard`, these structs compile to extremely small binary arrays (utilizing varint encoding). 
* **Type Safety**: The compiler guarantees that if the server updates its schema, the client must compile against the matching version, preventing network data-drift crashes.

---

## 📡 4. Networking: Quinn (QUIC on UDP)

We leverage QUIC for all network packet transportation to resolve mobile cell tower switching latency:

* **Native Quinn**: Native mobile clients connect directly to the server's UDP socket via Quinn.
* **Browser WebTransport**: Web clients fall back to the browser's native `WebTransport` protocol, communicating with the server over HTTP/3 (QUIC-based).
* **Key Benefits**:
  * **Connection Migration**: Players can transition from Wi-Fi to cellular data without dropping their active QUIC session.
  * **Multiplexed Streams**: Out-of-band communication (such as lobby chat) runs on separate QUIC streams, ensuring that dropped packets in chat never block critical game physics updates.

---

## 👑 5. Topology: Hybrid Authoritative

The engine supports two distinct topologies to allow seamless playing against the computer on the device without network connection:

### A. Server-Authoritative (Online Multiplayer Mode)
* **The Server**: A headless, async Rust application running in the cloud.
* **Verification**: The client apps act as replication receivers. The server processes all turns, rolls the dice, evaluates card drops, and calculates score deltas.
* **Lobby Management**: Matchmaking is organized through room codes. Players create a room code on the server, and friends join the lobby outbound, completely bypassing NAT carrier firewalls.

### B. Client-Authoritative (Local Offline Mode)
* **Self-Contained Loop**: The client app loads `paradox_core` logic directly in-memory, acting as the authority. 
* **Zero Connectivity**: No UDP sockets or serialization pipelines are initialized.
* **Local Bot solver**: The local Bevy client triggers the MDP AI solver systems off-thread on the phone's native CPU async compute thread pools, feeding actions back into the local state machine directly.

---

## 📂 6. Modular Project File Layout

To ensure absolute compliance with the **300-Line Limit** for Rust source files, the project's logic and UI layouts are distributed across the following modular file structure:

### A. `protocol` (Shared Crate)
*   `src/lib.rs` (Crate boundary re-exports and shared enums)
*   `src/physics.rs` (Stack-allocated `SlideTracker` cycle detection and clamping)
*   `src/telemetry.rs` (Postcard telemetry struct schemas)

### B. `server` (Authoritative Game Server)
*   `src/main.rs` (Socket bootstrapper and core loop orchestrator)
*   `src/physics/validation.rs` (InputValidator and physics boundary checks)
*   `src/ai/mdp_state.rs` (1D cell discretization and state vectors)
*   `src/ai/mdp_solver/mod.rs` (AI computation scheduler and async pool manager)
*   `src/ai/mdp_solver/transitions.rs` (Stochastic transition matrix modeling)
*   `src/ai/mdp_solver/iteration.rs` (Pure mathematical Bellman update sweeps)
*   `src/ai/mdp_solver/rewards.rs` (Wager-aware and terrain-specific utility matrices)
*   `src/ai/systems.rs` (Bevy ECS game loop wrappers and event dispatchers)

### C. `client` (Bevy Game Client)
*   `src/main.rs` (Bevy UI plugin and canvas bootstrapper)
*   `src/ui/mod.rs` (UI state controllers)
*   `src/ui/layout/mod.rs` (Styling property constants and node spawners)
*   `src/ui/layout/turn_order.rs` (Bevy native node structures for player order and AIO)
*   `src/ui/layout/wager.rs` (Bevy native UI trees for card selection and drafting)

