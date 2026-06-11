use bevy::prelude::*;
use crate::ui::components::*;
use crate::network::events::ClientActionRequest;
use protocol::messages::ClientAction;

pub fn show_landing_screen_system(
    mut landing_query: Query<&mut Style, With<LandingScreenNode>>,
    mut gameplay_query: Query<&mut Style, (With<GameplayScreenNode>, Without<LandingScreenNode>)>,
) {
    if let Ok(mut style) = landing_query.get_single_mut() {
        style.display = Display::Flex;
    }
    if let Ok(mut style) = gameplay_query.get_single_mut() {
        style.display = Display::None;
    }
}

pub fn show_gameplay_screen_system(
    mut landing_query: Query<&mut Style, With<LandingScreenNode>>,
    mut gameplay_query: Query<&mut Style, (With<GameplayScreenNode>, Without<LandingScreenNode>)>,
) {
    if let Ok(mut style) = landing_query.get_single_mut() {
        style.display = Display::None;
    }
    if let Ok(mut style) = gameplay_query.get_single_mut() {
        style.display = Display::Flex;
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
            next_state.set(ClientScreenState::Gameplay);
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
    hamburger_btn: Query<&Interaction, (Changed<Interaction>, With<HamburgerButtonNode>)>,
) {
    for interaction in hamburger_btn.iter() {
        if *interaction == Interaction::Pressed {
            events.send(ClientActionRequest(ClientAction::LeaveRoom));
            next_state.set(ClientScreenState::Landing);
        }
    }
}
