use crate::messages::{ClientAction, GameStateEnum};

/// Gates client actions based on the active player.
#[inline(always)]
pub fn validate_turn(active_player_id: u64, acting_player_id: u64) -> bool {
    active_player_id == acting_player_id
}

/// Gates client actions based on the current FSM state.
pub fn is_valid_action(action: &ClientAction, current_state: GameStateEnum) -> bool {
    match current_state {
        GameStateEnum::Lobby => {
            matches!(
                action,
                ClientAction::CreateRoom
                    | ClientAction::JoinRoom { .. }
                    | ClientAction::LeaveRoom
            )
        }
        GameStateEnum::MarkerPlacement => {
            matches!(
                action,
                ClientAction::DraftCard { .. } | ClientAction::SkipPlacement
            )
        }
        GameStateEnum::AwaitingTurn => {
            matches!(action, ClientAction::RollDice { .. })
        }
        GameStateEnum::Rolling => false,
        GameStateEnum::Moving => false,
        GameStateEnum::BananaChoice => {
            matches!(action, ClientAction::ChooseBananaSlide { .. })
        }
        GameStateEnum::HazardAlert => {
            matches!(action, ClientAction::AcknowledgeAlert)
        }
        GameStateEnum::HoleCompleted => false,
        GameStateEnum::MatchCompleted => false,
    }
}
