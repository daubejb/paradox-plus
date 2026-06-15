pub mod resources;
pub mod systems;

use bevy::prelude::*;
use resources::{GameAudioAssets, AudioLoaded};
use systems::{
    load_audio_assets_system, check_audio_loading_system,
    play_ui_interaction_sounds, play_gameplay_sounds
};

pub struct ClientAudioPlugin;

impl Plugin for ClientAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAudioAssets>()
            .init_resource::<AudioLoaded>()
            .add_systems(Startup, load_audio_assets_system)
            .add_systems(
                Update,
                (
                    check_audio_loading_system,
                    play_ui_interaction_sounds,
                    play_gameplay_sounds,
                ),
            );
    }
}
