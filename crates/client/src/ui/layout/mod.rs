pub mod top_hud;
pub mod board;
pub mod bottom_bar;

use bevy::prelude::*;
use crate::ui::components::RootUiNode;

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
        top_hud::spawn_top_hud(parent, &asset_server);
        board::spawn_central_board(parent, &asset_server);
        bottom_bar::spawn_bottom_controls(parent, &asset_server);
    });
}
