# ADR 0001: Server Dual Library-Binary Target Layout

## Context
To prevent source files from exceeding the strict systems-level **300-line limit** defined in [AGENTS.md](file:///Users/jeff/Developer/paradox-plus/AGENTS.md), unit and integration tests must be isolated in external files (e.g. under `tests/` directories) rather than inline inside production code blocks.
However, because Rust integration tests compile as separate external crates, they can only import modules from a package if that package builds a library target (`lib.rs`). The `crates/server` package was originally configured strictly as a binary target (`main.rs`), making its systems inaccessible to external test binaries.

## Decision
We decided to convert the `crates/server` package into a dual Library/Binary target structure:
1. Added `crates/server/src/lib.rs` to expose the `systems` submodules.
2. Kept `crates/server/src/main.rs` as the binary entry point.
3. Configured `main.rs` and the tests under `crates/server/tests/fsm_tests.rs` to link against and import from the unified library namespace (`server::systems`).

## Consequences
* **Hygiene Compliance:** Production files stay well under 100 lines each, easily complying with the 300-line ceiling.
* **Verification Separation:** Tests are cleanly housed in external files, preventing production builds from carrying test dependencies or boilerplate.
* **Target Uniformity:** Binary compilation remains identical, but modules are cleanly reusable for test harnesses.
