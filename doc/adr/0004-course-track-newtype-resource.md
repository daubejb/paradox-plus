# ADR 0004: Course Track Newtype Resource

## Context
In Paradox Plus, the protocol crate is kept completely Bevy-free. As a result, the `ActiveCourseTrack` struct defined in the protocol package does not derive Bevy's `Resource` trait.
In Rust, implementing an external trait (`bevy::prelude::Resource`) for an external type (`protocol::terrain::ActiveCourseTrack`) inside the server package violates Rust's orphan rules and results in a compilation failure (`error[E0117]`).

## Decision
We decided to introduce a newtype wrapper structure `CourseTrackResource` within the server package (`crates/server/src/ai/components.rs`):
```rust
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct CourseTrackResource(pub protocol::terrain::ActiveCourseTrack);
```
This newtype is owned by the server package and thus can safely derive or implement Bevy's `Resource` trait, circumventing the orphan rules entirely. All server systems requiring the active course track query `Res<CourseTrackResource>` and access the underlying `ActiveCourseTrack` structure via the `.0` tuple accessor.

## Consequences
* **Hygiene Gating:** The protocol package remains clean, lightweight, and completely Bevy-free.
* **Compilation Integrity:** Rust orphan rules are strictly respected, resolving the compiler error cleanly.
* **Developer Ergonomics:** Access to the active course track is maintained with a simple `.0` field dereference in the server systems and tests.
