use bevy::prelude::*;
use crate::ui::components::{
    ClientScreenState, GameSettings, SettingsScreenNode, SoundToggleButtonNode,
    SoundToggleTextNode, CloseSettingsButtonNode, LandingScreenNode,
    SoloSetupScreenNode, GameplayScreenNode,
};

pub fn show_settings_screen_system(
    mut landing_query: Query<&mut Style, With<LandingScreenNode>>,
    mut setup_query: Query<&mut Style, (With<SoloSetupScreenNode>, Without<LandingScreenNode>)>,
    mut gameplay_query: Query<&mut Style, (With<GameplayScreenNode>, Without<LandingScreenNode>, Without<SoloSetupScreenNode>)>,
    mut settings_query: Query<&mut Style, (With<SettingsScreenNode>, Without<LandingScreenNode>, Without<SoloSetupScreenNode>, Without<GameplayScreenNode>)>,
) {
    if let Ok(mut style) = landing_query.get_single_mut() {
        style.display = Display::None;
    }
    if let Ok(mut style) = setup_query.get_single_mut() {
        style.display = Display::None;
    }
    if let Ok(mut style) = gameplay_query.get_single_mut() {
        style.display = Display::None;
    }
    if let Ok(mut style) = settings_query.get_single_mut() {
        style.display = Display::Flex;
    }
}

pub fn handle_settings_button_clicks(
    mut settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<ClientScreenState>>,
    sound_toggle_btn: Query<&Interaction, (Changed<Interaction>, With<SoundToggleButtonNode>)>,
    close_btn: Query<&Interaction, (Changed<Interaction>, With<CloseSettingsButtonNode>)>,
) {
    for interaction in sound_toggle_btn.iter() {
        if *interaction == Interaction::Pressed {
            settings.sound_effects_enabled = !settings.sound_effects_enabled;
        }
    }
    for interaction in close_btn.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(ClientScreenState::Landing);
        }
    }
}

pub fn update_settings_screen_ui(
    settings: Res<GameSettings>,
    mut toggle_query: Query<(&mut BackgroundColor, &mut BorderColor, &mut Style), With<SoundToggleButtonNode>>,
    mut text_query: Query<&mut Text, With<SoundToggleTextNode>>,
) {
    if !settings.is_changed() {
        return;
    }

    if let Ok((mut bg, mut border, mut style)) = toggle_query.get_single_mut() {
        if settings.sound_effects_enabled {
            *bg = Color::srgb(0.10, 0.50, 0.22).into();
            *border = Color::srgb(0.15, 0.80, 0.35).into();
            style.justify_content = JustifyContent::FlexEnd;
        } else {
            *bg = Color::srgb(0.04, 0.10, 0.06).into();
            *border = Color::srgb(0.15, 0.25, 0.20).into();
            style.justify_content = JustifyContent::FlexStart;
        }
    }

    if let Ok(mut text) = text_query.get_single_mut() {
        if let Some(section) = text.sections.first_mut() {
            section.value = if settings.sound_effects_enabled {
                "ON".to_string()
            } else {
                "OFF".to_string()
            };
        }
    }
}
