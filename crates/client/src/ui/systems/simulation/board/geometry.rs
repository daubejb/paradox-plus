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
    let r = (h * 0.45).min(w * 0.30).max(40.0);
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

use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

pub fn generate_quad_tile_mesh(
    c_out_start: Vec2,
    c_out_end: Vec2,
    c_in_end: Vec2,
    c_in_start: Vec2,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());

    // Standard Bevy CCW winding order:
    // 0: Bottom-Left (Outer Start)
    // 1: Bottom-Right (Outer End)
    // 2: Top-Right (Inner End)
    // 3: Top-Left (Inner Start)
    let positions = vec![
        [c_out_start.x, c_out_start.y, 0.0],
        [c_out_end.x, c_out_end.y, 0.0],
        [c_in_end.x, c_in_end.y, 0.0],
        [c_in_start.x, c_in_start.y, 0.0],
    ];

    let normals = vec![[0.0, 0.0, 1.0]; 4];
    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

    // Triangles: 0->1->2 and 0->2->3
    let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);
    mesh
}

pub fn calculate_line_segment_transform_and_size(
    start: Vec2,
    end: Vec2,
    thickness: f32,
    z_order: f32,
) -> (Transform, Vec2) {
    let dir = end - start;
    let length = dir.length();
    let angle = dir.y.atan2(dir.x);
    let midpoint = (start + end) / 2.0;

    let transform = Transform {
        translation: midpoint.extend(z_order),
        rotation: Quat::from_rotation_z(angle),
        ..Default::default()
    };

    let size = Vec2::new(length, thickness);
    (transform, size)
}

/// Calculates the outer boundary point by casting a ray from the midline position
/// in the outward normal direction, intersecting it with a flattened (chamfered box) outer boundary.
pub fn calculate_outer_point(
    pos: Vec2,
    normal: Vec2,
    r: f32,
    l: f32,
    d: f32,
    is_portrait: bool,
) -> Vec2 {
    // Determine outer boundaries of the octagon based on layout orientation
    let x_limit = if is_portrait { r + d } else { l / 2.0 + r + d };
    let y_limit = if is_portrait { l / 2.0 + r + d } else { r + d };

    // Flat horizontal/vertical edge limits (where the corner chamfers begin).
    let (x_top, y_side) = if is_portrait {
        (0.35 * x_limit, l / 2.0)
    } else {
        (l / 2.0, 0.35 * y_limit)
    };

    let s_x = normal.x.signum();
    let s_y = normal.y.signum();

    // Corner diagonal line: s_x * dy * x + s_y * dx * y = c
    let dx = x_limit - x_top;
    let dy = y_limit - y_side;
    let c = x_limit * y_limit - x_top * y_side;

    let mut min_t = f32::MAX;

    // 1. Intersect with vertical boundary: x = s_x * x_limit
    if normal.x.abs() > 1e-5 {
        let t = (s_x * x_limit - pos.x) / normal.x;
        if t > 0.0 && t < min_t {
            min_t = t;
        }
    }

    // 2. Intersect with horizontal boundary: y = s_y * y_limit
    if normal.y.abs() > 1e-5 {
        let t = (s_y * y_limit - pos.y) / normal.y;
        if t > 0.0 && t < min_t {
            min_t = t;
        }
    }

    // 3. Intersect with corner diagonal boundary
    let denom = s_x * dy * normal.x + s_y * dx * normal.y;
    if denom.abs() > 1e-5 {
        let t = (c - s_x * dy * pos.x - s_y * dx * pos.y) / denom;
        if t > 0.0 && t < min_t {
            min_t = t;
        }
    }

    // Fallback if no intersection found (should not happen for valid rays)
    if min_t == f32::MAX {
        pos + normal * d
    } else {
        pos + normal * min_t
    }
}

