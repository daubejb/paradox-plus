use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellLayout {
    pub position: Vec2,
    pub rotation_angle: f32,
}

const TARGET_ASPECT_RATIO: f32 = 0.85;
const VIEWPORT_PADDING: f32 = 24.0;

const MIN_OUTER_WIDTH: f32 = 336.0;
const MIN_OUTER_HEIGHT: f32 = 376.0;

const MIDLINE_PADDING: f32 = 96.0;
const MIN_MIDLINE_WIDTH: f32 = 240.0;
const MIN_MIDLINE_HEIGHT: f32 = 280.0;

/// Client-side visual corner radius coefficient.
/// Strictly used for rendering and visual interpolation in `FixedToFloatPlugin`.
/// All authoritative simulation coordinates are calculated using fixed-point math on the server.
const RADIUS_COEFFICIENT: f32 = 0.28;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrackGeometry {
    pub r: f32,
    pub l_h: f32,
    pub l_v: f32,
    pub outer_width: f32,
    pub outer_height: f32,
}

impl TrackGeometry {
    pub fn calculate(viewport_size: Vec2) -> Self {
        // Clamp minimum viewport size to prevent zero or negative dimensions
        let width = viewport_size.x.max(MIN_OUTER_WIDTH + VIEWPORT_PADDING);
        let height = viewport_size.y.max(MIN_OUTER_HEIGHT + VIEWPORT_PADDING);
        
        let available_width = width - VIEWPORT_PADDING;
        let available_height = height - VIEWPORT_PADDING;
        
        let (outer_width, outer_height) = if available_width / available_height > TARGET_ASPECT_RATIO {
            // Viewport is wider than target aspect ratio (bounded by height)
            (available_height * TARGET_ASPECT_RATIO, available_height)
        } else {
            // Viewport is taller than target aspect ratio (bounded by width)
            (available_width, available_width / TARGET_ASPECT_RATIO)
        };

        let midline_width = (outer_width - MIDLINE_PADDING).max(MIN_MIDLINE_WIDTH);
        let midline_height = (outer_height - MIDLINE_PADDING).max(MIN_MIDLINE_HEIGHT);

        // Scale corner radius relative to the width (smaller dimension) and clamp to prevent segment underflow
        let r = (midline_width * RADIUS_COEFFICIENT)
            .min(midline_width * 0.5)
            .min(midline_height * 0.5);
        let l_h = (midline_width - 2.0 * r).max(0.0);
        let l_v = (midline_height - 2.0 * r).max(0.0);

        debug_assert!(l_v >= 0.0, "Vertical segment length must be non-negative");
        debug_assert!(l_h >= 0.0, "Horizontal segment length must be non-negative");

        Self {
            r,
            l_h,
            l_v,
            outer_width,
            outer_height,
        }
    }
}

/// Computes the 2D position and rotation of a cell along a parametric capsule track.
pub fn calculate_capsule_layout(
    idx: f32,
    total_cells: usize,
    viewport_size: Vec2,
) -> CellLayout {
    let geom = TrackGeometry::calculate(viewport_size);
    let (r, l_h, l_v) = (geom.r, geom.l_h, geom.l_v);

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

    let tangent_angle = tangent.y.atan2(tangent.x);
    let rotation_angle = tangent_angle + std::f32::consts::FRAC_PI_2;

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


