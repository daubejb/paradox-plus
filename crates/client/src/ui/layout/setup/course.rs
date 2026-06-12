use bevy::prelude::*;
use crate::ui::components::{CourseGreenButtonNode, CourseBlueButtonNode};

pub fn spawn_course_section(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
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
            "SELECT COURSE",
            TextStyle {
                font_size: 11.0,
                color: Color::srgb(0.6, 0.75, 0.65),
                ..default()
            },
        ));

        // Two-column button container
        sec.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                column_gap: Val::Px(12.0),
                ..default()
            },
            ..default()
        }).with_children(|row| {
            // GREEN Course Button
            row.spawn((
                ButtonBundle {
                    style: Style {
                        flex_grow: 1.0,
                        height: Val::Px(56.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(1.5)),
                        row_gap: Val::Px(2.0),
                        ..default()
                    },
                    background_color: Color::srgb(0.01, 0.30, 0.10).into(), // Pre-selected highlight green
                    border_color: Color::srgb(0.10, 0.80, 0.20).into(),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                CourseGreenButtonNode,
            )).with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "GREEN COURSE",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                btn.spawn(TextBundle::from_section(
                    "Classic Fairways",
                    TextStyle {
                        font_size: 9.0,
                        color: Color::srgb(0.6, 0.8, 0.65),
                        ..default()
                    },
                ));
            });

            // BLUE Course Button
            row.spawn((
                ButtonBundle {
                    style: Style {
                        flex_grow: 1.0,
                        height: Val::Px(56.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(1.5)),
                        row_gap: Val::Px(2.0),
                        ..default()
                    },
                    background_color: Color::srgb(0.04, 0.10, 0.06).into(),
                    border_color: Color::srgb(0.15, 0.25, 0.20).into(),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                CourseBlueButtonNode,
            )).with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "BLUE COURSE",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                btn.spawn(TextBundle::from_section(
                    "Coastal Breeze",
                    TextStyle {
                        font_size: 9.0,
                        color: Color::srgb(0.6, 0.7, 0.8),
                        ..default()
                    },
                ));
            });
        });
    });
}
