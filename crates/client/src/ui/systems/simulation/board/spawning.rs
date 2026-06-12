use bevy::prelude::*;
use bevy::sprite::{ColorMesh2dBundle, Mesh2dHandle, ColorMaterial};
use protocol::terrain::presets::get_course_preset;
use crate::ui::components::{BoardContainerNode, CurrentHole, GameSettings, BoardCellNode, PlayerTokenMarker, WagerTokenMarker};
use crate::ui::systems::simulation::board::geometry::{
    calculate_capsule_layout, generate_quad_tile_mesh, calculate_line_segment_transform_and_size,
};

#[derive(Component)]
pub struct TrackTileVisuals {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<ColorMaterial>,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardWorldRoot;

/// Rebuilds the 2D Sprite gameboard when the current hole resource changes.
pub fn rebuild_board_on_hole_change_system(
    mut commands: Commands,
    current_hole: Res<CurrentHole>,
    settings: Res<GameSettings>,
    container_query: Query<&Node, With<BoardContainerNode>>,
    root_query: Query<Entity, With<BoardWorldRoot>>,
    visuals_query: Query<&TrackTileVisuals>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut last_size: Local<Option<Vec2>>,
) {
    let Ok(node) = container_query.get_single() else {
        return;
    };

    let raw_size = node.size();

    // Check if the container size has resolved to a non-zero value and changed
    let size_changed = match *last_size {
        None => true, // Rebuild on first run
        Some(old_size) => (raw_size - old_size).length_squared() > 1e-4,
    };

    let hole_changed = current_hole.is_changed();

    // If neither size nor hole changed, skip rebuilding
    if !size_changed && !hole_changed {
        return;
    }

    *last_size = Some(raw_size);

    // Use fallback size if size has not resolved (e.g. in headless tests)
    let size = if raw_size.x <= 0.0 || raw_size.y <= 0.0 {
        Vec2::new(400.0, 400.0)
    } else {
        raw_size
    };

    // Clean up old board asset allocations
    for visuals in visuals_query.iter() {
        meshes.remove(&visuals.mesh_handle);
        materials.remove(&visuals.material_handle);
    }

    // Clean up old board root and its children recursively to prevent memory leaks
    for root_entity in root_query.iter() {
        commands.entity(root_entity).despawn_recursive();
    }


    // Spawn new BoardWorldRoot
    let root_entity = commands.spawn((
        SpatialBundle::default(),
        BoardWorldRoot,
    )).id();

    // Spawn cell tiles along capsule track trajectory
    if let Some(preset) = get_course_preset(&settings.course, current_hole.0) {
        let total_cells = preset.cells.len();
        let layout_capacity = total_cells.max(27);

        let tile_thickness = 72.0;

        commands.entity(root_entity).with_children(|board| {
            for idx in 0..layout_capacity {
                let layout = calculate_capsule_layout(idx as f32, layout_capacity, size);

                // Calculate boundary coordinates
                let layout_start = calculate_capsule_layout(idx as f32 - 0.5, layout_capacity, size);
                let layout_end = calculate_capsule_layout(idx as f32 + 0.5, layout_capacity, size);

                let perp_start = Vec2::new(layout_start.rotation_angle.cos(), layout_start.rotation_angle.sin());
                let perp_end = Vec2::new(layout_end.rotation_angle.cos(), layout_end.rotation_angle.sin());

                let c_in_start = layout_start.position - perp_start * (tile_thickness / 2.0);
                let c_out_start = layout_start.position + perp_start * (tile_thickness / 2.0);
                let c_out_end = layout_end.position + perp_end * (tile_thickness / 2.0);
                let c_in_end = layout_end.position - perp_end * (tile_thickness / 2.0);

                // Create mesh using standard Bevy CCW winding
                let mesh = generate_quad_tile_mesh(c_out_start, c_out_end, c_in_end, c_in_start);
                let mesh_handle = meshes.add(mesh);

                let color = if idx < total_cells {
                    let cell_type = preset.cells[idx];
                    match cell_type {
                        protocol::terrain::TerrainType::TeeBox => Color::srgb(0.2, 0.6, 0.3),
                        protocol::terrain::TerrainType::Fairway => Color::srgb(0.3, 0.7, 0.4),
                        protocol::terrain::TerrainType::Rough => Color::srgb(0.25, 0.5, 0.3),
                        protocol::terrain::TerrainType::Bunker => Color::srgb(0.8, 0.7, 0.5),
                        protocol::terrain::TerrainType::Water => Color::srgb(0.1, 0.4, 0.7),
                        protocol::terrain::TerrainType::OutOfBounds => Color::srgb(0.9, 0.2, 0.2),
                        protocol::terrain::TerrainType::Green(_) => Color::srgb(0.1, 0.5, 0.2),
                    }
                } else {
                    Color::srgba(0.16, 0.26, 0.20, 0.4)
                };

                let material_handle = materials.add(ColorMaterial::from(color));

                let z = if idx < total_cells { 0.0 } else { -0.1 };

                // Spawn mesh background
                board.spawn((
                    ColorMesh2dBundle {
                        mesh: Mesh2dHandle(mesh_handle.clone()),
                        material: material_handle.clone(),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, z)),
                        ..default()
                    },
                    TrackTileVisuals {
                        mesh_handle,
                        material_handle,
                    },
                ));

                // Spawn outer border segment
                let (outer_transform, outer_size) = calculate_line_segment_transform_and_size(
                    c_out_start,
                    c_out_end,
                    2.0,
                    0.5,
                );
                board.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(1.0, 1.0, 1.0, 0.25),
                        custom_size: Some(outer_size),
                        ..default()
                    },
                    transform: outer_transform,
                    ..default()
                });

                // Spawn inner border segment
                let (inner_transform, inner_size) = calculate_line_segment_transform_and_size(
                    c_in_start,
                    c_in_end,
                    2.0,
                    0.5,
                );
                board.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(1.0, 1.0, 1.0, 0.25),
                        custom_size: Some(inner_size),
                        ..default()
                    },
                    transform: inner_transform,
                    ..default()
                });

                // Spawn radial divider line at start boundary
                let (div_transform, div_size) = calculate_line_segment_transform_and_size(
                    c_in_start,
                    c_out_start,
                    2.0,
                    0.6,
                );
                board.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(1.0, 1.0, 1.0, 0.25),
                        custom_size: Some(div_size),
                        ..default()
                    },
                    transform: div_transform,
                    ..default()
                });

                // Spawn logical gameplay markers (text & tokens)
                if idx < total_cells {
                    let cell_type = preset.cells[idx];
                    let name = match cell_type {
                        protocol::terrain::TerrainType::TeeBox => "TEE".to_string(),
                        protocol::terrain::TerrainType::Fairway => format!("{} FW", idx),
                        protocol::terrain::TerrainType::Rough => format!("{} RGH", idx),
                        protocol::terrain::TerrainType::Bunker => format!("{} BNK", idx),
                        protocol::terrain::TerrainType::Water => format!("{} WTR", idx),
                        protocol::terrain::TerrainType::OutOfBounds => format!("{} OB", idx),
                        protocol::terrain::TerrainType::Green(putts) => format!("G{}", putts),
                    };

                    board.spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(layout.position.extend(0.0))
                                .with_rotation(Quat::from_rotation_z(layout.rotation_angle)),
                            ..default()
                        },
                        BoardCellNode { index: idx as u32 },
                    )).with_children(|cell_sprite| {
                        cell_sprite.spawn(Text2dBundle {
                            text: Text::from_section(
                                name,
                                TextStyle {
                                    font_size: 9.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                                .with_rotation(Quat::from_rotation_z(-layout.rotation_angle)),
                            ..default()
                        });

                        // Player Token Marker
                        cell_sprite.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::srgb(0.95, 0.85, 0.1),
                                    custom_size: Some(Vec2::splat(8.0)),
                                    ..default()
                                },
                                transform: Transform::from_translation(Vec3::new(20.0, -10.0, 2.0)),
                                visibility: Visibility::Hidden,
                                ..default()
                            },
                            PlayerTokenMarker,
                        ));

                        // Wager Token Marker
                        cell_sprite.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::NONE,
                                    custom_size: Some(Vec2::splat(12.0)),
                                    ..default()
                                },
                                transform: Transform::from_translation(Vec3::new(-20.0, 10.0, 2.0)),
                                visibility: Visibility::Hidden,
                                ..default()
                            },
                            WagerTokenMarker,
                        )).with_children(|wager_indicator| {
                            wager_indicator.spawn(Text2dBundle {
                                text: Text::from_section(
                                    "",
                                    TextStyle {
                                        font_size: 8.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                                ..default()
                            });
                        });
                    });
                }
            }
        });
    }
}
