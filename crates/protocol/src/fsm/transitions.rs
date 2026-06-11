use crate::messages::GameStateEnum;

/// Resolves the next state based on the current state and game context.
pub fn resolve_next_state(
    current_state: GameStateEnum,
    mode: u8, // 0: Standard, 1: Wager Cards
    is_last_hole: bool,
    has_more_placements: bool,
) -> GameStateEnum {
    match current_state {
        GameStateEnum::Lobby => {
            if mode == 1 {
                GameStateEnum::MarkerPlacement
            } else {
                GameStateEnum::AwaitingTurn
            }
        }
        GameStateEnum::MarkerPlacement => {
            if has_more_placements {
                GameStateEnum::MarkerPlacement
            } else {
                GameStateEnum::AwaitingTurn
            }
        }
        GameStateEnum::AwaitingTurn => GameStateEnum::Rolling,
        GameStateEnum::Rolling => GameStateEnum::Moving,
        GameStateEnum::Moving => GameStateEnum::AwaitingTurn,
        GameStateEnum::BananaChoice => GameStateEnum::AwaitingTurn,
        GameStateEnum::HazardAlert => GameStateEnum::AwaitingTurn,
        GameStateEnum::HoleCompleted => {
            if is_last_hole {
                GameStateEnum::MatchCompleted
            } else if mode == 1 {
                GameStateEnum::MarkerPlacement
            } else {
                GameStateEnum::AwaitingTurn
            }
        }
        GameStateEnum::MatchCompleted => GameStateEnum::MatchCompleted,
    }
}
