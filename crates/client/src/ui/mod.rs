pub mod components;
pub mod layout;
pub mod systems;

use bevy::prelude::*;

pub struct ClientUiPlugin;

impl Plugin for ClientUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, layout::spawn_ui_layout)
            .add_systems(
                Update,
                (
                    systems::handle_button_hover,
                    systems::handle_roll_buttons,
                    systems::handle_wager_card_buttons,
                ),
            );
    }
}
