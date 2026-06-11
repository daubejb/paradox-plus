use bevy::prelude::*;
use std::sync::mpsc::{Receiver, Sender};
use protocol::messages::{ClientAction, ServerUpdate, GameStateEnum, Scorecard};
use protocol::terrain::presets::get_course_preset;
use heapless::Vec as HVec;

pub mod state;
pub mod handlers;

use state::OfflineServerState;

#[derive(Resource)]
pub struct LocalServerChannels {
    pub action_rx: std::sync::Mutex<Receiver<Vec<u8>>>,
    pub update_tx: Sender<Vec<u8>>,
    pub send_buf: std::sync::Mutex<Vec<u8>>,
}

pub fn local_offline_server_system(
    channels: Res<LocalServerChannels>,
    mut state: ResMut<OfflineServerState>,
) {
    let rx = match channels.action_rx.lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };

    let mut send_buf = match channels.send_buf.lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };

    let course = match get_course_preset("green", state.current_hole) {
        Some(c) => c,
        None => return,
    };

    let mut count = 0;
    while let Ok(raw_payload) = rx.try_recv() {
        if count >= 10 {
            break;
        }
        count += 1;

        if let Ok(action) = postcard::from_bytes::<ClientAction>(&raw_payload) {
            let updates = handlers::handle_action(&mut state, &action, &course);
            for update in updates {
                send_buf.resize(65536, 0);
                if let Ok(serialized) = postcard::to_slice(&update, &mut *send_buf) {
                    let len = serialized.len();
                    let bytes = send_buf[..len].to_vec();
                    let _ = channels.update_tx.send(bytes);
                }
            }
        }
    }
}

pub fn trigger_initial_state_sync(
    mut state: ResMut<OfflineServerState>,
    channels: Res<LocalServerChannels>,
) {
    if state.is_initialized {
        return;
    }
    state.is_initialized = true;

    let mut player_positions = HVec::new();
    player_positions.push(state.player_position).unwrap();
    let mut player_scores = HVec::new();
    player_scores.push(Scorecard {
        running_strokes: 0,
        total_strokes: 0,
        earned_cards: HVec::new(),
    }).unwrap();

    let initial_update = ServerUpdate::StateSync {
        sequence: state.sequence,
        game_state: state.game_state,
        active_player_id: state.active_player_id,
        current_hole: state.current_hole,
        player_positions,
        player_scores,
        placed_wagers: HVec::new(),
    };

    let mut buf = vec![0u8; 65536];
    if let Ok(serialized) = postcard::to_slice(&initial_update, &mut buf) {
        let len = serialized.len();
        let bytes = buf[..len].to_vec();
        let _ = channels.update_tx.send(bytes);
    }
}
