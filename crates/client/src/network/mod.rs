pub mod events;
pub mod resources;
pub mod systems;

pub use events::{ServerUpdateEvent, ClientActionRequest};
pub use resources::{NetworkSerializationBuffer, ClientActionSender, ServerUpdateReceiver, MAX_PACKET_SIZE};

use bevy::prelude::*;

pub struct ClientNetworkPlugin;

impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ServerUpdateEvent>()
            .add_event::<ClientActionRequest>()
            .init_resource::<NetworkSerializationBuffer>()
            .add_systems(PreUpdate, systems::poll_server_updates_system)
            .add_systems(PostUpdate, systems::send_client_actions_system);
    }
}
