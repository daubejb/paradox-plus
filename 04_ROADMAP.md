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

### [x] Iteration 7: Game Selection Landing Page (Completed 2026-06-11)
*   **Branch**: `feat/7-landing-page`
*   **Objectives**:
    - Implement a responsive dark-green mobile-optimized landing screen spawner in `crates/client/src/ui/layout/landing.rs` loading branding assets.
    - Set up `ClientScreenState` managing state transition visibility to avoid per-frame Bevy/Taffy layout thrashing.
    - Implement gameplay-exit returning to landing page and authoritatively resetting loopback server state on `ClientAction::LeaveRoom`.
*   **Verification**:
    - Integration tests in `crates/client/tests/ui_tests.rs`:
      - `test_screen_state_transitions` (verifying state machine visibility display properties and click events).
      - `test_board_rebuild_on_hole_change` (verifying rebuilds function under new states).

### Iteration 8: Telemetry Pipeline & Handicap Index Scorer
*   **Objectives**:
    - Implement lock-free telemetry channel pipeline in `crates/server/src/db/profile.rs` using `crossbeam_channel`.
    - Implement DNF penalty scorer ($\text{Par}_h + 6$) to prevent rage-quit exploits.
    - Calculate global leaderboard Handicap Index using the best 8 of 20 matches.
*   **Verification**:
    - Scorer tests in `crates/server/tests/analytics_tests.rs`:
      - `test_handicap_scorer_with_dnf_penalty` (verifying DNF raises handicap).
      - `test_non_blocking_telemetry_drain` (verifying zero impact on tick rate).

### [x] Iteration 9: Scrolling Ticker style Leaderboard (Completed 2026-06-12)
*   **Objectives**:
    - Implement a scrolling ticker style leaderboard below the top menu/HUD.
    - Display player ranks (with circle badges), names, and scores relative to par updated only when a hole is completed (even par "E" before any completed holes).
    - Highlight active player with green pill background and outline.
    - Add horizontal auto-scrolling ticker behavior when track overflows container width.
    - Add a "View Full" button at the right.
*   **Verification**:
    - Headless Bevy UI test in `crates/client/tests/ui_tests.rs`:
      - `test_leaderboard_ticker_hierarchy_and_updates` (verifying container spawning, player entries sorting, par-relative completion score updates, and persistent scores during active play).
    - WASM target verification.

### [x] Iteration 10: 2D World Space Board Rendering Refactoring (Completed 2026-06-12)
*   **Objectives**:
    - Transition the gameplay board from Bevy UI flex layout to a 2D World Space (`bevy_sprite`) paradigm.
    - Spawn cells as 2D Sprite board tiles along a parametric capsule track.
    - Spawn a 2D camera viewport sync'd with Bevy UI spacers.
    - Implement distance-based screen-to-world click raycasting for draft interactions.
    - Support headless test execution fallback to logical coordinates.
*   **Verification**:
    - Headless Bevy unit tests in `crates/client/tests/ui_tests.rs`:
      - `test_capsule_geometry_calculations` (verifying geometry and rotation calculations).
      - `test_board_rebuild_on_hole_change` (verifying spawning and rebuilding cells).
      - `test_wager_card_selection_interaction` (verifying 2D click-raycast drafts).

### [x] Iteration 11: 1-Die Limit in the Rough & Sand Bunker (Completed 2026-06-12)
*   **Objectives**:
    - Restrict dice rolls to exactly 1 die when the player is in the Rough (unless overridden by their own active Guardian Shield) or in a Sand Bunker.
    - Hide the "Roll 2" button in the UI under these conditions.
    - Authoritatively clamp the dice count to 1 inside the offline loopback simulator.
    - Introduce type-safe `CardType` enum to replace magic `u8` numbers in both client and server crates.
*   **Verification**:
    - Integration tests in `crates/client/tests/loopback_tests.rs`:
      - `test_loopback_rough_dice_limit` (verifying dice clamp on Rough, and override when own Guardian Shield is active).
    - Verified all unit and integration tests compile and pass.
    - Verified WASM target compatibility check.

### [x] Iteration 12: Premium Poker Chip Player Token (Completed 2026-06-13)
*   **Objectives**:
    - Upgrade the 2D visual player ball marker to a premium poker chip style token featuring the player's first initial in the center.
    - Implement a `PlayerTokenAssets` Bevy resource to cache circle and rectangle meshes and color materials, avoiding asset duplication and memory leaks.
    - Isolate the spawner and resource definition in `token.rs` to maintain compliance with the 300-line budget limit.
    - Centered player's uppercase first initial in the center, and counter-rotated the text relative to the curved cell's rotation to keep it upright and readable.
*   **Verification**:
    - Gracefully support headless tests suite by checking asset resource availability in `FromWorld`.
    - Run all unit and integration tests (`cargo test`).

### [x] Iteration 13: Upright Custom Wager Markers (Completed 2026-06-13)
*   **Objectives**:
    - Upgrade the 2D visual wager markers (Guardian Shield, Trickster Banana, Golden Die) to high-fidelity upright custom shapes.
    - Implement a flat parent-child entity traversal query using `WagerVisual` to toggle visibility of the active wager type.
    - Guard change detection for both root and child visibility modifications to protect Bevy's rendering hierarchy cache.
    - Keep all modified client-side rendering files strictly under the 300-line budget limit.
*   **Verification**:
    - Run native and WASM target compilation checks.
    - Run all unit and integration tests (`cargo test`).

### [x] Iteration 14: Mobile Emulation & Build Commands (Completed 2026-06-13)
*   **Objectives**:
    - Configure the `client` package Cargo dependencies and targets to support library compilation (`staticlib`, `cdylib`, `rlib`) for mobile targets.
    - Implement modular FFI entry points inside a dedicated `crates/client/src/mobile.rs` module.
    - Scaffold a minimal native Xcode/Swift application wrapper under `ios/` to initialize and run the Bevy engine.
    - Add Makefile targets for starting Android emulators and iOS simulators dynamically.
    - Add build/deploy pipeline commands to Makefile to install to Android via USB-C and push to TestFlight via Transporter CLI.
*   **Verification**:
    - Build static library targets using cargo and run all workspace tests successfully.

### [x] Iteration 15: Mobile UI Layout & Aspect Ratio Adjustments (Completed 2026-06-13)
*   **Objectives**:
    - Prevent top HUD encroachment under the native device status bar notch.
    - Overhaul the bottom control buttons into a non-overlapping two-row Column configuration.
    - Avoid central board cropping on narrow portrait aspect ratios (e.g. Pixel 10 Pro).
    - Implement a runtime-responsive safe-area adjustment system to handle WASM mobile browsers and desktop window resizing.
*   **Verification**:
    - Run all unit and integration tests (`cargo test`).
    - Verify file line count budget limits (under 300 lines) for modified files.
    - Compile mobile targets via `make build-android` and `make build-iphone-sim`.

---


## 📈 Retrospective Log

- **Remediation Phase:** Successfully critiqued and updated [PARADOX_GAME.md](file:///Users/jeff/Developer/paradox-plus/doc/PARADOX_GAME.md) to address five core vulnerabilities (turn order asymmetry, physics sliding cycle deadlocks, non-Markovian MDP state spaces, terrain stroke ambiguities, and host migration race conditions). All architectural designs comply with Bevy native UI layouts, authoritative server validation, Postcard type-safe serialization, and the 300-line source file limit.
- **Creator Onboarding:** Integrated the [CREATOR_SETUP_AND_PROCESS.md](file:///Users/jeff/Developer/paradox-plus/doc/CREATOR_SETUP_AND_PROCESS.md) guide, documenting the 12-step operational state machine, automated plan critique tool, testing targets, and core engine guardrails tailored for the Paradox Plus Bevy codebase.
- **Systems Design Spec**: Drafted the [doc/06_SYSTEMS_DESIGN.md](file:///Users/jeff/Developer/paradox-plus/doc/06_SYSTEMS_DESIGN.md) document detailing discrete Bevy ECS coordinate representations, stack-allocated SlideTrackers, pre-allocated stack serialization, loops dampers, and cooperative async AI cancellation loops. Verified WASM target compatibility.
- **Iteration 4 (Async AI & Polling):** Completed implementation of off-thread AI solver execution using Bevy's `AsyncComputeTaskPool` with zero-allocation inputs, non-blocking polling loops, turn timeout takeover, and cooperative cancellation. Addressed Rust orphan rules via `CourseTrackResource` wrapper, and successfully configured multithreading feature inside Bevy dependencies. All tests pass.
- **Iteration 5 (Client Replication & Presenter):** Implemented client-side network polling and FSM replication with zero-heap frame loops. Built `FixedToFloatPlugin` translating fixed-point discrete positions into native float coordinates inside Bevy's `PostUpdate` phase, preserving read-only properties of gameplay-authoritative state. Handled WASM target constraints (Mutex-wrapped receivers, boxed heapless Vec buffers, and getrandom configuration flags) to ensure full compilation compatibility for `wasm32-unknown-unknown`. All tests pass.
- **Iteration 7 (Game Selection Landing Page):** Implemented responsive dark-green mobile-optimized landing screen. Managed layout transitions on entering respective states to prevent Taffy layout recalculation overhead on every update. Handled game exit safely by resetting authoritative offline server state and returning client view to landing. All tests pass.
- **Iteration 9 (Scrolling Ticker style Leaderboard):** Implemented a responsive scrolling ticker leaderboard showing player rank badges, names, and par relative scores updated only when holes are completed (Even Par "E" initially). Highlighted the active player, and implemented a Bevy UI autoscroll track when content exceeds container width. Extracted systems to `leaderboard.rs` to maintain strict compliance with the 300-line budget limit. All tests pass.
- **Iteration 10 (2D World Space Board Rendering):** Refactored the gameplay viewport to a 2D world space paradigm using sprite tiles on a parametric capsule track layout. Standardized screen-to-world raycasting distance checks for click drafting, and added headless testing fallback logic. Clean recursive despawning avoids WASM leaks. All tests pass.
- **Visual Refinement (Racetrack Tiles & Dividers):** Adjusted racetrack rendering logic so that green tile colors are precisely constrained within the radial divider boundaries, preventing corner bleeding. Removed boundary dot child sprites to clean up the track design and match the visual inspiration layout. All tests pass.
- **Iteration 11 (1-Die Limit in the Rough & Sand Bunker):** Enforced the 1-die roll restriction on Rough and Bunker terrain. Extracted the simulated roll handler into a new submodule `roll.rs` to respect the 300-line budget limit. Replaced magic card numbers with a type-safe `CardType` enum across all crates, ensuring bounded and type-safe serialization. All tests pass.
- **Visual Styling (Board Colors & Text Contrast):** Refactored cell tile background color configurations to distinguish the Green (lightest), Fairway (mid-shade), and Rough (darkest) green terrains. Extracted styling lookup logic into a new `style.rs` helper module to keep `spawning.rs` safely under the 300-line budget limit. Increased text label font size to `12.0` and implemented dynamic contrast adjustments (black on light terrains, white on dark terrains) to optimize legibility. All tests pass.
- **Track Widening & Chamfering:** Widened the horizontal span of the capsule track by increasing the midline radius calculation multipliers. Implemented a ray-casting intersection solver (`calculate_outer_point`) in `geometry.rs` to flatten the top and bottom outer perimeter of the track, matching the rectangular/chamfered look of the inspiration design while keeping the inner loop curved and the dividers radial. Resolved an in/out naming and direction vector swap in Bevy rendering variables to fix coordinate starburst artifacts. Added unit tests for intersection math. All tests pass.
- **Rounded Rectangle Track Geometry:** Replaced the capsule layout and raycasting chamfer calculations with a dedicated 8-segment portrait rounded rectangle trajectory for the midline path, producing perfectly straight vertical (left/right) and horizontal (top/bottom) walls and rounded corners (quarter-circles) on both the inside and outside of the track using clean parallel offsets. Resolved overlapping cell click hitboxes on smaller viewports by refactoring the click handler to select the closest cell under the threshold rather than the first matching element. All unit and integration tests compile and pass successfully.
- **Track Proportions Optimization:** Optimized track geometry dimensions to occupy the maximum width and height of the board viewport while maintaining a strict `0.68` portrait aspect ratio stadium layout. Encapsulated bounds calculations in a structured `TrackGeometry` model, scaling the corner radius by width and adding defensive clamping to prevent divide-by-zeros or negative segment parameters. Ensured Tee Box (index 0) remains fixed on the left vertical segment under all viewport scales. All tests passed.
- **Track Proportions and Width Expansion:** Expanded overall track width horizontally by increasing the portrait aspect ratio target to `0.85` and flattened the top and bottom straight walls by decreasing the corner radius coefficient to `0.22`. Increased track lane cell width (tile thickness) to `96.0`. Enforced safe minimum bounds to guarantee positive inner corner radius and prevent layout coordinate underflow. All tests passed.
- **Track Corner Radius Optimization:** Increased `RADIUS_COEFFICIENT` to `0.28` in `geometry.rs`, making the corner caps noticeably rounder and increasing the minimum inner corner radius to a smooth `27.6` units. Documented constant scopes for client-side rendering boundaries to clarify authoritative validation separation. All tests passed.
- **Curved Racetrack Corners:** Subdivided the outer/inner cell boundary coords and cell tile backgrounds into 8 subdivisions. Consolidated border curves into unified ribbon meshes (one outer, one inner) to reduce Bevy entity overhead from 640 down to 2. Extracted geometry calculations to `borders.rs` to maintain compliance with the 300-line budget limit. All tests pass.
- **Bunker Dice Choice Correction:** Fixed the Sand Bunker escape check to allow rolling 1 or 2 dice (per PARADOX_GAME.md rules). If 2 dice are chosen, the sum of the dice is checked (even sum escapes, odd fails). Rough terrain remains clamped to 1-die limit unless shielded. Added integration tests to verify both choices and limits. All tests pass.
- **Iteration 13 (Upright Custom Wager Markers):** Redesigned the wager markers on the 2D gameboard to use custom procedurally-generated upright tokens (blue crest shield polygon, yellow single banana crescent polygon, gold pip-die) scaled uniformly to 16.0 x 16.0. Implemented parent-relative flat visual toggling with visibility change-detection guards. All tests passed.
- **Scorecard Earned Cards Reset Bugfix:** Added `cards_earned_this_hole` tracker to the protocol `Scorecard` struct and client offline loopback server state to isolate cards earned on the current completed hole from the cumulative card inventory. Updated Bevy UI renderer to display only these cards on hole completion. All unit and integration tests compile and pass.
- **Banana Slide Clicks Interaction:** Fixed a bug where players were unable to click cells to slide 0-4 spaces after landing on a Trickster Banana. Registered `ActivePlayerId` resource in the client network replication plugin and refactored the board click system to resolve path clicks against reachable cells during `BananaChoice`, dispatching the `ChooseBananaSlide` action. All tests passed.
- **Match Completed Scorecard Screen:** Implemented the End of Round scorecard summary screen overlay with a detailed Front 9 / Back 9 hole breakdown. Tracked strokes history authoritatively using a bounded `heapless::Vec` in `Scorecard` messages and loopback states. Resolved a bug preventing play again or return to main menu buttons from functioning by correcting early returns on non-existent course presets. Added unit/integration tests and verified WASM target compatibility. All tests pass.
- **In-Progress Scorecard Toggle:** Renamed "VIEW FULL" button to "SCORECARD". Toggling it during active gameplay opens the in-progress scorecard overlay. Implemented a "BACK TO GAME" button to return to play, while hiding "PLAY AGAIN" and "MAIN MENU". Handled clean visibility toggling for HUD, central board, and bottom bar without dynamic heap allocations in the hot render loop. Extracted scorecard buttons layout to a dedicated `scorecard_buttons` module to strictly comply with the 300-line budget limit. All tests pass.
- **Iteration 15 (Mobile UI Layout & Aspect Ratio Adjustments):** Solved HUD safe area status bar overlap and bottom bar button clutter by transitioning to a dynamic safe-area update system based on Bevy window size query. Overhauled the bottom bar to a two-row column flex layout. Updated the board camera to `ScalingMode::AutoMin` to fit the rounded-rectangle layout vertically and horizontally on any viewport. Handled mobile WASM and desktop testing capabilities. Checked that all files conform to the strict 300-line budget limit. All tests pass.
- **iOS Simulator Configuration Fix (2026-06-13):** Resolved simulator installation failure caused by missing `CFBundleExecutable` key in `Info.plist`. Confirmed successful iOS build, bundle packaging, and simulator launch.
- **iOS Screen Resolution Alignment Fix (2026-06-13):** Resolved layout off-centering and edge clipping on iOS simulator by passing UIKit logical screen bounds from Swift to Bevy's window resolution configuration. Fixed HUD name update bug. All tests pass.
- **iOS App Icon Scaling Refinement (2026-06-13):** Resized the central Paradox die logo emblem in the app icon to a balanced 55% of the total icon dimensions using a custom PIL script. Blended the resized logo's RGB and Alpha channels using alpha-minimum blending to avoid boundary artifacts. Discarded simulator caching issues by performing a clean reinstall of the application. The app icon proportions now align with Apple's human interface guidelines and match other native apps.















