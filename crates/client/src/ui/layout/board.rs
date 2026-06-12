use bevy::prelude::*;
use crate::ui::components::BoardContainerNode;

pub const TILE_SIZE: f32 = 54.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellLayout {
    pub left_pct: f32,
    pub top_pct: f32,
    pub rotation_angle: f32,
}

/// Computes the ellipse board coordinates and rotation angles.
pub fn calculate_cell_layout(idx: usize, total_cells: usize) -> CellLayout {
    let n = total_cells as f32;
    let i = idx as f32;

    // Parametric angle starting at bottom center (6 o'clock)
    let theta = std::f32::consts::FRAC_PI_2 + (2.0 * std::f32::consts::PI * i) / n;

    let left_pct = 50.0 + 38.0 * theta.cos();
    let top_pct = 50.0 + 40.0 * theta.sin();

    let mut rotation_angle = theta - std::f32::consts::FRAC_PI_2;

    // Keep text/cell facing right-side up
    if rotation_angle.cos() < 0.0 {
        rotation_angle += std::f32::consts::PI;
    }

    CellLayout {
        left_pct,
        top_pct,
        rotation_angle,
    }
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
