use super::super::state::OfflineServerState;
use protocol::messages::{ServerUpdate, GameStateEnum, CardType};
use protocol::terrain::{ActiveCourseTrack, TerrainType};
use super::movement;
use super::terrain;
use heapless::Vec as HVec;
use rand::Rng;

fn send_state_sync(state: &mut OfflineServerState, mut updates: Vec<ServerUpdate>) -> Vec<ServerUpdate> {
    let mut player_positions = HVec::new();
    player_positions.push(state.player_position).unwrap();
    let mut player_scores = HVec::new();
    player_scores.push(state.build_scorecard()).unwrap();

    let mut wagers = HVec::new();
    for w in &state.placed_wagers {
        let _ = wagers.push(w.clone());
    }

    updates.push(ServerUpdate::StateSync {
        sequence: state.sequence,
        game_state: state.game_state,
        active_player_id: state.active_player_id,
        current_hole: state.current_hole,
        player_positions,
        player_scores,
        placed_wagers: wagers,
    });

    updates
}

pub fn handle_choose_banana_slide(
    state: &mut OfflineServerState,
    step_count: u8,
    course: &ActiveCourseTrack,
) -> Vec<ServerUpdate> {
    let mut updates = Vec::new();
    state.cards_earned_this_hole.clear();

    if state.game_state != GameStateEnum::BananaChoice {
        return send_state_sync(state, updates);
    }

    let step_count = step_count.min(4);
    let current_pos = state.player_position;

    // 1. Resolve position with direction-aware movement
    let (target_pos, dir) = movement::resolve_movement_position(
        current_pos,
        step_count,
        state.direction,
        course,
    );
    let mut final_pos = target_pos;
    let next_dir = dir;

    // Check if there is a wager token on target cell
    let mut has_shield = false;
    let mut trigger_golden_die = false;
    let mut trigger_banana = false;

    if let Some(wager) = state.placed_wagers.iter().find(|w| w.cell_index == final_pos) {
        match wager.card_type {
            CardType::Shield => {
                has_shield = true;
            }
            CardType::Banana => {
                trigger_banana = true;
            }
            CardType::GoldenDie => {
                trigger_golden_die = true;
            }
        }
    }

    // 2. Resolve terrain landing rules (using Shield override if present)
    let actual_terrain = course.cells.get(final_pos as usize).copied().unwrap_or(TerrainType::Fairway);
    let target_terrain = if has_shield {
        TerrainType::Fairway
    } else {
        actual_terrain
    };

    let landing_res = terrain::resolve_landing(final_pos as u16, current_pos as u16, target_terrain);
    final_pos = landing_res.final_cell as u32;
    let penalty_strokes = landing_res.penalty_strokes;
    let completed_hole = landing_res.completed_hole;

    // Update local state
    state.player_position = final_pos;
    state.direction = next_dir;
    state.strokes = state.strokes.saturating_add(penalty_strokes as u32); // 0 shot strokes added

    // Trigger nested Golden Die
    if trigger_golden_die {
        state.strokes = state.strokes.saturating_sub(2);
        if state.inventory.len() < 16 {
            state.inventory.push(2);
        }
        updates.push(ServerUpdate::AlertTriggered {
            alert_message: heapless::String::try_from("Triggered Golden Die! -2 Strokes, earned Golden Die card.").unwrap(),
        });
    }

    // Trigger nested Shield card draw
    if has_shield {
        let mut rng = rand::thread_rng();
        let draw = rng.gen_range(0..=2);
        if state.inventory.len() < 16 {
            state.inventory.push(draw);
        }
        let card_name = match draw {
            0 => "Shield",
            1 => "Banana",
            _ => "Golden Die",
        };
        updates.push(ServerUpdate::AlertTriggered {
            alert_message: heapless::String::try_from(format!("Shield triggered! Drew {}.", card_name).as_str()).unwrap(),
        });
    }

    // Transition state
    if completed_hole {
        state.game_state = GameStateEnum::HoleCompleted;
        state.hole_completed_timer_ms = Some(0);

        let strokes_u16 = u16::try_from(state.strokes).unwrap_or(u16::MAX);
        if state.strokes_per_hole.push(strokes_u16).is_err() {
            bevy::log::error!(
                "Failed to record strokes for hole {}: Scorecard capacity exceeded",
                state.strokes_per_hole.len() + 1
            );
        }

        // Scorecard par card earning
        let score_relative_to_par = state.strokes as i32 - course.par as i32;
        let mut rng = rand::thread_rng();
        let mut earned_cards = Vec::new();
        if score_relative_to_par <= -3 || state.strokes == 1 {
            earned_cards.push(2);
        } else if score_relative_to_par == -2 {
            let card = if rng.gen_bool(0.5) { 1 } else { 2 };
            earned_cards.push(card);
        } else if score_relative_to_par == -1 {
            let r = rng.gen_range(0..3);
            earned_cards.push(r);
        }
        for card in earned_cards {
            if state.inventory.len() < 16 {
                state.inventory.push(card);
            }
            let _ = state.cards_earned_this_hole.push(card);
        }
    } else if trigger_banana {
        // Nested banana trigger! State remains BananaChoice
        let mut rng = rand::thread_rng();
        let draw = rng.gen_range(1..=2);
        if state.inventory.len() < 16 {
            state.inventory.push(draw);
        }
        let card_name = if draw == 1 { "Banana" } else { "Golden Die" };
        updates.push(ServerUpdate::AlertTriggered {
            alert_message: heapless::String::try_from(format!("Banana triggered! Drew {}, slide 0-4.", card_name).as_str()).unwrap(),
        });
    } else {
        state.game_state = GameStateEnum::AwaitingTurn;
    }

    state.sequence += 1;
    send_state_sync(state, updates)
}
