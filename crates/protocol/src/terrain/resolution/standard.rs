use super::bunker::TerrainResolution;

/// Resolves standard terrain landings (Fairway, Rough, Water, Out-of-Bounds).
pub fn resolve_standard_landing(
    target_cell: u16,
    origin_cell: u16,
    terrain_type: crate::terrain::types::TerrainType,
) -> TerrainResolution {
    match terrain_type {
        crate::terrain::types::TerrainType::Fairway | crate::terrain::types::TerrainType::TeeBox | crate::terrain::types::TerrainType::Rough => {
            TerrainResolution {
                final_cell: target_cell,
                shot_strokes: 1,
                penalty_strokes: 0,
                completed_hole: false,
            }
        }
        crate::terrain::types::TerrainType::Water => {
            TerrainResolution {
                final_cell: target_cell,
                shot_strokes: 1,
                penalty_strokes: 1,
                completed_hole: false,
            }
        }
        crate::terrain::types::TerrainType::OutOfBounds => {
            TerrainResolution {
                final_cell: origin_cell,
                shot_strokes: 1,
                penalty_strokes: 1,
                completed_hole: false,
            }
        }
        _ => {
            // Fallback for safety
            TerrainResolution {
                final_cell: target_cell,
                shot_strokes: 1,
                penalty_strokes: 0,
                completed_hole: false,
            }
        }
    }
}
