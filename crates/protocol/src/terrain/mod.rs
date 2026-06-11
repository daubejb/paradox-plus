pub mod types;
pub mod resolution;
pub mod presets;

pub use types::{TerrainType, ActiveCourseTrack, MAX_HOLE_CELLS};
pub use resolution::{resolve_bunker_escape, resolve_green_putting, resolve_standard_landing, TerrainResolution};
pub use presets::get_course_preset;

