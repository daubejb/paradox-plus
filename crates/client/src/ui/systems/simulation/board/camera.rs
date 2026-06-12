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
                    // Render on top of the main UI camera (which remains at default order 0)
                    // physically constrained to the central spacer viewport node
                    order: 1,
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

    let physical_width = window.physical_width();
    let physical_height = window.physical_height();

    // Deactivate if window minimized or physical bounds are invalid
    if physical_width == 0 || physical_height == 0 {
        camera.is_active = false;
        return;
    }

    let scale_factor = window.scale_factor() as f32;
    let half_size = size / 2.0;
    let translation = transform.translation();

    // Map UI layout space (origin center of screen, Y-up) to physical Viewport space (origin top-left, Y-down)
    let left_window = (window.width() / 2.0) + (translation.x - half_size.x);
    let top_window = (window.height() / 2.0) - (translation.y + half_size.y);

    // Safeguard physical positions to be strictly within window dimensions

    let left_physical = (left_window * scale_factor).max(0.0).min(physical_width.saturating_sub(1) as f32) as u32;
    let top_physical = (top_window * scale_factor).max(0.0).min(physical_height.saturating_sub(1) as f32) as u32;

    let max_width = physical_width.saturating_sub(left_physical);
    let max_height = physical_height.saturating_sub(top_physical);

    // Enforce viewport size >= 1 to prevent validation layer panics
    let width_physical = (size.x * scale_factor).max(1.0).min(max_width as f32) as u32;
    let height_physical = (size.y * scale_factor).max(1.0).min(max_height as f32) as u32;

    if width_physical == 0 || height_physical == 0 {
        camera.is_active = false;
        return;
    }

    camera.is_active = true;
    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(left_physical, top_physical),
        physical_size: UVec2::new(width_physical, height_physical),
        depth: 0.0..1.0,
    });

    // Enforce 1:1 aspect ratio locks inside orthographic projections to prevent squishing
    let is_portrait = size.y > size.x;
    if is_portrait {
        projection.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical(560.0);
    } else {
        projection.scaling_mode = bevy::render::camera::ScalingMode::FixedHorizontal(560.0);
    }
}
