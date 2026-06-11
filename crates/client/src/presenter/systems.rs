use bevy::prelude::*;
use crate::replication::Ball;
use crate::presenter::components::BallVisualInterpolation;
use crate::presenter::helpers::{get_cell_spatial_position, get_cell_spatial_vector};

pub fn fixed_to_float_translation_system(
    mut query: Query<(&Ball, &BallVisualInterpolation, &mut Transform)>,
) {
    for (ball, visual, mut transform) in query.iter_mut() {
        if let Some(cell_pos) = get_cell_spatial_position(ball.cell_index) {
            if let Some(dir_vec) = get_cell_spatial_vector(ball.direction) {
                let offset = dir_vec * visual.slide_offset;
                transform.translation = cell_pos + offset;
            }
        }
    }
}
