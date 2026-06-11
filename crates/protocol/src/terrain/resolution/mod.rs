pub mod bunker;
pub mod green;
pub mod standard;

pub use bunker::{resolve_bunker_escape, TerrainResolution};
pub use green::resolve_green_putting;
pub use standard::resolve_standard_landing;
