use bevy::prelude::*;
use protocol::messages::{ClientAction, MAX_PACKET_SIZE};

#[derive(Clone, Debug)]
pub struct ClientActionMessage {
    pub player_id: u64,
    pub action: ClientAction,
}

#[derive(Resource)]
pub struct ServerActionReceiver {
    pub rx: tokio::sync::mpsc::Receiver<ClientActionMessage>,
}

#[derive(Resource)]
pub struct NetworkSerializationBuffer {
    pub buffer: Vec<u8>,
}

impl Default for NetworkSerializationBuffer {
    fn default() -> Self {
        Self {
            buffer: vec![0u8; MAX_PACKET_SIZE],
        }
    }
}

#[derive(Event, Debug, Clone)]
pub struct ClientActionEvent {
    pub player_id: u64,
    pub action: ClientAction,
}

/// Polls the server network receiver and dispatches events inside Bevy.
pub fn validate_actions_system(
    mut receiver: ResMut<ServerActionReceiver>,
    mut action_events: EventWriter<ClientActionEvent>,
) {
    while let Ok(msg) = receiver.rx.try_recv() {
        action_events.send(ClientActionEvent {
            player_id: msg.player_id,
            action: msg.action,
        });
    }
}
