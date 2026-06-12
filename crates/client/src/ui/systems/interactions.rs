use bevy::prelude::*;
use crate::ui::components::{RollOneButtonNode, RollTwoButtonNode, WagerCardButtonNode, SkipPlacementButtonNode};
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
    mut events: EventWriter<ClientActionRequest>,
    card_query: Query<(&Interaction, &WagerCardButtonNode), Changed<Interaction>>,
) {
    for (interaction, card) in card_query.iter() {
        if *interaction == Interaction::Pressed {
            events.send(ClientActionRequest(ClientAction::DraftCard {
                card_type: card.card_type,
                cell_index: 10, // Mock cell index for draft placement validation
            }));
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
