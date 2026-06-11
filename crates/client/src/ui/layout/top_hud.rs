use bevy::prelude::*;
use crate::ui::components::{TopHudNode, HamburgerButtonNode, HoleInfoNode, PlayerInfoNode};

pub fn spawn_top_hud(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(70.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                ..default()
            },
            background_color: Color::srgb(0.05, 0.15, 0.10).into(), // Dark green header overlay
            ..default()
        },
        TopHudNode,
    )).with_children(|hud| {
        // 1. Hamburger button (Top Left)
        hud.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgb(0.1, 0.25, 0.18).into(),
                border_color: Color::srgb(0.2, 0.4, 0.3).into(),
                ..default()
            },
            HamburgerButtonNode,
        )).with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                "☰",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });

        // 2. Hole Details (Top Middle)
        hud.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            HoleInfoNode,
        )).with_children(|info| {
            info.spawn(TextBundle::from_section(
                "HOLE 7",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            info.spawn(TextBundle::from_section(
                "PAR 5 • 27 SPACES",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.7, 0.9, 0.8),
                    ..default()
                },
            ));
        });

        // 3. Active Player Info (Top Right)
        hud.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            PlayerInfoNode,
        )).with_children(|player| {
            // Avatar Placeholder (a roundish box)
            player.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgb(0.8, 0.6, 0.2).into(), // Warm gold avatar placeholder
                border_color: Color::WHITE.into(),
                ..default()
            });

            // Name & Differential Stack
            player.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    ..default()
                },
                ..default()
            }).with_children(|text_stack| {
                text_stack.spawn(TextBundle::from_section(
                    "DAVID",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                text_stack.spawn(TextBundle::from_section(
                    "🏆 -2",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.9, 0.8, 0.2),
                        ..default()
                    },
                ));
            });
        });
    });
}
