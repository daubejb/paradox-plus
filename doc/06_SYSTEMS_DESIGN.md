# ⛳ Paradox Plus: Systems Design Specification

This document maps the logical game rules of [PARADOX_GAME.md](file:///Users/jeff/Developer/paradox-plus/doc/PARADOX_GAME.md) and the networking models of [05_SYSTEMS_ARCHITECTURE.md](file:///Users/jeff/Developer/paradox-plus/doc/05_SYSTEMS_ARCHITECTURE.md) into concrete **Bevy ECS Architecture** blueprints. It specifies all Components, Resources, Events, Plugins, and file structures to guide modular, compile-safe implementation.

---

## 🛠️ 1. Bevy ECS Data Model

All states and parameters are modeled using standard Bevy `Component` or `Resource` structures. To comply with core hygiene guardrails, all position and math parameters use `fixed::types::I32F32` from the `fixed` crate.

### A. ECS Components (Entities)

Entity types are defined in `crates/protocol/src/lib.rs` (shared messages and types) or localized client/server modules:

```rust
use bevy::prelude::*;
use fixed::types::I32F32;
use heapless::String;
use crate::messages::MAX_PLAYER_NAME_LEN;

/// Unique marker for a player entity.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    pub player_id: u64,
}

/// Stores the authoritative fixed-point movement coordinates.
/// Read-only on the client; mutated strictly by the server.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ball {
    pub cell_index: u32,
    pub slide_offset: I32F32,     // Interpolated sub-cell sliding progress
    pub direction: MovementDirection,
    pub origin_cell: u32,         // Starting cell for the current shot (for OB resets)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MovementDirection {
    Forward,
    Reverse,
}

/// Identifies a wager card drafted on the board.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WagerToken {
    pub card_type: u8,            // 0: Guardian Shield, 1: Banana Slip, 2: Golden Die
    pub owner_id: u64,
    pub cell_index: u32,
}
```

### B. ECS Resources (Global States)

Resources represent match-level parameters, local configurations, and socket connections:

```rust
use bevy::prelude::*;
use heapless::Vec as HVec;
use crate::messages::{MAX_PLAYERS, Scorecard};

/// Holds the configuration of the current active hole track.
#[derive(Resource, Default, Clone, PartialEq, Eq)]
pub struct ActiveCourseTrack {
    pub hole_index: u8,
    pub par: u8,
    pub total_cells: u32,
    pub cells: Vec<TerrainType>, // Sized per-hole layout, loaded at transitions
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerrainType {
    TeeBox,
    Fairway,
    Rough,
    Bunker,
    Water,
    OutOfBounds,
    Green(u8), // Green zones g0 (Cup) to g3
}

/// Tracks lobby session standings and scores.
#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub struct MatchStandings {
    pub player_ids: HVec<u64, MAX_PLAYERS>,
    pub scorecards: HVec<Scorecard, MAX_PLAYERS>,
}

/// Holds the Quinn socket connection or WebTransport driver handle.
#[derive(Resource)]
pub struct NetworkClientConnection {
    pub connection_id: u64,
    pub out_sender: tokio::sync::mpsc::Sender<Vec<u8>>,
}
```

---

## 🔄 2. State Machine Coordination

We leverage Bevy's built-in `States` enum to drive FSM changes. System execution is gated strictly via the `OnEnter`, `OnExit`, and `in_state` decorators:

```rust
use bevy::prelude::*;
use crate::messages::GameStateEnum;

/// Registers the Bevy states that mirror ServerUpdate::StateSync.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum BevyGameState {
    #[default]
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

### Turn Lifecycle Gateways
* **Server FSM**: Runs authoritative checks. It transitions the state on valid action receipts (e.g. `ClientAction::RollDice` during `AwaitingTurn` triggers transition to `Rolling`).
* **Client Replication**: When a client receives `ServerUpdate::StateSync`, a system compares `ServerUpdate::StateSync::game_state` with Bevy's active state. If mismatched, it commands state transit:
  ```rust
  fn sync_state_from_server(
      update_event: EventReader<ServerUpdateEvent>,
      mut next_state: ResMut<NextState<BevyGameState>>,
  ) {
      for event in update_event.read() {
          if let ServerUpdate::StateSync { game_state, .. } = &event.payload {
              let target_state = map_enum_to_bevy_state(*game_state);
              next_state.set(target_state);
          }
      }
  }
  ```

---

## 📡 3. Event Channels & Packet Serialization

### A. ECS Event Triggers

Network events are converted into ECS event instances to decouple I/O drivers from gameplay systems:

```rust
use bevy::prelude::*;
use crate::messages::{ClientAction, ServerUpdate};

/// Dispatched by the network poller thread when a server update is received.
#[derive(Event, Debug, Clone)]
pub struct ServerUpdateEvent {
    pub payload: ServerUpdate,
}

/// Dispatched by the UI or local simulation systems to request action transmission.
#[derive(Event, Debug, Clone)]
pub struct ClientActionRequest {
    pub payload: ClientAction,
}
```

### B. Network Polling Architecture

QUIC socket channels run on an async thread managed by Quinn or WebTransport.
1. **Reception**: The async task receives binary slices, deserializes them using `postcard::from_bytes`, and passes the resulting `ServerUpdate` struct to a thread-safe `crossbeam_channel::Sender`.
2. **Bevy Integration**: A Bevy system running in the `PreUpdate` phase drains the receiver queue and dispatches `ServerUpdateEvent` instances into Bevy's ECS event channel.
3. **Transmission**: A Bevy system in the `PostUpdate` phase listens for `ClientActionRequest`, serializes payloads via `postcard::to_allocvec_cobs` (or fixed slices), and writes them to the QUIC outbound socket queue.

---

## 🎨 4. Responsive UI & Fixed-to-Float Translation

### A. `FixedToFloatPlugin` Presenter
Gameplay components (`Ball`) must only be mutated by server-state snapshots. Visual presentation (transforms) is managed by translation systems executing in Bevy's `PostUpdate` phase:

```rust
/// Translates fixed-point coordinate indices to float-based Transform nodes.
pub fn fixed_to_float_translation_system(
    query: Query<(&Ball, &mut Transform), Changed<Ball>>,
    course: Res<ActiveCourseTrack>,
) {
    for (ball, mut transform) in query.iter_mut() {
        // Calculate cell geometric coordinate from cell_index
        let cell_coord = get_cell_spatial_position(ball.cell_index);
        
        // Add sliding sub-cell offsets
        let slide_modifier = get_cell_spatial_vector(ball.cell_index, ball.direction);
        let offset = slide_modifier * ball.slide_offset.to_num::<f32>();
        
        // Smoothly update visual Bevy transform coordinates
        transform.translation.x = cell_coord.x + offset.x;
        transform.translation.y = cell_coord.y + offset.y;
    }
}
```

### B. UI Spawning and Node Trees (`bevy_ui` + Taffy)
To construct premium and highly responsive layouts:
* **UI Root Node**: Instantiated as a screen-filling grid container:
  ```rust
  Node {
      display: Display::Grid,
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      grid_template_columns: vec![GridTrack::flex(1.0)],
      grid_template_rows: vec![GridTrack::auto(), GridTrack::flex(1.0)],
      ..default()
  }
  ```
* **Card Selection Cards (Glassmorphic panels)**: Spawned inside the `MarkerPlacement` state. Responsive styling properties (flex columns, margins) reside strictly within `client/src/ui/layout/wager.rs`.
* **Micro-Animations**: Hover actions trigger scaling transforms (`Transform::from_scale`) and color shifts, smoothed via delta time interpolations in updating systems.

---

## 🤖 5. Off-Thread AI Solver Scheduling

To avoid blocking the main server or client threads, MDP solver sweeps execute off-thread.

```rust
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use crates::protocol::physics::StructuralEpoch;

/// Component holding the task handle.
#[derive(Component)]
pub struct ActiveMdpSolverTask {
    pub task: Task<Option<Vec<u8>>>,
    pub structural_epoch: StructuralEpoch, // Epoch at solver trigger
}

/// Triggers computation off-thread in the AsyncComputeTaskPool.
pub fn trigger_ai_solver_system(
    mut commands: Commands,
    active_bot_query: Query<Entity, With<ActiveBotTurn>>,
    structural_epoch: Res<StructuralEpoch>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    for bot_entity in active_bot_query.iter() {
        let epoch_snapshot = structural_epoch.clone();
        
        // Schedule Bellman sweep sweeps asynchronously
        let task = thread_pool.spawn(async move {
            run_bellman_sweeps(epoch_snapshot)
        });
        
        commands.entity(bot_entity).insert(ActiveMdpSolverTask {
            task,
            structural_epoch: structural_epoch.clone(),
        });
    }
}
```

* **Staleness verification**: A Bevy system checks active tasks for completion. If the task is finished, it compares `task.structural_epoch` with the current `StructuralEpoch`. If matched, the policy is unpacked and executed. If mismatched, the result is discarded and the greedy fallback is triggered.

---

## 📂 6. Modular Project File Boundaries

To enforce the **300-Line Limit** for Rust source files, code is partitioned as follows:

```
crates/
├── protocol/
│   └── src/
│       ├── lib.rs                 # [<100 lines] Re-exports, structural layout enums
│       ├── messages.rs            # [<150 lines] Bounded heapless ClientAction/ServerUpdate
│       ├── physics.rs             # [<250 lines] Bounded SlideTracker & SlideError
│       └── telemetry.rs           # [<150 lines] TelemetryHeader, HolePerformanceEvent
├── server/
│   └── src/
│       ├── main.rs                # [<200 lines] Command boots, Quinn configuration
│       ├── physics/
│       │   └── validation.rs      # [<250 lines] ClientAction validation logic
│       ├── ai/
│       │   ├── mod.rs             # [<200 lines] Systems coordinating AI task spawning
│       │   ├── mdp_state.rs       # [<150 lines] 1D cell discretization, state vectors
│       │   └── mdp_solver/
│       │       ├── mod.rs         # [<100 lines] Solver API definitions
│       │       ├── transitions.rs # [<200 lines] Transition distribution models
│       │       ├── iteration.rs   # [<200 lines] Dynamically gated Value Iteration sweeps
│       │       └── rewards.rs     # [<200 lines] Wager-aware utility matrix mapping
│       └── systems.rs             # [<250 lines] Authoritative server game loops
└── client/
    └── src/
        ├── main.rs                # [<200 lines] Bevy App builder, plugin setups
        ├── network.rs             # [<250 lines] QUIC/WebTransport async poller connection
        ├── ui/
        │   ├── mod.rs             # [<150 lines] UI state resource controllers
        │   └── layout/
        │       ├── mod.rs         # [<150 lines] Grid styling properties, node builders
        │       ├── turn_order.rs  # [<200 lines] Scorecard dashboard layouts
        │       └── wager.rs       # [<200 lines] Card wager panels selection layout
```
