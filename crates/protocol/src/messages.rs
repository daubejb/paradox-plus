use serde::{Serialize, Deserialize};
use heapless::{Vec as HVec, String as HString};
use crate::physics::MovementDirection;

// Compile-time size boundaries for zero-heap serialization safety
pub const MAX_PLAYERS: usize = 8;
pub const MAX_WAGERS: usize = 16;
pub const MAX_PATH_STEPS: usize = 32;
pub const MAX_ROOM_CODE_LEN: usize = 6;
pub const MAX_PLAYER_NAME_LEN: usize = 16;
pub const MAX_PACKET_SIZE: usize = 65536;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ClientAction {
    // Lobby Matching Actions
    CreateRoom,
    JoinRoom {
        code: HString<MAX_ROOM_CODE_LEN>,
        name: HString<MAX_PLAYER_NAME_LEN>,
    },
    LeaveRoom,
    
    StartPractice {
        nickname: HString<MAX_PLAYER_NAME_LEN>,
        course: HString<8>,
        is_wager_mode: bool,
    },
    
    // Marker Placement Phase
    DraftCard { card_type: CardType, cell_index: u32 },
    SkipPlacement,
    
    // Normal Play Phase
    RollDice { dice_count: u8 },
    
    // Banana Choice Transition
    ChooseBananaSlide { step_count: u8 },
    
    // Game/UI alerts
    AcknowledgeAlert,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ServerUpdate {
    // Room Lobbies updates
    RoomCreated { code: HString<MAX_ROOM_CODE_LEN> },
    RoomJoined { players: HVec<PlayerInfo, MAX_PLAYERS> },
    
    // Game FSM State Sync
    StateSync {
        sequence: u64,
        game_state: GameStateEnum,
        active_player_id: u64,
        current_hole: u8,
        player_positions: HVec<u32, MAX_PLAYERS>,
        player_directions: HVec<MovementDirection, MAX_PLAYERS>,
        player_scores: HVec<Scorecard, MAX_PLAYERS>,
        placed_wagers: HVec<WagerToken, MAX_WAGERS>,
    },
    
    // Immediate action events (dice roll triggers, slide sequences, putting outcomes)
    DiceRollOutcome { roll_values: HVec<u8, 2> }, // Max 2 dice
    SlideTransition { path: HVec<u32, MAX_PATH_STEPS> },
    AlertTriggered { alert_message: HString<64> },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PlayerInfo {
    pub player_id: u64,
    pub name: HString<MAX_PLAYER_NAME_LEN>,
    pub is_ready: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Scorecard {
    pub running_strokes: u16,
    pub total_strokes: u16,
    pub earned_cards: HVec<u8, 16>, // Max 16 earned cards
    pub cards_earned_this_hole: HVec<u8, 4>, // Max 4 earned cards on this hole
    pub strokes_per_hole: HVec<u16, 18>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WagerToken {
    pub card_type: CardType,
    pub owner_id: u64,
    pub cell_index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CardType {
    Shield = 0,
    Banana = 1,
    GoldenDie = 2,
}

impl TryFrom<u8> for CardType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Shield),
            1 => Ok(Self::Banana),
            2 => Ok(Self::GoldenDie),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameStateEnum {
    Lobby,
    MarkerPlacement,
    AwaitingTurn,
    Rolling,
    Moving,
    BananaChoice,
    HazardAlert,
    HoleCompleted,
    MatchCompleted,
}
