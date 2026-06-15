# ADR 0009: 2D World Space Board Rendering

## Status
Proposed

## Context
The central gameboard was originally implemented using Bevy UI layout nodes (buttons). However, as we introduce visual features (tile rotations, custom shaders, and particle layering), Bevy UI presents several limitations:
1. Bevy UI does not natively support rotation of individual node components.
2. Nesting visual effects (like WGSL shaders and 3D/2D particle emitters) inside UI layout trees can trigger expensive layout calculations (Taffy flexbox solver passes) on every frame.
3. Complex geometry curves (such as a parametric capsule layout) are difficult to model natively using standard UI node grids.

## Decision
We will shift the rendering of the central board from Bevy UI nodes to a 2D World Space (`bevy_sprite`) paradigm:
1. Keep the top header HUD and bottom controls as `bevy_ui` nodes.
2. Replace the center of the UI with a transparent spacer node.
3. Spawn a 2D Camera targeting the coordinates of this central spacer node.
4. Render each tile as a 2D Sprite mesh entity, aligned along a parametric capsule track layout.
5. Project screen clicks into 2D world space coordinates using the 2D Camera projection to detect tile selections.

## Consequences
- **Pros:**
  - Unlocks individual tile rotations and alignment along complex curved trajectories.
  - Improves rendering performance by isolating board geometry updates from Bevy UI flexbox calculations.
  - Allows easy integration of custom WGSL shaders and particle emitters on the board.
- **Cons:**
  - Requires manual mouse/touch coordinate projection (raycasting) for cell click detection instead of native button interactions.
  - Manually scales camera viewport projection to maintain correct aspect ratios inside the UI spacer bounds.
