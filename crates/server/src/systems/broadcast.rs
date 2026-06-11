use bevy::prelude::*;
use std::net::SocketAddr;
use protocol::messages::ServerUpdate;
use heapless::Vec as HVec;
use super::fsm::ServerGameState;
use super::validation::NetworkSerializationBuffer;

#[derive(Resource)]
pub struct ServerActionSender {
    pub tx: tokio::sync::mpsc::Sender<(SocketAddr, Vec<u8>)>,
    pub clients: Vec<SocketAddr>,
}

/// Broadcasts FSM state changes to all connected clients using postcard serialization.
pub fn broadcast_state_sync_system(
    game_state: Res<ServerGameState>,
    sender: Res<ServerActionSender>,
    mut serialize_buf: ResMut<NetworkSerializationBuffer>,
) {
    if game_state.is_changed() {
        let update = ServerUpdate::StateSync {
            sequence: game_state.sequence,
            game_state: game_state.state,
            active_player_id: game_state.active_player_id,
            current_hole: game_state.current_hole,
            player_positions: HVec::new(),
            player_scores: HVec::new(),
            placed_wagers: HVec::new(),
        };

        // Bounded, zero-heap serialization in hot transmission loops
        if let Ok(serialized) = postcard::to_slice(&update, &mut serialize_buf.buffer) {
            let data = serialized.to_vec();
            for client in &sender.clients {
                let _ = sender.tx.try_send((*client, data.clone()));
            }
        }
    }
}
