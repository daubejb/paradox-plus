use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellLayout {
    pub position: Vec2,
    pub rotation_angle: f32,
}

/// Computes the 2D position and rotation of a cell along a parametric capsule track.
pub fn calculate_capsule_layout(
    idx: usize,
    total_cells: usize,
    viewport_size: Vec2,
) -> CellLayout {
    // Determine bounds and aspect ratio locks
    let w = viewport_size.x;
    let h = viewport_size.y;

    // Define racetrack dimensions: straight length l, semicircle radius r
    let r = (h * 0.33).min(w * 0.20).max(40.0);
    let l = (w * 0.50).max(60.0);

    let perimeter = 2.0 * l + 2.0 * std::f32::consts::PI * r;

    // Distribute cells evenly along the perimeter
    let fraction = idx as f32 / total_cells as f32;
    let s = fraction * perimeter;

    let mut pos = Vec2::ZERO;
    let tangent;

    if s < l {
        // 1. Top straight segment (left to right)
        pos.x = -l / 2.0 + s;
        pos.y = r;
        tangent = Vec2::new(1.0, 0.0);
    } else if s < l + std::f32::consts::PI * r {
        // 2. Right semicircle (top to bottom, clockwise)
        let arc_s = s - l;
        let theta = std::f32::consts::FRAC_PI_2 - (arc_s / r);
        pos.x = l / 2.0 + r * theta.cos();
        pos.y = r * theta.sin();
        tangent = Vec2::new(-theta.sin(), theta.cos());
    } else if s < 2.0 * l + std::f32::consts::PI * r {
        // 3. Bottom straight segment (right to left)
        let straight_s = s - (l + std::f32::consts::PI * r);
        pos.x = l / 2.0 - straight_s;
        pos.y = -r;
        tangent = Vec2::new(-1.0, 0.0);
    } else {
        // 4. Left semicircle (bottom to top, clockwise)
        let arc_s = s - (2.0 * l + std::f32::consts::PI * r);
        let theta = -std::f32::consts::FRAC_PI_2 - (arc_s / r);
        pos.x = -l / 2.0 + r * theta.cos();
        pos.y = r * theta.sin();
        tangent = Vec2::new(-theta.sin(), theta.cos());
    }

    // Perpendicular outwards rotation
    let tangent_angle = tangent.y.atan2(tangent.x);
    let rotation_angle = tangent_angle - std::f32::consts::FRAC_PI_2;

    CellLayout {
        position: pos,
        rotation_angle,
    }
}
