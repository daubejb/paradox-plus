use bevy::prelude::*;
use crate::ui::components::BoardContainerNode;

/// Fixed stadium-loop center coordinates (left%, top%) for the 27 tiles.
/// Tile 0 is the Tee (bottom center), progressing clockwise.
pub const BOARD_TILE_POSITIONS: [(f32, f32); 27] = [
    // Bottom Center (Tee, index 0)
    (44.0, 90.0),
    // Left bottom curve
    (34.0, 88.0),
    (25.0, 83.0),
    (18.0, 76.0),
    // Left column going up
    (14.0, 67.0),
    (12.0, 58.0),
    (12.0, 48.0),
    (12.0, 38.0),
    // Left top curve
    (14.0, 29.0),
    (18.0, 20.0),
    (25.0, 13.0),
    (34.0, 8.0),
    (44.0, 6.0),
    // Top Center (index 13)
    (50.0, 5.0),
    // Right top curve
    (56.0, 6.0),
    (66.0, 8.0),
    (75.0, 13.0),
    (82.0, 20.0),
    (86.0, 29.0),
    // Right column going down
    (88.0, 38.0),
    (88.0, 48.0),
    (88.0, 58.0),
    (88.0, 67.0),
    // Right bottom curve
    (82.0, 76.0),
    (75.0, 83.0),
    (66.0, 88.0),
    // Bottom Center close (index 26)
    (56.0, 90.0),
];

pub const TILE_SIZE: f32 = 54.0;
pub const TILE_OFFSET_X: f32 = 6.0;   // Half of (54px / 450px * 100)
pub const TILE_OFFSET_Y: f32 = 4.65;  // Half of (54px / 580px * 100)

pub fn spawn_central_board(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        },
        BoardContainerNode,
    ));
}
