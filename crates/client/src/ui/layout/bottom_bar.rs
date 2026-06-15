use bevy::prelude::*;
use protocol::messages::CardType;
use crate::ui::components::{
    BottomBarNode, RollOneButtonNode, RollTwoButtonNode, WagerPanelNode, WagerCardButtonNode,
    SkipPlacementButtonNode, WagerCardQtyTextNode
};

pub fn spawn_bottom_controls(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
    let bottom_padding = if cfg!(any(target_os = "android", target_os = "ios")) {
        24.0
    } else {
        10.0
    };
    let bar_height = if cfg!(any(target_os = "android", target_os = "ios")) {
        110.0 + 24.0
    } else {
        110.0
    };

    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(bar_height),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                padding: UiRect {
                    left: Val::Px(12.0),
                    right: Val::Px(12.0),
                    top: Val::Px(10.0),
                    bottom: Val::Px(bottom_padding),
                },
                row_gap: Val::Px(8.0),
                ..default()
            },
            background_color: Color::srgb(0.05, 0.15, 0.10).into(), // Match top HUD color
            ..default()
        },
        BottomBarNode,
    )).with_children(|bar| {
        // Row 1: Action/Roll Buttons
        bar.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(15.0),
                ..default()
            },
            ..default()
        }).with_children(|row| {
            // Roll 1 Die
            row.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.15, 0.45, 0.25).into(),
                    border_color: Color::srgb(0.3, 0.7, 0.4).into(),
                    ..default()
                },
                RollOneButtonNode,
            )).with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "ROLL 1 DIE",
                    TextStyle {
                        font_size: 11.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Roll 2 Dice
            row.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.15, 0.45, 0.25).into(),
                    border_color: Color::srgb(0.3, 0.7, 0.4).into(),
                    ..default()
                },
                RollTwoButtonNode,
            )).with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "ROLL 2 DICE",
                    TextStyle {
                        font_size: 11.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Skip Placement
            row.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        display: Display::None,
                        ..default()
                    },
                    background_color: Color::srgb(0.65, 0.50, 0.15).into(),
                    border_color: Color::srgb(0.85, 0.70, 0.25).into(),
                    ..default()
                },
                SkipPlacementButtonNode,
            )).with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "SKIP PLACE",
                    TextStyle {
                        font_size: 10.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });

        // Row 2: Wager Cards draft panel
        bar.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12.0),
                    ..default()
                },
                ..default()
            },
            WagerPanelNode,
        )).with_children(|row| {
            // Guardian Shield
            row.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(90.0),
                        height: Val::Px(38.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.35, 0.5).into(),
                    border_color: Color::srgb(0.4, 0.6, 0.8).into(),
                    ..default()
                },
                WagerCardButtonNode { card_type: CardType::Shield },
            )).with_children(|btn| {
                btn.spawn((
                    TextBundle::from_section(
                        "SHIELD (0)",
                        TextStyle {
                            font_size: 11.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    WagerCardQtyTextNode { card_type: CardType::Shield },
                ));
            });

            // Banana Slip
            row.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(90.0),
                        height: Val::Px(38.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.7, 0.6, 0.1).into(),
                    border_color: Color::srgb(0.9, 0.8, 0.2).into(),
                    ..default()
                },
                WagerCardButtonNode { card_type: CardType::Banana },
            )).with_children(|btn| {
                btn.spawn((
                    TextBundle::from_section(
                        "BANANA (0)",
                        TextStyle {
                            font_size: 11.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    WagerCardQtyTextNode { card_type: CardType::Banana },
                ));
            });

            // Golden Die
            row.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(90.0),
                        height: Val::Px(38.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.6, 0.1, 0.1).into(),
                    border_color: Color::srgb(0.8, 0.3, 0.3).into(),
                    ..default()
                },
                WagerCardButtonNode { card_type: CardType::GoldenDie },
            )).with_children(|btn| {
                btn.spawn((
                    TextBundle::from_section(
                        "GOLDEN (0)",
                        TextStyle {
                            font_size: 11.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    WagerCardQtyTextNode { card_type: CardType::GoldenDie },
                ));
            });
        });
    });
}
