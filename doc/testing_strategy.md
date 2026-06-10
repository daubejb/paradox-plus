# Paradox Golf Testing Strategy

This document establishes the official multi-layered testing protocol and verification boundaries for all Paradox Golf crates (`client`, `server`, `protocol`).

---

## 🧪 1. Multi-Layered Test Topology

```
             ┌──────────────────────────────────────────────┐
             │       D. Bevy UI Headless Tests              │
             │       (Element queries, state assertions)    │
             └──────────────────────┬───────────────────────┘
                                    ▼
             ┌──────────────────────────────────────────────┐
             │       C. Host Migration Integration          │
             │       (Mock connections, action replays)     │
             └──────────────────────┬───────────────────────┘
                                    ▼
             ┌──────────────────────────────────────────────┐
             │       B. AI Bot MDP Policy Regression        │
             │       (Value iteration output stability)     │
             └──────────────────────┬───────────────────────┘
                                    ▼
             ┌──────────────────────────────────────────────┐
             │       A. Unit Testing (Crate Isolation)      │
             │       (Slide cycles, score mutations, sync)  │
             └──────────────────────────────────────────────┘
```

---

## 📦 2. Test Specifications

### A. Unit Testing (Local Verification)
Unit tests must verify logic in isolation and be runnable with a standard `cargo test`.
1. **Deterministic Physics & Slide Clamping:**
   *   Test targets: `protocol/src/physics.rs`
   *   Test cases: Verify that a sequence of slide triggers exceeding 16 moves is clamped to Space 1 and records a $+2$ Stroke penalty.
   *   Verify that pushbacks going below Space 1 are clamped to Space 1 and toggle player direction to `'forward'`.
2. **Scorecard Mutation Operations:**
   *   Test targets: `server/src/physics/validation.rs`
   *   Test cases: Assert that terrain landing states increment $S_{shot}$ and $S_{penalty}$ correctly according to the vocabulary specification table.
3. **Serialization Consistency:**
   *   Test targets: `protocol/src/migration.rs`
   *   Test cases: Validate round-trip Postcard serialization and deserialization of `MigrationHandshake` and `MigrationStatePayload` to ensure zero padding leaks or target mismatches.

### B. AI Bot Policy Regression
To guarantee the AI solver remains stable and optimal when pathfinding variables adjust.
1. **Solver Convergence Checks:**
   *   Verify that `server/src/ai/mdp_solver/iteration.rs` completes value iteration and converges in $\le 50$ sweeps on standard hole configurations.
2. **Heuristic Fallback Verification:**
   *   Simulate a policy invalidation (epoch mismatch) and assert that the bot immediately and safely switches execution to the 1D6 greedy fallback action.

### C. Multiplayer Integration (Host Migration)
Verify client-server synchronization under packet loss or sudden host disconnects.
1. **Authoritative Input Buffering:**
   *   Spawn a mock server and two mock clients. Force a host migration.
   *   Verify that the newly promoted host successfully processes the clients' `MigrationHandshake` buffers, rejects invalid sequence numbers, and executes a correct fast-forward catch-up.
   *   Assert that clients lock their input with the `MigrationOverlay` UI block if migration time exceeds 120 frames (2 seconds).

### D. UX/UI Native Testing (Bevy App Harness)
UI validation must occur natively inside Bevy ECS without web browsers.
1. **DOM-Free Rendering Harness:**
   *   Instantiate `App` in headless mode within tests.
   *   Query the ECS hierarchy for the existence of `bevy_ui` components (e.g. `Node`, `Text` components or the custom `MigrationOverlayMarker`).
   *   Simulate click events by sending mock `Interaction::Pressed` events and asserting state mutations.

---

## 📈 3. Roadmap Integration Tasks
To track development progress, we list these validation steps in `04_ROADMAP.md`:
*   `[ ]` Implement local unit tests for `SlideTracker` cycle resolutions in `protocol/src/physics.rs`.
*   `[ ]` Implement AI solver policy regression test suite in `server/src/ai/mdp_solver/`.
*   `[ ]` Build mock host migration integration runner in `server/tests/`.
*   `[ ]` Create headless Bevy UI component verification tests in `client/tests/`.
