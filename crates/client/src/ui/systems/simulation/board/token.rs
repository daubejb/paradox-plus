use bevy::prelude::*;
use bevy::sprite::{ColorMesh2dBundle, Mesh2dHandle};
use std::f32::consts::TAU;
use crate::ui::components::PlayerTokenMarker;

#[derive(Resource)]
pub struct PlayerTokenAssets {
    pub outer_circle_mesh: Handle<Mesh>,
    pub inner_circle_mesh: Handle<Mesh>,
    pub dash_mesh: Handle<Mesh>,
    pub gold_material: Handle<ColorMaterial>,
    pub white_material: Handle<ColorMaterial>,
    pub charcoal_material: Handle<ColorMaterial>,
}

impl FromWorld for PlayerTokenAssets {
    fn from_world(world: &mut World) -> Self {
        let (outer, inner, dash) = if let Some(mut meshes) = world.get_resource_mut::<Assets<Mesh>>() {
            (
                meshes.add(Circle::new(12.0)),
                meshes.add(Circle::new(8.0)),
                meshes.add(Rectangle::new(3.0, 1.0)),
            )
        } else {
            (Handle::default(), Handle::default(), Handle::default())
        };

        let (gold, white, charcoal) = if let Some(mut materials) = world.get_resource_mut::<Assets<ColorMaterial>>() {
            (
                materials.add(ColorMaterial::from(Color::srgb(0.85, 0.65, 0.13))),
                materials.add(ColorMaterial::from(Color::WHITE)),
                materials.add(ColorMaterial::from(Color::srgb(0.1, 0.1, 0.1))),
            )
        } else {
            (Handle::default(), Handle::default(), Handle::default())
        };

        Self {
            outer_circle_mesh: outer,
            inner_circle_mesh: inner,
            dash_mesh: dash,
            gold_material: gold,
            white_material: white,
            charcoal_material: charcoal,
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
