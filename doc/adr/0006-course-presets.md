# ADR 0006: Course Presets in Shared Protocol Crate

## Context
Originally, the course layouts (such as the Green course and the Blue course) were not statically defined in the codebase. However, game simulation (physics checks, bunker escapes, water hazard transitions) and AI planning loops (Markov Decision Process Value Iteration sweeps) are executed authoritatively on the Server, while the Client needs to render the corresponding layout visually and predict sliding motions locally without visual stutter.

To ensure consistency between client-side rendering and server-side simulation/planning, both subsystems must share the identical cell index layout definitions. Loading these maps dynamically from JSON sidecars at runtime would require parsing logic (`serde_json`), which adds binary size overhead to WebAssembly targets, complicates filesystem accesses on sandbox/web hosts, and introduces potential runtime deserialization errors.

## Decision
To avoid runtime file reading panics and keep the systems deterministic and fast, we made the following decisions:
1. Compiled the raw layout configurations of `courses.json` directly into static Rust code in the `protocol` crate (`crates/protocol/src/terrain/presets/`).
2. Split the course presets into compact green and blue modules to respect the 300-line budget limit.
3. Provided a unified function `get_course_preset(course: &str, hole_id: u8) -> Option<ActiveCourseTrack>` that maps these static representations into stack-allocated, bounded `ActiveCourseTrack` structs, prepending the `TeeBox` tile at cell index 0.

## Consequences
* **consensus-Safe Layouts:** The server's AI bot planning and client-side presentation are guaranteed to operate on the identical track cells without desync risk.
* **WASM/Sandboxed Compatibility:** Eliminates dynamic file system IO or HTTP calls to fetch course maps, allowing the game to boot instantly on web canvases and headless test environments.
* **Low Memory Footprint:** Bounded type representation (`heapless::Vec`) eliminates dynamic heap allocations during lookup queries.
