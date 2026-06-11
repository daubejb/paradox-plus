# Paradox Plus Workspace Sitemap

This sitemap registers the files, modules, and boundaries in the repository. To prevent code duplication and domain leakage, all agents must inspect this sitemap during **Step 2 (Select)** and update it during **Step 11 (Sync)**.

---

## 🗺️ Crate Topology

| Crate | Directory | Purpose | Detail Level |
| :--- | :--- | :--- | :--- |
| `client` | `crates/client/` | Bevy application client, input listeners, UI panels, Metal/Vulkan blitter. | High |
| `server` | `crates/server/` | Authoritative Rust game server, room-code matching, state validation. | High |
| `protocol` | `crates/protocol/` | Shared serialization structs, enums, network payload packages (Postcard). | High |
| `critique` | `tools/critique/` | Standalone CLI tool to review and critique implementation plans against codebase rules. | Low |

---

## 📦 Module Indices

### 1. `protocol` (Shared Network Types)
* `src/lib.rs`: Exports shared messages, actions, and structs.

### 2. `client` (Bevy Game Interface)
* `src/main.rs`: Entry point. Launches Bevy App, setups Winit and event loops.

### 3. `server` (Authoritative Game Server)
* `src/main.rs`: Entry point. Boots Quinn QUIC socket listener.

### 4. `critique` (Tooling Orchestrator)
* `src/main.rs`: Entry point. Locates active implementation plan, sends it to `agy`, and writes critique.

### 5. `doc` (Architectural Documentation)
* `doc/testing_strategy.md`: Multi-layered testing strategy (unit, regression, integration, headless Bevy UI).
* `doc/CREATOR_SETUP_AND_PROCESS.md`: Creator onboarding setup, operational state machine, and core guardrails.
* `doc/05_SYSTEMS_ARCHITECTURE.md`: Core system design, data serialization schemas, and network topologies.
* `doc/PARADOX_GAME.md`: The core gameplay specs, FSM states, terrain strategies, and AI solver.
* `README.md`: Entry-point repository overview, setup, and key documentation index.


