use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::ui::components::{BoardCellNode, SelectedWagerCard, ClientScreenState, CursorPositionOverride};
use crate::ui::systems::simulation::board::camera::BoardCameraNode;
use crate::network::events::ClientActionRequest;
use protocol::messages::ClientAction;

/// Raycasts cursor position to detect clicks on 2D Sprite cells, dispatching wager placement actions or slide choices.
pub fn handle_board_clicks_system(
    screen_state: Res<State<ClientScreenState>>,
    game_state: Res<State<crate::replication::ClientGameState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<BoardCameraNode>>,
    cell_query: Query<(&GlobalTransform, &BoardCellNode)>,
    ball_query: Query<(&crate::replication::Ball, &crate::replication::Player)>,
    active_player_id: Res<crate::replication::ActivePlayerId>,
    settings: Res<crate::ui::components::GameSettings>,
    current_hole: Res<crate::ui::components::CurrentHole>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut selected_card: ResMut<SelectedWagerCard>,
    mut events: EventWriter<ClientActionRequest>,
    cursor_pos_override: Option<Res<CursorPositionOverride>>,
) {
    if *screen_state.get() != ClientScreenState::Gameplay {
        return;
    }

    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok((camera, camera_transform)) = camera_query.get_single() else { return };

    let cursor_pos = if let Some(over) = cursor_pos_override.as_ref().and_then(|o| o.0) {
        Some(over)
    } else if let Ok(window) = window_query.get_single() {
        window.cursor_position()
    } else {
        None
    };

    let Some(cursor_position) = cursor_pos else { return };

    let world_position = if camera.logical_viewport_size().is_none() {
        Some(cursor_position)
    } else {
        let scale_factor = window_query.get_single().map(|w| w.scale_factor() as f32).unwrap_or(1.0);
        let viewport_offset = if let Some(viewport) = &camera.viewport {
            Vec2::new(viewport.physical_position.x as f32, viewport.physical_position.y as f32) / scale_factor
        } else {
            Vec2::ZERO
        };
        let viewport_position = cursor_position - viewport_offset;
        camera.viewport_to_world_2d(camera_transform, viewport_position)
    };

    let Some(world_position) = world_position else { return };

    let mut closest_cell = None;
    let mut min_dist = f32::MAX;
    for (global_transform, cell) in cell_query.iter() {
        let cell_pos = global_transform.translation().xy();
        let dist = world_position.distance(cell_pos);
        if dist < min_dist {
            min_dist = dist;
            closest_cell = Some(cell.index);
        }
    }

    let Some(cell_index) = closest_cell else { return };
    if min_dist > 60.0 {
        return;
    }

    if *game_state.get() == crate::replication::ClientGameState::BananaChoice {
        let active_id = active_player_id.0;
        let active_ball = ball_query.iter().find(|(_, p)| p.player_id == active_id).map(|(b, _)| b);
        
        if let Some(ball) = active_ball {
            if let Some(preset) = protocol::terrain::presets::get_course_preset(&settings.course, current_hole.0) {
                let mut clicked_step = None;
                for step in 0..=4 {
                    let (target_pos, _) = crate::ui::systems::simulation::loopback::handlers::movement::resolve_movement_position(
                        ball.cell_index as u32,
                        step,
                        ball.direction,
                        &preset,
                    );
                    if target_pos == cell_index {
                        clicked_step = Some(step);
                        break;
                    }
                }
                
                if let Some(step) = clicked_step {
                    events.send(ClientActionRequest(ClientAction::ChooseBananaSlide {
                        step_count: step,
                    }));
                }
            }
        }
    } else if let Some(card_type) = selected_card.0 {
        events.send(ClientActionRequest(ClientAction::DraftCard {
            card_type,
            cell_index,
        }));
        selected_card.0 = None; // Clear selection after drafting
    }
}
