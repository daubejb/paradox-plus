use bevy::prelude::*;
use protocol::messages::GameStateEnum;
use protocol::physics::MovementDirection;

#[derive(Resource, Debug, Clone)]
pub struct OfflineServerState {
    pub current_hole: u8,
    pub player_position: u32,
    pub strokes: u32,
    pub direction: MovementDirection,
    pub game_state: GameStateEnum,
    pub active_player_id: u64,
    pub sequence: u64,
    pub is_initialized: bool,
    pub hole_completed_timer_ms: Option<u32>,
    pub course: String,
    pub is_wager_mode: bool,
    pub player_name: String,
    pub inventory: Vec<u8>,
    pub placed_wagers: Vec<protocol::messages::WagerToken>,
}

impl Default for OfflineServerState {
    fn default() -> Self {
        Self {
            current_hole: 1,
            player_position: 0, // Start on TeeBox
            strokes: 0,
            direction: MovementDirection::Forward,
            game_state: GameStateEnum::AwaitingTurn,
            active_player_id: 1234,
            sequence: 0,
            is_initialized: false,
            hole_completed_timer_ms: None,
            course: "green".to_string(),
            is_wager_mode: false,
            player_name: "David".to_string(),
            inventory: Vec::new(),
            placed_wagers: Vec::new(),
        }
    }
}
