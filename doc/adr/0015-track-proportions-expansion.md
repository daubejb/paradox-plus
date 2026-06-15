# ADR 0015 - Track Proportions and Width Expansion

## Context

The previous track layout (ADR 0014) fit portrait screens using a narrow vertical aspect ratio of `0.68` and a lane width (tile thickness) of `72.0`. However, this left the track looking narrow, with tight top and bottom curves, and relatively thin tiles.

Visual requirements specified:
1. Expanding the overall width of the track horizontally to utilize more left-to-right real estate of the viewport.
2. Flattening the top and bottom straight walls of the track.
3. Increasing the cell lane thickness to make the track look more robust and readable.

## Decision

We modified [geometry.rs](file:///Users/jeff/Developer/paradox-plus/crates/client/src/ui/systems/simulation/board/geometry.rs) and [spawning.rs](file:///Users/jeff/Developer/paradox-plus/crates/client/src/ui/systems/simulation/board/spawning.rs) as follows:
1. **Aspect Ratio**: Increased `TARGET_ASPECT_RATIO` to `0.85` (width / height = 0.85). This broadens the track horizontally while maintaining portrait mode layout compatibility.
2. **Radius Coefficient**: Reduced `RADIUS_COEFFICIENT` to `0.22` (down from `0.35`). This creates smaller, tighter corner curves and increases the length of the flat horizontal segments (`l_h`).
3. **Tile Thickness**: Increased `tile_thickness` and `MIDLINE_PADDING` to `96.0` (up from `72.0`). This makes the individual cell lanes wider and matches the visual representation of the lane.
4. **Safety Margins**: Increased `MIN_MIDLINE_WIDTH` to `240.0` (minimum outer width of `336.0`) to guarantee that the inner corner radius remains positive and safe ($R_{\text{inner}} \approx 4.8$) under extreme scales.

## Consequences

- The track displays with much wider individual cells and flatter, wider top/bottom walls.
- The layout occupies more horizontal space in portrait viewports.
- All files remain strictly under 300 lines of code.
