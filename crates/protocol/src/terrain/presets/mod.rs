pub mod green;
pub mod blue;

use crate::terrain::{ActiveCourseTrack, TerrainType};
use heapless::Vec as HVec;

/// Returns the course preset by color name and hole index.
/// Prepend TeeBox (TerrainType::TeeBox) at index 0.
pub fn get_course_preset(course: &str, hole_id: u8) -> Option<ActiveCourseTrack> {
    let (par, spaces) = match course {
        "green" => green::get_hole_data(hole_id)?,
        "blue" => blue::get_hole_data(hole_id)?,
        _ => return None,
    };

    let mut cells = HVec::new();
    // Prefix with TeeBox at index 0
    if cells.push(TerrainType::TeeBox).is_err() {
        return None;
    }

    for &space in spaces {
        let cell_type = match space {
            "f" => TerrainType::Fairway,
            "r" => TerrainType::Rough,
            "s" => TerrainType::Bunker,
            "w" => TerrainType::Water,
            "lb" => TerrainType::OutOfBounds,
            "g0" => TerrainType::Green(0),
            "g1" => TerrainType::Green(1),
            "g2" => TerrainType::Green(2),
            "g3" => TerrainType::Green(3),
            _ => TerrainType::Fairway,
        };
        if cells.push(cell_type).is_err() {
            return None;
        }
    }

    Some(ActiveCourseTrack {
        hole_index: hole_id,
        par,
        total_cells: cells.len() as u32,
        cells,
    })
}
