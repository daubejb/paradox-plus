use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RootUiNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TopHudNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct HamburgerButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct HoleInfoNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerInfoNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardContainerNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BottomBarNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RollOneButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RollTwoButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WagerPanelNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WagerCardButtonNode {
    pub card_type: u8,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct HoleTitleTextNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct HoleStatsTextNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerScoreTextNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RollStatusTextNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardCellNode {
    pub index: u32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerTokenMarker;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentHole(pub u8);

impl Default for CurrentHole {
    fn default() -> Self {
        Self(u8::MAX) // Sentinel value to guarantee first sync triggers change detection
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ClientScreenState {
    #[default]
    Landing,
    SoloSetup,
    Gameplay,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Standard,
    WagerCards,
}

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct GameSettings {
    pub nickname: String,
    pub course: String,
    pub mode: GameMode,
    pub is_input_focused: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            nickname: "David".to_string(),
            course: "green".to_string(),
            mode: GameMode::Standard,
            is_input_focused: false,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LandingScreenNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameplayScreenNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoloPracticeButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct VsBotsButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnlineMultiplayerButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatsButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewRulesButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LandingStatusTextNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoloSetupScreenNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct NicknameInputContainerNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct NicknameTextNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CourseGreenButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CourseBlueButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeStandardButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeWagerButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayGameButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CancelSetupButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkipPlacementButtonNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WagerCardQtyTextNode {
    pub card_type: u8,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WagerTokenMarker;

#[derive(Resource, Default, Debug, Clone)]
pub struct ClientWagers(pub Vec<protocol::messages::WagerToken>);

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectedWagerCard(pub Option<u8>);

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq)]
pub struct CursorPositionOverride(pub Option<Vec2>);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadioDotNode {
    pub mode: GameMode,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerNameTextNode;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeaderboardTickerContainerNode;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct LeaderboardTickerTrackNode {
    pub scroll_offset: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewFullLeaderboardButtonNode;

#[derive(Resource, Default, Debug, Clone)]
pub struct LeaderboardCompletedHolesScore {
    pub player_par_scores: Vec<i32>,
    pub last_completed_hole: u8,
}



