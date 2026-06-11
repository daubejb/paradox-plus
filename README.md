# ⛳ Paradox Plus

**Paradox Plus** is a multiplayer, tactical golf-duel game built natively in **Rust** using the **Bevy Engine**. 

It combines traditional golf mechanics (such as standard dice rolling, hazards, and automatic putting) with dynamic card-wager prophecies and traps drafted onto a linear grid board.

---

## 📂 Workspace Structure

The project is structured as a Cargo workspace with partitioned responsibilities:

*   **`crates/client/`**: The frontend WASM/Native Bevy application. Manages input listeners, UI panels, and transform animations. (Targets `wasm32-unknown-unknown` and native).
*   **`crates/server/`**: The authoritative game server listening on Quinn QUIC sockets. Evaluates state mutations, slide physics cascades, card wagers, and AI bot solvers.
*   **`crates/protocol/`**: The shared serialization boundaries containing type-safe network structs/enums serialized via Postcard.
*   **`tools/critique/`**: Command-line developer tool for verifying implementation plans against architectural rules.

---

## 🛠️ Development & Tooling Commands

### 1. Planning Critique
Before executing any development tasks, write your implementation plan in the brain folder and run:
```bash
make critique-plan
```
This builds and executes the automated critique CLI to verify rule compliance.

### 2. Cargo Building & Checking
Check all packages:
```bash
cargo check
```

Check client/protocol compilation for WASM:
```bash
cargo check --target wasm32-unknown-unknown -p client -p protocol
```

### 3. Running local tests
Run the deterministic physics, cycle validations, and serialization checks:
```bash
cargo test
```

---

## 📖 Key Documentation Links

*   [doc/PARADOX_GAME.md](file:///Users/jeff/Developer/paradox-plus/doc/PARADOX_GAME.md) - The core system specs, math models, and physics algorithms.
*   [doc/05_SYSTEMS_ARCHITECTURE.md](file:///Users/jeff/Developer/paradox-plus/doc/05_SYSTEMS_ARCHITECTURE.md) - Core system design, data serialization schemas, and network topologies.
*   [doc/CREATOR_SETUP_AND_PROCESS.md](file:///Users/jeff/Developer/paradox-plus/doc/CREATOR_SETUP_AND_PROCESS.md) - Creator onboarding setup, operational state machine, and core guardrails.
*   [doc/testing_strategy.md](file:///Users/jeff/Developer/paradox-plus/doc/testing_strategy.md) - The multi-layered test specs (unit, AI, integration, and Bevy UI).
*   [04_ROADMAP.md](file:///Users/jeff/Developer/paradox-plus/04_ROADMAP.md) - Active milestones, milestones progress, and retrospects.
*   [MODULE_MAP.md](file:///Users/jeff/Developer/paradox-plus/MODULE_MAP.md) - Site sitemap registering file boundaries to prevent domain leakage.
*   [AGENTS.md](file:///Users/jeff/Developer/paradox-plus/AGENTS.md) - Repository development rules and core hygiene guardrails.
