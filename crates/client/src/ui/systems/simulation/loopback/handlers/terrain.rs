use protocol::terrain::resolution::bunker::{resolve_bunker_escape, TerrainResolution};
use protocol::terrain::resolution::standard::resolve_standard_landing;
use protocol::terrain::resolution::green::resolve_green_putting;
use protocol::terrain::TerrainType;

pub fn resolve_landing(
    target_cell: u16,
    origin_cell: u16,
    terrain_type: TerrainType,
) -> TerrainResolution {
    match terrain_type {
        TerrainType::Green(tier) => resolve_green_putting(target_cell, tier),
        other => resolve_standard_landing(target_cell, origin_cell, other),
    }
}

pub fn resolve_bunker(
    current_cell: u16,
    target_cell: u16,
    roll_sum: u8,
) -> TerrainResolution {
    resolve_bunker_escape(current_cell, target_cell, roll_sum)
}
