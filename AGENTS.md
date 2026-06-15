# Antigravity Workspace Execution Profile: Paradox Plus (Bevy Edition)

You must strictly execute all development tasks according to the following 12-step state machine. You are forbidden from jumping directly to implementation (Step 7) without publishing an explicit implementation plan and obtaining user approval first.

## The Iteration Protocol
1. **Discover:** Read `/04_ROADMAP.md` at the root to check active milestone tasks.
2. **Select:** Inspect `/MODULE_MAP.ron` to verify where target crates, helper modules, and structs reside (to prevent re-implementing existing structures).
3. **Map:** Verify which subsystem (`client`, `server`, `protocol`) is impacted by the change.
4. **Audit Tasks:** Read and audit the active `task.ron` schema inside the active conversation directory to identify outstanding requirements.
5. **Plan:** Write a formal "Implementation Plan" as a markdown artifact containing discrete checkbox tasks. The plan MUST include a "Verification Plan" detailing the specific Unit, Regression, or Integration tests that will validate the proposed changes, and a mandatory checklist task checking for system-level design pivots requiring Architecture Decision Records (ADRs). Run `make critique-plan` to execute the automated plan critique and output `implementation_plan_critique.md` in the active conversation directory. Address any identified concerns in the plan.
6. **Guard:** Run local checks (e.g. `cargo check`). Stop and obtain user approval on the plan before proceeding.
7. **Execute:** Modify code files carefully. Maintain existing comments/documentation.
8. **Verify:** Run standard unit tests (`cargo test`) and compile checks (`cargo check -p client -p protocol --target wasm32-unknown-unknown`) to verify WASM and native target compatibility. If checks or tests fail:
    *   Do **NOT** run destructive git commands (e.g. `git checkout -- .` or `git reset`).
    *   Preserve the failed changes using `git stash save "automatic-stash-before-retry"`.
    *   Return to **Step 7** to correct the implementation.
    *   If failures exceed 3 consecutive attempts, output compilation logs and test outputs to `FATAL_ERROR.md` and halt execution.
9. **Post-Audit:** Verify code changes against Section 2 Core Hygiene Guardrails.
10. **Record:** If any system-level design pivots occurred, write an Architecture Decision Record under `/doc/adr/`.
11. **Sync Logs & Sitemap:** Check off completed items in `/04_ROADMAP.md` and update `task.ron`. Update `/MODULE_MAP.ron` to document added, modified, or deleted files. Log a brief retrospective.
12. **Ship:** Execute Git commit with descriptive messages, push upstream, and exit.

## Section 2: Core Hygiene Guardrails
*   **300-Line Limit & Submodule Separation:** No individual Rust source file (excluding test suites or benchmarks) may exceed 300 lines of code. Split files into granular submodules (e.g., `mod.rs`, `components.rs`, `systems.rs`, `events.rs`) within a module folder when they approach this limit. Verify file sizes using `wc -l`.
*   **Pure Rust ECS (No DOM):** Absolutely never use HTML, CSS, JavaScript, WebViews, or DOM elements. The inclusion of DOM-specific crates (e.g., `web-sys`, `std::web`) in client packages is strictly prohibited. All UI representation, rendering, and interaction must occur natively inside Bevy (`bevy_ui` + Taffy + WGSL shaders).
*   **Authoritative Server Validation & Fixed-Point Math:** All gameplay state mutations, card draws, and movement resolutions must be evaluated on the authoritative Server. The Client only renders interpolated states. To prevent cross-platform desyncs (Native vs. WASM), all simulation/physics calculations must use fixed-point arithmetic (`fixed::types::I32F32` from the `fixed` crate) or integer math. Floating-point operations (`f32`/`f64`) are strictly banned in gameplay logic. Because fixed-point multiplication/division is susceptible to overflow, game logic must use safe arithmetic operations (like `checked_add`, `checked_mul`, or `saturating_mul`) instead of standard math operators (`+`, `*`).
*   **Fixed-to-Float Render Bridge:** The client-side presentation mapping must explicitly translate fixed-point positions into Bevy's native floating-point `Transform` components via a read-only translation system (`FixedToFloatPlugin`) in Bevy's `PostUpdate` phase. Under no circumstances may client-side render systems write to or mutate authoritative fixed-point coordinate state components.
*   **WASM Safety & Allocation Optimization:** Client crates must register `console_error_panic_hook::set_once()` upon entry. To prevent WASM heap exhaustion, avoid custom dynamic heap allocations (such as instantiating raw `Vec`, `HashMap`, or `.clone()` calls) inside hot loop systems. Reuse collection vectors by utilizing Bevy's `Local<Vec<T>>` resources, and ensure they are explicitly cleared (`vec.clear()`) at the start or end of each system run to prevent stale state bugs and memory leaks. Defer game state transitions to playing modes until all asset handles are completely loaded.
*   **Type-Safe & Bounded Serialization:** All network payloads must be serialized/deserialized using `Postcard` and compile-time verified structs/enums shared in the `protocol` crate. Enforce a maximum packet size of 64KB (`MAX_PACKET_SIZE = 65536`) at the connection layer. All serialized types must use bounded sizes (e.g. `heapless::Vec`, `heapless::String`, or static arrays) to prevent deserialization memory attacks. Deserialization errors must drop the connection/packet safely without panicking.
*   **Test-Driven Architecture (TDA):** Every physics mutation, scorecard operation, AI solver feature, or serialization payload must have a corresponding test target in the same PR/commit. No logic changes may be shipped without matching unit or integration coverage as specified in `/doc/testing_strategy.md`.
