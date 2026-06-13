use bevy::prelude::*;
use crate::ui::components::{
    RollOneButtonNode, RollTwoButtonNode, WagerCardButtonNode, SkipPlacementButtonNode,
    SelectedWagerCard, ScorecardButtonNode, CloseScorecardButtonNode, ShowScorecard,
    ClientScorecards
};
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
    scorecards: Res<ClientScorecards>,
    card_query: Query<(&Interaction, &WagerCardButtonNode), Changed<Interaction>>,
) {
    let score_val = scorecards.0.first();
    let shield_count = score_val.map(|s| s.earned_cards.iter().filter(|&&c| c == 0).count()).unwrap_or(0);
    let banana_count = score_val.map(|s| s.earned_cards.iter().filter(|&&c| c == 1).count()).unwrap_or(0);
    let die_count = score_val.map(|s| s.earned_cards.iter().filter(|&&c| c == 2).count()).unwrap_or(0);

    for (interaction, card) in card_query.iter() {
        if *interaction == Interaction::Pressed {
            let count = match card.card_type {
                protocol::messages::CardType::Shield => shield_count,
                protocol::messages::CardType::Banana => banana_count,
                protocol::messages::CardType::GoldenDie => die_count,
            };
            if count > 0 {
                if selected_card.0 == Some(card.card_type) {
                    selected_card.0 = None;
                } else {
                    selected_card.0 = Some(card.card_type);
                }
            }
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
    mut show_scorecard: ResMut<ShowScorecard>,
    play_again_query: Query<&Interaction, (Changed<Interaction>, With<crate::ui::components::PlayAgainButtonNode>)>,
    main_menu_query: Query<&Interaction, (Changed<Interaction>, With<crate::ui::components::MainMenuButtonNode>)>,
) {
    for interaction in play_again_query.iter() {
        if *interaction == Interaction::Pressed {
            show_scorecard.0 = false;
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
            show_scorecard.0 = false;
            events.send(ClientActionRequest(ClientAction::LeaveRoom));
            screen_state.set(crate::ui::components::ClientScreenState::Landing);
        }
    }
}

/// System to handle opening and closing of the scorecard overlay.
pub fn handle_scorecard_toggle_buttons(
    mut show_scorecard: ResMut<ShowScorecard>,
    scorecard_btn_query: Query<&Interaction, (Changed<Interaction>, With<ScorecardButtonNode>)>,
    close_btn_query: Query<&Interaction, (Changed<Interaction>, With<CloseScorecardButtonNode>)>,
) {
    for interaction in scorecard_btn_query.iter() {
        if *interaction == Interaction::Pressed {
            show_scorecard.0 = true;
        }
    }
    for interaction in close_btn_query.iter() {
        if *interaction == Interaction::Pressed {
            show_scorecard.0 = false;
        }
    }
}
