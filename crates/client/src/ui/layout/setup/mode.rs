use bevy::prelude::*;
use crate::ui::components::{ModeStandardButtonNode, ModeWagerButtonNode, RadioDotNode, GameMode};

pub fn spawn_mode_section(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            row_gap: Val::Px(6.0),
            ..default()
        },
        ..default()
    }).with_children(|sec| {
        // Label
        sec.spawn(TextBundle::from_section(
            "GAMEPLAY MODE",
            TextStyle {
                font_size: 11.0,
                color: Color::srgb(0.6, 0.75, 0.65),
                ..default()
            },
        ));

        // Vertical stack of mode buttons
        sec.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                row_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        }).with_children(|col| {
            // STANDARD PLAY
            col.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(14.0)),
                        border: UiRect::all(Val::Px(1.5)),
                        ..default()
                    },
                    background_color: Color::srgb(0.01, 0.30, 0.10).into(), // Pre-selected
                    border_color: Color::srgb(0.10, 0.80, 0.20).into(),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                ModeStandardButtonNode,
            )).with_children(|btn| {
                // Radio Circle Outer
                btn.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(16.0),
                        height: Val::Px(16.0),
                        border: UiRect::all(Val::Px(1.5)),
                        margin: UiRect::right(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: Color::srgb(0.6, 0.8, 0.65).into(),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                }).with_children(|outer| {
                    // Radio Dot Inner
                    outer.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(8.0),
                                height: Val::Px(8.0),
                                display: Display::Flex,
                                ..default()
                            },
                            background_color: Color::srgb(0.10, 0.80, 0.20).into(),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            ..default()
                        },
                        RadioDotNode { mode: GameMode::Standard },
                    ));
                });

                // Mode Info Text
                btn.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                }).with_children(|info| {
                    info.spawn(TextBundle::from_section(
                        "STANDARD PLAY",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                    info.spawn(TextBundle::from_section(
                        "Classic dice roll golfing rules",
                        TextStyle {
                            font_size: 9.0,
                            color: Color::srgb(0.6, 0.8, 0.65),
                            ..default()
                        },
                    ));
                });
            });

            // WAGER CARDS
            col.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(14.0)),
                        border: UiRect::all(Val::Px(1.5)),
                        ..default()
                    },
                    background_color: Color::srgb(0.04, 0.10, 0.06).into(),
                    border_color: Color::srgb(0.15, 0.25, 0.20).into(),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                ModeWagerButtonNode,
            )).with_children(|btn| {
                // Radio Circle Outer
                btn.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(16.0),
                        height: Val::Px(16.0),
                        border: UiRect::all(Val::Px(1.5)),
                        margin: UiRect::right(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: Color::srgb(0.6, 0.8, 0.65).into(),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                }).with_children(|outer| {
                    // Radio Dot Inner
                    outer.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(8.0),
                                height: Val::Px(8.0),
                                display: Display::None,
                                ..default()
                            },
                            background_color: Color::srgb(0.10, 0.80, 0.20).into(),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            ..default()
                        },
                        RadioDotNode { mode: GameMode::WagerCards },
                    ));
                });

                // Mode Info Text
                btn.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                }).with_children(|info| {
                    info.spawn(TextBundle::from_section(
                        "WAGER CARDS",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                    info.spawn(TextBundle::from_section(
                        "Draft power-up cards with tokens",
                        TextStyle {
                            font_size: 9.0,
                            color: Color::srgb(0.6, 0.8, 0.65),
                            ..default()
                        },
                    ));
                });
            });
        });
    });
}
