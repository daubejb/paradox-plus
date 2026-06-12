use crate::ai::mdp_state::MdpState;
use crate::ai::mdp_solver::table::MdpSolverTable;
use crate::ai::mdp_solver::transitions::{get_transitions, TransitionOutcome};
use protocol::terrain::{ActiveCourseTrack, TerrainType};
use protocol::messages::CardType;
use fixed::types::I32F32;
use std::sync::atomic::{AtomicBool, Ordering};

const BELLMAN_THRESHOLD: I32F32 = I32F32::from_bits(42949673); // 0.01

/// Computes expected transition cost for a single outcome.
pub fn calculate_bellman_update(
    reward: I32F32,
    probability: I32F32,
    next_state_value: I32F32,
) -> I32F32 {
    let sum = reward.saturating_add(next_state_value);
    probability.saturating_mul(sum)
}

/// Initializes the MDP solver table values with the course heuristic.
pub fn initialize_table(
    table: &mut MdpSolverTable,
    course: &ActiveCourseTrack,
) {
    table.clear();
    // Find the Cup (g0) cell index
    let mut cup_idx = 0u16;
    for (i, cell_type) in course.cells.iter().enumerate() {
        if let TerrainType::Green(0) = cell_type {
            cup_idx = i as u16;
            break;
        }
    }

    let active_states = course.cells.len() * 2;
    for idx in 0..active_states {
        if let Some(state) = MdpState::from_index(idx) {
            let cell = state.cell_index;
            let terrain = course.cells.get(cell as usize).copied().unwrap_or(TerrainType::Fairway);
            match terrain {
                TerrainType::Green(tier) => {
                    let putting_penalty = match tier {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        _ => 3,
                    };
                    table.values[idx] = I32F32::from_num(putting_penalty);
                }
                _ => {
                    let dist = if cell > cup_idx {
                        cell.saturating_sub(cup_idx)
                    } else {
                        cup_idx.saturating_sub(cell)
                    };
                    let dist_fixed = I32F32::from_num(dist);
                    table.values[idx] = dist_fixed.saturating_div(I32F32::from_num(4)).saturating_add(I32F32::from_num(2));
                }
            }
        }
    }
}

/// Executes value iteration sweeps until convergence or cancellation.
/// Returns true if converged, false if cancelled or limit reached.
pub fn value_iteration_sweep(
    table: &mut MdpSolverTable,
    course: &ActiveCourseTrack,
    placed_wagers: &[protocol::messages::WagerToken],
    active_player_id: u64,
    cancel_flag: &AtomicBool,
) -> bool {
    let active_states = course.cells.len() * 2;
    if active_states == 0 {
        return true;
    }

    let mut transitions_buf = heapless::Vec::<TransitionOutcome, 16>::new();

    for _sweep in 0..150 {
        // Sweep boundary cancellation check
        if cancel_flag.load(Ordering::Relaxed) {
            return false;
        }

        let mut max_delta = I32F32::ZERO;

        for idx in 0..active_states {
            // Check cancellation every 1024 updates
            if (idx & 1023) == 0 && cancel_flag.load(Ordering::Relaxed) {
                return false;
            }

            let state = match MdpState::from_index(idx) {
                Some(s) => s,
                None => continue,
            };

            let cell = state.cell_index;
            let terrain = course.cells.get(cell as usize).copied().unwrap_or(TerrainType::Fairway);
            
            // Terminal Green states are skipped (they have fixed values)
            if let TerrainType::Green(_) = terrain {
                continue;
            }

            // Determine if 2 dice are allowed
            let on_rough = terrain == TerrainType::Rough;
            let has_own_shield = placed_wagers.iter().any(|w| {
                w.cell_index == cell as u32 && w.card_type == CardType::Shield && w.owner_id == active_player_id
            });
            let can_roll_2_dice = !on_rough || has_own_shield;

            // Evaluate 1 die expected value
            get_transitions(state.clone(), 1, course, placed_wagers, active_player_id, &table.values, &mut transitions_buf);
            let mut best_q = I32F32::ZERO;
            for outcome in &transitions_buf {
                let next_idx = outcome.next_state.to_index().unwrap_or(0);
                let next_val = table.values[next_idx];
                let update = calculate_bellman_update(outcome.reward, outcome.probability, next_val);
                best_q = best_q.saturating_add(update);
            }

            // Evaluate 2 dice expected value (if allowed)
            if can_roll_2_dice {
                get_transitions(state, 2, course, placed_wagers, active_player_id, &table.values, &mut transitions_buf);
                let mut q_2 = I32F32::ZERO;
                for outcome in &transitions_buf {
                    let next_idx = outcome.next_state.to_index().unwrap_or(0);
                    let next_val = table.values[next_idx];
                    let update = calculate_bellman_update(outcome.reward, outcome.probability, next_val);
                    q_2 = q_2.saturating_add(update);
                }
                if q_2 < best_q {
                    best_q = q_2;
                }
            }

            // Update value table and track delta
            let old_val = table.values[idx];
            let delta = if old_val > best_q {
                old_val.saturating_sub(best_q)
            } else {
                best_q.saturating_sub(old_val)
            };
            if delta > max_delta {
                max_delta = delta;
            }
            table.values[idx] = best_q;
        }

        // Convergence check
        if max_delta < BELLMAN_THRESHOLD {
            return true;
        }
    }

    true
}
