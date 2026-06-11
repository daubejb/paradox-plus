use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerrainResolution {
    pub final_cell: u16,
    pub shot_strokes: u16,
    pub penalty_strokes: u16,
    pub completed_hole: bool,
}

/// Resolves an escape attempt from a Sand Bunker space.
/// - Even roll escapes successfully to target_cell.
/// - Odd roll fails escape, ball remains on current_cell.
pub fn resolve_bunker_escape(
    current_cell: u16,
    target_cell: u16,
    roll_sum: u8,
) -> TerrainResolution {
    if roll_sum % 2 == 0 {
        TerrainResolution {
            final_cell: target_cell,
            shot_strokes: 1,
            penalty_strokes: 0,
            completed_hole: false,
        }
    } else {
        TerrainResolution {
            final_cell: current_cell,
            shot_strokes: 1,
            penalty_strokes: 0,
            completed_hole: false,
        }
    }
}
