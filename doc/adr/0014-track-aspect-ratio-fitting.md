# ADR 0014 - Constant Aspect Ratio Fitting for Track Geometry

## Context

The previous track layouts (ADR 0012, ADR 0013) scaled or rotated the board's stadium geometry dynamically. In portrait mode, this produced a tall stadium, while in landscape it transposed the coordinates. This created inconsistencies, violated the target horizontal proportions of the visual inspiration design, and led to visual layout squishing when viewports scaled.

The visual design requires:
1. The track must maintain a strict, consistent ~1.38 landscape aspect ratio (matching the inspiration's ratio of 4-cell horizontal segments to 2-cell vertical segments) in all viewport orientations.
2. The track must dynamically scale to occupy the maximum width and height possible in the viewport area while leaving a small, consistent padding boundary.
3. The layout sequence (Tee Box starting on the left vertical segment and proceeding clockwise) must remain invariant across all viewports.

## Decision

We refactored [geometry.rs](file:///Users/jeff/Developer/paradox-plus/crates/client/src/ui/systems/simulation/board/geometry.rs) to encapsulate fitting logic within a `TrackGeometry` model:
1. **Aspect Ratio Fitting Algorithm**: We implement a fitting solver that bounds the track's outer dimensions (`outer_width`, `outer_height`) by either width or height depending on the viewport's aspect ratio relative to `1.38`.
2. **Defensive Clamping**: Enforced viewport padding (24.0px) and clamped minimum dimension bounds to prevent divisions-by-zero or negative segment lengths on minimized windows or initial headless frames.
3. **No Transposition**: Removed coordinate transpositions and rotation changes, keeping index 0 (Tee Box) on the left vertical wall winding clockwise under all aspect ratios.

## Consequences

- The visual board layout accurately and consistently mirrors the landscape stadium proportions of the design inspiration.
- Prevented potential math NaNs and rendering panics from negative layout coordinates.
- Maintained strict compliance with the 300-line budget limit.
