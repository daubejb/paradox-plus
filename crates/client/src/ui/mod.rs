pub mod components;
pub mod layout;
pub mod systems;

use bevy::prelude::*;
use components::ClientScreenState;

pub struct ClientUiPlugin;

impl Plugin for ClientUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<systems::simulation::OfflineServerState>()
            .init_resource::<components::GameSettings>()
            .init_state::<ClientScreenState>()
            .add_systems(Startup, layout::spawn_ui_layout)
            .add_systems(
                OnEnter(ClientScreenState::Landing),
                systems::show_landing_screen_system,
            )
            .add_systems(
                OnEnter(ClientScreenState::SoloSetup),
                systems::show_setup_screen_system,
            )
            .add_systems(
                OnEnter(ClientScreenState::Gameplay),
                systems::show_gameplay_screen_system,
            )
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
                    systems::handle_landing_button_clicks,
                    systems::handle_gameplay_exit,
                    systems::handle_setup_button_clicks,
                    systems::handle_nickname_keyboard_input,
                    systems::update_setup_screen_ui,
                    systems::simulation::update_board_cell_visuals,
                    systems::simulation::update_ui_elements_system,
                    systems::simulation::rebuild_board_on_hole_change_system
                        .run_if(resource_changed::<components::CurrentHole>)
                        .after(crate::replication::sync_state_from_server),
                ),
            );
    }
}

