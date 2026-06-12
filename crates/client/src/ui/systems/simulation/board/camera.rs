use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::PrimaryWindow;
use crate::ui::components::{BoardContainerNode, ClientScreenState};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardCameraNode;

/// Spawns the 2D gameboard camera if it does not already exist.
pub fn setup_board_camera_system(
    mut commands: Commands,
    query: Query<Entity, With<BoardCameraNode>>,
) {
    if query.is_empty() {
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    // Render underneath the main UI (which defaults to order 1)
                    order: 0,
                    ..default()
                },
                ..default()
            },
            BoardCameraNode,
        ));
    }
}

/// Synchronizes the 2D camera's viewport to align precisely with the Bevy UI layout spacer.
pub fn sync_board_camera_viewport_system(
    screen_state: Res<State<ClientScreenState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&mut Camera, &mut OrthographicProjection), With<BoardCameraNode>>,
    container_query: Query<(&Node, &GlobalTransform), With<BoardContainerNode>>,
) {
    let Ok((mut camera, mut projection)) = camera_query.get_single_mut() else {
        return;
    };

    // Deactivate 2D camera if we are not actively in the gameplay screen
    if *screen_state.get() != ClientScreenState::Gameplay {
        camera.is_active = false;
        return;
    }

    let Ok(window) = window_query.get_single() else {
        camera.is_active = false;
        return;
    };

    let Ok((node, transform)) = container_query.get_single() else {
        camera.is_active = false;
        return;
    };

    let raw_size = node.size();
    let size = if raw_size.x <= 0.0 || raw_size.y <= 0.0 {
        Vec2::new(400.0, 400.0)
    } else {
        raw_size
    };

    camera.is_active = true;

    // Convert logical UI spacer bounds to physical coordinates for camera viewport bounds
    let scale_factor = window.scale_factor();
    let half_size = size / 2.0;
    let translation = transform.translation();

    let left = (translation.x - half_size.x) * scale_factor;
    let top = (translation.y - half_size.y) * scale_factor;
    let width = size.x * scale_factor;
    let height = size.y * scale_factor;

    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(left.max(0.0) as u32, top.max(0.0) as u32),
        physical_size: UVec2::new(width.max(1.0) as u32, height.max(1.0) as u32),
        depth: 0.0..1.0,
    });

    // Enforce 1:1 aspect ratio locks inside orthographic projections to prevent squishing
    projection.scaling_mode = bevy::render::camera::ScalingMode::Fixed {
        width: 500.0,
        height: 500.0,
    };
}
