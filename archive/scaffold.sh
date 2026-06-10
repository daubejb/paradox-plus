#!/bin/bash
set -e

echo "🚀 Bootstrapping paradox-plus workspace..."

# 1. Create directory structure
mkdir -p doc/adr
mkdir -p crates/client/src
mkdir -p crates/server/src
mkdir -p crates/protocol/src

# 2. Initialize git
git init

# 3. Create Cargo.toml Workspace root
cat << 'EOF' > Cargo.toml
[workspace]
resolver = "2"
members = [
    "crates/client",
    "crates/server",
    "crates/protocol",
]
EOF

# 4. Create dummy main/lib files so it immediately compiles
echo 'fn main() { println!("Client running!"); }' > crates/client/src/main.rs
echo 'fn main() { println!("Server running!"); }' > crates/server/src/main.rs
echo '// Shared message and protocol structs' > crates/protocol/src/lib.rs

# 5. Populate AGENTS.md
cat << 'EOF' > AGENTS.md
# Antigravity Workspace Execution Profile: Paradox Plus (Bevy Edition)

You must strictly execute all development tasks according to the following 12-step state machine. You are forbidden from jumping directly to implementation (Step 7) without publishing an explicit implementation plan and obtaining user approval first.

## The Iteration Protocol
1. **Discover:** Read `/04_ROADMAP.md` at the root to check active milestone tasks.
2. **Select:** Inspect `/MODULE_MAP.md` to verify where target crates, helper modules, and structs reside (to prevent re-implementing existing structures).
3. **Map:** Verify which subsystem (`client`, `server`, `protocol`) is impacted by the change.
4. **Audit Tasks:** Read the subsystem's specific `TASK.md` (or the brain's `task.md`) for outstanding checklists.
5. **Plan:** Write a formal "Implementation Plan" as a markdown artifact containing discrete checkbox tasks.
6. **Guard:** Run local checks (e.g. `cargo check`). Stop and obtain user approval on the plan before proceeding.
7. **Execute:** Modify code files carefully. Maintain existing comments/documentation.
8. **Verify:** Run `cargo test` and verify compile targets (`wasm32` and native).
9. **Post-Audit:** Verify code changes against Section 2 Core Hygiene Guardrails.
10. **Record:** If any system-level design pivots occurred, write an Architecture Decision Record under `/doc/adr/`.
11. **Sync Logs & Sitemap:** Check off completed items in `/04_ROADMAP.md` and the subsystem checklists. Update `/MODULE_MAP.md` to document added, modified, or deleted files. Log a brief retrospective.
12. **Ship:** Execute Git commit with descriptive messages, push upstream, and exit.

## Section 2: Core Hygiene Guardrails
* **300-Line Limit:** No individual Rust source file (excluding test suites or benchmarks) may exceed 300 lines of code. Split files into granular modules or components if they approach this limit.
* **Pure Rust ECS (No DOM):** Absolutely never use HTML, CSS, JavaScript, WebViews, or DOM elements. All UI representation, rendering, and interaction must occur natively inside Bevy (`bevy_ui` + Taffy + WGSL shaders).
* **Authoritative Server Validation:** All gameplay state mutations, card draws, and movement resolutions must be evaluated on the authoritative Server. The Client only renders interpolated states and sends user-action requests.
* **Type-Safe Serialization:** All network payloads must be serialized/deserialized using `Postcard` and compile-time verified structs/enums shared in the `protocol` crate.
EOF

# 6. Populate MODULE_MAP.md
cat << 'EOF' > MODULE_MAP.md
# Paradox Plus Workspace Sitemap

This sitemap registers the files, modules, and boundaries in the repository. To prevent code duplication and domain leakage, all agents must inspect this sitemap during **Step 2 (Select)** and update it during **Step 11 (Sync)**.

---

## 🗺️ Crate Topology

| Crate | Directory | Purpose | Detail Level |
| :--- | :--- | :--- | :--- |
| `client` | `crates/client/` | Bevy application client, input listeners, UI panels, Metal/Vulkan blitter. | High |
| `server` | `crates/server/` | Authoritative Rust game server, room-code matching, state validation. | High |
| `protocol` | `crates/protocol/` | Shared serialization structs, enums, network payload packages (Postcard). | High |

---

## 📦 Module Indices

### 1. `protocol` (Shared Network Types)
* `src/lib.rs`: Exports shared messages, actions, and structs.

### 2. `client` (Bevy Game Interface)
* `src/main.rs`: Entry point. Launches Bevy App, setups Winit and event loops.

### 3. `server` (Authoritative Game Server)
* `src/main.rs`: Entry point. Boots Quinn QUIC socket listener.
EOF

# 7. Create skeleton process files
echo '# Paradox Plus Roadmap' > 04_ROADMAP.md
echo '# Paradox Game Specification' > PARADOX_GAME.md
echo 'Created basic files.'

echo "✅ Bootstrapped! Run 'chmod +x scaffold.sh' and clean up the script when done."
