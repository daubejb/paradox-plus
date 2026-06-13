use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

/// Generates a flat 2D crest shield polygon (7 vertices, centered at origin).
pub fn generate_shield_mesh() -> Mesh {
    // 7 vertices defining the crest shield geometry
    let positions = vec![
        [0.0, 8.0, 0.0],   // 0: Top-center dip
        [-6.0, 10.0, 0.0], // 1: Top-left peak
        [-6.0, 0.0, 0.0],  // 2: Mid-left edge
        [0.0, -10.0, 0.0], // 3: Bottom point
        [6.0, 0.0, 0.0],   // 4: Mid-right edge
        [6.0, 10.0, 0.0],  // 5: Top-right peak
        [0.0, 0.0, 0.0],   // 6: Center anchor for triangulation
    ];

    // Flat shading normals pointing straight out of the screen (+Z)
    let normals = vec![[0.0, 0.0, 1.0]; 7];

    // Simple normalized UV coordinates mapped from the bounding box
    let uvs = vec![
        [0.5, 0.9],
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

/// Generates a flat 2D 3-lobed splayed banana peel polygon (7 vertices).
pub fn generate_banana_peel_mesh() -> Mesh {
    let positions = vec![
        [0.0, 0.0, 0.0],    // 0: Stem center
        [-2.0, 8.0, 0.0],   // 1: Left lobe tip
        [-7.0, 2.0, 0.0],   // 2: Left lobe base
        [0.0, -9.0, 0.0],   // 3: Middle lobe tip
        [7.0, 2.0, 0.0],    // 4: Right lobe base
        [2.0, 8.0, 0.0],    // 5: Right lobe tip
        [0.0, 3.0, 0.0],    // 6: Central splay junction
    ];

    let normals = vec![[0.0, 0.0, 1.0]; 7];

    let uvs = vec![
        [0.5, 0.5],
        [0.35, 0.9],
        [0.05, 0.6],
        [0.5, 0.05],
        [0.95, 0.6],
        [0.65, 0.9],
        [0.5, 0.65],
    ];

    // CCW winding order to prevent backface culling
    let indices = Indices::U32(vec![
        0, 6, 1, // Left lobe partition A
        1, 2, 6, // Left lobe partition B
        2, 3, 6, // Center lobe partition A
        3, 4, 6, // Center lobe partition B
        4, 5, 6, // Right lobe partition A
        5, 6, 0, // Right lobe partition B
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
