use bevy::prelude::*;
use crate::ui::components::*;

pub fn spawn_landing_screen(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::axes(Val::Px(20.0), Val::Px(40.0)),
                ..default()
            },
            background_color: Color::srgb(0.02, 0.12, 0.06).into(), // Dark green background
            ..default()
        },
        LandingScreenNode,
    )).with_children(|screen| {
        // 1. Title/Logo at the top
        let logo_handle: Handle<Image> = asset_server.load("brand_logo.png");
        screen.spawn(ImageBundle {
            style: Style {
                width: Val::Px(280.0),
                height: Val::Px(140.0),
                margin: UiRect::top(Val::Px(40.0)),
                ..default()
            },
            image: logo_handle.into(),
            ..default()
        });

        // 2. Button Stack in the middle
        screen.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            ..default()
        }).with_children(|stack| {
            // SOLO PRACTICE
            stack.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(240.0),
                        height: Val::Px(46.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.01, 0.22, 0.08).into(),
                    border_color: Color::srgb(0.05, 0.65, 0.25).into(),
                    border_radius: BorderRadius::all(Val::Px(23.0)),
                    ..default()
                },
                SoloPracticeButtonNode,
            )).with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "👤 SOLO PRACTICE",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // VS COMPUTER BOTS
            stack.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(240.0),
                        height: Val::Px(46.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.01, 0.22, 0.08).into(),
                    border_color: Color::srgb(0.05, 0.65, 0.25).into(),
                    border_radius: BorderRadius::all(Val::Px(23.0)),
                    ..default()
                },
                VsBotsButtonNode,
            )).with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "🤖 VS COMPUTER BOTS",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // ONLINE MULTIPLAYER
            stack.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(240.0),
                        height: Val::Px(46.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.01, 0.18, 0.06).into(),
                    border_color: Color::srgb(0.45, 0.40, 0.10).into(), // Warm goldish border
                    border_radius: BorderRadius::all(Val::Px(23.0)),
                    ..default()
                },
                OnlineMultiplayerButtonNode,
            )).with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "🌐 ONLINE MULTIPLAYER",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            // STATISTICS
            stack.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(180.0),
                        height: Val::Px(38.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.01, 0.15, 0.05).into(),
                    border_color: Color::srgb(0.25, 0.45, 0.30).into(),
                    border_radius: BorderRadius::all(Val::Px(19.0)),
                    ..default()
                },
                StatsButtonNode,
            )).with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "📊 STATISTICS",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.85, 0.95, 0.90),
                        ..default()
                    },
                ));
            });
        });

        // 3. Footer Links and Status Area at the bottom
        screen.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }).with_children(|footer_container| {
            // View Rules and Settings row
            footer_container.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    width: Val::Percent(80.0),
                    ..default()
                },
                ..default()
            }).with_children(|row| {
                // View Rules
                row.spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    },
                    ViewRulesButtonNode,
                )).with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "ⓘ View Rules",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::srgb(0.7, 0.9, 0.8),
                            ..default()
                        },
                    ));
                });

                // Settings
                row.spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    },
                    SettingsButtonNode,
                )).with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "⚙ Settings",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::srgb(0.7, 0.9, 0.8),
                            ..default()
                        },
                    ));
                });
            });

            // Feedback Status Text area
            footer_container.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 11.0,
                        color: Color::srgb(0.9, 0.8, 0.3),
                        ..default()
                    },
                ),
                LandingStatusTextNode,
            ));
        });
    });
}
