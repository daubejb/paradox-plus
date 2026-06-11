use serde::{Serialize, Deserialize};
use heapless::Vec as HVec;

pub const MAX_HOLE_CELLS: usize = 256;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerrainType {
    TeeBox,
    Fairway,
    Rough,
    Bunker,
    Water,
    OutOfBounds,
    Green(u8), // Green zones g0 (Cup) to g3
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ActiveCourseTrack {
    pub hole_index: u8,
    pub par: u8,
    pub total_cells: u32,
    pub cells: HVec<TerrainType, MAX_HOLE_CELLS>,
}

impl Default for ActiveCourseTrack {
    fn default() -> Self {
        Self {
            hole_index: 0,
            par: 3,
            total_cells: 0,
            cells: HVec::new(),
        }
    }
}
