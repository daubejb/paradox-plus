use super::bunker::TerrainResolution;

/// Resolves a Green landing zone transition.
/// - Green 0 (Cup): +0 Putts (S_penalty = 0)
/// - Green 1: +1 Putts (S_penalty = 1)
/// - Green 2: +2 Putts (S_penalty = 2)
/// - Green 3: +3 Putts (S_penalty = 3)
pub fn resolve_green_putting(
    target_cell: u16,
    green_tier: u8,
) -> TerrainResolution {
    let penalty_strokes = match green_tier {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 3,
    };
    TerrainResolution {
        final_cell: target_cell,
        shot_strokes: 1, // 1 approach shot
        penalty_strokes,
        completed_hole: true,
    }
}
