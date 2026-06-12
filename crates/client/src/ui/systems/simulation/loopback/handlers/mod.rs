pub mod dice;
pub mod terrain;
pub mod movement;
pub mod wager;
pub mod banana;

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
            let mut final_pos = target_pos;
            let next_dir = dir;

            // Check if there is a wager token on target cell before resolving terrain
            let mut has_shield = false;
            let mut trigger_golden_die = false;
            let mut trigger_banana = false;

            if let Some(wager) = state.placed_wagers.iter().find(|w| w.cell_index == final_pos) {
                match wager.card_type {
                    0 => { // Guardian Shield
                        has_shield = true;
                    }
                    1 => { // Trickster Banana
                        trigger_banana = true;
                    }
                    2 => { // Golden Die
                        trigger_golden_die = true;
                    }
                    _ => {}
                }
            }

            // 3. Resolve landing terrain rules
            let actual_terrain = course.cells.get(final_pos as usize).copied().unwrap_or(TerrainType::Fairway);
            let target_terrain = if has_shield {
                TerrainType::Fairway
            } else {
                actual_terrain
            };

            let landing_res = terrain::resolve_landing(final_pos as u16, current_pos as u16, target_terrain);
            
            final_pos = landing_res.final_cell as u32;
            let shot_strokes = landing_res.shot_strokes;
            let penalty_strokes = landing_res.penalty_strokes;
            let completed_hole = landing_res.completed_hole;

            // Update local state
            state.player_position = final_pos;
            state.direction = next_dir;
            state.strokes += (shot_strokes + penalty_strokes) as u32;

            if trigger_golden_die {
                state.strokes = state.strokes.saturating_sub(2);
                if state.inventory.len() < 16 {
                    state.inventory.push(2);
                }
                updates.push(ServerUpdate::AlertTriggered {
                    alert_message: heapless::String::try_from("Triggered Golden Die! -2 Strokes, earned Golden Die card.").unwrap(),
                });
            }

            if has_shield {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let draw = rng.gen_range(0..3); // 0, 1, 2
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

            if trigger_banana {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let draw = rng.gen_range(1..=2); // 1 or 2 (Banana or Golden Die)
                if state.inventory.len() < 16 {
                    state.inventory.push(draw);
                }
                let card_name = if draw == 1 { "Banana" } else { "Golden Die" };
                updates.push(ServerUpdate::AlertTriggered {
                    alert_message: heapless::String::try_from(format!("Banana triggered! Drew {}, slide 0-4.", card_name).as_str()).unwrap(),
                });
            }

            if completed_hole && state.strokes == 0 {
                state.strokes = 1;
            }

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
                    if state.inventory.len() < 16 {
                        state.inventory.push(card);
                    }
                }
            }

            if completed_hole {
                state.game_state = GameStateEnum::HoleCompleted;
            } else if trigger_banana {
                state.game_state = GameStateEnum::BananaChoice;
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
            updates.extend(wager::handle_draft_card(state, *card_type, *cell_index, course));
        }
        ClientAction::SkipPlacement => {
            updates.extend(wager::handle_skip_placement(state));
        }
        ClientAction::ChooseBananaSlide { step_count } => {
            updates.extend(banana::handle_choose_banana_slide(state, *step_count, course));
        }
        _ => {}
    }

    updates
}
