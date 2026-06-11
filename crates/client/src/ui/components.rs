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
