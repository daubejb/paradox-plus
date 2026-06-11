pub mod top_hud;
pub mod board;
pub mod bottom_bar;
pub mod landing;

use bevy::prelude::*;
use crate::ui::components::{RootUiNode, GameplayScreenNode};
use landing::spawn_landing_screen;

pub fn spawn_ui_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::srgb(0.08, 0.22, 0.14).into(), // Rich forest green theme
            ..default()
        },
        RootUiNode,
    )).with_children(|parent| {
        // 1. Spawn Landing Page Screen (visible by default)
        spawn_landing_screen(parent, &asset_server);

        // 2. Spawn Gameplay Screen (hidden by default)
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Stretch,
                    justify_content: JustifyContent::SpaceBetween,
                    display: Display::None, // Hidden until Solo Practice is chosen
                    ..default()
                },
                ..default()
            },
            GameplayScreenNode,
        )).with_children(|gameplay_container| {
            top_hud::spawn_top_hud(gameplay_container, &asset_server);
            board::spawn_central_board(gameplay_container, &asset_server);
            bottom_bar::spawn_bottom_controls(gameplay_container, &asset_server);
        });
    });
}
