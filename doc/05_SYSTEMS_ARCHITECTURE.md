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
use heapless::{Vec as HVec, String as HString};

// Compile-time size boundaries for zero-heap serialization safety
pub const MAX_PLAYERS: usize = 8;
pub const MAX_WAGERS: usize = 16;
pub const MAX_PATH_STEPS: usize = 32;
pub const MAX_ROOM_CODE_LEN: usize = 6;
pub const MAX_PLAYER_NAME_LEN: usize = 16;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ClientAction {
    // Lobby Matching Actions
    CreateRoom,
    JoinRoom {
        code: HString<MAX_ROOM_CODE_LEN>,
        name: HString<MAX_PLAYER_NAME_LEN>,
    },
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
    RoomCreated { code: HString<MAX_ROOM_CODE_LEN> },
    RoomJoined { players: HVec<PlayerInfo, MAX_PLAYERS> },
    
    // Game FSM State Sync
    StateSync {
        sequence: u64,
        game_state: GameStateEnum,
        active_player_id: u64,
        current_hole: u8,
        player_positions: HVec<u32, MAX_PLAYERS>,
        player_scores: HVec<Scorecard, MAX_PLAYERS>,
        placed_wagers: HVec<WagerToken, MAX_WAGERS>,
    },
    
    // Immediate action events (dice roll triggers, slide sequences, putting outcomes)
    DiceRollOutcome { roll_values: HVec<u8, 2> }, // Max 2 dice
    SlideTransition { path: HVec<u32, MAX_PATH_STEPS> },
    AlertTriggered { alert_message: HString<64> },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PlayerInfo {
    pub player_id: u64,
    pub name: HString<MAX_PLAYER_NAME_LEN>,
    pub is_ready: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Scorecard {
    pub running_strokes: u16,
    pub total_strokes: u16,
    pub earned_cards: HVec<u8, 4>, // Max 4 earned cards
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
 
* **Postcard Driver**: Using `postcard`, these structs serialize to extremely compact binary slices using varint encoding, guaranteeing high efficiency over mobile connections.
* **Type Safety & Heap Protection**: The usage of `heapless` structures protects both native and WASM target loops from heap thrashing and prevents Denial of Service (OOM) attacks from malformed packet length prefixes.
 
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

### C. Turn Timeout & AI Takeover
To prevent a disconnected or inactive player from blocking match progression during consecutive play on a hole:
* **Detection**: If the active player fails to submit a gameplay action within 30 seconds, the server triggers a turn timeout.
* **Background Solving**: The server schedules the `MdpSolver` task asynchronously in the server's task pool to prevent blocking the main event tick loop.
* **Action Execution**: The calculated optimal action is executed automatically on behalf of the player, allowing the match to proceed.

---

## 🧱 7. Authoritative Physics & Slide Engine Rules

To prevent client/server simulation desyncs and eliminate infinite loops and array overflow panics, the physics engine enforces the following constraints:

### A. Bounded & Encapsulated `SlideTracker`
To prevent out-of-bounds panics on the fixed-size visited cells array in `protocol/src/physics.rs`, fields are encapsulated. Cell recording is bounded strictly by `MAX_SLIDES` (16) to guarantee safe array access even if caller loops fail to terminate.
* If a cell is visited twice in a slide sequence or the limit is exceeded, a cycle error is returned, resetting the player to Space 1 with a $+2$ Stroke penalty.

### B. Deferred Direction Toggles
Overshoot and Undershoot laws (toggling direction between `forward` and `reverse` relative to the Green) are **deferred** and evaluated only once at the very end of a player's movement turn (when the ball has come to a rest), rather than mid-slide. This prevents complex multi-card slide sequences from entering infinite direction-toggling oscillation loops.

### C. Banana Slip Search Boundaries
When a player lands on an opponent's Trickster (Banana Slip) card, they are pushed back 4 spaces (opposite of current movement direction). If the cell is occupied, the player slides forward (in current movement direction) tile-by-tile.
* To prevent the trap from acting as an accelerator (sliding the player past their original pre-trap starting space), the search range is strictly capped at `player_pos - 1`. If the search reaches the triggering cell index without finding a free space, the slide resolves to a fallback reset (Space 1) with a $+2$ stroke penalty.

---

## 🤖 8. AI Solver Epoch & Convergence Rules

To guarantee mathematically sound bot solver decisions on all target platforms while avoiding thread-starvation, the solver operates under these guidelines:

### A. Structural State Epoch Validation
The `MdpSolverTask` uses a `StructuralEpoch` snapshot instead of a global server state epoch:
* `StructuralEpoch` tracks only the physical layout (player positions, scorecards, placed wager tokens).
* Out-of-band updates (such as chat logs or connection status changes) do not invalidate the solver's active tasks, preventing AI starvation under active lobbies.

### B. Dynamic Bellman Convergence
The MDP solver operates over a 1D discretized state space $s = (x_{cell}, d, x_{origin\_cell})$ solved via Value Iteration sweeps using **fixed-point arithmetic** (`fixed::types::I32F32` from the `fixed` crate).
* Sweeps do not run on a hard-coded limit. The solver runs dynamic sweeps until the maximum value difference (Bellman residual) $\Delta V < \text{CONVERGENCE_EPSILON}$ (representing $0.01$ strokes in fixed-point, `I32F32::from_bits(42949673)`), capped by a safety maximum of `150` sweeps. This ensures correct hazard risk evaluations on complex holes with OB reset coupling.

---

## 📊 9. Resilient Analytics & Handicap Calculation

The server teleboards and Handicap Index algorithms are guarded against competitive exploits:

### A. Incomplete Matches (DNF) Penalty
If a player disconnects or quits a match early, their scorecard is completed by the server assigning a **Default DNF Penalty Score** of $\text{Par}_h + 6$ strokes for each unplayed hole.
* This generates a very high Hole Differential ($D_h$) for the match, ensuring the match is not selected in the player's "best 8" index.
* However, the match remains registered in the player's running "last 20" matches, pushing out older, better scores and increasing their Handicap Index, eliminating the exploit of rage-quitting to freeze competitive standings.

---
 
## 📂 10. Modular Project File Layout
 
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
*   `src/ai/mdp_solver/iteration.rs` (Pure mathematical Bellman update sweeps using fixed-point arithmetic)
*   `src/ai/mdp_solver/rewards.rs` (Wager-aware and terrain-specific utility matrices)
*   `src/ai/systems.rs` (Bevy ECS game loop wrappers and event dispatchers)
 
### C. `client` (Bevy Game Client)
*   `src/main.rs` (Bevy UI plugin and canvas bootstrapper)
*   `src/ui/mod.rs` (UI state controllers)
*   `src/ui/layout/mod.rs` (Styling property constants and node spawners)
*   `src/ui/layout/turn_order.rs` (Bevy native node structures for player order and AIO)
*   `src/ui/layout/wager.rs` (Bevy native UI trees for card selection and drafting)

