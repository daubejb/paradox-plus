use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

/// Generates a flat 2D crest shield polygon (7 vertices, centered at origin).
/// Resized to exactly 16.0 x 16.0 (matching the Golden Die size).
pub fn generate_shield_mesh() -> Mesh {
    let positions = vec![
        [0.0, 6.0, 0.0],   // 0: Top-center dip
        [-8.0, 8.0, 0.0],  // 1: Top-left peak
        [-8.0, 0.0, 0.0],  // 2: Mid-left edge
        [0.0, -8.0, 0.0],  // 3: Bottom point
        [8.0, 0.0, 0.0],   // 4: Mid-right edge
        [8.0, 8.0, 0.0],   // 5: Top-right peak
        [0.0, 0.0, 0.0],   // 6: Center anchor for triangulation
    ];

    // Flat shading normals pointing straight out of the screen (+Z)
    let normals = vec![[0.0, 0.0, 1.0]; 7];

    // Simple normalized UV coordinates mapped from the bounding box
    let uvs = vec![
        [0.5, 0.875],
        [0.0, 1.0],
        [0.0, 0.5],
        [0.5, 0.0],
        [1.0, 0.5],
        [1.0, 1.0],
        [0.5, 0.5],
    ];

    // Triangulate using a fan-like structure around the center vertex (6)
    // CCW winding order to prevent backface culling
    let indices = Indices::U32(vec![
        6, 0, 1, // Top-left wedge
        6, 1, 2, // Mid-left wedge
        6, 2, 3, // Bottom-left wedge
        6, 3, 4, // Bottom-right wedge
        6, 4, 5, // Mid-right wedge
        6, 5, 0, // Top-right wedge
    ]);

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);

    mesh
}

/// Generates a flat 2D single banana crescent polygon (10 vertices).
/// Resized to exactly 16.0 x 16.0 (matching the Golden Die size).
pub fn generate_banana_mesh() -> Mesh {
    let positions = vec![
        [-6.0, 8.0, 0.0],   // 0: Inner Left peak (stem)
        [-8.0, 6.0, 0.0],   // 1: Outer Left peak
        [-3.0, 0.0, 0.0],   // 2: Inner Mid-Left
        [-4.0, -4.0, 0.0],  // 3: Outer Mid-Left
        [0.0, -3.0, 0.0],   // 4: Inner Center
        [0.0, -8.0, 0.0],   // 5: Outer Center
        [3.0, 0.0, 0.0],    // 6: Inner Mid-Right
        [4.0, -4.0, 0.0],   // 7: Outer Mid-Right
        [6.0, 8.0, 0.0],    // 8: Inner Right peak (tip)
        [8.0, 6.0, 0.0],    // 9: Outer Right peak
    ];

    let normals = vec![[0.0, 0.0, 1.0]; 10];

    let uvs = vec![
        [0.125, 1.0],
        [0.0, 0.875],
        [0.3125, 0.5],
        [0.25, 0.25],
        [0.5, 0.3125],
        [0.5, 0.0],
        [0.6875, 0.5],
        [0.75, 0.25],
        [0.875, 1.0],
        [1.0, 0.875],
    ];

    // CCW winding order to prevent backface culling
    let indices = Indices::U32(vec![
        0, 1, 3,
        0, 3, 2,
        2, 3, 5,
        2, 5, 4,
        4, 5, 7,
        4, 7, 6,
        6, 7, 9,
        6, 9, 8,
    ]);

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);

    mesh
}
