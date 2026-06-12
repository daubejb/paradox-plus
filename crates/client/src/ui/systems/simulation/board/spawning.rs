use bevy::prelude::*;
use protocol::terrain::presets::get_course_preset;
use crate::ui::components::{BoardContainerNode, CurrentHole, GameSettings, BoardCellNode, PlayerTokenMarker, WagerTokenMarker};
use crate::ui::systems::simulation::board::geometry::calculate_capsule_layout;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardWorldRoot;

/// Rebuilds the 2D Sprite gameboard when the current hole resource changes.
pub fn rebuild_board_on_hole_change_system(
    mut commands: Commands,
    current_hole: Res<CurrentHole>,
    settings: Res<GameSettings>,
    container_query: Query<&Node, With<BoardContainerNode>>,
    root_query: Query<Entity, With<BoardWorldRoot>>,
) {
    // Clean up old board root and its children recursively to prevent memory leaks
    for root_entity in root_query.iter() {
        commands.entity(root_entity).despawn_recursive();
    }

    let Ok(node) = container_query.get_single() else {
        return;
    };

    let raw_size = node.size();
    let size = if raw_size.x <= 0.0 || raw_size.y <= 0.0 {
        Vec2::new(400.0, 400.0)
    } else {
        raw_size
    };

    // Spawn new BoardWorldRoot
    let root_entity = commands.spawn((
        SpatialBundle::default(),
        BoardWorldRoot,
    )).id();

    // Spawn cell tiles along capsule track trajectory
    if let Some(preset) = get_course_preset(&settings.course, current_hole.0) {
        let total_cells = preset.cells.len();

        let is_portrait = size.y > size.x;
        let (w, h) = if is_portrait {
            (size.y, size.x)
        } else {
            (size.x, size.y)
        };
        let r = (h * 0.33).min(w * 0.20).max(40.0);
        let l = (w * 0.50).max(60.0);
        let perimeter = 2.0 * l + 2.0 * std::f32::consts::PI * r;
        let spacing = perimeter / total_cells as f32;
        let tile_length = spacing * 1.35;
        let tile_thickness = 72.0;

        commands.entity(root_entity).with_children(|board| {
            for (idx, &cell_type) in preset.cells.iter().enumerate() {
                let layout = calculate_capsule_layout(idx, total_cells, size);

                let name = match cell_type {
                    protocol::terrain::TerrainType::TeeBox => "TEE".to_string(),
                    protocol::terrain::TerrainType::Fairway => format!("{} FW", idx),
                    protocol::terrain::TerrainType::Rough => format!("{} RGH", idx),
                    protocol::terrain::TerrainType::Bunker => format!("{} BNK", idx),
                    protocol::terrain::TerrainType::Water => format!("{} WTR", idx),
                    protocol::terrain::TerrainType::OutOfBounds => format!("{} OB", idx),
                    protocol::terrain::TerrainType::Green(putts) => format!("G{}", putts),
                };

                let color = match cell_type {
                    protocol::terrain::TerrainType::TeeBox => Color::srgb(0.2, 0.6, 0.3),
                    protocol::terrain::TerrainType::Fairway => Color::srgb(0.3, 0.7, 0.4),
                    protocol::terrain::TerrainType::Rough => Color::srgb(0.25, 0.5, 0.3),
                    protocol::terrain::TerrainType::Bunker => Color::srgb(0.8, 0.7, 0.5),
                    protocol::terrain::TerrainType::Water => Color::srgb(0.1, 0.4, 0.7),
                    protocol::terrain::TerrainType::OutOfBounds => Color::srgb(0.9, 0.2, 0.2),
                    protocol::terrain::TerrainType::Green(_) => Color::srgb(0.1, 0.5, 0.2),
                };

                board.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::new(tile_thickness, tile_length)),
                            ..default()
                        },
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
                        // Position slightly in front of the cell background to prevent z-fighting, and counteract parent rotation
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                            .with_rotation(Quat::from_rotation_z(-layout.rotation_angle)),
                        ..default()
                    });

                    // 1. Player Token Marker
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

                    // 2. Wager Token Marker
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
        });
    }
}
