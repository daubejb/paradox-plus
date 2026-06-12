pub mod dice;
pub mod terrain;
pub mod movement;

use super::state::OfflineServerState;
use protocol::messages::{ClientAction, ServerUpdate, GameStateEnum, Scorecard};
use protocol::terrain::{ActiveCourseTrack, TerrainType};
use heapless::Vec as HVec;

pub fn handle_action(
    state: &mut OfflineServerState,
    action: &ClientAction,
    course: &ActiveCourseTrack,
) -> Vec<ServerUpdate> {
    let mut updates = Vec::new();

    match action {
        ClientAction::RollDice { dice_count } => {
            let dice_count = *dice_count;
            let current_pos = state.player_position;
            let mut final_pos;
            let next_dir;
            let shot_strokes;
            let penalty_strokes;
            let completed_hole;

            // 1. Bunker escape logic
            let current_terrain = course.cells.get(current_pos as usize).copied().unwrap_or(TerrainType::Fairway);
            let roll_sum = if current_terrain == TerrainType::Bunker {
                // Odd rolls fail escape, even rolls escape
                let r = dice::roll_single_die();
                updates.push(ServerUpdate::DiceRollOutcome {
                    roll_values: {
                        let mut v = HVec::new();
                        v.push(r).unwrap();
                        v
                    }
                });
                
                let _res = terrain::resolve_bunker(current_pos as u16, current_pos as u16, r);
                if r % 2 != 0 {
                    // Escape failed
                    updates.push(ServerUpdate::AlertTriggered {
                        alert_message: heapless::String::try_from("Bunker escape failed! Odd roll.").unwrap(),
                    });
                    state.strokes += 1;
                    state.sequence += 1;
                    
                    let mut player_positions = HVec::new();
                    player_positions.push(current_pos).unwrap();
                    let mut player_scores = HVec::new();
                    player_scores.push(Scorecard {
                        running_strokes: state.strokes as u16,
                        total_strokes: state.strokes as u16,
                        earned_cards: HVec::new(),
                    }).unwrap();

                    updates.push(ServerUpdate::StateSync {
                        sequence: state.sequence,
                        game_state: GameStateEnum::AwaitingTurn,
                        active_player_id: state.active_player_id,
                        current_hole: state.current_hole,
                        player_positions,
                        player_scores,
                        placed_wagers: HVec::new(),
                    });
                    return updates;
                }
                r
            } else {
                // Normal rolling
                let roll_sum = if dice_count == 2 {
                    let (d1, d2) = dice::roll_two_dice();
                    updates.push(ServerUpdate::DiceRollOutcome {
                        roll_values: {
                            let mut v = HVec::new();
                            v.push(d1).unwrap();
                            v.push(d2).unwrap();
                            v
                        }
                    });
                    d1 + d2
                } else {
                    let r = dice::roll_single_die();
                    updates.push(ServerUpdate::DiceRollOutcome {
                        roll_values: {
                            let mut v = HVec::new();
                            v.push(r).unwrap();
                            v
                        }
                    });
                    r
                };
                roll_sum
            };

            // 2. Resolve movement position (overshoot/undershoot/clamping)
            let (target_pos, dir) = movement::resolve_movement_position(
                current_pos,
                roll_sum,
                state.direction,
                course,
            );
            final_pos = target_pos;
            next_dir = dir;

            // 3. Resolve landing terrain rules
            let target_terrain = course.cells.get(final_pos as usize).copied().unwrap_or(TerrainType::Fairway);
            let landing_res = terrain::resolve_landing(final_pos as u16, current_pos as u16, target_terrain);
            
            final_pos = landing_res.final_cell as u32;
            shot_strokes = landing_res.shot_strokes;
            penalty_strokes = landing_res.penalty_strokes;
            completed_hole = landing_res.completed_hole;

            // Update local state
            state.player_position = final_pos;
            state.direction = next_dir;
            state.strokes += (shot_strokes + penalty_strokes) as u32;
            state.sequence += 1;

            let mut earned_cards: HVec<u8, 4> = HVec::new();
            if state.is_wager_mode && completed_hole {
                let score_relative_to_par = state.strokes as i32 - course.par as i32;
                use rand::Rng;
                let mut rng = rand::thread_rng();
                if score_relative_to_par <= -3 || state.strokes == 1 {
                    let _ = earned_cards.push(2);
                } else if score_relative_to_par == -2 {
                    let card = if rng.gen_bool(0.5) { 1 } else { 2 };
                    let _ = earned_cards.push(card);
                } else if score_relative_to_par == -1 {
                    let r = rng.gen_range(0..3);
                    let _ = earned_cards.push(r);
                }
                
                // Store cards in persistent state inventory for next hole placement
                for &card in &earned_cards {
                    state.inventory.push(card);
                }
            }

            if completed_hole {
                state.game_state = GameStateEnum::HoleCompleted;
            } else {
                state.game_state = GameStateEnum::AwaitingTurn;
            }

            // Create and push state sync update
            let mut player_positions = HVec::new();
            player_positions.push(final_pos).unwrap();
            let mut player_scores = HVec::new();
            
            let mut hand = HVec::new();
            for &c in &state.inventory {
                let _ = hand.push(c);
            }
            
            player_scores.push(Scorecard {
                running_strokes: state.strokes as u16,
                total_strokes: state.strokes as u16,
                earned_cards: hand,
            }).unwrap();

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
        }
        ClientAction::DraftCard { card_type, cell_index } => {
            let card_type = *card_type;
            let cell_index = *cell_index;

            if let Some(pos) = state.inventory.iter().position(|&c| c == card_type) {
                state.inventory.remove(pos);
                state.placed_wagers.push(protocol::messages::WagerToken {
                    card_type,
                    owner_id: state.active_player_id,
                    cell_index,
                });
                state.sequence += 1;

                if state.inventory.is_empty() {
                    state.game_state = GameStateEnum::AwaitingTurn;
                }

                let card_name = match card_type {
                    0 => "Guardian Shield",
                    1 => "Trickster Banana",
                    _ => "Golden Die",
                };
                updates.push(ServerUpdate::AlertTriggered {
                    alert_message: heapless::String::try_from(format!("Placed {} wager!", card_name).as_str()).unwrap(),
                });
            } else {
                updates.push(ServerUpdate::AlertTriggered {
                    alert_message: heapless::String::try_from("No card of that type in hand!").unwrap(),
                });
            }

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
            }).unwrap();

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
        }
        ClientAction::SkipPlacement => {
            state.game_state = GameStateEnum::AwaitingTurn;
            state.sequence += 1;

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
            }).unwrap();

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
        }
        _ => {}
    }

    updates
}
