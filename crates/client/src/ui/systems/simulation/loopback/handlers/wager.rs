use super::super::state::OfflineServerState;
use protocol::messages::{ServerUpdate, GameStateEnum, CardType};
use protocol::terrain::{ActiveCourseTrack, TerrainType};
use heapless::Vec as HVec;

fn send_state_sync(state: &mut OfflineServerState, mut updates: Vec<ServerUpdate>) -> Vec<ServerUpdate> {
    let mut player_positions = HVec::new();
    player_positions.push(state.player_position).unwrap();
    let mut player_scores = HVec::new();
    player_scores.push(state.build_scorecard()).unwrap();

    let mut wagers = HVec::new();
    for w in &state.placed_wagers {
        let _ = wagers.push(w.clone());
    }

    updates.push(ServerUpdate::StateSync {
        sequence: state.sequence,
        game_state: state.game_state,
        active_player_id: state.active_player_id,
        current_hole: state.current_hole,
        player_positions,
        player_scores,
        placed_wagers: wagers,
    });

    updates
}

pub fn handle_draft_card(
    state: &mut OfflineServerState,
    card_type: CardType,
    cell_index: u32,
    course: &ActiveCourseTrack,
) -> Vec<ServerUpdate> {
    let mut updates = Vec::new();

    // 1. Validation Checks

    // Rule 1: Only one wager card per tile
    if state.placed_wagers.iter().any(|w| w.cell_index == cell_index) {
        updates.push(ServerUpdate::AlertTriggered {
            alert_message: heapless::String::try_from("Tile already has a wager token placed!").unwrap(),
        });
        return send_state_sync(state, updates);
    }

    // Rule 2: No wagers can be placed on the Tee box (Space 0) or Green tiles
    if cell_index == 0 {
        updates.push(ServerUpdate::AlertTriggered {
            alert_message: heapless::String::try_from("Cannot place wager on Tee Box!").unwrap(),
        });
        return send_state_sync(state, updates);
    }

    let cell_terrain = course.cells.get(cell_index as usize).copied();
    if let Some(TerrainType::Green(_)) = cell_terrain {
        updates.push(ServerUpdate::AlertTriggered {
            alert_message: heapless::String::try_from("Cannot place wager on Green tiles!").unwrap(),
        });
        return send_state_sync(state, updates);
    }

    // Rule 3: Guardian Shield can only be placed on a Hazard cell (Rough, Bunker, Water, OutOfBounds)
    if card_type == CardType::Shield {
        let is_hazard = match cell_terrain {
            Some(TerrainType::Rough) | Some(TerrainType::Bunker) | Some(TerrainType::Water) | Some(TerrainType::OutOfBounds) => true,
            _ => false,
        };
        if !is_hazard {
            updates.push(ServerUpdate::AlertTriggered {
                alert_message: heapless::String::try_from("Guardian Shield must be placed on a Hazard!").unwrap(),
            });
            return send_state_sync(state, updates);
        }
    }

    // Rule 4: Trickster and Golden Die can only be placed on non-hazard track tiles (Fairway)
    if card_type == CardType::Banana || card_type == CardType::GoldenDie {
        let is_fairway = match cell_terrain {
            Some(TerrainType::Fairway) => true,
            _ => false,
        };
        if !is_fairway {
            let card_name = if card_type == CardType::Banana { "Trickster Banana" } else { "Golden Die" };
            updates.push(ServerUpdate::AlertTriggered {
                alert_message: heapless::String::try_from(format!("{} must be placed on Fairway!", card_name).as_str()).unwrap(),
            });
            return send_state_sync(state, updates);
        }
    }

    // Rule 5: Trickster Banana cannot be placed 4 spaces ahead of an Out-of-Bounds (OB) cell
    if card_type == CardType::Banana {
        let ahead_idx = cell_index + 4;
        if let Some(TerrainType::OutOfBounds) = course.cells.get(ahead_idx as usize) {
            updates.push(ServerUpdate::AlertTriggered {
                alert_message: heapless::String::try_from("Trickster Banana cannot be placed 4 spaces before OB!").unwrap(),
            });
            return send_state_sync(state, updates);
        }
    }

    // 2. Execution
    if let Some(pos) = state.inventory.iter().position(|&c| c == card_type as u8) {
        state.inventory.remove(pos);
        state.placed_wagers.push(protocol::messages::WagerToken {
            card_type,
            owner_id: state.active_player_id,
            cell_index,
        });
        state.sequence += 1;

        if state.inventory.is_empty() {
            state.game_state = GameStateEnum::AwaitingTurn;
        }

        let card_name = match card_type {
            CardType::Shield => "Guardian Shield",
            CardType::Banana => "Trickster Banana",
            CardType::GoldenDie => "Golden Die",
        };
        updates.push(ServerUpdate::AlertTriggered {
            alert_message: heapless::String::try_from(format!("Placed {} wager!", card_name).as_str()).unwrap(),
        });
    } else {
        updates.push(ServerUpdate::AlertTriggered {
            alert_message: heapless::String::try_from("No card of that type in hand!").unwrap(),
        });
    }

    send_state_sync(state, updates)
}

pub fn handle_skip_placement(state: &mut OfflineServerState) -> Vec<ServerUpdate> {
    let updates = Vec::new();

    state.game_state = GameStateEnum::AwaitingTurn;
    state.sequence += 1;

    send_state_sync(state, updates)
}
