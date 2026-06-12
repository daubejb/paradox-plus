use bevy::prelude::*;
use crate::ui::components::{RollOneButtonNode, RollTwoButtonNode, WagerCardButtonNode, SkipPlacementButtonNode, SelectedWagerCard, BoardCellNode};
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

/// System to handle board cell clicks to draft/place the selected card.
pub fn handle_board_cell_clicks(
    mut selected_card: ResMut<SelectedWagerCard>,
    mut events: EventWriter<ClientActionRequest>,
    cell_query: Query<(&Interaction, &BoardCellNode), Changed<Interaction>>,
) {
    let card_type = match selected_card.0 {
        Some(c) => c,
        None => return,
    };

    for (interaction, cell) in cell_query.iter() {
        if *interaction == Interaction::Pressed {
            events.send(ClientActionRequest(ClientAction::DraftCard {
                card_type,
                cell_index: cell.index,
            }));
            selected_card.0 = None; // Clear selection after drafting
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
