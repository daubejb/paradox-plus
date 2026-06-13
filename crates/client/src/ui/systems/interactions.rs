use bevy::prelude::*;
use crate::ui::components::{RollOneButtonNode, RollTwoButtonNode, WagerCardButtonNode, SkipPlacementButtonNode, SelectedWagerCard};
use crate::network::events::ClientActionRequest;
use protocol::messages::ClientAction;

/// System to handle dice roll buttons clicks.
pub fn handle_roll_buttons(
    mut events: EventWriter<ClientActionRequest>,
    roll_one_query: Query<&Interaction, (Changed<Interaction>, With<RollOneButtonNode>)>,
    roll_two_query: Query<&Interaction, (Changed<Interaction>, With<RollTwoButtonNode>)>,
) {
    for interaction in roll_one_query.iter() {
        if *interaction == Interaction::Pressed {
            events.send(ClientActionRequest(ClientAction::RollDice { dice_count: 1 }));
        }
    }
    for interaction in roll_two_query.iter() {
        if *interaction == Interaction::Pressed {
            events.send(ClientActionRequest(ClientAction::RollDice { dice_count: 2 }));
        }
    }
}

/// System to handle wager card buttons clicks.
pub fn handle_wager_card_buttons(
    mut selected_card: ResMut<SelectedWagerCard>,
    card_query: Query<(&Interaction, &WagerCardButtonNode), Changed<Interaction>>,
) {
    for (interaction, card) in card_query.iter() {
        if *interaction == Interaction::Pressed {
            selected_card.0 = Some(card.card_type);
        }
    }
}


/// System to handle skip placement button clicks.
pub fn handle_skip_placement_button(
    mut events: EventWriter<ClientActionRequest>,
    query: Query<&Interaction, (Changed<Interaction>, With<SkipPlacementButtonNode>)>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            events.send(ClientActionRequest(ClientAction::SkipPlacement));
        }
    }
}

/// System to handle match completed scorecard button clicks.
pub fn handle_match_completed_buttons(
    mut events: EventWriter<ClientActionRequest>,
    settings: Res<crate::ui::components::GameSettings>,
    mut screen_state: ResMut<NextState<crate::ui::components::ClientScreenState>>,
    play_again_query: Query<&Interaction, (Changed<Interaction>, With<crate::ui::components::PlayAgainButtonNode>)>,
    main_menu_query: Query<&Interaction, (Changed<Interaction>, With<crate::ui::components::MainMenuButtonNode>)>,
) {
    for interaction in play_again_query.iter() {
        if *interaction == Interaction::Pressed {
            let nickname = heapless::String::try_from(settings.nickname.as_str()).unwrap_or_default();
            let course = heapless::String::try_from(settings.course.as_str()).unwrap_or_default();
            let is_wager_mode = settings.mode == crate::ui::components::GameMode::WagerCards;
            
            events.send(ClientActionRequest(ClientAction::StartPractice {
                nickname,
                course,
                is_wager_mode,
            }));
        }
    }
    for interaction in main_menu_query.iter() {
        if *interaction == Interaction::Pressed {
            events.send(ClientActionRequest(ClientAction::LeaveRoom));
            screen_state.set(crate::ui::components::ClientScreenState::Landing);
        }
    }
}
