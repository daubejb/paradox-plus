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
    time: Res<Time>,
) {
    if state.game_state == GameStateEnum::HoleCompleted {
        let delta_ms = time.delta().as_millis() as u32;
        let timer = state.hole_completed_timer_ms.get_or_insert(0);
        *timer = timer.saturating_add(delta_ms);

        if *timer >= 3000 {
            state.hole_completed_timer_ms = None;
            state.current_hole = state.current_hole.saturating_add(1);

            if state.current_hole > 18 {
                state.game_state = GameStateEnum::MatchCompleted;
            } else {
                state.player_position = 0;
                state.strokes = 0;
                state.direction = protocol::physics::MovementDirection::Forward;
                state.placed_wagers.clear();
                state.cards_earned_this_hole.clear();
                if state.is_wager_mode && !state.inventory.is_empty() {
                    state.game_state = GameStateEnum::MarkerPlacement;
                } else {
                    state.game_state = GameStateEnum::AwaitingTurn;
                }
            }
            state.sequence = state.sequence.saturating_add(1);

            let mut player_positions = HVec::new();
            player_positions.push(state.player_position).unwrap();
            let mut player_scores = HVec::new();
            let mut hand = HVec::new();
            for &c in &state.inventory {
                let _ = hand.push(c);
            }
            player_scores.push(Scorecard {
                running_strokes: state.strokes as u16,
                total_strokes: state.strokes as u16,
                earned_cards: hand,
                cards_earned_this_hole: HVec::new(),
            }).unwrap();

            let mut wagers = HVec::new();
            for w in &state.placed_wagers {
                let _ = wagers.push(w.clone());
            }

            let update = ServerUpdate::StateSync {
                sequence: state.sequence,
                game_state: state.game_state,
                active_player_id: state.active_player_id,
                current_hole: state.current_hole,
                player_positions,
                player_scores,
                placed_wagers: wagers,
            };

            let mut send_buf = match channels.send_buf.lock() {
                Ok(guard) => guard,
                Err(_) => return,
            };
            send_buf.resize(65536, 0);
            if let Ok(serialized) = postcard::to_slice(&update, &mut *send_buf) {
                let len = serialized.len();
                let bytes = send_buf[..len].to_vec();
                let _ = channels.update_tx.send(bytes);
            }
        }
    } else {
        state.hole_completed_timer_ms = None;
    }

    let rx = match channels.action_rx.lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };

    let mut send_buf = match channels.send_buf.lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };

    let course = match get_course_preset(&state.course, state.current_hole) {
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
            if let ClientAction::StartPractice { nickname, course: course_name, is_wager_mode } = &action {
                *state = OfflineServerState::default();
                state.course = course_name.to_string();
                state.is_wager_mode = *is_wager_mode;
                state.player_name = nickname.to_string();
                state.is_initialized = true;
                
                let mut player_positions = HVec::new();
                player_positions.push(state.player_position).unwrap();
                let mut player_scores = HVec::new();
                player_scores.push(Scorecard {
                    running_strokes: 0,
                    total_strokes: 0,
                    earned_cards: HVec::new(),
                    cards_earned_this_hole: HVec::new(),
                }).unwrap();

                let update = ServerUpdate::StateSync {
                    sequence: state.sequence,
                    game_state: state.game_state,
                    active_player_id: state.active_player_id,
                    current_hole: state.current_hole,
                    player_positions,
                    player_scores,
                    placed_wagers: HVec::new(),
                };

                send_buf.resize(65536, 0);
                if let Ok(serialized) = postcard::to_slice(&update, &mut *send_buf) {
                    let len = serialized.len();
                    let bytes = send_buf[..len].to_vec();
                    let _ = channels.update_tx.send(bytes);
                }
                continue;
            }

            if action == ClientAction::LeaveRoom {
                *state = OfflineServerState::default();
                
                let mut player_positions = HVec::new();
                player_positions.push(state.player_position).unwrap();
                let mut player_scores = HVec::new();
                player_scores.push(Scorecard {
                    running_strokes: 0,
                    total_strokes: 0,
                    earned_cards: HVec::new(),
                    cards_earned_this_hole: HVec::new(),
                }).unwrap();

                let update = ServerUpdate::StateSync {
                    sequence: state.sequence,
                    game_state: state.game_state,
                    active_player_id: state.active_player_id,
                    current_hole: state.current_hole,
                    player_positions,
                    player_scores,
                    placed_wagers: HVec::new(),
                };

                send_buf.resize(65536, 0);
                if let Ok(serialized) = postcard::to_slice(&update, &mut *send_buf) {
                    let len = serialized.len();
                    let bytes = send_buf[..len].to_vec();
                    let _ = channels.update_tx.send(bytes);
                }
                continue;
            }

            if !protocol::fsm::is_valid_action(&action, state.game_state) {
                continue;
            }

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
