pub mod components;
pub mod helpers;
pub mod systems;

pub use components::BallVisualInterpolation;

use bevy::prelude::*;

pub struct FixedToFloatPlugin;

impl Plugin for FixedToFloatPlugin {
    fn build(&self, app: &mut App) {
        // Enforce scheduling in PostUpdate to prevent frame sync lag
        app.add_systems(PostUpdate, systems::fixed_to_float_translation_system);
    }
}
