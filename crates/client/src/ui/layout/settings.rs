use bevy::prelude::*;
use crate::ui::components::{SettingsScreenNode, SoundToggleButtonNode, SoundToggleTextNode, CloseSettingsButtonNode};

pub fn spawn_settings_screen(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
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
        SettingsScreenNode,
    )).with_children(|overlay| {
        // 2. Center Settings Card Dialog (Glassmorphism look)
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
                "SETTINGS",
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

            // Settings Content Section
            card.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    row_gap: Val::Px(12.0),
                    ..default()
                },
                ..default()
            }).with_children(|content| {
                // Category Label
                content.spawn(TextBundle::from_section(
                    "AUDIO SETTINGS",
                    TextStyle {
                        font_size: 11.0,
                        color: Color::srgb(0.6, 0.75, 0.65),
                        ..default()
                    },
                ));

                // Sound Effects row
                content.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::vertical(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                }).with_children(|row| {
                    row.spawn(TextBundle::from_section(
                        "Sound Effects",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));

                    // Control container: Indicator Text + Switch Button
                    row.spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    }).with_children(|controls| {
                        // ON / OFF text label
                        controls.spawn((
                            TextBundle::from_section(
                                "ON",
                                TextStyle {
                                    font_size: 12.0,
                                    color: Color::srgb(0.6, 0.8, 0.65),
                                    ..default()
                                },
                            ),
                            SoundToggleTextNode,
                        ));

                        // Capsule toggle pill button
                        controls.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(54.0),
                                    height: Val::Px(30.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    padding: UiRect::all(Val::Px(2.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::FlexEnd, // Default to ON
                                    ..default()
                                },
                                background_color: Color::srgb(0.10, 0.50, 0.22).into(), // Default ON green
                                border_color: Color::srgb(0.15, 0.80, 0.35).into(),     // Bright green border
                                border_radius: BorderRadius::all(Val::Px(15.0)),
                                ..default()
                            },
                            SoundToggleButtonNode,
                        )).with_children(|btn| {
                            // Visual slider white dot
                            btn.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Px(22.0),
                                    height: Val::Px(22.0),
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                border_radius: BorderRadius::all(Val::Px(11.0)),
                                ..default()
                            });
                        });
                    });
                });
            });

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
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            }).with_children(|footer| {
                // CLOSE button
                footer.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
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
                    CloseSettingsButtonNode,
                )).with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "CLOSE",
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
