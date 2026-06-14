use bevy::prelude::*;
use bevy::asset::LoadState;
use crate::network::ServerUpdateEvent;
use crate::ui::components::GameSettings;
use protocol::messages::{ServerUpdate, GameStateEnum};
use protocol::terrain::presets::get_course_preset;
use protocol::terrain::TerrainType;
use super::resources::{GameAudioAssets, AudioLoaded};

pub fn load_audio_assets_system(
    audio_sources: Option<Res<Assets<AudioSource>>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if audio_sources.is_none() {
        return;
    }
    let assets = GameAudioAssets {
        button_hover: asset_server.load("sounds/ui_hover.wav"),
        button_click: asset_server.load("sounds/ui_click.wav"),
        dice_roll: asset_server.load("sounds/dice_roll.wav"),
        wager_draft: asset_server.load("sounds/wager_draft.wav"),
        wager_place: asset_server.load("sounds/wager_place.wav"),
        land_fairway: asset_server.load("sounds/land_fairway.wav"),
        land_rough: asset_server.load("sounds/land_rough.wav"),
        land_bunker: asset_server.load("sounds/land_bunker.wav"),
        land_water: asset_server.load("sounds/land_water.wav"),
        land_ob: asset_server.load("sounds/land_ob.wav"),
        hole_complete: asset_server.load("sounds/hole_complete.wav"),
        match_complete: asset_server.load("sounds/match_complete.wav"),
    };
    commands.insert_resource(assets);
}

pub fn check_audio_loading_system(
    audio_sources: Option<Res<Assets<AudioSource>>>,
    asset_server: Res<AssetServer>,
    audio_assets: Res<GameAudioAssets>,
    mut loaded: ResMut<AudioLoaded>,
) {
    if audio_sources.is_none() {
        return;
    }
    if loaded.0 {
        return;
    }

    let states = [
        asset_server.get_load_state(&audio_assets.button_hover),
        asset_server.get_load_state(&audio_assets.button_click),
        asset_server.get_load_state(&audio_assets.dice_roll),
        asset_server.get_load_state(&audio_assets.wager_draft),
        asset_server.get_load_state(&audio_assets.wager_place),
        asset_server.get_load_state(&audio_assets.land_fairway),
        asset_server.get_load_state(&audio_assets.land_rough),
        asset_server.get_load_state(&audio_assets.land_bunker),
        asset_server.get_load_state(&audio_assets.land_water),
        asset_server.get_load_state(&audio_assets.land_ob),
        asset_server.get_load_state(&audio_assets.hole_complete),
        asset_server.get_load_state(&audio_assets.match_complete),
    ];

    if states.iter().all(|state| *state == Some(LoadState::Loaded)) {
        loaded.0 = true;
        info!("Audio assets successfully loaded.");
    }
}

pub fn play_ui_interaction_sounds(
    audio_sources: Option<Res<Assets<AudioSource>>>,
    audio_assets: Res<GameAudioAssets>,
    loaded: Res<AudioLoaded>,
    settings: Res<GameSettings>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
) {
    if audio_sources.is_none() || !loaded.0 || !settings.sound_effects_enabled {
        return;
    }

    for interaction in &interaction_query {
        match interaction {
            Interaction::Pressed => {
                commands.spawn(AudioBundle {
                    source: audio_assets.button_click.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
            Interaction::Hovered => {
                commands.spawn(AudioBundle {
                    source: audio_assets.button_hover.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
            Interaction::None => {}
        }
    }
}

pub fn play_gameplay_sounds(
    audio_sources: Option<Res<Assets<AudioSource>>>,
    audio_assets: Res<GameAudioAssets>,
    loaded: Res<AudioLoaded>,
    mut update_events: EventReader<ServerUpdateEvent>,
    settings: Res<GameSettings>,
    mut commands: Commands,
) {
    if audio_sources.is_none() || !loaded.0 || !settings.sound_effects_enabled {
        return;
    }

    for event in update_events.read() {
        match &event.0 {
            ServerUpdate::DiceRollOutcome { .. } => {
                commands.spawn(AudioBundle {
                    source: audio_assets.dice_roll.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
            ServerUpdate::SlideTransition { path } => {
                if let Some(target_cell) = path.last() {
                    let current_hole = 1; // Default fallback index
                    if let Some(preset) = get_course_preset(&settings.course, current_hole) {
                        if let Some(terrain) = preset.cells.get(*target_cell as usize) {
                            let source = match terrain {
                                TerrainType::Fairway | TerrainType::TeeBox | TerrainType::Green(_) => {
                                    audio_assets.land_fairway.clone()
                                }
                                TerrainType::Rough => audio_assets.land_rough.clone(),
                                TerrainType::Bunker => audio_assets.land_bunker.clone(),
                                TerrainType::Water => audio_assets.land_water.clone(),
                                TerrainType::OutOfBounds => audio_assets.land_ob.clone(),
                            };
                            commands.spawn(AudioBundle {
                                source,
                                settings: PlaybackSettings::DESPAWN,
                            });
                        }
                    }
                }
            }
            ServerUpdate::StateSync { game_state, placed_wagers, .. } => {
                if !placed_wagers.is_empty() {
                    commands.spawn(AudioBundle {
                        source: audio_assets.wager_place.clone(),
                        settings: PlaybackSettings::DESPAWN,
                    });
                }

                match game_state {
                    GameStateEnum::HoleCompleted => {
                        commands.spawn(AudioBundle {
                            source: audio_assets.hole_complete.clone(),
                            settings: PlaybackSettings::DESPAWN,
                        });
                    }
                    GameStateEnum::MatchCompleted => {
                        commands.spawn(AudioBundle {
                            source: audio_assets.match_complete.clone(),
                            settings: PlaybackSettings::DESPAWN,
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
