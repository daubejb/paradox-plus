use bevy::prelude::*;
use protocol::messages::{GameStateEnum, Scorecard};
use protocol::physics::MovementDirection;
use heapless::Vec as HVec;

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
    pub cards_earned_this_hole: HVec<u8, 4>,
    pub strokes_per_hole: HVec<u16, 18>,
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
            cards_earned_this_hole: HVec::new(),
            strokes_per_hole: HVec::new(),
        }
    }
}

impl OfflineServerState {
    pub fn build_scorecard(&self) -> Scorecard {
        let mut hand = HVec::new();
        for &c in &self.inventory {
            let _ = hand.push(c);
        }
        let mut cards_earned = HVec::new();
        for &c in &self.cards_earned_this_hole {
            let _ = cards_earned.push(c);
        }
        let total_strokes = self.strokes_per_hole
            .iter()
            .fold(0u16, |acc, &s| acc.saturating_add(s));

        let current_strokes: u16 = self.strokes.try_into().unwrap_or(u16::MAX);
        let total = total_strokes.saturating_add(current_strokes);

        Scorecard {
            running_strokes: current_strokes,
            total_strokes: total,
            earned_cards: hand,
            cards_earned_this_hole: cards_earned,
            strokes_per_hole: self.strokes_per_hole.clone(),
        }
    }
}
