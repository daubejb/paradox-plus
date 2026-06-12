use bevy::prelude::*;
use protocol::messages::CardType;
use crate::ui::components::{
    BottomBarNode, RollOneButtonNode, RollTwoButtonNode, WagerPanelNode, WagerCardButtonNode,
    SkipPlacementButtonNode, WagerCardQtyTextNode
};

pub fn spawn_bottom_controls(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(160.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            background_color: Color::srgb(0.05, 0.15, 0.10).into(), // Match top HUD color
            ..default()
        },
        BottomBarNode,
    )).with_children(|bar| {
        // 1. Wager Card Draft Panel (Top row of bottom bar)
        bar.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(15.0),
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },
            WagerPanelNode,
        )).with_children(|panel| {
            // Guardian Shield (card_type = 0)
            panel.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(85.0),
                        height: Val::Px(40.0),
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

            // Banana Slip (card_type = 1)
            panel.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(85.0),
                        height: Val::Px(40.0),
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

            // Golden Die (card_type = 2)
            panel.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(85.0),
                        height: Val::Px(40.0),
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

        // 2. Roll Action Buttons (Bottom row of bottom bar)
        bar.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|buttons_row| {
            // Roll 1 Die button (Bottom Left)
            buttons_row.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(120.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::right(Val::Px(10.0)),
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
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Roll 2 Dice button (Bottom Right)
            buttons_row.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(120.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::left(Val::Px(10.0)),
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
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // Skip Placement button (initially hidden)
            buttons_row.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(260.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        display: Display::None,
                        ..default()
                    },
                    background_color: Color::srgb(0.65, 0.50, 0.15).into(), // Warm gold
                    border_color: Color::srgb(0.85, 0.70, 0.25).into(),
                    ..default()
                },
                SkipPlacementButtonNode,
            )).with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "SKIP PLACEMENT",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });
    });
}
