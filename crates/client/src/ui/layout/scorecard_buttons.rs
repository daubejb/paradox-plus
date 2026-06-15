use bevy::prelude::*;
use crate::ui::components::{CloseScorecardButtonNode, PlayAgainButtonNode, MainMenuButtonNode};

pub fn spawn_scorecard_buttons(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            column_gap: Val::Px(12.0),
            ..default()
        },
        ..default()
    }).with_children(|buttons| {
        // 1. Back to Game Button
        buttons.spawn((
            ButtonBundle {
                style: Style {
                    flex_grow: 1.0,
                    height: Val::Px(44.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    display: Display::None, // Controlled dynamically
                    ..default()
                },
                background_color: Color::srgb(0.85, 0.65, 0.15).into(), // Gold background
                border_color: Color::srgb(0.95, 0.8, 0.2).into(), // Bright gold border
                border_radius: BorderRadius::all(Val::Px(22.0)),
                ..default()
            },
            CloseScorecardButtonNode,
        )).with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                "BACK TO GAME",
                TextStyle {
                    font_size: 13.0,
                    color: Color::srgb(0.01, 0.12, 0.04), // Dark green text on gold
                    ..default()
                },
            ));
        });

        // 2. Play Again Button
        buttons.spawn((
            ButtonBundle {
                style: Style {
                    flex_grow: 1.0,
                    height: Val::Px(44.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    display: Display::None, // Controlled dynamically
                    ..default()
                },
                background_color: Color::srgb(0.02, 0.35, 0.15).into(), // Dark green background
                border_color: Color::srgb(0.05, 0.65, 0.25).into(),
                border_radius: BorderRadius::all(Val::Px(22.0)),
                ..default()
            },
            PlayAgainButtonNode,
        )).with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                "PLAY AGAIN",
                TextStyle { font_size: 13.0, color: Color::WHITE, ..default() },
            ));
        });

        // 3. Main Menu Button
        buttons.spawn((
            ButtonBundle {
                style: Style {
                    flex_grow: 1.0,
                    height: Val::Px(44.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    display: Display::None, // Controlled dynamically
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                border_color: Color::srgb(0.5, 0.6, 0.55).into(),
                border_radius: BorderRadius::all(Val::Px(22.0)),
                ..default()
            },
            MainMenuButtonNode,
        )).with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                "MAIN MENU",
                TextStyle { font_size: 13.0, color: Color::srgb(0.85, 0.9, 0.88), ..default() },
            ));
        });
    });
}
