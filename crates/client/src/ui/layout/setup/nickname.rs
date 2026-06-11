use bevy::prelude::*;
use crate::ui::components::{NicknameInputContainerNode, NicknameTextNode};

pub fn spawn_nickname_section(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
    // Container for nickname label and input box
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
            "YOUR NICKNAME",
            TextStyle {
                font_size: 11.0,
                color: Color::srgb(0.6, 0.75, 0.65),
                ..default()
            },
        ));

        // Input Field Button Box
        sec.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(44.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    padding: UiRect::left(Val::Px(16.0)),
                    border: UiRect::all(Val::Px(1.5)),
                    ..default()
                },
                background_color: Color::srgb(0.04, 0.10, 0.06).into(),
                border_color: Color::srgb(0.15, 0.25, 0.20).into(),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            NicknameInputContainerNode,
        )).with_children(|btn| {
            btn.spawn((
                TextBundle::from_section(
                    "David",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                NicknameTextNode,
            ));
        });
    });
}
