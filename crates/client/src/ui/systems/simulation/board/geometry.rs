use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellLayout {
    pub position: Vec2,
    pub rotation_angle: f32,
}

/// Computes the 2D position and rotation of a cell along a parametric capsule track.
pub fn calculate_capsule_layout(
    idx: f32,
    total_cells: usize,
    viewport_size: Vec2,
) -> CellLayout {
    // Check if orientation is portrait
    let is_portrait = viewport_size.y > viewport_size.x;

    // Swap width and height for calculations if in portrait mode
    let (w, h) = if is_portrait {
        (viewport_size.y, viewport_size.x)
    } else {
        (viewport_size.x, viewport_size.y)
    };

    // Define racetrack dimensions: straight length l, semicircle radius r
    let r = (h * 0.40).min(w * 0.25).max(40.0);
    let l = (w * 0.35).max(60.0);

    let perimeter = 2.0 * l + 2.0 * std::f32::consts::PI * r;

    // Distribute cells evenly along the perimeter with clean loop wrapping
    let capacity = total_cells as f32;
    let wrapped_idx = (idx % capacity + capacity) % capacity;
    let fraction = wrapped_idx / capacity;
    let s = fraction * perimeter;

    let mut pos = Vec2::ZERO;
    let mut tangent;

    if s < l {
        // 1. Bottom straight segment (left to right)
        pos.x = -l / 2.0 + s;
        pos.y = -r;
        tangent = Vec2::new(1.0, 0.0);
    } else if s < l + std::f32::consts::PI * r {
        // 2. Right semicircle (bottom to top, clockwise)
        let arc_s = s - l;
        let theta = -std::f32::consts::FRAC_PI_2 + (arc_s / r);
        pos.x = l / 2.0 + r * theta.cos();
        pos.y = r * theta.sin();
        tangent = Vec2::new(-theta.sin(), theta.cos());
    } else if s < 2.0 * l + std::f32::consts::PI * r {
        // 3. Top straight segment (right to left)
        let straight_s = s - (l + std::f32::consts::PI * r);
        pos.x = l / 2.0 - straight_s;
        pos.y = r;
        tangent = Vec2::new(-1.0, 0.0);
    } else {
        // 4. Left semicircle (top to bottom, clockwise)
        let arc_s = s - (2.0 * l + std::f32::consts::PI * r);
        let theta = std::f32::consts::FRAC_PI_2 + (arc_s / r);
        pos.x = -l / 2.0 + r * theta.cos();
        pos.y = r * theta.sin();
        tangent = Vec2::new(-theta.sin(), theta.cos());
    }

    if is_portrait {
        // Transpose coordinates and tangent components to rotate 90 degrees and maintain bottom-left starting point
        pos = Vec2::new(pos.y, pos.x);
        tangent = Vec2::new(tangent.y, tangent.x);
    }

    // Perpendicular outwards rotation
    let tangent_angle = tangent.y.atan2(tangent.x);
    let rotation_angle = tangent_angle - std::f32::consts::FRAC_PI_2;

    CellLayout {
        position: pos,
        rotation_angle,
    }
}
