use bevy::prelude::*;
use crate::ui::components::{SoloSetupScreenNode, PlayGameButtonNode, CancelSetupButtonNode};

pub mod nickname;
pub mod course;
pub mod mode;

pub fn spawn_setup_screen(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    // 1. Semi-transparent backdrop overlay full screen
    parent.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(16.0)),
                display: Display::None, // Hidden initially
                ..default()
            },
            background_color: Color::srgba(0.01, 0.05, 0.03, 0.92).into(), // High-contrast transparent dark green overlay
            ..default()
        },
        SoloSetupScreenNode,
    )).with_children(|overlay| {
        // 2. Center Setup Card Dialog (Glassmorphism look)
        overlay.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                max_width: Val::Px(360.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                border: UiRect::all(Val::Px(2.0)),
                row_gap: Val::Px(20.0),
                ..default()
            },
            background_color: Color::srgba(0.03, 0.15, 0.08, 0.95).into(), // Deep forest overlay panel
            border_color: Color::srgba(0.1, 0.5, 0.25, 0.4).into(),
            border_radius: BorderRadius::all(Val::Px(16.0)),
            ..default()
        }).with_children(|card| {
            // Screen Header
            card.spawn(TextBundle::from_section(
                "SOLO PRACTICE",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Horizontal Separator
            card.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    ..default()
                },
                background_color: Color::srgba(0.15, 0.35, 0.2, 0.5).into(),
                ..default()
            });

            // Spawners for Sub-sections
            nickname::spawn_nickname_section(card, asset_server);
            course::spawn_course_section(card, asset_server);
            mode::spawn_mode_section(card, asset_server);

            // Horizontal Separator
            card.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    ..default()
                },
                background_color: Color::srgba(0.15, 0.35, 0.2, 0.5).into(),
                ..default()
            });

            // Action Footer controls
            card.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            }).with_children(|footer| {
                // Cancel link button
                footer.spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            height: Val::Px(40.0),
                            padding: UiRect::horizontal(Val::Px(12.0)),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    },
                    CancelSetupButtonNode,
                )).with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "Cancel",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::srgb(0.7, 0.85, 0.75),
                            ..default()
                        },
                    ));
                });

                // PLAY GAME button
                footer.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(120.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.5)),
                            ..default()
                        },
                        background_color: Color::srgb(0.10, 0.50, 0.22).into(),
                        border_color: Color::srgb(0.15, 0.80, 0.35).into(),
                        border_radius: BorderRadius::all(Val::Px(20.0)),
                        ..default()
                    },
                    PlayGameButtonNode,
                )).with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "PLAY GAME",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
            });
        });
    });
}
