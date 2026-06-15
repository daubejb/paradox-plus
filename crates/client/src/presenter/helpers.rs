use bevy::prelude::*;
use fixed::types::I32F32;
use protocol::physics::MovementDirection;

/// Project cell coordinates safely to a 3D float vector using checked fixed-point spacing.
pub fn get_cell_spatial_position(cell_index: u16) -> Option<Vec3> {
    let spacing = I32F32::from_num(2.5);
    let index_fixed = I32F32::from_num(cell_index);
    let x_fixed = index_fixed.checked_mul(spacing)?;
    Some(Vec3::new(x_fixed.to_num::<f32>(), 0.0, 0.0))
}

/// Project direction offset vector safely to a 3D float vector using checked fixed-point spacing.
pub fn get_cell_spatial_vector(direction: MovementDirection) -> Option<Vec3> {
    let spacing = I32F32::from_num(2.5);
    let val_fixed = match direction {
        MovementDirection::Forward => spacing,
        MovementDirection::Reverse => spacing.checked_neg()?,
    };
    Some(Vec3::new(val_fixed.to_num::<f32>(), 0.0, 0.0))
}
