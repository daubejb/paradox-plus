use bevy::prelude::*;
use protocol::messages::{GameStateEnum, ClientAction};
use protocol::fsm::{validate_turn, is_valid_action, resolve_next_state};
use super::validation::ClientActionEvent;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerGameState {
    pub state: GameStateEnum,
    pub active_player_id: u64,
    pub current_hole: u8,
    pub sequence: u64,
}

impl Default for ServerGameState {
    fn default() -> Self {
        Self {
            state: GameStateEnum::Lobby,
            active_player_id: 0,
            current_hole: 0,
            sequence: 0,
        }
    }
}

/// Updates the server state machine based on validated client actions.
pub fn fsm_tick_system(
    mut events: EventReader<ClientActionEvent>,
    mut game_state: ResMut<ServerGameState>,
) {
    for ev in events.read() {
        // Gating turn order unless it's the lobby state
        if game_state.state != GameStateEnum::Lobby {
            if !validate_turn(game_state.active_player_id, ev.player_id) {
                // Reject actions sent out of turn
                continue;
            }
        }

        // Validate FSM state-action mapping
        if is_valid_action(&ev.action, game_state.state) {
            match &ev.action {
                ClientAction::CreateRoom => {
                    game_state.state = GameStateEnum::Lobby;
                }
                ClientAction::JoinRoom { .. } => {
                    // Handled by matchmaking setup
                }
                ClientAction::LeaveRoom => {
                    // Handled by connection handlers
                }
                ClientAction::DraftCard { .. } => {
                    // Marker placement card draft
                }
                ClientAction::SkipPlacement => {
                    // End placement, resolve transition to AwaitingTurn
                    game_state.state = resolve_next_state(game_state.state, 1, false, false);
                }
                ClientAction::RollDice { .. } => {
                    // Dice rolling state transition
                    game_state.state = GameStateEnum::Rolling;
                }
                ClientAction::ChooseBananaSlide { .. } => {
                    game_state.state = GameStateEnum::AwaitingTurn;
                }
                ClientAction::AcknowledgeAlert => {
                    game_state.state = GameStateEnum::AwaitingTurn;
                }
            }
            game_state.sequence = game_state.sequence.saturating_add(1);
        }
    }
}
