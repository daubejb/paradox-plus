use bevy::prelude::*;
use crate::ui::components::*;
use crate::network::events::ClientActionRequest;
use protocol::messages::ClientAction;

pub fn show_landing_screen_system(
    mut landing_query: Query<&mut Style, With<LandingScreenNode>>,
    mut setup_query: Query<&mut Style, (With<SoloSetupScreenNode>, Without<LandingScreenNode>)>,
    mut gameplay_query: Query<&mut Style, (With<GameplayScreenNode>, Without<LandingScreenNode>, Without<SoloSetupScreenNode>)>,
) {
    if let Ok(mut style) = landing_query.get_single_mut() {
        style.display = Display::Flex;
    }
    if let Ok(mut style) = setup_query.get_single_mut() {
        style.display = Display::None;
    }
    if let Ok(mut style) = gameplay_query.get_single_mut() {
        style.display = Display::None;
    }
}

pub fn show_gameplay_screen_system(
    mut landing_query: Query<&mut Style, With<LandingScreenNode>>,
    mut setup_query: Query<&mut Style, (With<SoloSetupScreenNode>, Without<LandingScreenNode>)>,
    mut gameplay_query: Query<&mut Style, (With<GameplayScreenNode>, Without<LandingScreenNode>, Without<SoloSetupScreenNode>)>,
    mut wager_panel_query: Query<&mut Style, (With<WagerPanelNode>, Without<LandingScreenNode>, Without<SoloSetupScreenNode>, Without<GameplayScreenNode>)>,
    settings: Res<GameSettings>,
) {
    if let Ok(mut style) = landing_query.get_single_mut() {
        style.display = Display::None;
    }
    if let Ok(mut style) = setup_query.get_single_mut() {
        style.display = Display::None;
    }
    if let Ok(mut style) = gameplay_query.get_single_mut() {
        style.display = Display::Flex;
    }
    if let Ok(mut style) = wager_panel_query.get_single_mut() {
        style.display = match settings.mode {
            GameMode::Standard => Display::None,
            GameMode::WagerCards => Display::Flex,
        };
    }
}

pub fn handle_landing_button_clicks(
    mut next_state: ResMut<NextState<ClientScreenState>>,
    mut status_query: Query<&mut Text, With<LandingStatusTextNode>>,
    solo_btn: Query<&Interaction, (Changed<Interaction>, With<SoloPracticeButtonNode>)>,
    other_btns: Query<
        &Interaction,
        (
            Changed<Interaction>,
            Or<(
                With<VsBotsButtonNode>,
                With<OnlineMultiplayerButtonNode>,
                With<StatsButtonNode>,
                With<ViewRulesButtonNode>,
                With<SettingsButtonNode>,
            )>,
        ),
    >,
) {
    for interaction in solo_btn.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(ClientScreenState::SoloSetup);
        }
    }

    for interaction in other_btns.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut text) = status_query.get_single_mut() {
                if let Some(section) = text.sections.first_mut() {
                    section.value = "Under development!".to_string();
                }
            }
        }
    }
}

pub fn handle_gameplay_exit(
    mut next_state: ResMut<NextState<ClientScreenState>>,
    mut events: EventWriter<ClientActionRequest>,
    mut settings: ResMut<GameSettings>,
    mut show_scorecard: ResMut<ShowScorecard>,
    hamburger_btn: Query<&Interaction, (Changed<Interaction>, With<HamburgerButtonNode>)>,
) {
    for interaction in hamburger_btn.iter() {
        if *interaction == Interaction::Pressed {
            show_scorecard.0 = false;
            *settings = GameSettings::default();
            events.send(ClientActionRequest(ClientAction::LeaveRoom));
            next_state.set(ClientScreenState::Landing);
        }
    }
}
