use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct GameAudioAssets {
    pub button_hover: Handle<AudioSource>,
    pub button_click: Handle<AudioSource>,
    pub dice_roll: Handle<AudioSource>,
    pub wager_draft: Handle<AudioSource>,
    pub wager_place: Handle<AudioSource>,
    pub land_fairway: Handle<AudioSource>,
    pub land_rough: Handle<AudioSource>,
    pub land_bunker: Handle<AudioSource>,
    pub land_water: Handle<AudioSource>,
    pub land_ob: Handle<AudioSource>,
    pub hole_complete: Handle<AudioSource>,
    pub match_complete: Handle<AudioSource>,
}

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AudioLoaded(pub bool);
