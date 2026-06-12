use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::ui::components::{BoardCellNode, SelectedWagerCard, ClientScreenState, CursorPositionOverride};
use crate::ui::systems::simulation::board::camera::BoardCameraNode;
use crate::network::events::ClientActionRequest;
use protocol::messages::ClientAction;

/// Raycasts cursor position to detect clicks on 2D Sprite cells, dispatching wager placement actions.
pub fn handle_board_clicks_system(
    screen_state: Res<State<ClientScreenState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<BoardCameraNode>>,
    cell_query: Query<(&GlobalTransform, &BoardCellNode)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut selected_card: ResMut<SelectedWagerCard>,
    mut events: EventWriter<ClientActionRequest>,
    cursor_pos_override: Option<Res<CursorPositionOverride>>,
) {
    if *screen_state.get() != ClientScreenState::Gameplay {
        return;
    }

    let card_type = match selected_card.0 {
        Some(c) => c,
        None => return,
    };

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

    if let Some(cursor_position) = cursor_pos {
        let world_position = if camera.logical_viewport_size().is_none() {
            Some(cursor_position)
        } else {
            camera.viewport_to_world_2d(camera_transform, cursor_position)
        };

        if let Some(world_position) = world_position {
            for (global_transform, cell) in cell_query.iter() {
                let cell_pos = global_transform.translation().xy();
                
                // Each cell sprite is 42x42. Radial distance check <= 22.0 is extremely robust and rotation-independent.
                if world_position.distance(cell_pos) <= 22.0 {
                    events.send(ClientActionRequest(ClientAction::DraftCard {
                        card_type,
                        cell_index: cell.index,
                    }));
                    selected_card.0 = None; // Clear selection after drafting
                    break;
                }
            }
        }
    }
}
