use bevy::prelude::*;

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
    Gameplay,
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

