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
    let is_portrait = viewport_size.y > viewport_size.x;

    let (w, h) = if is_portrait {
        (viewport_size.y, viewport_size.x)
    } else {
        (viewport_size.x, viewport_size.y)
    };

    // Define racetrack dimensions:
    // r_val: corner radius
    // l_h: horizontal straight length
    // l_v: vertical straight length
    let r_val = (h * 0.22).max(30.0);
    let l_h = (h * 0.28).max(40.0);
    let l_v = (w * 0.48 - 2.0 * r_val).max(40.0);

    let (r, l_h, l_v) = (r_val, l_h, l_v);

    let arc = std::f32::consts::FRAC_PI_2 * r;
    let perimeter = 2.0 * l_h + 2.0 * l_v + 4.0 * arc;

    let capacity = total_cells as f32;
    let wrapped_idx = (idx % capacity + capacity) % capacity;
    let fraction = wrapped_idx / capacity;
    let s = fraction * perimeter;

    let mut pos = Vec2::ZERO;
    let tangent;

    if s < l_v {
        // 1. Left straight segment (going bottom to top)
        pos.x = -l_h / 2.0 - r;
        pos.y = -l_v / 2.0 + s;
        tangent = Vec2::new(0.0, 1.0);
    } else if s < l_v + arc {
        // 2. Top-left corner (angle goes from PI to PI/2)
        let phi = (s - l_v) / r;
        let theta = std::f32::consts::PI - phi;
        pos.x = -l_h / 2.0 + r * theta.cos();
        pos.y = l_v / 2.0 + r * theta.sin();
        tangent = Vec2::new(theta.sin(), -theta.cos());
    } else if s < l_v + l_h + arc {
        // 3. Top straight segment (going left to right)
        let ds = s - (l_v + arc);
        pos.x = -l_h / 2.0 + ds;
        pos.y = l_v / 2.0 + r;
        tangent = Vec2::new(1.0, 0.0);
    } else if s < l_v + l_h + 2.0 * arc {
        // 4. Top-right corner (angle goes from PI/2 to 0)
        let phi = (s - (l_v + l_h + arc)) / r;
        let theta = std::f32::consts::FRAC_PI_2 - phi;
        pos.x = l_h / 2.0 + r * theta.cos();
        pos.y = l_v / 2.0 + r * theta.sin();
        tangent = Vec2::new(theta.sin(), -theta.cos());
    } else if s < 2.0 * l_v + l_h + 2.0 * arc {
        // 5. Right straight segment (going top to bottom)
        let ds = s - (l_v + l_h + 2.0 * arc);
        pos.x = l_h / 2.0 + r;
        pos.y = l_v / 2.0 - ds;
        tangent = Vec2::new(0.0, -1.0);
    } else if s < 2.0 * l_v + l_h + 3.0 * arc {
        // 6. Bottom-right corner (angle goes from 0 to -PI/2)
        let phi = (s - (2.0 * l_v + l_h + 2.0 * arc)) / r;
        let theta = -phi;
        pos.x = l_h / 2.0 + r * theta.cos();
        pos.y = -l_v / 2.0 + r * theta.sin();
        tangent = Vec2::new(theta.sin(), -theta.cos());
    } else if s < 2.0 * l_v + 2.0 * l_h + 3.0 * arc {
        // 7. Bottom straight segment (going right to left)
        let ds = s - (2.0 * l_v + l_h + 3.0 * arc);
        pos.x = l_h / 2.0 - ds;
        pos.y = -l_v / 2.0 - r;
        tangent = Vec2::new(-1.0, 0.0);
    } else {
        // 8. Bottom-left corner (angle goes from -PI/2 to -PI)
        let phi = (s - (2.0 * l_v + 2.0 * l_h + 3.0 * arc)) / r;
        let theta = -std::f32::consts::FRAC_PI_2 - phi;
        pos.x = -l_h / 2.0 + r * theta.cos();
        pos.y = -l_v / 2.0 + r * theta.sin();
        tangent = Vec2::new(theta.sin(), -theta.cos());
    }

    let mut final_pos = pos;
    let mut final_tangent = tangent;

    if !is_portrait {
        // Transpose if landscape to rotate 90 degrees
        final_pos = Vec2::new(pos.y, pos.x);
        final_tangent = Vec2::new(tangent.y, tangent.x);
    }

    let tangent_angle = final_tangent.y.atan2(final_tangent.x);
    let rotation_angle = if is_portrait {
        tangent_angle + std::f32::consts::FRAC_PI_2
    } else {
        tangent_angle - std::f32::consts::FRAC_PI_2
    };

    CellLayout {
        position: final_pos,
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


