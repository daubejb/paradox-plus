use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use super::geometry::{calculate_capsule_layout, calculate_line_segment_transform_and_size};

/// Generates a subdivided tile mesh for a cell that curves along the track geometry.
pub fn generate_subdivided_tile_mesh(
    idx: usize,
    total_cells: usize,
    viewport_size: Vec2,
    subdivisions: usize,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());

    let tile_thickness = 96.0;
    let d = tile_thickness / 2.0;

    let mut positions = Vec::with_capacity(2 * (subdivisions + 1));
    let mut normals = Vec::with_capacity(2 * (subdivisions + 1));
    let mut uvs = Vec::with_capacity(2 * (subdivisions + 1));

    // Outer boundary points
    for i in 0..=subdivisions {
        let t = idx as f32 - 0.5 + (i as f32 / subdivisions as f32);
        let layout = calculate_capsule_layout(t, total_cells, viewport_size);
        let perp = Vec2::new(layout.rotation_angle.cos(), layout.rotation_angle.sin());
        let p_out = layout.position + perp * d;
        positions.push([p_out.x, p_out.y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([i as f32 / subdivisions as f32, 0.0]);
    }

    // Inner boundary points
    for i in 0..=subdivisions {
        let t = idx as f32 - 0.5 + (i as f32 / subdivisions as f32);
        let layout = calculate_capsule_layout(t, total_cells, viewport_size);
        let perp = Vec2::new(layout.rotation_angle.cos(), layout.rotation_angle.sin());
        let p_in = layout.position - perp * d;
        positions.push([p_in.x, p_in.y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([i as f32 / subdivisions as f32, 1.0]);
    }

    let mut indices = Vec::with_capacity(6 * subdivisions);
    for i in 0..subdivisions {
        let out_start = i as u32;
        let out_end = (i + 1) as u32;
        let in_end = (subdivisions + 1 + i + 1) as u32;
        let in_start = (subdivisions + 1 + i) as u32;

        // Winding CCW:
        // Triangle 1: in_start -> in_end -> out_start
        indices.push(in_start);
        indices.push(in_end);
        indices.push(out_start);

        // Triangle 2: out_start -> in_end -> out_end
        indices.push(out_start);
        indices.push(in_end);
        indices.push(out_end);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Generates a single continuous closed ribbon mesh with correct CCW winding.
pub fn generate_border_ribbon_mesh(
    layout_capacity: usize,
    viewport_size: Vec2,
    subdivisions: usize,
    offset_distance: f32,
    thickness: f32,
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());

    let half_w = thickness * 0.5;
    let m = layout_capacity * subdivisions;

    let mut positions = Vec::with_capacity(2 * (m + 1));
    let mut normals = Vec::with_capacity(2 * (m + 1));
    let mut uvs = Vec::with_capacity(2 * (m + 1));

    for j in 0..=m {
        let t = j as f32 / subdivisions as f32 - 0.5;
        let layout = calculate_capsule_layout(t, layout_capacity, viewport_size);
        let perp = Vec2::new(layout.rotation_angle.cos(), layout.rotation_angle.sin());
        
        let center = layout.position + perp * offset_distance;
        let v_in = center - perp * half_w;
        let v_out = center + perp * half_w;

        // Use Z = 0.5 to prevent Z-fighting with cell background tiles
        positions.push([v_in.x, v_in.y, 0.5]);
        positions.push([v_out.x, v_out.y, 0.5]);

        normals.push([0.0, 0.0, 1.0]);
        normals.push([0.0, 0.0, 1.0]);

        let u = j as f32 / m as f32;
        uvs.push([u, 0.0]);
        uvs.push([u, 1.0]);
    }

    let mut indices = Vec::with_capacity(6 * m);
    for j in 0..m {
        let c_inner = (2 * j) as u32;
        let c_outer = (2 * j + 1) as u32;
        let n_inner = (2 * (j + 1)) as u32;
        let n_outer = (2 * (j + 1) + 1) as u32;

        // Triangle 1 (CCW)
        indices.push(c_inner);
        indices.push(n_inner);
        indices.push(c_outer);

        // Triangle 2 (CCW)
        indices.push(c_outer);
        indices.push(n_inner);
        indices.push(n_outer);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Spawns the starting radial divider line sprite for a cell.
pub fn spawn_radial_divider(
    builder: &mut ChildBuilder,
    idx: usize,
    total_cells: usize,
    viewport_size: Vec2,
) {
    let tile_thickness = 96.0;
    let d = tile_thickness / 2.0;

    let layout_start = calculate_capsule_layout(idx as f32 - 0.5, total_cells, viewport_size);
    let perp_start = Vec2::new(layout_start.rotation_angle.cos(), layout_start.rotation_angle.sin());

    let c_in_start = layout_start.position - perp_start * d;
    let c_out_start = layout_start.position + perp_start * d;

    let (div_transform, div_size) = calculate_line_segment_transform_and_size(
        c_in_start,
        c_out_start,
        2.0,
        0.6,
    );

    builder.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.25),
            custom_size: Some(div_size),
            ..default()
        },
        transform: div_transform,
        ..default()
    });
}
