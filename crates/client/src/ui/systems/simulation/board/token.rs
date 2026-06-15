use bevy::prelude::*;
use bevy::sprite::{ColorMesh2dBundle, Mesh2dHandle};
use std::f32::consts::TAU;
use crate::ui::components::{PlayerTokenMarker, WagerTokenMarker};
use protocol::messages::CardType;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WagerVisual {
    pub card_type: CardType,
}

#[derive(Resource)]
pub struct PlayerTokenAssets {
    pub outer_circle_mesh: Handle<Mesh>,
    pub inner_circle_mesh: Handle<Mesh>,
    pub dash_mesh: Handle<Mesh>,
    pub gold_material: Handle<ColorMaterial>,
    pub white_material: Handle<ColorMaterial>,
    pub charcoal_material: Handle<ColorMaterial>,

    // Wager-specific meshes and materials
    pub wager_shield_mesh: Handle<Mesh>,
    pub wager_banana_mesh: Handle<Mesh>,
    pub wager_die_mesh: Handle<Mesh>,
    pub wager_pip_mesh: Handle<Mesh>,
    pub wager_shield_material: Handle<ColorMaterial>,
    pub wager_banana_material: Handle<ColorMaterial>,
}

impl FromWorld for PlayerTokenAssets {
    fn from_world(world: &mut World) -> Self {
        let (outer, inner, dash, shield_mesh, banana_mesh, die_mesh, pip_mesh) =
            if let Some(mut meshes) = world.get_resource_mut::<Assets<Mesh>>() {
                (
                    meshes.add(Circle::new(12.0)),
                    meshes.add(Circle::new(8.0)),
                    meshes.add(Rectangle::new(3.0, 1.0)),
                    meshes.add(super::wager_meshes::generate_shield_mesh()),
                    meshes.add(super::wager_meshes::generate_banana_mesh()),
                    meshes.add(Rectangle::new(16.0, 16.0)),
                    meshes.add(Circle::new(1.5)),
                )
            } else {
                (
                    Handle::default(), Handle::default(), Handle::default(),
                    Handle::default(), Handle::default(), Handle::default(), Handle::default()
                )
            };

        let (gold, white, charcoal, blue, yellow) =
            if let Some(mut materials) = world.get_resource_mut::<Assets<ColorMaterial>>() {
                (
                    materials.add(ColorMaterial::from(Color::srgb(0.85, 0.65, 0.13))),
                    materials.add(ColorMaterial::from(Color::WHITE)),
                    materials.add(ColorMaterial::from(Color::srgb(0.1, 0.1, 0.1))),
                    materials.add(ColorMaterial::from(Color::srgb(0.2, 0.4, 0.8))),
                    materials.add(ColorMaterial::from(Color::srgb(0.9, 0.8, 0.1))),
                )
            } else {
                (
                    Handle::default(), Handle::default(), Handle::default(),
                    Handle::default(), Handle::default()
                )
            };

        Self {
            outer_circle_mesh: outer,
            inner_circle_mesh: inner,
            dash_mesh: dash,
            gold_material: gold,
            white_material: white,
            charcoal_material: charcoal,

            wager_shield_mesh: shield_mesh,
            wager_banana_mesh: banana_mesh,
            wager_die_mesh: die_mesh,
            wager_pip_mesh: pip_mesh,
            wager_shield_material: blue,
            wager_banana_material: yellow,
        }
    }
}

pub fn spawn_player_token(
    builder: &mut ChildBuilder,
    assets: &PlayerTokenAssets,
    nickname: &str,
    layout_rotation: f32,
) {
    let first_initial = nickname
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());

    // Spawn Root container with PlayerTokenMarker component
    builder.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(20.0, -10.0, 2.0)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayerTokenMarker,
    )).with_children(|parent| {
        // 1. Spawn Outer Gold Circle (Base)
        parent.spawn(ColorMesh2dBundle {
            mesh: Mesh2dHandle(assets.outer_circle_mesh.clone()),
            material: assets.gold_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });

        // 2. Spawn 6 Radial Dashes around the rim (radius 10.0)
        for i in 0..6 {
            let angle = (i as f32) * (TAU / 6.0);
            let dash_transform = Transform::from_xyz(
                10.0 * angle.cos(),
                10.0 * angle.sin(),
                0.1,
            ).with_rotation(Quat::from_rotation_z(angle));

            parent.spawn(ColorMesh2dBundle {
                mesh: Mesh2dHandle(assets.dash_mesh.clone()),
                material: assets.white_material.clone(),
                transform: dash_transform,
                ..default()
            });
        }

        // 3. Spawn Inner Charcoal Circle Inlay
        parent.spawn(ColorMesh2dBundle {
            mesh: Mesh2dHandle(assets.inner_circle_mesh.clone()),
            material: assets.charcoal_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.2),
            ..default()
        }).with_children(|inner_parent| {
            // 4. Centered Uppercase First Initial (Counter-rotated relative to layout_rotation)
            inner_parent.spawn(Text2dBundle {
                text: Text::from_section(
                    first_initial,
                    TextStyle {
                        font: Handle::default(),
                        font_size: 10.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 0.3)
                    .with_rotation(Quat::from_rotation_z(-layout_rotation)),
                ..default()
            });
        });
    });
}

pub fn spawn_wager_token(
    builder: &mut ChildBuilder,
    assets: &PlayerTokenAssets,
    layout_rotation: f32,
) {
    // Spawn Root container with WagerTokenMarker component
    // Positioned at Vec3::new(-20.0, 10.0, 2.0)
    // Counter-rotated relative to layout_rotation so everything inside it stays upright!
    builder.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(-20.0, 10.0, 2.0))
                .with_rotation(Quat::from_rotation_z(-layout_rotation)),
            visibility: Visibility::Hidden,
            ..default()
        },
        WagerTokenMarker,
    )).with_children(|parent| {
        // 1. Guardian Shield Child
        parent.spawn((
            SpatialBundle::default(),
            WagerVisual { card_type: CardType::Shield },
        )).with_children(|shield| {
            // Blue Crest Shield (custom mesh)
            shield.spawn(ColorMesh2dBundle {
                mesh: Mesh2dHandle(assets.wager_shield_mesh.clone()),
                material: assets.wager_shield_material.clone(),
                ..default()
            });
            // White "S" text in center
            shield.spawn(Text2dBundle {
                text: Text::from_section(
                    "S",
                    TextStyle {
                        font: Handle::default(),
                        font_size: 9.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 0.1),
                ..default()
            });
        });

        // 2. Trickster Banana Child
        parent.spawn((
            SpatialBundle::default(),
            WagerVisual { card_type: CardType::Banana },
        )).with_children(|banana| {
            // Yellow Single Banana (custom mesh)
            banana.spawn(ColorMesh2dBundle {
                mesh: Mesh2dHandle(assets.wager_banana_mesh.clone()),
                material: assets.wager_banana_material.clone(),
                ..default()
            });
            // Charcoal/Black "B" text in center
            banana.spawn(Text2dBundle {
                text: Text::from_section(
                    "B",
                    TextStyle {
                        font: Handle::default(),
                        font_size: 9.0,
                        color: Color::srgb(0.1, 0.1, 0.1),
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 0.1),
                ..default()
            });
        });

        // 3. Golden Die Child
        parent.spawn((
            SpatialBundle::default(),
            WagerVisual { card_type: CardType::GoldenDie },
        )).with_children(|die| {
            // Gold Square
            die.spawn(ColorMesh2dBundle {
                mesh: Mesh2dHandle(assets.wager_die_mesh.clone()),
                material: assets.gold_material.clone(),
                ..default()
            });

            // 5 Pips (Charcoal dots)
            let pip_offset = 4.2;
            let pips = [
                Vec2::new(0.0, 0.0),
                Vec2::new(-pip_offset, pip_offset),
                Vec2::new(pip_offset, pip_offset),
                Vec2::new(-pip_offset, -pip_offset),
                Vec2::new(pip_offset, -pip_offset),
            ];

            for pip_pos in pips {
                die.spawn(ColorMesh2dBundle {
                    mesh: Mesh2dHandle(assets.wager_pip_mesh.clone()),
                    material: assets.charcoal_material.clone(),
                    transform: Transform::from_translation(pip_pos.extend(0.1)),
                    ..default()
                });
            }
        });
    });
}
