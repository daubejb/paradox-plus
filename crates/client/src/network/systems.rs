use bevy::prelude::*;
use protocol::messages::ServerUpdate;
use crate::network::events::{ServerUpdateEvent, ClientActionRequest};
use crate::network::resources::{
    NetworkSerializationBuffer, ClientActionSender, ServerUpdateReceiver, MAX_PACKET_SIZE
};

pub fn poll_server_updates_system(
    receiver: ResMut<ServerUpdateReceiver>,
    mut event_writer: EventWriter<ServerUpdateEvent>,
    mut local_event_queue: Local<Vec<ServerUpdate>>,
) {
    local_event_queue.clear();
    if let Ok(rx) = receiver.receiver.lock() {
        while let Ok(raw_payload) = rx.try_recv() {
            if raw_payload.len() > MAX_PACKET_SIZE {
                warn!("Dropped oversized packet of length {}", raw_payload.len());
                continue;
            }

            // Bounded postcard deserialization with zero-panic protection
            match postcard::from_bytes::<ServerUpdate>(&raw_payload) {
                Ok(update) => {
                    local_event_queue.push(update);
                }
                Err(err) => {
                    error!("Protocol Deserialization Error: {:?}", err);
                }
            }
        }
    }

    for update in local_event_queue.iter() {
        event_writer.send(ServerUpdateEvent(update.clone()));
    }
}

pub fn send_client_actions_system(
    mut events: EventReader<ClientActionRequest>,
    mut serialization_buf: ResMut<NetworkSerializationBuffer>,
    sender: Res<ClientActionSender>,
) {
    for ev in events.read() {
        serialization_buf.buffer.clear();
        // Resize buffer to capacity safely to get a full mutable slice
        if serialization_buf.buffer.resize(MAX_PACKET_SIZE, 0).is_err() {
            error!("Failed to resize serialization buffer");
            continue;
        }

        // Serialize into the buffer on the heap
        match postcard::to_slice(&ev.0, &mut *serialization_buf.buffer) {
            Ok(serialized) => {
                let len = serialized.len();
                let bytes = serialization_buf.buffer[..len].to_vec();
                if let Err(err) = sender.sender.send(bytes) {
                    error!("Failed to send client action: {:?}", err);
                }
            }
            Err(err) => {
                error!("Failed to serialize client action: {:?}", err);
            }
        }
    }
}
