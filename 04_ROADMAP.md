# Paradox Plus Development Roadmap

This document catalogs the active milestones, development backlog iterations, and retrospect logs for **Paradox Plus**. Development follows the 12-step iteration protocol described in [AGENTS.md](file:///Users/jeff/Developer/paradox-plus/AGENTS.md).

---

## 🗺️ Active Milestones

- [x] Phase 0: Game Specification Audit & Remediation (Completed 2026-06-10)
- [x] Phase 1: Systems Design & Critique Integration (Completed 2026-06-11)
- [ ] Phase 2: Iterative Development Backlog Execution

---

## 📦 Iterative Development Backlog

### [x] Iteration 1: Shared Protocol & Bounded Physics Loop (Completed 2026-06-11)
*   **Branch**: `feat/1-protocol-physics`
*   **Objectives**:
    - Implement serialization types (`ClientAction`, `ServerUpdate`, `Scorecard`) in `crates/protocol/src/messages.rs`.
    - Implement stack-allocated `SlideTracker` cycle check in `crates/protocol/src/physics.rs`.
    - Implement terrain resolution strategy math (Bunker odd/even escape, Water penalty, OB origin resets, Green automatic putts) in `crates/protocol/src/terrain.rs`.
*   **Verification**:
    - Unit tests in `crates/protocol/tests/physics_tests.rs`:
      - `test_postcard_serialization_bounds` (verifying size checks).
      - `test_slide_tracker_cycle_detection` (verifying +2 penalty on loop).
      - `test_terrain_strategies` (verifying Bunker odd/even escape, OB resets, and Water hazard scoring).

### [x] Iteration 2: Authoritative Server & Event Tick Loop (Completed 2026-06-11)
*   **Branch**: `feat/2-server-coordinator`
*   **Objectives**:
    - Boot Quinn UDP QUIC socket listener and WebTransport fallbacks in `crates/server/src/main.rs`.
    - Implement pre-allocated `NetworkSerializationBuffer` to prevent WASM stack overflow.
    - Implement the authoritative FSM Coordinator tick system (`crates/server/src/systems.rs`) verifying turn-gating rules and state transitions.
*   **Verification**:
    - Unit tests in `crates/server/tests/connection_tests.rs` & `crates/server/tests/fsm_tests.rs`:
      - `test_packet_serialization_without_alloc` (verifying pre-allocated heap serialization).
      - `test_unauthorized_action_rejection` (verifying client actions sent out of turn are rejected).

### [x] Iteration 3: Procedural AI Bot Decision Solver (Completed 2026-06-11)
*   **Branch**: `feat/3-ai-solver`
*   **Objectives**:
    - Implement 1D discretized cell mapping in `crates/server/src/ai/mdp_state.rs`.
    - Implement procedural transition matrix calculator (`get_transitions`) to avoid the 16 GB dense matrix RAM overhead.
    - Implement wager-aware rewards and loop damper checks (`triggered_wagers` vector) to prevent infinite loop exploitation under Wager Persistence settings.
    - Implement dynamic fixed-point Value Iteration sweeps capped at 150.
*   **Verification**:
    - Unit tests in `crates/server/src/ai/mdp_solver/`:
      - `test_solver_convergence` (verifying Bellman sweeps converge).
      - `test_solver_damper_prevention` (verifying bots do not get trapped in infinite score loops).

### [x] Iteration 4: Async AI Scheduling & Task Polling (Completed 2026-06-11)
*   **Branch**: `feat/4-ai-async-polling`
*   **Objectives**:
    - Implement off-thread AI solver task scheduler using Bevy's `AsyncComputeTaskPool` in `crates/server/src/ai/mod.rs`.
    - Implement Bevy non-blocking task poller loop and cooperative task cancellation checking for `StructuralEpoch` updates.
    - Implement Turn Timeout AI Takeover scheduler.
*   **Verification**:
    - Integration tests in `crates/server/tests/ai_async_tests.rs`:
      - `test_non_blocking_polling_loop` (verifying main thread doesn't lock).
      - `test_stale_task_cancellation` (verifying tasks are immediately dropped if epoch changes).
      - `test_turn_timeout_takeover` (verifying timeout takeover is triggered after 15 seconds).

### [x] Iteration 5: Network Client Replication & Visual Presenter (Completed 2026-06-11)
*   **Branch**: `feat/5-client-replication`
*   **Objectives**:
    - Boot client network poller thread and translate inbound packets to Bevy FSM state transitions.
    - Implement `FixedToFloatPlugin` and `BallVisualInterpolation` translating cell indices and slide offsets to visual float coordinates.
*   **Verification**:
    - Mock tests in `crates/client/tests/replication_tests.rs`:
      - `test_client_replication_sync` (verifying FSM replicates server).
      - `test_fixed_to_float_interpolation` (verifying smooth transform movements).
    - WASM target compilation verification:
      - `RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo check -p client --target wasm32-unknown-unknown` verifying WebAssembly compatibility.

### [x] Iteration 6: Responsive UI Layouts & Standings HUD (Completed 2026-06-11)
*   **Branch**: `feat/6-client-ui-layouts`
*   **Objectives**:
    - Build grid/flexbox screen spawner in `crates/client/src/ui/layout/mod.rs` using `bevy_ui` + Taffy.
    - Implement standings HUD, turn order indicators, and wager card draft panels.
*   **Verification**:
    - Headless Bevy UI tests:
      - `test_ui_node_hierarchy` (verifying grid alignments).
      - `test_wager_card_selection_interaction` (verifying UI actions dispatch `ClientActionRequest` events).

### Iteration 7: Telemetry Pipeline & Handicap Index Scorer
*   **Branch**: `feat/7-analytics-handicap`
*   **Objectives**:
    - Implement lock-free telemetry channel pipeline in `crates/server/src/db/profile.rs` using `crossbeam_channel`.
    - Implement DNF penalty scorer ($\text{Par}_h + 6$) to prevent rage-quit exploits.
    - Calculate global leaderboard Handicap Index using the best 8 of 20 matches.
*   **Verification**:
    - Scorer tests in `crates/server/tests/analytics_tests.rs`:
      - `test_handicap_scorer_with_dnf_penalty` (verifying DNF raises handicap).
      - `test_non_blocking_telemetry_drain` (verifying zero impact on tick rate).

---

## 📈 Retrospective Log

- **Remediation Phase:** Successfully critiqued and updated [PARADOX_GAME.md](file:///Users/jeff/Developer/paradox-plus/PARADOX_GAME.md) to address five core vulnerabilities (turn order asymmetry, physics sliding cycle deadlocks, non-Markovian MDP state spaces, terrain stroke ambiguities, and host migration race conditions). All architectural designs comply with Bevy native UI layouts, authoritative server validation, Postcard type-safe serialization, and the 300-line source file limit.
- **Creator Onboarding:** Integrated the [CREATOR_SETUP_AND_PROCESS.md](file:///Users/jeff/Developer/paradox-plus/doc/CREATOR_SETUP_AND_PROCESS.md) guide, documenting the 12-step operational state machine, automated plan critique tool, testing targets, and core engine guardrails tailored for the Paradox Plus Bevy codebase.
- **Systems Design Spec**: Drafted the [doc/06_SYSTEMS_DESIGN.md](file:///Users/jeff/Developer/paradox-plus/doc/06_SYSTEMS_DESIGN.md) document detailing discrete Bevy ECS coordinate representations, stack-allocated SlideTrackers, pre-allocated stack serialization, loops dampers, and cooperative async AI cancellation loops. Verified WASM target compatibility.
- **Iteration 4 (Async AI & Polling):** Completed implementation of off-thread AI solver execution using Bevy's `AsyncComputeTaskPool` with zero-allocation inputs, non-blocking polling loops, turn timeout takeover, and cooperative cancellation. Addressed Rust orphan rules via `CourseTrackResource` wrapper, and successfully configured multithreading feature inside Bevy dependencies. All tests pass.
- **Iteration 5 (Client Replication & Presenter):** Implemented client-side network polling and FSM replication with zero-heap frame loops. Built `FixedToFloatPlugin` translating fixed-point discrete positions into native float coordinates inside Bevy's `PostUpdate` phase, preserving read-only properties of gameplay-authoritative state. Handled WASM target constraints (Mutex-wrapped receivers, boxed heapless Vec buffers, and getrandom configuration flags) to ensure full compilation compatibility for `wasm32-unknown-unknown`. All tests pass.

