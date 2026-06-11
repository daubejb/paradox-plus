use crate::ai::mdp_state::MdpState;
use crate::ai::mdp_solver::physics::resolve_physics_step;
use protocol::terrain::ActiveCourseTrack;
use fixed::types::I32F32;

const PROB_1D6: I32F32 = I32F32::from_bits(715827883);
const PROB_2D6_BASE: I32F32 = I32F32::from_bits(119304647);

#[derive(Debug, Clone)]
pub struct TransitionOutcome {
    pub next_state: MdpState,
    pub probability: I32F32,
    pub reward: I32F32,
}

/// Returns the pre-calculated probability of rolling a sum under given dice count.
pub fn get_dice_probability(roll_sum: u8, dice_count: u8) -> I32F32 {
    if dice_count == 1 {
        if (1..=6).contains(&roll_sum) {
            PROB_1D6
        } else {
            I32F32::ZERO
        }
    } else {
        let multiplier = match roll_sum {
            2 | 12 => 1,
            3 | 11 => 2,
            4 | 10 => 3,
            5 | 9 => 4,
            6 | 8 => 5,
            7 => 6,
            _ => 0,
        };
        if multiplier > 0 {
            I32F32::from_bits(PROB_2D6_BASE.to_bits() * multiplier)
        } else {
            I32F32::ZERO
        }
    }
}

/// Populates a list of transition outcomes for a given action.
pub fn get_transitions(
    state: MdpState,
    dice_count: u8,
    course: &ActiveCourseTrack,
    placed_wagers: &[protocol::messages::WagerToken],
    active_player_id: u64,
    table: &[I32F32],
    out_transitions: &mut heapless::Vec<TransitionOutcome, 16>,
) {
    out_transitions.clear();
    let outcomes = if dice_count == 1 { 1..=6 } else { 2..=12 };
    for roll in outcomes {
        let prob = get_dice_probability(roll, dice_count);
        if prob == I32F32::ZERO {
            continue;
        }
        let (next_state, reward) = resolve_physics_step(state.clone(), roll, course, placed_wagers, active_player_id, table);
        let _ = out_transitions.push(TransitionOutcome {
            next_state,
            probability: prob,
            reward,
        });
    }
}
