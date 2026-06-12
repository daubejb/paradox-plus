# ADR 0010: Procedural Tile Mesh Rendering

## Status
Accepted

## Context
Following the transition to a 2D World Space board (ADR 0009), the cell tiles were initially represented using rotated rectangular sprites (`SpriteBundle`). However, rendering rectangular shapes along a curved parametric capsule track introduces two major visual defects:
1. **Triangular Gaps:** On the curved sections, adjacent rectangular tiles diverge at their outer perimeters, creating visible triangular gaps.
2. **Color Bleeding:** Overlapping rotated rectangles cause cell backgrounds to bleed across the perpendicular radial divider lines.

To align the board aesthetics with a drawn racetrack design where tiles fit perfectly together, we need a rendering strategy where cell boundaries align exactly with their shared radial divider lines.

## Decision
We will transition the client board tile backgrounds from flat rectangular sprites to procedurally generated 2D quad meshes (`ColorMesh2dBundle`):
1. Keep the main board cell nodes as invisible marker entities (`SpatialBundle`) to preserve click interaction, local token offsets, and text placement.
2. For each tile, calculate the four corner coordinates in 2D world space:
   - Outer-start and inner-start at boundary index `idx - 0.5`.
   - Outer-end and inner-end at boundary index `idx + 0.5`.
3. Construct a standard Bevy CCW winding quad mesh from these coordinates, registering it to `Assets<Mesh>`.
4. Spawn the mesh as a sibling entity carrying a `TrackTileVisuals` component.
5. Release GPU and CPU memory allocations by explicitly removing the custom meshes and materials from the asset resources on board rebuilds/cleanup.
6. Connect the outer corners and inner corners of all tiles using thin segment sprites to form the continuous outer and inner ovals of the track.

## Consequences
- **Pros:**
  - Eliminates all visual gaps on curved track segments, connecting adjacent tiles perfectly.
  - Constrains cell background colors strictly within their perpendicular radial boundaries (zero bleeding).
  - Produces a polished, continuous "drawn oval" track visual.
- **Cons:**
  - Requires dynamic mesh generation and asset management (`Assets<Mesh>` and `Assets<ColorMaterial>`).
  - Requires explicit cleanup systems to prevent memory leaks from unused handles in Bevy's asset database.
