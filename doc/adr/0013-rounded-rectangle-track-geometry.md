# ADR 0013 - Rounded Rectangle Track Geometry

## Context

The previous track geometry (ADR 0012) implemented a capsule midline with a ray-cast octagonal outer boundary to flatten the top, bottom, left, and right sides. However, this resulted in sharp diagonal outer corners and mismatched inner/outer curvature, which did not align with the rounded corner requirements of the game board.

The visual design requires:
1. Both the inner and outer boundaries must have perfectly straight vertical (left/right) and horizontal (top/bottom) walls.
2. All four corners on both the inner and outer boundaries must be rounded curves (quarter-circles), ensuring symmetrical parallel offsets.
3. The track division lines must remain perpendicular to the midline (drawn tangent to the inner perimeter).

## Decision

We replaced the capsule and ray-casting model with a dedicated 8-segment portrait rounded rectangle trajectory for the midline path:
1. **Midline Segments**: The trajectory consists of 4 flat walls (left, top, right, bottom) and 4 rounded corners (top-left, top-right, bottom-right, bottom-left) using quarter-circle arcs of radius `r`.
2. **Parallel Offsets**: The outer and inner coordinates are calculated by offsetting along the unit normal vector (`c_out = position + normal * d` and `c_in = position - normal * d`). Since the midline itself is a rounded rectangle, this parallel offset automatically forms flat walls and rounded corners on both the inside and outside.
3. **Outward Normals**: Corrected normal vector directions to point outwards consistently (+90 degrees in portrait and -90 degrees in landscape).
4. **Closest-Cell Clicks**: Refactored the cursor click raycast helper in [interaction.rs](file:///Users/jeff/Developer/paradox-plus/crates/client/src/ui/systems/simulation/board/interaction.rs) to select the closest cell under the threshold rather than the first matching query element. This resolves overlapping hit bounds on smaller viewports.

## Consequences

- Resolves the flat top/bottom/left/right and rounded corner requirements for the entire track structure.
- Simplifies the math by removing ray-casting and reverting to clean parallel offsets.
- Maintains compliance with the 300-line budget limit.
- Click interaction behaves reliably regardless of viewport dimensions.
