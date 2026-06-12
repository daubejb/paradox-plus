use crate::ai::mdp_state::MdpState;
use crate::ai::mdp_solver::landing::{is_ob_or_occupied, resolve_landing_simple};
use protocol::terrain::{ActiveCourseTrack, TerrainType};
use protocol::physics::{MovementDirection, SlideTracker};
use protocol::messages::CardType;
use fixed::types::I32F32;

/// Resolves a single procedural physics step.
pub fn resolve_physics_step(
    start_state: MdpState,
    roll: u8,
    course: &ActiveCourseTrack,
    placed_wagers: &[protocol::messages::WagerToken],
    active_player_id: u64,
    table: &[I32F32],
) -> (MdpState, I32F32) {
    let current_cell = start_state.cell_index;
    let mut direction = start_state.direction;
    let origin_cell = start_state.origin_cell;
    let mut triggered_wagers = start_state.triggered_wagers.clone();

    // 1. Escape check if starting from Bunker
    let start_terrain = course.cells.get(current_cell as usize).copied().unwrap_or(TerrainType::Fairway);
    if start_terrain == TerrainType::Bunker {
        if roll % 2 != 0 {
            // Escape failed: remains on bunker, shot stroke is 1
            return (start_state, I32F32::from_num(1));
        }
    }

    // 2. Move ball based on direction
    let mut target_cell;
    let total_cells = course.cells.len() as u16;

    // Scan course to find Green range
    let mut max_green_idx = 0u16;
    let mut min_green_idx = total_cells;
    for (i, cell_type) in course.cells.iter().enumerate() {
        if let TerrainType::Green(_) = cell_type {
            let idx = i as u16;
            if idx > max_green_idx {
                max_green_idx = idx;
            }
            if idx < min_green_idx {
                min_green_idx = idx;
            }
        }
    }

    match direction {
        MovementDirection::Forward => {
            target_cell = current_cell.saturating_add(roll as u16);
            if target_cell > max_green_idx {
                direction = MovementDirection::Reverse;
            }
            if target_cell >= total_cells {
                target_cell = total_cells.saturating_sub(1);
            }
        }
        MovementDirection::Reverse => {
            if current_cell < roll as u16 {
                target_cell = 1;
                direction = MovementDirection::Forward;
            } else {
                target_cell = current_cell.saturating_sub(roll as u16);
                if target_cell < min_green_idx {
                    direction = MovementDirection::Forward;
                }
                if target_cell < 1 {
                    target_cell = 1;
                    direction = MovementDirection::Forward;
                }
            }
        }
    }

    // 3. Resolve landing
    let mut final_cell = target_cell;
    let mut penalty = I32F32::ZERO;

    // Check if there is a wager token on target cell
    let mut terrain_override = None;
    for wager in placed_wagers {
        if wager.cell_index == final_cell as u32 {
            if !triggered_wagers.contains(&final_cell) {
                let _ = triggered_wagers.push(final_cell);
                match wager.card_type {
                    CardType::Shield => {
                        if wager.owner_id == active_player_id {
                            terrain_override = Some(TerrainType::Fairway);
                        } else {
                            penalty = penalty.saturating_add(I32F32::from_num(1));
                        }
                    }
                    CardType::Banana => {
                        if wager.owner_id == active_player_id {
                            // Own Banana Choice - bot chooses best option (0..4).
                            let mut best_state = None;
                            let mut best_cost = I32F32::MAX;
                            for k in 0..=4 {
                                let mut adv_cell;
                                let mut adv_direction = direction;
                                match adv_direction {
                                    MovementDirection::Forward => {
                                        adv_cell = final_cell.saturating_add(k as u16);
                                        if adv_cell > max_green_idx {
                                            adv_direction = MovementDirection::Reverse;
                                        }
                                        if adv_cell >= total_cells {
                                            adv_cell = total_cells.saturating_sub(1);
                                        }
                                    }
                                    MovementDirection::Reverse => {
                                        if final_cell < k as u16 {
                                            adv_cell = 1;
                                            adv_direction = MovementDirection::Forward;
                                        } else {
                                            adv_cell = final_cell.saturating_sub(k as u16);
                                            if adv_cell < min_green_idx {
                                                adv_direction = MovementDirection::Forward;
                                            }
                                            if adv_cell < 1 {
                                                adv_cell = 1;
                                                adv_direction = MovementDirection::Forward;
                                            }
                                        }
                                    }
                                }
                                let (opt_state, opt_reward) = resolve_landing_simple(
                                    adv_cell,
                                    adv_direction,
                                    origin_cell,
                                    course,
                                    placed_wagers,
                                    active_player_id,
                                    &mut triggered_wagers.clone(),
                                );
                                let state_idx = opt_state.to_index().unwrap_or(0);
                                let state_val = table.get(state_idx).copied().unwrap_or(I32F32::ZERO);
                                let total_cost = opt_reward.saturating_add(state_val);
                                if total_cost < best_cost {
                                    best_cost = total_cost;
                                    best_state = Some((opt_state, opt_reward));
                                }
                            }
                            if let Some((opt_state, opt_reward)) = best_state {
                                final_cell = opt_state.cell_index;
                                direction = opt_state.direction;
                                penalty = penalty.saturating_add(opt_reward);
                            }
                        } else {
                            // Opponent Banana - push back 4 spaces
                            let push_dir = match direction {
                                MovementDirection::Forward => -1,
                                MovementDirection::Reverse => 1,
                            };
                            let mut push_target = final_cell as i16 + push_dir * 4;
                            if push_target < 1 {
                                push_target = 1;
                            }
                            if push_target >= total_cells as i16 {
                                push_target = (total_cells - 1) as i16;
                            }
                            final_cell = push_target as u16;

                            let mut slide_tracker = SlideTracker::new();
                            let mut slide_failed = false;
                            while is_ob_or_occupied(final_cell, course, placed_wagers) {
                                let step = match direction {
                                    MovementDirection::Forward => 1,
                                    MovementDirection::Reverse => -1,
                                };
                                let next = final_cell as i16 + step;
                                if next < 1 {
                                    final_cell = 1;
                                    direction = MovementDirection::Forward;
                                    break;
                                }
                                if next >= total_cells as i16 {
                                    final_cell = total_cells - 1;
                                    break;
                                }
                                final_cell = next as u16;
                                if slide_tracker.record_and_check_cycle(final_cell as usize).is_err() {
                                    slide_failed = true;
                                    break;
                                }
                            }

                            if slide_failed {
                                final_cell = 1;
                                penalty = penalty.saturating_add(I32F32::from_num(2));
                                direction = MovementDirection::Forward;
                            }
                        }
                    }
                    CardType::GoldenDie => {
                        if wager.owner_id == active_player_id {
                            penalty = penalty.saturating_sub(I32F32::from_num(2));
                        } else {
                            penalty = penalty.saturating_add(I32F32::from_num(2));
                        }
                    }
                }
            }
            break;
        }
    }

    let terrain_type = if let Some(t) = terrain_override {
        t
    } else {
        course.cells.get(final_cell as usize).copied().unwrap_or(TerrainType::Fairway)
    };

    match terrain_type {
        TerrainType::Fairway | TerrainType::TeeBox | TerrainType::Rough | TerrainType::Bunker => {
            let next_state = MdpState::new(final_cell, direction, origin_cell, triggered_wagers);
            (next_state, I32F32::from_num(1).saturating_add(penalty))
        }
        TerrainType::Water => {
            let next_state = MdpState::new(final_cell, direction, origin_cell, triggered_wagers);
            (next_state, I32F32::from_num(2).saturating_add(penalty))
        }
        TerrainType::OutOfBounds => {
            let next_state = MdpState::new(origin_cell, direction, origin_cell, triggered_wagers);
            (next_state, I32F32::from_num(2).saturating_add(penalty))
        }
        TerrainType::Green(tier) => {
            let next_state = MdpState::new(final_cell, direction, origin_cell, triggered_wagers);
            let putting_penalty = match tier {
                0 => 0,
                1 => 1,
                2 => 2,
                _ => 3,
            };
            (next_state, I32F32::from_num(1 + putting_penalty).saturating_add(penalty))
        }
    }
}
