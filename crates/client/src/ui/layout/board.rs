use bevy::prelude::*;
use crate::ui::components::BoardContainerNode;

pub const TILE_WIDTH: f32 = 60.0;
pub const TILE_HEIGHT: f32 = 52.0;
pub const TILE_HALF_WIDTH_PCT: f32 = 6.52;   // (30px / 460px * 100)
pub const TILE_HALF_HEIGHT_PCT: f32 = 4.41;  // (26px / 590px * 100)

/// Returns (left_percent, top_percent, rotation_radians) for stadium tile at index.
pub fn get_stadium_tile_transform(idx: usize) -> (f32, f32, f32) {
    let total_tiles = 27;
    let r = 200.0;
    let straight_len = 130.0;
    let arc_len = r * std::f32::consts::FRAC_PI_2; // 314.159
    let total_len = 2.0 * straight_len + 4.0 * arc_len; // 1516.64

    let container_width = 460.0;
    let container_height = 590.0;

    let s = (idx as f32) * (total_len / total_tiles as f32);

    let (x, y, alpha) = if s < arc_len {
        // Bottom-left curve: s in [0, 314.16]
        let phi = -std::f32::consts::FRAC_PI_2 - (s / r);
        let px = 230.0 + r * phi.cos();
        let py = 360.0 - r * phi.sin();
        let angle = -std::f32::consts::FRAC_PI_2 * (1.0 - s / arc_len);
        (px, py, angle)
    } else if s < arc_len + straight_len {
        // Left vertical segment: s in [314.16, 444.16]
        let dy = s - arc_len;
        let px = 30.0;
        let py = 360.0 - dy;
        (px, py, 0.0)
    } else if s < arc_len + straight_len + 2.0 * arc_len {
        // Top semicircle (top-left to top-right): s in [444.16, 1072.48]
        let ds = s - (arc_len + straight_len);
        let px = 230.0 + r * (std::f32::consts::PI - (ds / r)).cos();
        let py = 230.0 - r * (std::f32::consts::PI - (ds / r)).sin();
        let angle = if ds < arc_len {
            std::f32::consts::FRAC_PI_2 * (ds / arc_len)
        } else {
            std::f32::consts::FRAC_PI_2 * (1.0 - (ds - arc_len) / arc_len)
        };
        (px, py, angle)
    } else if s < arc_len + straight_len + 2.0 * arc_len + straight_len {
        // Right vertical segment: s in [1072.48, 1202.48]
        let dy = s - (arc_len + straight_len + 2.0 * arc_len);
        let px = 430.0;
        let py = 230.0 + dy;
        (px, py, 0.0)
    } else {
        // Bottom-right curve: s in [1202.48, 1516.64]
        let ds = s - (arc_len + straight_len + 2.0 * arc_len + straight_len);
        let px = 230.0 + r * (- (ds / r)).cos();
        let py = 360.0 - r * (- (ds / r)).sin();
        let angle = -std::f32::consts::FRAC_PI_2 * (ds / arc_len);
        (px, py, angle)
    };

    // Convert to percentage of container
    let left_pct = (x / container_width) * 100.0;
    let top_pct = (y / container_height) * 100.0;

    (left_pct, top_pct, alpha)
}

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
