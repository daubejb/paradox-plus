pub mod components;
pub mod layout;
pub mod systems;

use bevy::prelude::*;

pub struct ClientUiPlugin;

impl Plugin for ClientUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<systems::simulation::OfflineServerState>()
            .add_systems(Startup, (
                layout::spawn_ui_layout,
                systems::simulation::trigger_initial_state_sync,
            ))
            .add_systems(
                PreUpdate,
                systems::simulation::local_offline_server_system,
            )
            .add_systems(
                Update,
                (
                    systems::handle_button_hover,
                    systems::handle_roll_buttons,
                    systems::handle_wager_card_buttons,
                    systems::simulation::update_board_cell_visuals,
                    systems::simulation::update_ui_elements_system,
                    systems::simulation::rebuild_board_on_hole_change_system
                        .run_if(resource_changed::<components::CurrentHole>)
                        .after(crate::replication::sync_state_from_server),
                ),
            );
    }
}
