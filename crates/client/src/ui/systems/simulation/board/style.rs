use bevy::prelude::Color;
use protocol::terrain::TerrainType;

/// Returns the background color and high-contrast text label color for a given terrain type.
pub fn get_terrain_style(terrain_type: &TerrainType) -> (Color, Color) {
    match terrain_type {
        TerrainType::Green(_) => (Color::srgb(0.70, 0.95, 0.70), Color::BLACK),
        TerrainType::Fairway => (Color::srgb(0.20, 0.65, 0.30), Color::WHITE),
        TerrainType::Rough => (Color::srgb(0.08, 0.35, 0.12), Color::WHITE),
        TerrainType::Bunker => (Color::srgb(0.92, 0.85, 0.65), Color::BLACK),
        TerrainType::Water => (Color::srgb(0.10, 0.45, 0.80), Color::WHITE),
        TerrainType::OutOfBounds => (Color::srgb(0.80, 0.15, 0.15), Color::WHITE),
        TerrainType::TeeBox => (Color::srgb(0.20, 0.45, 0.35), Color::WHITE),
    }
}
