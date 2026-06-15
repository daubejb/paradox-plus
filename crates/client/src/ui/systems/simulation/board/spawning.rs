use bevy::prelude::*;
use bevy::sprite::{ColorMesh2dBundle, Mesh2dHandle, ColorMaterial};
use protocol::terrain::presets::get_course_preset;
use crate::ui::components::{BoardContainerNode, CurrentHole, GameSettings, BoardCellNode, RollStatusTextNode};
use crate::ui::systems::simulation::board::geometry::calculate_capsule_layout;

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
    token_assets: Res<super::token::PlayerTokenAssets>,
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
        let layout_capacity = 28;

        commands.entity(root_entity).with_children(|board| {
            // Generate and spawn unified outer border ribbon mesh
            let outer_mesh = super::borders::generate_border_ribbon_mesh(layout_capacity, size, 8, 48.0, 2.0);
            let outer_handle = meshes.add(outer_mesh);
            let outer_material = materials.add(ColorMaterial::from(Color::srgba(1.0, 1.0, 1.0, 0.25)));

            board.spawn((
                ColorMesh2dBundle {
                    mesh: Mesh2dHandle(outer_handle.clone()),
                    material: outer_material.clone(),
                    transform: Transform::from_translation(Vec3::ZERO),
                    ..default()
                },
                TrackTileVisuals {
                    mesh_handle: outer_handle,
                    material_handle: outer_material,
                },
            ));

            // Generate and spawn unified inner border ribbon mesh
            let inner_mesh = super::borders::generate_border_ribbon_mesh(layout_capacity, size, 8, -48.0, 2.0);
            let inner_handle = meshes.add(inner_mesh);
            let inner_material = materials.add(ColorMaterial::from(Color::srgba(1.0, 1.0, 1.0, 0.25)));

            board.spawn((
                ColorMesh2dBundle {
                    mesh: Mesh2dHandle(inner_handle.clone()),
                    material: inner_material.clone(),
                    transform: Transform::from_translation(Vec3::ZERO),
                    ..default()
                },
                TrackTileVisuals {
                    mesh_handle: inner_handle,
                    material_handle: inner_material,
                },
            ));

            // Spawn status text bottom-aligned inside the track interior
            let geom = super::geometry::TrackGeometry::calculate(size);
            let bottom_y = -geom.l_v / 2.0 - geom.r + 48.0;
            let text_y = bottom_y + 24.0;

            board.spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Press roll to start...".to_string(),
                            style: TextStyle {
                                font: Handle::default(),
                                font_size: 13.0,
                                color: Color::srgb(0.9, 0.8, 0.3),
                            },
                        }],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    text_2d_bounds: bevy::text::Text2dBounds {
                        size: Vec2::new(geom.l_h + 2.0 * geom.r - 120.0, 50.0),
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, text_y, 2.0)),
                    ..default()
                },
                RollStatusTextNode,
            ));

            for idx in 0..layout_capacity {
                let layout = calculate_capsule_layout(idx as f32, layout_capacity, size);

                // Create subdivided tile mesh
                let mesh = super::borders::generate_subdivided_tile_mesh(idx, layout_capacity, size, 8);
                let mesh_handle = meshes.add(mesh);

                let (color, text_color) = if idx < total_cells {
                    super::style::get_terrain_style(&preset.cells[idx])
                } else {
                    (Color::srgba(0.16, 0.26, 0.20, 0.4), Color::WHITE)
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

                // Spawn radial divider line at start boundary
                super::borders::spawn_radial_divider(board, idx, layout_capacity, size);

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
                                    font_size: 12.0,
                                    color: text_color,
                                    ..default()
                                },
                            ),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                                .with_rotation(Quat::from_rotation_z(-layout.rotation_angle)),
                            ..default()
                        });

                        // Player Token Marker (modelled as a poker chip with counter-rotation)
                        super::token::spawn_player_token(
                            cell_sprite,
                            &token_assets,
                            &settings.nickname,
                            layout.rotation_angle,
                        );

                        // Wager Token Marker (procedural custom wagers)
                        super::token::spawn_wager_token(
                            cell_sprite,
                            &token_assets,
                            layout.rotation_angle,
                        );
                    });
                }
            }
        });
    }
}
