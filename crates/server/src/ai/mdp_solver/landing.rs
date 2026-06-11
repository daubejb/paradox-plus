use crate::ai::mdp_state::MdpState;
use protocol::terrain::{ActiveCourseTrack, TerrainType};
use protocol::physics::MovementDirection;
use fixed::types::I32F32;

/// Helper to check if space is OB or occupied by any wager token.
pub fn is_ob_or_occupied(
    cell: u16,
    course: &ActiveCourseTrack,
    placed_wagers: &[protocol::messages::WagerToken],
) -> bool {
    if let Some(&terrain) = course.cells.get(cell as usize) {
        if terrain == TerrainType::OutOfBounds {
            return true;
        }
    }
    for wager in placed_wagers {
        if wager.cell_index == cell as u32 {
            return true;
        }
    }
    false
}

/// Resolves standard landing rules (hazards and simple wager tokens).
pub fn resolve_landing_simple(
    cell: u16,
    direction: MovementDirection,
    origin_cell: u16,
    course: &ActiveCourseTrack,
    placed_wagers: &[protocol::messages::WagerToken],
    active_player_id: u64,
    triggered_wagers: &mut heapless::Vec<u16, 4>,
) -> (MdpState, I32F32) {
    let mut penalty = I32F32::ZERO;
    let mut terrain_override = None;
    for wager in placed_wagers {
        if wager.cell_index == cell as u32 {
            if !triggered_wagers.contains(&cell) {
                let _ = triggered_wagers.push(cell);
                match wager.card_type {
                    0 => {
                        if wager.owner_id == active_player_id {
                            terrain_override = Some(TerrainType::Fairway);
                        } else {
                            penalty = penalty.saturating_add(I32F32::from_num(1));
                        }
                    }
                    2 => {
                        if wager.owner_id == active_player_id {
                            penalty = penalty.saturating_sub(I32F32::from_num(2));
                        } else {
                            penalty = penalty.saturating_add(I32F32::from_num(2));
                        }
                    }
                    _ => {}
                }
            }
            break;
        }
    }

    let terrain = if let Some(t) = terrain_override {
        t
    } else {
        course.cells.get(cell as usize).copied().unwrap_or(TerrainType::Fairway)
    };

    match terrain {
        TerrainType::Fairway | TerrainType::TeeBox | TerrainType::Rough | TerrainType::Bunker => {
            let next_state = MdpState::new(cell, direction, origin_cell, triggered_wagers.clone());
            (next_state, penalty)
        }
        TerrainType::Water => {
            let next_state = MdpState::new(cell, direction, origin_cell, triggered_wagers.clone());
            (next_state, I32F32::from_num(1).saturating_add(penalty))
        }
        TerrainType::OutOfBounds => {
            let next_state = MdpState::new(origin_cell, direction, origin_cell, triggered_wagers.clone());
            (next_state, I32F32::from_num(1).saturating_add(penalty))
        }
        TerrainType::Green(tier) => {
            let next_state = MdpState::new(cell, direction, origin_cell, triggered_wagers.clone());
            let putting_penalty = match tier {
                0 => 0,
                1 => 1,
                2 => 2,
                _ => 3,
            };
            (next_state, I32F32::from_num(putting_penalty).saturating_add(penalty))
        }
    }
}
