# ADR 0012 - Track Outer Boundary Chamfering and Widening

## Context

The Paradox Plus board rendering utilizes a 2D world-space representation based on a parametric capsule track. By default, offset coordinates from the capsule midline generated parallel curved inner and outer boundaries. However, visual design requirements called for:
1. Increasing overall track width to utilize more horizontal screen space in portrait viewports.
2. Flattening the outer top and bottom boundaries of the track to match a rectangular/chamfered look, while keeping the inner loop curved and maintaining perpendicular tile division lines (radial offsets).

## Decision

We resolved this presentation requirement purely in the client-side presentation layer:
1. Increased the capsule radius multipliers from `(h * 0.40).min(w * 0.25)` to `(h * 0.45).min(w * 0.30)` inside [geometry.rs](file:///Users/jeff/Developer/paradox-plus/crates/client/src/ui/systems/simulation/board/geometry.rs) and [spawning.rs](file:///Users/jeff/Developer/paradox-plus/crates/client/src/ui/systems/simulation/board/spawning.rs).
2. Introduced a ray-casting intersection function `calculate_outer_point` in [geometry.rs](file:///Users/jeff/Developer/paradox-plus/crates/client/src/ui/systems/simulation/board/geometry.rs) to snap the outer vertices of cells to a flat octagonal outer boundary. Tile divider lines remain aligned with the unit outward normals (perpendicular to the inner capsule loop).
3. The server and offline loopback simulation only validate the 1D discrete cell index (an integer), so using floating-point math for 2D cell sprite rendering is safe and does not cause network desyncs.

## Consequences

- Spawns tiles with flat top/bottom outer edges, vertical left/right outer edges, and diagonal corners.
- Keeps files (`spawning.rs` and `geometry.rs`) safely under the 300-line budget limit.
- Ensures all presentation logic is completely decoupled from authoritative server-side simulation.
