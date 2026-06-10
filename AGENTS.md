# Antigravity Workspace Execution Profile: Paradox Plus (Bevy Edition)

You must strictly execute all development tasks according to the following 12-step state machine. You are forbidden from jumping directly to implementation (Step 7) without publishing an explicit implementation plan and obtaining user approval first.

## The Iteration Protocol
1. **Discover:** Read `/04_ROADMAP.md` at the root to check active milestone tasks.
2. **Select:** Inspect `/MODULE_MAP.md` to verify where target crates, helper modules, and structs reside (to prevent re-implementing existing structures).
3. **Map:** Verify which subsystem (`client`, `server`, `protocol`) is impacted by the change.
4. **Audit Tasks:** Read the subsystem's specific `TASK.md` (or the brain's `task.md`) for outstanding checklists.
5. **Plan:** Write a formal "Implementation Plan" as a markdown artifact containing discrete checkbox tasks. The plan MUST include a "Verification Plan" detailing the specific Unit, Regression, or Integration tests that will validate the proposed changes.
6. **Guard:** Run local checks (e.g. `cargo check`). Stop and obtain user approval on the plan before proceeding.
7. **Execute:** Modify code files carefully. Maintain existing comments/documentation.
8. **Verify:** Run standard unit tests (`cargo test`) and compile checks (`cargo check -p client -p protocol --target wasm32-unknown-unknown`) to verify WASM and native target compatibility.
9. **Post-Audit:** Verify code changes against Section 2 Core Hygiene Guardrails.
10. **Record:** If any system-level design pivots occurred, write an Architecture Decision Record under `/doc/adr/`.
11. **Sync Logs & Sitemap:** Check off completed items in `/04_ROADMAP.md` and the subsystem checklists. Update `/MODULE_MAP.md` to document added, modified, or deleted files. Log a brief retrospective.
12. **Ship:** Execute Git commit with descriptive messages, push upstream, and exit.

## Section 2: Core Hygiene Guardrails
* **300-Line Limit:** No individual Rust source file (excluding test suites or benchmarks) may exceed 300 lines of code. Split files into granular modules or components if they approach this limit.
* **Pure Rust ECS (No DOM):** Absolutely never use HTML, CSS, JavaScript, WebViews, or DOM elements. All UI representation, rendering, and interaction must occur natively inside Bevy (`bevy_ui` + Taffy + WGSL shaders).
* **Authoritative Server Validation:** All gameplay state mutations, card draws, and movement resolutions must be evaluated on the authoritative Server. The Client only renders interpolated states and sends user-action requests.
* **Type-Safe Serialization:** All network payloads must be serialized/deserialized using `Postcard` and compile-time verified structs/enums shared in the `protocol` crate.
* **Test-Driven Architecture (TDA):** Every physics mutation, scorecard operation, AI solver feature, or serialization payload must have a corresponding test target in the same PR/commit. No logic changes may be shipped without matching unit or integration coverage as specified in `/doc/testing_strategy.md`.

